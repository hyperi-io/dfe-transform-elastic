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

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("event.kind", Value::Array(vec![json!("alert")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "json_event_original_to_json_90c6475f",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("json.findings")
                    && event.get("json.findings").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.updatedAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.createdAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.description") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.accountId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has("json.accountId") {
                event.rename("json.accountId", "aws.guardduty.account_id")?;
            }

            if event.has("json.arn") {
                event.rename("json.arn", "aws.guardduty.arn")?;
            }

            if event.has("json.confidence") {
                event.rename("json.confidence", "aws.guardduty.confidence")?;
            }

            let _cond = { event.has_value("json.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.guardduty.created_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_createdAt_to_aws_guardduty_created_at_24472679",
                    )?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.description") {
                event.rename("json.description", "aws.guardduty.description")?;
            }

            if event.has("json.id") {
                event.rename("json.id", "aws.guardduty.id")?;
            }

            if event.has("json.partition") {
                event.rename("json.partition", "aws.guardduty.partition")?;
            }

            if event.has("json.region") {
                event.rename("json.region", "aws.guardduty.region")?;
            }

            if event.has("json.resource.accessKeyDetails.accessKeyId") {
                event.rename(
                    "json.resource.accessKeyDetails.accessKeyId",
                    "aws.guardduty.resource.access_key_details.accesskey_id",
                )?;
            }

            if event.has("json.resource.accessKeyDetails.userType") {
                event.rename(
                    "json.resource.accessKeyDetails.userType",
                    "aws.guardduty.resource.access_key_details.user.type",
                )?;
            }

            if event.has("json.resource.accessKeyDetails.principalId") {
                event.rename(
                    "json.resource.accessKeyDetails.principalId",
                    "aws.guardduty.resource.access_key_details.principal_id",
                )?;
            }

            let _cond =
                { event.has_value("aws.guardduty.resource.access_key_details.principal_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("aws.guardduty.resource.access_key_details.principal_id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.resource.accessKeyDetails.userName") {
                event.rename(
                    "json.resource.accessKeyDetails.userName",
                    "aws.guardduty.resource.access_key_details.user.name",
                )?;
            }

            let _cond = { event.has_value("aws.guardduty.resource.access_key_details.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("aws.guardduty.resource.access_key_details.user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.resource.containerDetails.containerRuntime") {
                event.rename(
                    "json.resource.containerDetails.containerRuntime",
                    "aws.guardduty.resource.container_details.container_runtime",
                )?;
            }

            if event.has("json.resource.containerDetails.id") {
                event.rename(
                    "json.resource.containerDetails.id",
                    "aws.guardduty.resource.container_details.id",
                )?;
            }

            if event.has("json.resource.containerDetails.image") {
                event.rename(
                    "json.resource.containerDetails.image",
                    "aws.guardduty.resource.container_details.image.value",
                )?;
            }

            if event.has("json.resource.containerDetails.imagePrefix") {
                event.rename(
                    "json.resource.containerDetails.imagePrefix",
                    "aws.guardduty.resource.container_details.image.prefix",
                )?;
            }

            if event.has("json.resource.containerDetails.name") {
                event.rename(
                    "json.resource.containerDetails.name",
                    "aws.guardduty.resource.container_details.name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.resource.containerDetails.securityContext.privileged") {
                    if let Some(val) =
                        event.get("json.resource.containerDetails.securityContext.privileged")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.resource.containerDetails.securityContext.privileged"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.resource.container_details.security_context.privileged",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_resource_containerDetails_securityContext_privileged_to_aws_guardduty_resource_container_details_security_context_privileged_8d0f193a")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.container_details.security_context.privileged")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.security_context.privileged", v)?;
            }

            let _cond = {
                event
                    .get("json.resource.containerDetails.volumeMounts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.containerDetails.volumeMounts",
                    |event| {
                        if event.has("_ingest._value.mountPath") {
                            event
                                .rename("_ingest._value.mountPath", "_ingest._value.mount_path")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.containerDetails.volumeMounts") {
                event.rename(
                    "json.resource.containerDetails.volumeMounts",
                    "aws.guardduty.resource.container_details.volume_mounts",
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.scannedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.deviceName") {
                            event.rename(
                                "_ingest._value.deviceName",
                                "_ingest._value.device_name",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.scannedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.encryptionType") {
                            event.rename(
                                "_ingest._value.encryptionType",
                                "_ingest._value.encryption_type",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.scannedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.kmsKeyArn") {
                            event
                                .rename("_ingest._value.kmsKeyArn", "_ingest._value.kmskey_arn")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.scannedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.snapshotArn") {
                            event.rename(
                                "_ingest._value.snapshotArn",
                                "_ingest._value.snapshot_arn",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.scannedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.volumeArn") {
                            event
                                .rename("_ingest._value.volumeArn", "_ingest._value.volume.arn")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.scannedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.volumeSizeInGB") {
                                if let Some(val) = event.get("_ingest._value.volumeSizeInGB") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.volumeSizeInGB".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.volume.size_in_gb", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.scannedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.volumeType") {
                            event.rename(
                                "_ingest._value.volumeType",
                                "_ingest._value.volume.type",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.scannedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    |event| {
                        event.remove("_ingest._value.volumeSizeInGB");
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.ebsVolumeDetails.scannedVolumeDetails") {
                event.rename(
                    "json.resource.ebsVolumeDetails.scannedVolumeDetails",
                    "aws.guardduty.resource.ebs_volume_details.scanned_volume_details",
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.skippedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.deviceName") {
                            event.rename(
                                "_ingest._value.deviceName",
                                "_ingest._value.device_name",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.skippedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.encryptionType") {
                            event.rename(
                                "_ingest._value.encryptionType",
                                "_ingest._value.encryption_type",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.skippedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.kmsKeyArn") {
                            event
                                .rename("_ingest._value.kmsKeyArn", "_ingest._value.kmskey_arn")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.skippedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.snapshotArn") {
                            event.rename(
                                "_ingest._value.snapshotArn",
                                "_ingest._value.snapshot_arn",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.skippedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.volumeArn") {
                            event
                                .rename("_ingest._value.volumeArn", "_ingest._value.volume.arn")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.skippedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.volumeSizeInGB") {
                                if let Some(val) = event.get("_ingest._value.volumeSizeInGB") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.volumeSizeInGB".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.volume.size_in_gb", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.skippedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    |event| {
                        if event.has("_ingest._value.volumeType") {
                            event.rename(
                                "_ingest._value.volumeType",
                                "_ingest._value.volume.type",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ebsVolumeDetails.skippedVolumeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    |event| {
                        event.remove("_ingest._value.volumeSizeInGB");
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.ebsVolumeDetails.skippedVolumeDetails") {
                event.rename(
                    "json.resource.ebsVolumeDetails.skippedVolumeDetails",
                    "aws.guardduty.resource.ebs_volume_details.skipped_volume_details",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.resource.ecsClusterDetails.activeServicesCount") {
                    if let Some(val) =
                        event.get("json.resource.ecsClusterDetails.activeServicesCount")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.resource.ecsClusterDetails.activeServicesCount".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.resource.ecs_cluster_details.active_services_count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_resource_ecsClusterDetails_activeServicesCount_to_aws_guardduty_resource_ecs_cluster_details_active_services_count_f9b99b86")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.resource.ecsClusterDetails.arn") {
                event.rename(
                    "json.resource.ecsClusterDetails.arn",
                    "aws.guardduty.resource.ecs_cluster_details.arn",
                )?;
            }

            if event.has("json.resource.ecsClusterDetails.name") {
                event.rename(
                    "json.resource.ecsClusterDetails.name",
                    "aws.guardduty.resource.ecs_cluster_details.name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.resource.ecsClusterDetails.registeredContainerInstancesCount")
                {
                    if let Some(val) = event
                        .get("json.resource.ecsClusterDetails.registeredContainerInstancesCount")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.resource.ecsClusterDetails.registeredContainerInstancesCount".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.resource.ecs_cluster_details.registered_container_instances_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_resource_ecsClusterDetails_registeredContainerInstancesCount_to_aws_guardduty_resource_ecs_cluster_details_registered_container_instances_count_1ce7424b")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.resource.ecsClusterDetails.runningTasksCount") {
                    if let Some(val) =
                        event.get("json.resource.ecsClusterDetails.runningTasksCount")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.resource.ecsClusterDetails.runningTasksCount".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.resource.ecs_cluster_details.running_tasks_count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_resource_ecsClusterDetails_runningTasksCount_to_aws_guardduty_resource_ecs_cluster_details_running_tasks_count_71dc5e26")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.resource.ecsClusterDetails.status") {
                event.rename(
                    "json.resource.ecsClusterDetails.status",
                    "aws.guardduty.resource.ecs_cluster_details.status",
                )?;
            }

            if event.has("json.resource.ecsClusterDetails.tags") {
                event.rename(
                    "json.resource.ecsClusterDetails.tags",
                    "aws.guardduty.resource.ecs_cluster_details.tags",
                )?;
            }

            if event.has("json.resource.ecsClusterDetails.taskDetails.arn") {
                event.rename(
                    "json.resource.ecsClusterDetails.taskDetails.arn",
                    "aws.guardduty.resource.ecs_cluster_details.task_details.arn",
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ecsClusterDetails.taskDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ecsClusterDetails.taskDetails.containers",
                    |event| {
                        if event.has("_ingest._value.containerRuntime") {
                            event.rename(
                                "_ingest._value.containerRuntime",
                                "_ingest._value.container_runtime",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ecsClusterDetails.taskDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ecsClusterDetails.taskDetails.containers",
                    |event| {
                        if event.has("_ingest._value.image") {
                            event.rename("_ingest._value.image", "_ingest._value.image.value")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ecsClusterDetails.taskDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ecsClusterDetails.taskDetails.containers",
                    |event| {
                        if event.has("_ingest._value.imagePrefix") {
                            event.rename(
                                "_ingest._value.imagePrefix",
                                "_ingest._value.image.prefix",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ecsClusterDetails.taskDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ecsClusterDetails.taskDetails.containers",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.securityContext.privileged") {
                                if let Some(val) =
                                    event.get("_ingest._value.securityContext.privileged")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.securityContext.privileged"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.security_context.privileged",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ecsClusterDetails.taskDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ecsClusterDetails.taskDetails.containers",
                    |event| {
                        if event.has_value("_ingest._value.volumeMounts") {
                            foreach_array(event, "_ingest._value.volumeMounts", |event| {
                                if event.has("_ingest._value.mountPath") {
                                    event.rename(
                                        "_ingest._value.mountPath",
                                        "_ingest._value.mount_path",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ecsClusterDetails.taskDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ecsClusterDetails.taskDetails.containers",
                    |event| {
                        if event.has("_ingest._value.volumeMounts") {
                            event.rename(
                                "_ingest._value.volumeMounts",
                                "_ingest._value.volume_mounts",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ecsClusterDetails.taskDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ecsClusterDetails.taskDetails.containers",
                    |event| {
                        event.remove("_ingest._value.securityContext.privileged");
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.ecsClusterDetails.taskDetails.containers") {
                event.rename(
                    "json.resource.ecsClusterDetails.taskDetails.containers",
                    "aws.guardduty.resource.ecs_cluster_details.task_details.containers",
                )?;
            }

            if event.has("json.resource.ecsClusterDetails.taskDetails.definitionArn") {
                event.rename(
                    "json.resource.ecsClusterDetails.taskDetails.definitionArn",
                    "aws.guardduty.resource.ecs_cluster_details.task_details.definitionarn",
                )?;
            }

            if event.has("json.resource.ecsClusterDetails.taskDetails.group") {
                event.rename(
                    "json.resource.ecsClusterDetails.taskDetails.group",
                    "aws.guardduty.resource.ecs_cluster_details.task_details.group",
                )?;
            }

            let _cond =
                { event.has_value("json.resource.ecsClusterDetails.taskDetails.startedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.resource.ecsClusterDetails.taskDetails.startedAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.guardduty.resource.ecs_cluster_details.task_details.started_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_resource_ecsClusterDetails_taskDetails_startedAt_to_aws_guardduty_resource_ecs_cluster_details_task_details_started_at_b86c2e20")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.resource.ecsClusterDetails.taskDetails.startedBy") {
                event.rename(
                    "json.resource.ecsClusterDetails.taskDetails.startedBy",
                    "aws.guardduty.resource.ecs_cluster_details.task_details.started_by",
                )?;
            }

            if event.has("json.resource.ecsClusterDetails.taskDetails.tags") {
                event.rename(
                    "json.resource.ecsClusterDetails.taskDetails.tags",
                    "aws.guardduty.resource.ecs_cluster_details.task_details.tags",
                )?;
            }

            let _cond =
                { event.has_value("json.resource.ecsClusterDetails.taskDetails.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.resource.ecsClusterDetails.taskDetails.createdAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.guardduty.resource.ecs_cluster_details.task_details.created_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_resource_ecsClusterDetails_taskDetails_createdAt_to_aws_guardduty_resource_ecs_cluster_details_task_details_created_at_857f6997")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.resource.ecsClusterDetails.taskDetails.version") {
                event.rename(
                    "json.resource.ecsClusterDetails.taskDetails.version",
                    "aws.guardduty.resource.ecs_cluster_details.task_details.version",
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.ecsClusterDetails.taskDetails.volumes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.ecsClusterDetails.taskDetails.volumes",
                    |event| {
                        if event.has("_ingest._value.hostPath") {
                            event.rename("_ingest._value.hostPath", "_ingest._value.host_path")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.ecsClusterDetails.taskDetails.volumes") {
                event.rename(
                    "json.resource.ecsClusterDetails.taskDetails.volumes",
                    "aws.guardduty.resource.ecs_cluster_details.task_details.volumes",
                )?;
            }

            if event.has("json.resource.eksClusterDetails.arn") {
                event.rename(
                    "json.resource.eksClusterDetails.arn",
                    "aws.guardduty.resource.eks_cluster_details.arn",
                )?;
            }

            let _cond = { event.has_value("json.resource.eksClusterDetails.createdAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.resource.eksClusterDetails.createdAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.guardduty.resource.eks_cluster_details.created_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_resource_eksClusterDetails_createdAt_to_aws_guardduty_resource_eks_cluster_details_created_at_01775990")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.resource.eksClusterDetails.name") {
                event.rename(
                    "json.resource.eksClusterDetails.name",
                    "aws.guardduty.resource.eks_cluster_details.name",
                )?;
            }

            if event.has("json.resource.eksClusterDetails.status") {
                event.rename(
                    "json.resource.eksClusterDetails.status",
                    "aws.guardduty.resource.eks_cluster_details.status",
                )?;
            }

            if event.has("json.resource.eksClusterDetails.tags") {
                event.rename(
                    "json.resource.eksClusterDetails.tags",
                    "aws.guardduty.resource.eks_cluster_details.tags",
                )?;
            }

            if event.has("json.resource.eksClusterDetails.vpcId") {
                event.rename(
                    "json.resource.eksClusterDetails.vpcId",
                    "aws.guardduty.resource.eks_cluster_details.vpcid",
                )?;
            }

            if event.has("json.resource.instanceDetails.availabilityZone") {
                event.rename(
                    "json.resource.instanceDetails.availabilityZone",
                    "aws.guardduty.resource.instance_details.availability_zone",
                )?;
            }

            if event.has("json.resource.instanceDetails.iamInstanceProfile") {
                event.rename(
                    "json.resource.instanceDetails.iamInstanceProfile",
                    "aws.guardduty.resource.instance_details.iaminstance_profile",
                )?;
            }

            if event.has("json.resource.instanceDetails.imageDescription") {
                event.rename(
                    "json.resource.instanceDetails.imageDescription",
                    "aws.guardduty.resource.instance_details.image.description",
                )?;
            }

            if event.has("json.resource.instanceDetails.imageId") {
                event.rename(
                    "json.resource.instanceDetails.imageId",
                    "aws.guardduty.resource.instance_details.image.id",
                )?;
            }

            if event.has("json.resource.instanceDetails.instanceId") {
                event.rename(
                    "json.resource.instanceDetails.instanceId",
                    "aws.guardduty.resource.instance_details.instance.id",
                )?;
            }

            if event.has("json.resource.instanceDetails.instanceState") {
                event.rename(
                    "json.resource.instanceDetails.instanceState",
                    "aws.guardduty.resource.instance_details.instance.state",
                )?;
            }

            if event.has("json.resource.instanceDetails.instanceType") {
                event.rename(
                    "json.resource.instanceDetails.instanceType",
                    "aws.guardduty.resource.instance_details.instance.type",
                )?;
            }

            let _cond = { event.has_value("json.resource.instanceDetails.launchTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.resource.instanceDetails.launchTime")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.guardduty.resource.instance_details.launch_time",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_resource_instanceDetails_launchTime_to_aws_guardduty_resource_instance_details_launch_time_1614bf1e")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.ipv6Addresses") {
                            if let Some(Value::Array(items)) =
                                event.get("_ingest._value.ipv6Addresses").cloned()
                            {
                                // A NESTED loop borrows the same `_ingest._value` slot, so
                                // the enclosing element is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let mut out = Vec::with_capacity(items.len());
                                for item in items {
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if event.has_value("_ingest._value") {
                                            if let Some(val) = event.get("_ingest._value") {
                                                let converted = convert_value(val, "ip").map_err(
                                                    |message| TransformError::ParseError {
                                                        path: "_ingest._value".into(),
                                                        message,
                                                    },
                                                )?;
                                                event.set("_ingest._value", converted)?;
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        if event.remove("_ingest._value").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value".into(),
                                            });
                                        }
                                        event.append(
                                            "error.message",
                                            json!(
                                                event
                                                    .get("_ingest.on_failure_message")
                                                    .map_or_else(String::new, painless_to_string)
                                            ),
                                        )?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                event.set("_ingest._value.ipv6Addresses", Value::Array(out))?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set(
                        "json.resource.instanceDetails.networkInterfaces",
                        Value::Array(out),
                    )?;
                }
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has_value("_ingest._value.ipv6Addresses") {
                            foreach_array(event, "_ingest._value.ipv6Addresses", |event| {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, painless_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has("_ingest._value.ipv6Addresses") {
                            event.rename(
                                "_ingest._value.ipv6Addresses",
                                "_ingest._value.ipv6_addresses",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has("_ingest._value.networkInterfaceId") {
                            event.rename(
                                "_ingest._value.networkInterfaceId",
                                "_ingest._value.network_interface_id",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has("_ingest._value.privateDnsName") {
                            event.rename(
                                "_ingest._value.privateDnsName",
                                "_ingest._value.private.dns_name",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.privateIpAddress") {
                                if let Some(val) = event.get("_ingest._value.privateIpAddress") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.privateIpAddress".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.private.ip_address", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value.private.ip_address")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has_value("_ingest._value.privateIpAddresses") {
                            foreach_array(event, "_ingest._value.privateIpAddresses", |event| {
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.privateIpAddress") {
                                        if let Some(val) =
                                            event.get("_ingest._value.privateIpAddress")
                                        {
                                            let converted =
                                                convert_value(val, "ip").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.privateIpAddress"
                                                            .into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set(
                                                "_ingest._value.private.ip_address",
                                                converted,
                                            )?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.append(
                                        "error.message",
                                        json!(
                                            event
                                                .get("_ingest.on_failure_message")
                                                .map_or_else(String::new, painless_to_string)
                                        ),
                                    )?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has_value("_ingest._value.privateIpAddresses") {
                            foreach_array(event, "_ingest._value.privateIpAddresses", |event| {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.private.ip_address")
                                            .map_or_else(String::new, painless_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has_value("_ingest._value.privateIpAddresses") {
                            foreach_array(event, "_ingest._value.privateIpAddresses", |event| {
                                if event.has("_ingest._value.privateDnsName") {
                                    event.rename(
                                        "_ingest._value.privateDnsName",
                                        "_ingest._value.private.dns_name",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has_value("_ingest._value.privateIpAddresses") {
                            foreach_array(event, "_ingest._value.privateIpAddresses", |event| {
                                event.remove("_ingest._value.privateIpAddress");
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has("_ingest._value.privateIpAddresses") {
                            event.rename(
                                "_ingest._value.privateIpAddresses",
                                "_ingest._value.private.ip_addresses",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has("_ingest._value.publicDnsName") {
                            event.rename(
                                "_ingest._value.publicDnsName",
                                "_ingest._value.public.dns_name",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.publicIp") {
                                if let Some(val) = event.get("_ingest._value.publicIp") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.publicIp".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.public.ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value.public.ip")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has_value("_ingest._value.securityGroups") {
                            foreach_array(event, "_ingest._value.securityGroups", |event| {
                                if event.has("_ingest._value.groupId") {
                                    event.rename(
                                        "_ingest._value.groupId",
                                        "_ingest._value.group.id",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has_value("_ingest._value.securityGroups") {
                            foreach_array(event, "_ingest._value.securityGroups", |event| {
                                if event.has("_ingest._value.groupName") {
                                    event.rename(
                                        "_ingest._value.groupName",
                                        "_ingest._value.group.name",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has("_ingest._value.securityGroups") {
                            event.rename(
                                "_ingest._value.securityGroups",
                                "_ingest._value.security_groups",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has("_ingest._value.subnetId") {
                            event.rename("_ingest._value.subnetId", "_ingest._value.subnet_id")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        if event.has("_ingest._value.vpcId") {
                            event.rename("_ingest._value.vpcId", "_ingest._value.vpc_id")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.networkInterfaces")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.networkInterfaces",
                    |event| {
                        event.remove("_ingest._value.privateIpAddress");
                        event.remove("_ingest._value.publicIp");
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.instanceDetails.networkInterfaces") {
                event.rename(
                    "json.resource.instanceDetails.networkInterfaces",
                    "aws.guardduty.resource.instance_details.network_interfaces",
                )?;
            }

            if event.has("json.resource.instanceDetails.outpostArn") {
                event.rename(
                    "json.resource.instanceDetails.outpostArn",
                    "aws.guardduty.resource.instance_details.outpost_arn",
                )?;
            }

            if event.has("json.resource.instanceDetails.platform") {
                event.rename(
                    "json.resource.instanceDetails.platform",
                    "aws.guardduty.resource.instance_details.platform",
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.productCodes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.productCodes",
                    |event| {
                        if event.has("_ingest._value.productCodeId") {
                            event.rename(
                                "_ingest._value.productCodeId",
                                "_ingest._value.product_code.id",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.instanceDetails.productCodes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.instanceDetails.productCodes",
                    |event| {
                        if event.has("_ingest._value.productCodeType") {
                            event.rename(
                                "_ingest._value.productCodeType",
                                "_ingest._value.product_code.type",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.instanceDetails.productCodes") {
                event.rename(
                    "json.resource.instanceDetails.productCodes",
                    "aws.guardduty.resource.instance_details.product_codes",
                )?;
            }

            if event.has("json.resource.instanceDetails.tags") {
                event.rename(
                    "json.resource.instanceDetails.tags",
                    "aws.guardduty.resource.instance_details.tags",
                )?;
            }

            if event.has("json.resource.kubernetesDetails.kubernetesUserDetails.uid") {
                event.rename(
                    "json.resource.kubernetesDetails.kubernetesUserDetails.uid",
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.uid",
                )?;
            }

            let _cond = {
                event.has_value(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.uid",
                )
            };
            if _cond {
                event.append_unique("related.user", json!(event.get("aws.guardduty.resource.kubernetes_details.kubernetes_user_details.uid").map_or_else(String::new, painless_to_string)))?;
            }

            if event.has("json.resource.kubernetesDetails.kubernetesUserDetails.username") {
                event.rename(
                    "json.resource.kubernetesDetails.kubernetesUserDetails.username",
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.user_name",
                )?;
            }

            let _cond = {
                event.has_value(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.user_name",
                )
            };
            if _cond {
                event.append_unique("related.user", json!(event.get("aws.guardduty.resource.kubernetes_details.kubernetes_user_details.user_name").map_or_else(String::new, painless_to_string)))?;
            }

            if event.has("json.resource.kubernetesDetails.kubernetesUserDetails.groups") {
                event.rename(
                    "json.resource.kubernetesDetails.kubernetesUserDetails.groups",
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.groups",
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers",
                    |event| {
                        if event.has("_ingest._value.containerRuntime") {
                            event.rename(
                                "_ingest._value.containerRuntime",
                                "_ingest._value.container_runtime",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers",
                    |event| {
                        if event.has("_ingest._value.image") {
                            event.rename("_ingest._value.image", "_ingest._value.image.value")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers",
                    |event| {
                        if event.has("_ingest._value.imagePrefix") {
                            event.rename(
                                "_ingest._value.imagePrefix",
                                "_ingest._value.image.prefix",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.securityContext.privileged") {
                                if let Some(val) =
                                    event.get("_ingest._value.securityContext.privileged")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.securityContext.privileged"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.security_context.privileged",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers",
                    |event| {
                        if event.has_value("_ingest._value.volumeMounts") {
                            foreach_array(event, "_ingest._value.volumeMounts", |event| {
                                if event.has("_ingest._value.mountPath") {
                                    event.rename(
                                        "_ingest._value.mountPath",
                                        "_ingest._value.mount_path",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers",
                    |event| {
                        if event.has("_ingest._value.volumeMounts") {
                            event.rename(
                                "_ingest._value.volumeMounts",
                                "_ingest._value.volume_mounts",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers",
                    |event| {
                        event.remove("_ingest._value.securityContext.privileged");
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers") {
                event.rename("json.resource.kubernetesDetails.kubernetesWorkloadDetails.containers", "aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.containers")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.hostNetwork",
                ) {
                    if let Some(val) = event.get(
                        "json.resource.kubernetesDetails.kubernetesWorkloadDetails.hostNetwork",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.resource.kubernetesDetails.kubernetesWorkloadDetails.hostNetwork".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.host_network", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_resource_kubernetesDetails_kubernetesWorkloadDetails_hostNetwork_to_aws_guardduty_resource_kubernetes_details_kubernetes_workload_details_host_network_0f6297bc")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.resource.kubernetesDetails.kubernetesWorkloadDetails.name") {
                event.rename(
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.name",
                    "aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.name",
                )?;
            }

            if event.has("json.resource.kubernetesDetails.kubernetesWorkloadDetails.namespace") {
                event.rename("json.resource.kubernetesDetails.kubernetesWorkloadDetails.namespace", "aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.name_space")?;
            }

            if event.has("json.resource.kubernetesDetails.kubernetesWorkloadDetails.type") {
                event.rename(
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.type",
                    "aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.type",
                )?;
            }

            if event.has("json.resource.kubernetesDetails.kubernetesWorkloadDetails.uid") {
                event.rename(
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.uid",
                    "aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.uid",
                )?;
            }

            let _cond = {
                event
                    .get("json.resource.kubernetesDetails.kubernetesWorkloadDetails.volumes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.volumes",
                    |event| {
                        if event.has("_ingest._value.hostPath") {
                            event.rename("_ingest._value.hostPath", "_ingest._value.host_path")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has("json.resource.kubernetesDetails.kubernetesWorkloadDetails.volumes") {
                event.rename(
                    "json.resource.kubernetesDetails.kubernetesWorkloadDetails.volumes",
                    "aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.volumes",
                )?;
            }

            if event.has("json.resource.rdsDbInstanceDetails.dbInstanceIdentifier") {
                event.rename(
                    "json.resource.rdsDbInstanceDetails.dbInstanceIdentifier",
                    "aws.guardduty.resource.rdsdb_instance_details.instance_identifier",
                )?;
            }

            if event.has("json.resource.rdsDbInstanceDetails.engine") {
                event.rename(
                    "json.resource.rdsDbInstanceDetails.engine",
                    "aws.guardduty.resource.rdsdb_instance_details.engine",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.resource.rdsDbInstanceDetails.engineVersion") {
                    if let Some(val) = event.get("json.resource.rdsDbInstanceDetails.engineVersion")
                    {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.resource.rdsDbInstanceDetails.engineVersion".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.resource.rdsdb_instance_details.engine_version",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_resource_rdsDbInstanceDetails_engineVersion_to_aws_guardduty_resource_rdsdb_instance_details_engine_version_e4e870d4")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.resource.rdsDbInstanceDetails.dbClusterIdentifier") {
                event.rename(
                    "json.resource.rdsDbInstanceDetails.dbClusterIdentifier",
                    "aws.guardduty.resource.rdsdb_instance_details.cluster_identifier",
                )?;
            }

            if event.has("json.resource.rdsDbInstanceDetails.dbInstanceArn") {
                event.rename(
                    "json.resource.rdsDbInstanceDetails.dbInstanceArn",
                    "aws.guardduty.resource.rdsdb_instance_details.instance_arn",
                )?;
            }

            if event.has("json.resource.rdsDbUserDetails.user") {
                event.rename(
                    "json.resource.rdsDbUserDetails.user",
                    "aws.guardduty.resource.rdsdb_user_details.user",
                )?;
            }

            let _cond = { event.has_value("aws.guardduty.resource.rdsdb_user_details.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("aws.guardduty.resource.rdsdb_user_details.user")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.resource.rdsDbUserDetails.application") {
                event.rename(
                    "json.resource.rdsDbUserDetails.application",
                    "aws.guardduty.resource.rdsdb_user_details.application",
                )?;
            }

            if event.has("json.resource.rdsDbUserDetails.database") {
                event.rename(
                    "json.resource.rdsDbUserDetails.database",
                    "aws.guardduty.resource.rdsdb_user_details.database",
                )?;
            }

            if event.has("json.resource.rdsDbUserDetails.ssl") {
                event.rename(
                    "json.resource.rdsDbUserDetails.ssl",
                    "aws.guardduty.resource.rdsdb_user_details.ssl",
                )?;
            }

            if event.has("json.resource.rdsDbUserDetails.authMethod") {
                event.rename(
                    "json.resource.rdsDbUserDetails.authMethod",
                    "aws.guardduty.resource.rdsdb_user_details.auth_method",
                )?;
            }

            if event.has("json.resource.resourceType") {
                event.rename("json.resource.resourceType", "aws.guardduty.resource.type")?;
            }

            let _cond = {
                event
                    .get("json.resource.s3BucketDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.resource.s3BucketDetails", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.createdAt") {
                            if let Some(parsed) = parse_date_out(
                                &date_str,
                                &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                                None,
                                None,
                            ) {
                                event.set("_ingest._value.created_at", parsed)?;
                            }
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.resource.s3BucketDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.resource.s3BucketDetails", |event| {
                    if event.has("_ingest._value.defaultServerSideEncryption.encryptionType") {
                        event.rename(
                            "_ingest._value.defaultServerSideEncryption.encryptionType",
                            "_ingest._value.default_server_side_encryption.encryption_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.resource.s3BucketDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.resource.s3BucketDetails", |event| {
                    if event.has("_ingest._value.defaultServerSideEncryption.kmsMasterKeyArn") {
                        event.rename(
                            "_ingest._value.defaultServerSideEncryption.kmsMasterKeyArn",
                            "_ingest._value.default_server_side_encryption.kms_masterkey_arn",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.resource.s3BucketDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.resource.s3BucketDetails", |event| {
                    if event.has("_ingest._value.publicAccess") {
                        event.rename(
                            "_ingest._value.publicAccess",
                            "_ingest._value.public_access",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.resource.s3BucketDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.resource.s3BucketDetails", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.owner.id")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.resource.s3BucketDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.resource.s3BucketDetails", |event| {
                    event.remove("_ingest._value.createdAt");
                    Ok(())
                })?;
            }

            if event.has("json.resource.s3BucketDetails") {
                event.rename(
                    "json.resource.s3BucketDetails",
                    "aws.guardduty.resource.s3_bucket_details",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.schemaVersion") {
                    if let Some(val) = event.get("json.schemaVersion") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.schemaVersion".into(),
                                message,
                            }
                        })?;
                        event.set("aws.guardduty.schema_version", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_schemaVersion_to_aws_guardduty_schema_version_69dc54d8",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.action.actionType") {
                event.rename(
                    "json.service.action.actionType",
                    "aws.guardduty.service.action.type",
                )?;
            }

            if event.has("json.service.action.awsApiCallAction.affectedResources") {
                event.rename(
                    "json.service.action.awsApiCallAction.affectedResources",
                    "aws.guardduty.service.action.aws_api_call_action.affected_resources",
                )?;
            }

            if event.has("json.service.action.awsApiCallAction.api") {
                event.rename(
                    "json.service.action.awsApiCallAction.api",
                    "aws.guardduty.service.action.aws_api_call_action.api",
                )?;
            }

            if event.has("json.service.action.awsApiCallAction.callerType") {
                event.rename(
                    "json.service.action.awsApiCallAction.callerType",
                    "aws.guardduty.service.action.aws_api_call_action.caller_type",
                )?;
            }

            if event.has("json.service.action.awsApiCallAction.domainDetails.domain") {
                event.rename(
                    "json.service.action.awsApiCallAction.domainDetails.domain",
                    "aws.guardduty.service.action.aws_api_call_action.domain_details.domain",
                )?;
            }

            let _cond = {
                event.has_value(
                    "aws.guardduty.service.action.aws_api_call_action.domain_details.domain",
                )
            };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("aws.guardduty.service.action.aws_api_call_action.domain_details.domain").map_or_else(String::new, painless_to_string)))?;
            }

            if event.has("json.service.action.awsApiCallAction.errorCode") {
                event.rename(
                    "json.service.action.awsApiCallAction.errorCode",
                    "aws.guardduty.service.action.aws_api_call_action.error_code",
                )?;
            }

            if event.has("json.service.action.awsApiCallAction.remoteAccountDetails.accountId") {
                event.rename("json.service.action.awsApiCallAction.remoteAccountDetails.accountId", "aws.guardduty.service.action.aws_api_call_action.remote_account_details.account_id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.service.action.awsApiCallAction.remoteAccountDetails.affiliated",
                ) {
                    if let Some(val) = event
                        .get("json.service.action.awsApiCallAction.remoteAccountDetails.affiliated")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.action.awsApiCallAction.remoteAccountDetails.affiliated".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.action.aws_api_call_action.remote_account_details.affiliated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_awsApiCallAction_remoteAccountDetails_affiliated_to_aws_guardduty_service_action_aws_api_call_action_remote_account_details_affiliated_e8fe1585")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.action.awsApiCallAction.remoteIpDetails.city.cityName") {
                event.rename(
                    "json.service.action.awsApiCallAction.remoteIpDetails.city.cityName",
                    "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.city.name",
                )?;
            }

            if event.has("json.service.action.awsApiCallAction.remoteIpDetails.country.countryCode")
            {
                event.rename("json.service.action.awsApiCallAction.remoteIpDetails.country.countryCode", "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.country.code")?;
            }

            if event.has("json.service.action.awsApiCallAction.remoteIpDetails.country.countryName")
            {
                event.rename("json.service.action.awsApiCallAction.remoteIpDetails.country.countryName", "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.country.name")?;
            }

            if event.has("json.service.action.awsApiCallAction.remoteIpDetails.geoLocation") {
                event.rename("json.service.action.awsApiCallAction.remoteIpDetails.geoLocation", "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.geo_location")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.service.action.awsApiCallAction.remoteIpDetails.ipAddressV4")
                {
                    if let Some(val) = event
                        .get("json.service.action.awsApiCallAction.remoteIpDetails.ipAddressV4")
                    {
                        let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.action.awsApiCallAction.remoteIpDetails.ipAddressV4".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.ip_address_v4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_awsApiCallAction_remoteIpDetails_ipAddressV4_to_aws_guardduty_service_action_aws_api_call_action_remote_ip_details_ip_address_v4_0fc42b8a")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.ip_address_v4")
            };
            if _cond {
                event.append_unique("related.ip", json!(event.get("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            if event.has("json.service.action.awsApiCallAction.remoteIpDetails.organization.asn") {
                event.rename("json.service.action.awsApiCallAction.remoteIpDetails.organization.asn", "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.asn")?;
            }

            if event.has("json.service.action.awsApiCallAction.remoteIpDetails.organization.asnOrg")
            {
                event.rename("json.service.action.awsApiCallAction.remoteIpDetails.organization.asnOrg", "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.asnorg")?;
            }

            if event.has("json.service.action.awsApiCallAction.remoteIpDetails.organization.isp") {
                event.rename("json.service.action.awsApiCallAction.remoteIpDetails.organization.isp", "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.isp")?;
            }

            if event.has("json.service.action.awsApiCallAction.remoteIpDetails.organization.org") {
                event.rename("json.service.action.awsApiCallAction.remoteIpDetails.organization.org", "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.org")?;
            }

            if event.has("json.service.action.awsApiCallAction.serviceName") {
                event.rename(
                    "json.service.action.awsApiCallAction.serviceName",
                    "aws.guardduty.service.action.aws_api_call_action.service_name",
                )?;
            }

            if event.has("json.service.action.awsApiCallAction.userAgent") {
                event.rename(
                    "json.service.action.awsApiCallAction.userAgent",
                    "aws.guardduty.service.action.aws_api_call_action.user_agent",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.action.dnsRequestAction.blocked") {
                    if let Some(val) = event.get("json.service.action.dnsRequestAction.blocked") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.action.dnsRequestAction.blocked".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.action.dns_request_action.blocked",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_dnsRequestAction_blocked_to_aws_guardduty_service_action_dns_request_action_blocked_137598ec")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.action.dnsRequestAction.domain") {
                event.rename(
                    "json.service.action.dnsRequestAction.domain",
                    "aws.guardduty.service.action.dns_request_action.domain",
                )?;
            }

            let _cond =
                { event.has_value("aws.guardduty.service.action.dns_request_action.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("aws.guardduty.service.action.dns_request_action.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.service.action.dnsRequestAction.protocol") {
                event.rename(
                    "json.service.action.dnsRequestAction.protocol",
                    "aws.guardduty.service.action.dns_request_action.protocol",
                )?;
            }

            if event.has("json.service.action.kubernetesApiCallAction.parameters") {
                event.rename(
                    "json.service.action.kubernetesApiCallAction.parameters",
                    "aws.guardduty.service.action.kubernetes_api_call_action.parameters",
                )?;
            }

            if event
                .has("json.service.action.kubernetesApiCallAction.remoteIpDetails.city.cityName")
            {
                event.rename("json.service.action.kubernetesApiCallAction.remoteIpDetails.city.cityName", "aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.city.name")?;
            }

            if event.has(
                "json.service.action.kubernetesApiCallAction.remoteIpDetails.country.countryCode",
            ) {
                event.rename("json.service.action.kubernetesApiCallAction.remoteIpDetails.country.countryCode", "aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.country.code")?;
            }

            if event.has(
                "json.service.action.kubernetesApiCallAction.remoteIpDetails.country.countryName",
            ) {
                event.rename("json.service.action.kubernetesApiCallAction.remoteIpDetails.country.countryName", "aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.country.name")?;
            }

            if event.has("json.service.action.kubernetesApiCallAction.remoteIpDetails.geoLocation")
            {
                event.rename("json.service.action.kubernetesApiCallAction.remoteIpDetails.geoLocation", "aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.geo_location")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.service.action.kubernetesApiCallAction.remoteIpDetails.ipAddressV4",
                ) {
                    if let Some(val) = event.get(
                        "json.service.action.kubernetesApiCallAction.remoteIpDetails.ipAddressV4",
                    ) {
                        let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.action.kubernetesApiCallAction.remoteIpDetails.ipAddressV4".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.ip_address_v4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_kubernetesApiCallAction_remoteIpDetails_ipAddressV4_to_aws_guardduty_service_action_kubernetes_api_call_action_remote_ip_details_ip_address_v4_0f586928")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.ip_address_v4")
            };
            if _cond {
                event.append_unique("related.ip", json!(event.get("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            if event
                .has("json.service.action.kubernetesApiCallAction.remoteIpDetails.organization.asn")
            {
                event.rename("json.service.action.kubernetesApiCallAction.remoteIpDetails.organization.asn", "aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.asn")?;
            }

            if event.has(
                "json.service.action.kubernetesApiCallAction.remoteIpDetails.organization.asnOrg",
            ) {
                event.rename("json.service.action.kubernetesApiCallAction.remoteIpDetails.organization.asnOrg", "aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.asnorg")?;
            }

            if event
                .has("json.service.action.kubernetesApiCallAction.remoteIpDetails.organization.isp")
            {
                event.rename("json.service.action.kubernetesApiCallAction.remoteIpDetails.organization.isp", "aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.isp")?;
            }

            if event
                .has("json.service.action.kubernetesApiCallAction.remoteIpDetails.organization.org")
            {
                event.rename("json.service.action.kubernetesApiCallAction.remoteIpDetails.organization.org", "aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.org")?;
            }

            if event.has("json.service.action.kubernetesApiCallAction.requestUri") {
                event.rename(
                    "json.service.action.kubernetesApiCallAction.requestUri",
                    "aws.guardduty.service.action.kubernetes_api_call_action.request_uri",
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.kubernetesApiCallAction.sourceIPs")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event
                    .get("json.service.action.kubernetesApiCallAction.sourceIPs")
                    .cloned()
                {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            if event.remove("_ingest._value").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value".into(),
                                });
                            }
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set(
                        "json.service.action.kubernetesApiCallAction.sourceIPs",
                        Value::Array(out),
                    )?;
                }
            }

            let _cond = {
                event
                    .get("json.service.action.kubernetesApiCallAction.sourceIPs")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.kubernetesApiCallAction.sourceIPs",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has("json.service.action.kubernetesApiCallAction.sourceIPs") {
                event.rename(
                    "json.service.action.kubernetesApiCallAction.sourceIPs",
                    "aws.guardduty.service.action.kubernetes_api_call_action.source_ips",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.action.kubernetesApiCallAction.statusCode") {
                    if let Some(val) =
                        event.get("json.service.action.kubernetesApiCallAction.statusCode")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.action.kubernetesApiCallAction.statusCode"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.action.kubernetes_api_call_action.status_code",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_kubernetesApiCallAction_statusCode_to_aws_guardduty_service_action_kubernetes_api_call_action_status_code_a22b070c")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.action.kubernetesApiCallAction.userAgent") {
                event.rename(
                    "json.service.action.kubernetesApiCallAction.userAgent",
                    "aws.guardduty.service.action.kubernetes_api_call_action.user_agent",
                )?;
            }

            if event.has("json.service.action.kubernetesApiCallAction.verb") {
                event.rename(
                    "json.service.action.kubernetesApiCallAction.verb",
                    "aws.guardduty.service.action.kubernetes_api_call_action.verb",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.action.networkConnectionAction.blocked") {
                    if let Some(val) =
                        event.get("json.service.action.networkConnectionAction.blocked")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.action.networkConnectionAction.blocked".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.action.network_connection_action.blocked",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_networkConnectionAction_blocked_to_aws_guardduty_service_action_network_connection_action_blocked_57bbe3c2")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.action.networkConnectionAction.connectionDirection") {
                event.rename(
                    "json.service.action.networkConnectionAction.connectionDirection",
                    "aws.guardduty.service.action.network_connection_action.connection_direction",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.service.action.networkConnectionAction.localIpDetails.ipAddressV4",
                ) {
                    if let Some(val) = event.get(
                        "json.service.action.networkConnectionAction.localIpDetails.ipAddressV4",
                    ) {
                        let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.action.networkConnectionAction.localIpDetails.ipAddressV4".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.action.network_connection_action.local_ip_details.ip_address_v4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_networkConnectionAction_localIpDetails_ipAddressV4_to_aws_guardduty_service_action_network_connection_action_local_ip_details_ip_address_v4_816000bb")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.local_ip_details.ip_address_v4")
            };
            if _cond {
                event.append_unique("related.ip", json!(event.get("aws.guardduty.service.action.network_connection_action.local_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.service.action.networkConnectionAction.localPortDetails.port")
                {
                    if let Some(val) = event
                        .get("json.service.action.networkConnectionAction.localPortDetails.port")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.action.networkConnectionAction.localPortDetails.port".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.action.network_connection_action.local_port_details.port.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_networkConnectionAction_localPortDetails_port_to_aws_guardduty_service_action_network_connection_action_local_port_details_port_value_3dbc2d4f")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.action.networkConnectionAction.localPortDetails.portName") {
                event.rename("json.service.action.networkConnectionAction.localPortDetails.portName", "aws.guardduty.service.action.network_connection_action.local_port_details.port.name")?;
            }

            if event.has("json.service.action.networkConnectionAction.protocol") {
                event.rename(
                    "json.service.action.networkConnectionAction.protocol",
                    "aws.guardduty.service.action.network_connection_action.transport",
                )?;
            }

            if event
                .has("json.service.action.networkConnectionAction.remoteIpDetails.city.cityName")
            {
                event.rename("json.service.action.networkConnectionAction.remoteIpDetails.city.cityName", "aws.guardduty.service.action.network_connection_action.remote_ip_details.city.name")?;
            }

            if event.has(
                "json.service.action.networkConnectionAction.remoteIpDetails.country.countryCode",
            ) {
                event.rename("json.service.action.networkConnectionAction.remoteIpDetails.country.countryCode", "aws.guardduty.service.action.network_connection_action.remote_ip_details.country.code")?;
            }

            if event.has(
                "json.service.action.networkConnectionAction.remoteIpDetails.country.countryName",
            ) {
                event.rename("json.service.action.networkConnectionAction.remoteIpDetails.country.countryName", "aws.guardduty.service.action.network_connection_action.remote_ip_details.country.name")?;
            }

            if event.has("json.service.action.networkConnectionAction.remoteIpDetails.geoLocation")
            {
                event.rename("json.service.action.networkConnectionAction.remoteIpDetails.geoLocation", "aws.guardduty.service.action.network_connection_action.remote_ip_details.geo_location")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.service.action.networkConnectionAction.remoteIpDetails.ipAddressV4",
                ) {
                    if let Some(val) = event.get(
                        "json.service.action.networkConnectionAction.remoteIpDetails.ipAddressV4",
                    ) {
                        let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.action.networkConnectionAction.remoteIpDetails.ipAddressV4".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.action.network_connection_action.remote_ip_details.ip_address_v4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_networkConnectionAction_remoteIpDetails_ipAddressV4_to_aws_guardduty_service_action_network_connection_action_remote_ip_details_ip_address_v4_264baf15")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.remote_ip_details.ip_address_v4")
            };
            if _cond {
                event.append_unique("related.ip", json!(event.get("aws.guardduty.service.action.network_connection_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            if event
                .has("json.service.action.networkConnectionAction.remoteIpDetails.organization.asn")
            {
                event.rename("json.service.action.networkConnectionAction.remoteIpDetails.organization.asn", "aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.asn")?;
            }

            if event.has(
                "json.service.action.networkConnectionAction.remoteIpDetails.organization.asnOrg",
            ) {
                event.rename("json.service.action.networkConnectionAction.remoteIpDetails.organization.asnOrg", "aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.asnorg")?;
            }

            if event
                .has("json.service.action.networkConnectionAction.remoteIpDetails.organization.isp")
            {
                event.rename("json.service.action.networkConnectionAction.remoteIpDetails.organization.isp", "aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.isp")?;
            }

            if event
                .has("json.service.action.networkConnectionAction.remoteIpDetails.organization.org")
            {
                event.rename("json.service.action.networkConnectionAction.remoteIpDetails.organization.org", "aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.org")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("json.service.action.networkConnectionAction.remotePortDetails.port")
                {
                    if let Some(val) = event
                        .get("json.service.action.networkConnectionAction.remotePortDetails.port")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.action.networkConnectionAction.remotePortDetails.port".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.action.network_connection_action.remote_port_details.port.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_networkConnectionAction_remotePortDetails_port_to_aws_guardduty_service_action_network_connection_action_remote_port_details_port_value_11544c03")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.action.networkConnectionAction.remotePortDetails.portName") {
                event.rename("json.service.action.networkConnectionAction.remotePortDetails.portName", "aws.guardduty.service.action.network_connection_action.remote_port_details.port.name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.action.portProbeAction.blocked") {
                    if let Some(val) = event.get("json.service.action.portProbeAction.blocked") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.action.portProbeAction.blocked".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.action.port_probe_action.blocked",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_portProbeAction_blocked_to_aws_guardduty_service_action_port_probe_action_blocked_56873752")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.localIpDetails.ipAddressV4") {
                                if let Some(val) =
                                    event.get("_ingest._value.localIpDetails.ipAddressV4")
                                {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.localIpDetails.ipAddressV4"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.local_ip_details.ip_address_v4",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value.local_ip_details.ip_address_v4")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.localPortDetails.port") {
                                if let Some(val) = event.get("_ingest._value.localPortDetails.port")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.localPortDetails.port".into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.local_port_details.port.value",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.localPortDetails.portName") {
                            event.rename(
                                "_ingest._value.localPortDetails.portName",
                                "_ingest._value.local_port_details.port.name",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.remoteIpDetails.city.cityName") {
                            event.rename(
                                "_ingest._value.remoteIpDetails.city.cityName",
                                "_ingest._value.remote_ip_details.city.name",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.remoteIpDetails.country.countryCode") {
                            event.rename(
                                "_ingest._value.remoteIpDetails.country.countryCode",
                                "_ingest._value.remote_ip_details.country.code",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.remoteIpDetails.country.countryName") {
                            event.rename(
                                "_ingest._value.remoteIpDetails.country.countryName",
                                "_ingest._value.remote_ip_details.country.name",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.remoteIpDetails.geoLocation") {
                            event.rename(
                                "_ingest._value.remoteIpDetails.geoLocation",
                                "_ingest._value.remote_ip_details.geo_location",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.remoteIpDetails.ipAddressV4") {
                                if let Some(val) =
                                    event.get("_ingest._value.remoteIpDetails.ipAddressV4")
                                {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.remoteIpDetails.ipAddressV4"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.remote_ip_details.ip_address_v4",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value.remote_ip_details.ip_address_v4")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.remoteIpDetails.organization.isp") {
                            event.rename(
                                "_ingest._value.remoteIpDetails.organization.isp",
                                "_ingest._value.remote_ip_details.organization.isp",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.remoteIpDetails.organization.org") {
                            event.rename(
                                "_ingest._value.remoteIpDetails.organization.org",
                                "_ingest._value.remote_ip_details.organization.org",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.remoteIpDetails.organization.asn") {
                            event.rename(
                                "_ingest._value.remoteIpDetails.organization.asn",
                                "_ingest._value.remote_ip_details.organization.asn",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        if event.has("_ingest._value.remoteIpDetails.organization.asnOrg") {
                            event.rename(
                                "_ingest._value.remoteIpDetails.organization.asnOrg",
                                "_ingest._value.remote_ip_details.organization.asnorg",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.action.portProbeAction.portProbeDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.action.portProbeAction.portProbeDetails",
                    |event| {
                        event.remove("_ingest._value.localIpDetails.ipAddressV4");
                        event.remove("_ingest._value.localPortDetails.port");
                        event.remove("_ingest._value.remoteIpDetails.ipAddressV4");
                        Ok(())
                    },
                )?;
            }

            if event.has("json.service.action.portProbeAction.portProbeDetails") {
                event.rename(
                    "json.service.action.portProbeAction.portProbeDetails",
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                )?;
            }

            if event.has("json.service.action.rdsLoginAttemptAction.remoteIpDetails.city.cityName")
            {
                event.rename("json.service.action.rdsLoginAttemptAction.remoteIpDetails.city.cityName", "aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.city.name")?;
            }

            if event.has(
                "json.service.action.rdsLoginAttemptAction.remoteIpDetails.country.countryCode",
            ) {
                event.rename("json.service.action.rdsLoginAttemptAction.remoteIpDetails.country.countryCode", "aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.country.code")?;
            }

            if event.has(
                "json.service.action.rdsLoginAttemptAction.remoteIpDetails.country.countryName",
            ) {
                event.rename("json.service.action.rdsLoginAttemptAction.remoteIpDetails.country.countryName", "aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.country.name")?;
            }

            if event.has("json.service.action.rdsLoginAttemptAction.remoteIpDetails.geoLocation") {
                event.rename("json.service.action.rdsLoginAttemptAction.remoteIpDetails.geoLocation", "aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.geo_location")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.service.action.rdsLoginAttemptAction.remoteIpDetails.ipAddressV4",
                ) {
                    if let Some(val) = event.get(
                        "json.service.action.rdsLoginAttemptAction.remoteIpDetails.ipAddressV4",
                    ) {
                        let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.action.rdsLoginAttemptAction.remoteIpDetails.ipAddressV4".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.ip_address_v4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_action_rdsLoginAttemptAction_remoteIpDetails_ipAddressV4_to_aws_guardduty_service_action_rds_login_attempt_action_remote_ip_details_ip_address_v4_dafb99f0")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.ip_address_v4")
            };
            if _cond {
                event.append_unique("related.ip", json!(event.get("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            if event
                .has("json.service.action.rdsLoginAttemptAction.remoteIpDetails.organization.asn")
            {
                event.rename("json.service.action.rdsLoginAttemptAction.remoteIpDetails.organization.asn", "aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.asn")?;
            }

            if event.has(
                "json.service.action.rdsLoginAttemptAction.remoteIpDetails.organization.asnOrg",
            ) {
                event.rename("json.service.action.rdsLoginAttemptAction.remoteIpDetails.organization.asnOrg", "aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.asnorg")?;
            }

            if event
                .has("json.service.action.rdsLoginAttemptAction.remoteIpDetails.organization.isp")
            {
                event.rename("json.service.action.rdsLoginAttemptAction.remoteIpDetails.organization.isp", "aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.isp")?;
            }

            if event
                .has("json.service.action.rdsLoginAttemptAction.remoteIpDetails.organization.org")
            {
                event.rename("json.service.action.rdsLoginAttemptAction.remoteIpDetails.organization.org", "aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.org")?;
            }

            if event.has("json.service.additionalInfo") {
                event.rename(
                    "json.service.additionalInfo",
                    "aws.guardduty.service.additional_info",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.archived") {
                    if let Some(val) = event.get("json.service.archived") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.archived".into(),
                                message,
                            }
                        })?;
                        event.set("aws.guardduty.service.archived", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_service_archived_to_aws_guardduty_service_archived_9ca029b0",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.count") {
                    if let Some(val) = event.get("json.service.count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.count".into(),
                                message,
                            }
                        })?;
                        event.set("aws.guardduty.service.count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_service_count_to_aws_guardduty_service_count_4efaa3c8",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.detectorId") {
                event.rename(
                    "json.service.detectorId",
                    "aws.guardduty.service.detector_id",
                )?;
            }

            let _cond = { event.has_value("json.service.ebsVolumeScanDetails.scanCompletedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.service.ebsVolumeScanDetails.scanCompletedAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.guardduty.service.ebs_volume_scan_details.scan.completed_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_service_ebsVolumeScanDetails_scanCompletedAt_to_aws_guardduty_service_ebs_volume_scan_details_scan_completed_at_2625c888")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.ebsVolumeScanDetails.scanDetections.highestSeverityThreatDetails.count") {
                if let Some(val) = event.get("json.service.ebsVolumeScanDetails.scanDetections.highestSeverityThreatDetails.count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.ebsVolumeScanDetails.scanDetections.highestSeverityThreatDetails.count".into(),
                            message,
                        })?;
                    event.set("aws.guardduty.service.ebs_volume_scan_details.scan.detections.highest_severity_threat_details.count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_ebsVolumeScanDetails_scanDetections_highestSeverityThreatDetails_count_to_aws_guardduty_service_ebs_volume_scan_details_scan_detections_highest_severity_threat_details_count_95ce94a2")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.ebsVolumeScanDetails.scanDetections.highestSeverityThreatDetails.severity") {
                    event.rename("json.service.ebsVolumeScanDetails.scanDetections.highestSeverityThreatDetails.severity", "aws.guardduty.service.ebs_volume_scan_details.scan.detections.highest_severity_threat_details.severity")?;
                }

            if event.has("json.service.ebsVolumeScanDetails.scanDetections.highestSeverityThreatDetails.threatName") {
                    event.rename("json.service.ebsVolumeScanDetails.scanDetections.highestSeverityThreatDetails.threatName", "aws.guardduty.service.ebs_volume_scan_details.scan.detections.highest_severity_threat_details.threat_name")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.files",
                ) {
                    if let Some(val) = event.get(
                        "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.files",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.files".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.ebs_volume_scan_details.scan.detections.scanned_item_count.files", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_ebsVolumeScanDetails_scanDetections_scannedItemCount_files_to_aws_guardduty_service_ebs_volume_scan_details_scan_detections_scanned_item_count_files_f1a18931")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.totalGb",
                ) {
                    if let Some(val) = event.get(
                        "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.totalGb",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.totalGb".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.ebs_volume_scan_details.scan.detections.scanned_item_count.total_gb", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_ebsVolumeScanDetails_scanDetections_scannedItemCount_totalGb_to_aws_guardduty_service_ebs_volume_scan_details_scan_detections_scanned_item_count_total_gb_1d9cf176")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.volumes",
                ) {
                    if let Some(val) = event.get(
                        "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.volumes",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.ebsVolumeScanDetails.scanDetections.scannedItemCount.volumes".into(),
                            message,
                        })?;
                        event.set("aws.guardduty.service.ebs_volume_scan_details.scan.detections.scanned_item_count.volumes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_ebsVolumeScanDetails_scanDetections_scannedItemCount_volumes_to_aws_guardduty_service_ebs_volume_scan_details_scan_detections_scanned_item_count_volumes_7ac12249")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.itemCount") {
                if let Some(val) = event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.itemCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.itemCount".into(),
                            message,
                        })?;
                    event.set("aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.item_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_ebsVolumeScanDetails_scanDetections_threatDetectedByName_itemCount_to_aws_guardduty_service_ebs_volume_scan_details_scan_detections_threat_detected_by_name_item_count_9a021199")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.shortened") {
                if let Some(val) = event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.shortened") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.shortened".into(),
                            message,
                        })?;
                    event.set("aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.shortened", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_ebsVolumeScanDetails_scanDetections_threatDetectedByName_shortened_to_aws_guardduty_service_ebs_volume_scan_details_scan_detections_threat_detected_by_name_shortened_15f0e16c")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames",
                    |event| {
                        if event.has_value("_ingest._value.filePaths") {
                            foreach_array(event, "_ingest._value.filePaths", |event| {
                                if event.has("_ingest._value.fileName") {
                                    event.rename(
                                        "_ingest._value.fileName",
                                        "_ingest._value.file.name",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames",
                    |event| {
                        if event.has_value("_ingest._value.filePaths") {
                            foreach_array(event, "_ingest._value.filePaths", |event| {
                                if event.has("_ingest._value.filePath") {
                                    event.rename(
                                        "_ingest._value.filePath",
                                        "_ingest._value.file.path",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames",
                    |event| {
                        if event.has_value("_ingest._value.filePaths") {
                            foreach_array(event, "_ingest._value.filePaths", |event| {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value.hash")
                                            .map_or_else(String::new, painless_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames",
                    |event| {
                        if event.has_value("_ingest._value.filePaths") {
                            foreach_array(event, "_ingest._value.filePaths", |event| {
                                if event.has("_ingest._value.volumeArn") {
                                    event.rename(
                                        "_ingest._value.volumeArn",
                                        "_ingest._value.volume_arn",
                                    )?;
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames",
                    |event| {
                        if event.has("_ingest._value.filePaths") {
                            event
                                .rename("_ingest._value.filePaths", "_ingest._value.file_paths")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames",
                    |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.itemCount") {
                                if let Some(val) = event.get("_ingest._value.itemCount") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.itemCount".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.item_count", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames",
                    |event| {
                        event.remove("_ingest._value.itemCount");
                        Ok(())
                    },
                )?;
            }

            if event.has(
                "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames",
            ) {
                event.rename("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.threatNames", "aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.uniqueThreatNameCount") {
                if let Some(val) = event.get("json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.uniqueThreatNameCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.ebsVolumeScanDetails.scanDetections.threatDetectedByName.uniqueThreatNameCount".into(),
                            message,
                        })?;
                    event.set("aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.unique_threat_name_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_ebsVolumeScanDetails_scanDetections_threatDetectedByName_uniqueThreatNameCount_to_aws_guardduty_service_ebs_volume_scan_details_scan_detections_threat_detected_by_name_unique_threat_name_count_3e98a3cf")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.ebsVolumeScanDetails.scanDetections.threatsDetectedItemCount.files") {
                if let Some(val) = event.get("json.service.ebsVolumeScanDetails.scanDetections.threatsDetectedItemCount.files") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.service.ebsVolumeScanDetails.scanDetections.threatsDetectedItemCount.files".into(),
                            message,
                        })?;
                    event.set("aws.guardduty.service.ebs_volume_scan_details.scan.detections.threats_detected_item_count.files", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_ebsVolumeScanDetails_scanDetections_threatsDetectedItemCount_files_to_aws_guardduty_service_ebs_volume_scan_details_scan_detections_threats_detected_item_count_files_b421743e")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.ebsVolumeScanDetails.scanId") {
                event.rename(
                    "json.service.ebsVolumeScanDetails.scanId",
                    "aws.guardduty.service.ebs_volume_scan_details.scan.id",
                )?;
            }

            let _cond = { event.has_value("json.service.ebsVolumeScanDetails.scanStartedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.service.ebsVolumeScanDetails.scanStartedAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.guardduty.service.ebs_volume_scan_details.scan.started_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_service_ebsVolumeScanDetails_scanStartedAt_to_aws_guardduty_service_ebs_volume_scan_details_scan_started_at_40194f94")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.service.ebsVolumeScanDetails.sources") {
                event.rename(
                    "json.service.ebsVolumeScanDetails.sources",
                    "aws.guardduty.service.ebs_volume_scan_details.sources",
                )?;
            }

            if event.has("json.service.ebsVolumeScanDetails.triggerFindingId") {
                event.rename(
                    "json.service.ebsVolumeScanDetails.triggerFindingId",
                    "aws.guardduty.service.ebs_volume_scan_details.trigger_finding_id",
                )?;
            }

            let _cond = { event.has_value("json.service.eventFirstSeen") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.service.eventFirstSeen") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.guardduty.service.event.first_seen", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_service_eventFirstSeen_to_aws_guardduty_service_event_first_seen_97f7a6e3")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            let _cond = { event.has_value("json.service.eventLastSeen") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.service.eventLastSeen") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.guardduty.service.event.last_seen", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_service_eventLastSeen_to_aws_guardduty_service_event_last_seen_b0600769")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                event
                    .get("json.service.evidence.threatIntelligenceDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.evidence.threatIntelligenceDetails",
                    |event| {
                        if event.has("_ingest._value.threatListName") {
                            event.rename(
                                "_ingest._value.threatListName",
                                "_ingest._value.threat.list_name",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.service.evidence.threatIntelligenceDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.service.evidence.threatIntelligenceDetails",
                    |event| {
                        if event.has("_ingest._value.threatNames") {
                            event.rename(
                                "_ingest._value.threatNames",
                                "_ingest._value.threat.names",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has("json.service.evidence.threatIntelligenceDetails") {
                event.rename(
                    "json.service.evidence.threatIntelligenceDetails",
                    "aws.guardduty.service.evidence.threat_intelligence_details",
                )?;
            }

            if event.has("json.service.featureName") {
                event.rename(
                    "json.service.featureName",
                    "aws.guardduty.service.feature_name",
                )?;
            }

            if event.has("json.service.resourceRole") {
                event.rename(
                    "json.service.resourceRole",
                    "aws.guardduty.service.resource_role",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.addressFamily") {
                event.rename(
                    "json.service.runtimeDetails.context.addressFamily",
                    "aws.guardduty.service.runtime_details.context.address_family",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.commandLineExample") {
                event.rename(
                    "json.service.runtimeDetails.context.commandLineExample",
                    "aws.guardduty.service.runtime_details.context.command_line_example",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.fileSystemType") {
                event.rename(
                    "json.service.runtimeDetails.context.fileSystemType",
                    "aws.guardduty.service.runtime_details.context.file_system_type",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.flags") {
                event.rename(
                    "json.service.runtimeDetails.context.flags",
                    "aws.guardduty.service.runtime_details.context.flags",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.runtimeDetails.context.ianaProtocolNumber") {
                    if let Some(val) =
                        event.get("json.service.runtimeDetails.context.ianaProtocolNumber")
                    {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.runtimeDetails.context.ianaProtocolNumber"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.runtime_details.context.iana_protocol_number",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_runtimeDetails_context_ianaProtocolNumber_to_aws_guardduty_service_runtime_details_context_iana_protocol_number_d6e2598a")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.runtimeDetails.context.ldPreloadValue") {
                event.rename(
                    "json.service.runtimeDetails.context.ldPreloadValue",
                    "aws.guardduty.service.runtime_details.context.ld_preload",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.libraryPath") {
                event.rename(
                    "json.service.runtimeDetails.context.libraryPath",
                    "aws.guardduty.service.runtime_details.context.library_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.memoryRegions") {
                event.rename(
                    "json.service.runtimeDetails.context.memoryRegions",
                    "aws.guardduty.service.runtime_details.context.memory_regions",
                )?;
            }

            let _cond = { event.has_value("json.service.runtimeDetails.context.modifiedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.service.runtimeDetails.context.modifiedAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.guardduty.service.runtime_details.context.modified_at",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_service_runtimeDetails_context_modifiedAt_to_aws_guardduty_service_runtime_details_context_modified_at_d093de10")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.service.runtimeDetails.context.modifyingProcess") {
                event.rename(
                    "json.service.runtimeDetails.context.modifyingProcess",
                    "aws.guardduty.service.runtime_details.context.modifying_process",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.moduleFilePath") {
                event.rename(
                    "json.service.runtimeDetails.context.moduleFilePath",
                    "aws.guardduty.service.runtime_details.context.module_file_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.moduleName") {
                event.rename(
                    "json.service.runtimeDetails.context.moduleName",
                    "aws.guardduty.service.runtime_details.context.module_name",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.moduleSha256") {
                event.rename(
                    "json.service.runtimeDetails.context.moduleSha256",
                    "aws.guardduty.service.runtime_details.context.module_sha256",
                )?;
            }

            let _cond =
                { event.has_value("aws.guardduty.service.runtime_details.context.module_sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("aws.guardduty.service.runtime_details.context.module_sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.service.runtimeDetails.context.mountSource") {
                event.rename(
                    "json.service.runtimeDetails.context.mountSource",
                    "aws.guardduty.service.runtime_details.context.mount_source",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.mountTarget") {
                event.rename(
                    "json.service.runtimeDetails.context.mountTarget",
                    "aws.guardduty.service.runtime_details.context.mount_target",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.releaseAgentPath") {
                event.rename(
                    "json.service.runtimeDetails.context.releaseAgentPath",
                    "aws.guardduty.service.runtime_details.context.release_agent_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.runcBinaryPath") {
                event.rename(
                    "json.service.runtimeDetails.context.runcBinaryPath",
                    "aws.guardduty.service.runtime_details.context.runc_binary_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.scriptPath") {
                event.rename(
                    "json.service.runtimeDetails.context.scriptPath",
                    "aws.guardduty.service.runtime_details.context.script_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.serviceName") {
                event.rename(
                    "json.service.runtimeDetails.context.serviceName",
                    "aws.guardduty.service.runtime_details.context.service_name",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.shellHistoryFilePath") {
                event.rename(
                    "json.service.runtimeDetails.context.shellHistoryFilePath",
                    "aws.guardduty.service.runtime_details.context.shell_history_file_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.socketPath") {
                event.rename(
                    "json.service.runtimeDetails.context.socketPath",
                    "aws.guardduty.service.runtime_details.context.socket_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.targetProcess") {
                event.rename(
                    "json.service.runtimeDetails.context.targetProcess",
                    "aws.guardduty.service.runtime_details.context.target_process",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.threatFilePath") {
                event.rename(
                    "json.service.runtimeDetails.context.threatFilePath",
                    "aws.guardduty.service.runtime_details.context.threat_file_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.toolCategory") {
                event.rename(
                    "json.service.runtimeDetails.context.toolCategory",
                    "aws.guardduty.service.runtime_details.context.tool_category",
                )?;
            }

            if event.has("json.service.runtimeDetails.context.toolName") {
                event.rename(
                    "json.service.runtimeDetails.context.toolName",
                    "aws.guardduty.service.runtime_details.context.tool_name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.runtimeDetails.process.euid") {
                    if let Some(val) = event.get("json.service.runtimeDetails.process.euid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.runtimeDetails.process.euid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.runtime_details.process.euid",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_runtimeDetails_process_euid_to_aws_guardduty_service_runtime_details_process_euid_3a9e2743")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.runtimeDetails.process.executablePath") {
                event.rename(
                    "json.service.runtimeDetails.process.executablePath",
                    "aws.guardduty.service.runtime_details.process.executable_path",
                )?;
            }

            if event.has("json.service.runtimeDetails.process.executableSha256") {
                event.rename(
                    "json.service.runtimeDetails.process.executableSha256",
                    "aws.guardduty.service.runtime_details.process.executable_sha256",
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.runtime_details.process.executable_sha256")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("aws.guardduty.service.runtime_details.process.executable_sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.service.runtimeDetails.process.lineage") {
                event.rename(
                    "json.service.runtimeDetails.process.lineage",
                    "aws.guardduty.service.runtime_details.process.lineage",
                )?;
            }

            if event.has("json.service.runtimeDetails.process.name") {
                event.rename(
                    "json.service.runtimeDetails.process.name",
                    "aws.guardduty.service.runtime_details.process.name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.runtimeDetails.process.namespacePid") {
                    if let Some(val) = event.get("json.service.runtimeDetails.process.namespacePid")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.runtimeDetails.process.namespacePid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.runtime_details.process.namespace_pid",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_runtimeDetails_process_namespacePid_to_aws_guardduty_service_runtime_details_process_namespace_pid_9077017c")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.runtimeDetails.process.parentUuid") {
                event.rename(
                    "json.service.runtimeDetails.process.parentUuid",
                    "aws.guardduty.service.runtime_details.process.parent_uuid",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.runtimeDetails.process.pid") {
                    if let Some(val) = event.get("json.service.runtimeDetails.process.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.runtimeDetails.process.pid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.runtime_details.process.pid",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_runtimeDetails_process_pid_to_aws_guardduty_service_runtime_details_process_pid_3825c7e3")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.runtimeDetails.process.pwd") {
                event.rename(
                    "json.service.runtimeDetails.process.pwd",
                    "aws.guardduty.service.runtime_details.process.pwd",
                )?;
            }

            let _cond = { event.has_value("json.service.runtimeDetails.process.startTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.service.runtimeDetails.process.startTime")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.guardduty.service.runtime_details.process.start_time",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_service_runtimeDetails_process_startTime_to_aws_guardduty_service_runtime_details_process_start_time_6b8d1c9f")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.service.runtimeDetails.process.user") {
                event.rename(
                    "json.service.runtimeDetails.process.user",
                    "aws.guardduty.service.runtime_details.process.user",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.service.runtimeDetails.process.userId") {
                    if let Some(val) = event.get("json.service.runtimeDetails.process.userId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.service.runtimeDetails.process.userId".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.guardduty.service.runtime_details.process.user_id",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_service_runtimeDetails_process_userId_to_aws_guardduty_service_runtime_details_process_user_id_3b9bf15a")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.service.runtimeDetails.process.uuid") {
                event.rename(
                    "json.service.runtimeDetails.process.uuid",
                    "aws.guardduty.service.runtime_details.process.uuid",
                )?;
            }

            if event.has("json.service.serviceName") {
                event.rename(
                    "json.service.serviceName",
                    "aws.guardduty.service.service_name",
                )?;
            }

            if event.has("json.service.userFeedback") {
                event.rename(
                    "json.service.userFeedback",
                    "aws.guardduty.service.user_feedback",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.severity") {
                    if let Some(val) = event.get("json.severity") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severity".into(),
                                message,
                            }
                        })?;
                        event.set("aws.guardduty.severity.code", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_severity_to_aws_guardduty_severity_code_7415835e",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // SKIPPED: condition not transpiled: ctx.aws?.guardduty?.severity?.code != null && ctx.aws.guardduty.severity.code <= 8.9 && ctx.aws.guardduty.severity.code >= 7.0
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("aws.guardduty.severity.value", json!("High"))?;
            }

            // SKIPPED: condition not transpiled: ctx.aws?.guardduty?.severity?.code != null && ctx.aws.guardduty.severity.code <= 6.9 && ctx.aws.guardduty.severity.code >= 4.0
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("aws.guardduty.severity.value", json!("Medium"))?;
            }

            // SKIPPED: condition not transpiled: ctx.aws?.guardduty?.severity?.code != null && ctx.aws.guardduty.severity.code <= 3.9 && ctx.aws.guardduty.severity.code >= 1.0
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("aws.guardduty.severity.value", json!("Low"))?;
            }

            if event.has("json.title") {
                event.rename("json.title", "aws.guardduty.title")?;
            }

            if event.has("json.type") {
                event.rename("json.type", "aws.guardduty.type")?;
            }

            let _cond = { event.has_value("json.updatedAt") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updatedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.guardduty.updated_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_json_updatedAt_to_aws_guardduty_updated_at_709b31c4",
                    )?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if let Some(v) = event
                .get("aws.guardduty.updated_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.account_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.partition")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.service_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.service.name", v)?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.resource.ecs_cluster_details.task_details.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.resource.ecs_cluster_details.task_details.containers",
                    |event| {
                        event.append_unique(
                            "container.id",
                            json!(
                                event
                                    .get("_ingest._value.id")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.resource.ecs_cluster_details.task_details.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.resource.ecs_cluster_details.task_details.containers",
                    |event| {
                        event.append_unique(
                            "container.name",
                            json!(
                                event
                                    .get("_ingest._value.name")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.resource.ecs_cluster_details.task_details.containers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.resource.ecs_cluster_details.task_details.containers",
                    |event| {
                        event.append_unique(
                            "container.runtime",
                            json!(
                                event
                                    .get("_ingest._value.container_runtime")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.ip_address_v4")
            };
            if _cond {
                event.append_unique("source.address", json!(event.get("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.ip_address_v4")
            };
            if _cond {
                event.append_unique("source.address", json!(event.get("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.connection_direction") && event.get_str("aws.guardduty.service.action.network_connection_action.connection_direction").is_some_and(|s| s.to_lowercase() == "inbound")
            };
            if _cond {
                event.append_unique("source.address", json!(event.get("aws.guardduty.service.action.network_connection_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.connection_direction") && event.get_str("aws.guardduty.service.action.network_connection_action.connection_direction").is_some_and(|s| s.to_lowercase() == "outbound")
            };
            if _cond {
                event.append_unique("destination.address", json!(event.get("aws.guardduty.service.action.network_connection_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.connection_direction") && event.get_str("aws.guardduty.service.action.network_connection_action.connection_direction").is_some_and(|s| s.to_lowercase() == "outbound")
            };
            if _cond {
                event.append_unique("source.address", json!(event.get("aws.guardduty.service.action.network_connection_action.local_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.connection_direction") && event.get_str("aws.guardduty.service.action.network_connection_action.connection_direction").is_some_and(|s| s.to_lowercase() == "inbound")
            };
            if _cond {
                event.append_unique("destination.address", json!(event.get("aws.guardduty.service.action.network_connection_action.local_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.append_unique(
                            "source.address",
                            json!(
                                event
                                    .get("_ingest._value.remote_ip_details.ip_address_v4")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.ip_address_v4")
            };
            if _cond {
                event.append_unique("source.address", json!(event.get("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.ip_address_v4").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.asn")
            };
            if _cond {
                event.append_unique("source.as.number", json!(event.get("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.asn").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.asn")
            };
            if _cond {
                event.append_unique("source.as.number", json!(event.get("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.asn").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.asn")
            };
            if _cond {
                event.append_unique("source.as.number", json!(event.get("aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.asn").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.append_unique(
                            "source.as.number",
                            json!(
                                event
                                    .get("_ingest._value.remote_ip_details.organization.asn")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.asn")
            };
            if _cond {
                event.append_unique("source.as.number", json!(event.get("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.asn").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.get("source.as.number").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("source.as.number").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            if event.remove("_ingest._value").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value".into(),
                                });
                            }
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("source.as.number", Value::Array(out))?;
                }
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.asnorg")
            };
            if _cond {
                event.append_unique("source.as.organization.name", json!(event.get("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.asnorg").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.asnorg")
            };
            if _cond {
                event.append_unique("source.as.organization.name", json!(event.get("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.asnorg").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.asnorg")
            };
            if _cond {
                event.append_unique("source.as.organization.name", json!(event.get("aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.asnorg").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.append_unique(
                            "source.as.organization.name",
                            json!(
                                event
                                    .get("_ingest._value.remote_ip_details.organization.asnorg")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.asnorg")
            };
            if _cond {
                event.append_unique("source.as.organization.name", json!(event.get("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.asnorg").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value(
                    "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.city.name",
                )
            };
            if _cond {
                event.append_unique("source.geo.city_name", json!(event.get("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.city.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.city.name")
            };
            if _cond {
                event.append_unique("source.geo.city_name", json!(event.get("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.city.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.remote_ip_details.city.name")
            };
            if _cond {
                event.append_unique("source.geo.city_name", json!(event.get("aws.guardduty.service.action.network_connection_action.remote_ip_details.city.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.append_unique(
                            "source.geo.city_name",
                            json!(
                                event
                                    .get("_ingest._value.remote_ip_details.city.name")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.city.name")
            };
            if _cond {
                event.append_unique("source.geo.city_name", json!(event.get("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.city.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.country.code")
            };
            if _cond {
                event.append_unique("source.geo.country_iso_code", json!(event.get("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.country.code").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.country.code")
            };
            if _cond {
                event.append_unique("source.geo.country_iso_code", json!(event.get("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.country.code").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.remote_ip_details.country.code")
            };
            if _cond {
                event.append_unique("source.geo.country_iso_code", json!(event.get("aws.guardduty.service.action.network_connection_action.remote_ip_details.country.code").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.append_unique(
                            "source.geo.country_iso_code",
                            json!(
                                event
                                    .get("_ingest._value.remote_ip_details.country.code")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.country.code")
            };
            if _cond {
                event.append_unique("source.geo.country_iso_code", json!(event.get("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.country.code").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.country.name")
            };
            if _cond {
                event.append_unique("source.geo.country_name", json!(event.get("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.country.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.country.name")
            };
            if _cond {
                event.append_unique("source.geo.country_name", json!(event.get("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.country.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.remote_ip_details.country.name")
            };
            if _cond {
                event.append_unique("source.geo.country_name", json!(event.get("aws.guardduty.service.action.network_connection_action.remote_ip_details.country.name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.append_unique(
                            "source.geo.country_name",
                            json!(
                                event
                                    .get("_ingest._value.remote_ip_details.country.name")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.country.name")
            };
            if _cond {
                event.append_unique("source.geo.country_name", json!(event.get("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.country.name").map_or_else(String::new, painless_to_string)))?;
            }

            // Painless script
            // Source: def locationList = new ArrayList();\nif (ctx.aws?.guardduty?.service?.action?.aws_api_call_action?.remote_ip_details?.geo_location != null) {\n  locationList.add(ctx.aws.guardduty.service.action.aws_api_call_action.remote_ip_details.geo_location);\n}\nif (ctx.aws?.guardduty?.service?.action?.kubernetes_api_call_action?.remote_ip_details?.geo_location != null) {\n  locationList.add(ctx.aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.geo_location);\n}\nif (ctx.aws?.guardduty?.service?.action?.network_connection_action?.remote_ip_details?.geo_location != null) {\n  locationList.add(ctx.aws.guardduty.service.action.network_connection_action.remote_ip_details.geo_location);\n}\nif (ctx.aws?.guardduty?.service?.action?.port_probe_action?.port_probe_details instanceof List) {\n  for (list in ctx.aws.guardduty.service.action.port_probe_action.port_probe_details) {\n    locationList.add(list.remote_ip_details.geo_location);\n  }\n}\nif (ctx.aws?.guardduty?.service?.action?.rds_login_attempt_action?.remote_ip_details?.geo_location != null) {\n  locationList.add(ctx.aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.geo_location);\n}\nif (!(ctx.source instanceof HashMap)) {\n  ctx.source = new HashMap();\n}\nif (!(ctx.source.geo instanceof HashMap)) {\n  ctx.source.geo = new HashMap();\n}\nctx.source.geo.location = locationList;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def locationList = new ArrayList();\nif (ctx.aws?.guardduty?.service?.action?.aws_api_call_action?.remote_ip_details?.geo_location != null) {\n  locationList.add(ctx.aws.guardduty.service.action.aws_api_call_action.remote_ip_details.geo_location);\n}\nif (ctx.aws?.guardduty?.service?.action?.kubernetes_api_call_action?.remote_ip_details?.geo_location != null) {\n  locationList.add(ctx.aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.geo_location);\n}\nif (ctx.aws?.guardduty?.service?.action?.network_connection_action?.remote_ip_details?.geo_location != null) {\n  locationList.add(ctx.aws.guardduty.service.action.network_connection_action.remote_ip_details.geo_location);\n}\nif (ctx.aws?.guardduty?.service?.action?.port_probe_action?.port_probe_details instanceof List) {\n  for (list in ctx.aws.guardduty.service.action.port_probe_action.port_probe_details) {\n    locationList.add(list.remote_ip_details.geo_location);\n  }\n}\nif (ctx.aws?.guardduty?.service?.action?.rds_login_attempt_action?.remote_ip_details?.geo_location != null) {\n  locationList.add(ctx.aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.geo_location);\n}\nif (!(ctx.source instanceof HashMap)) {\n  ctx.source = new HashMap();\n}\nif (!(ctx.source.geo instanceof HashMap)) {\n  ctx.source.geo = new HashMap();\n}\nctx.source.geo.location = locationList;\n"#
                ),
            )?;

            if let Some(v) = event
                .get("aws.guardduty.service.action.dns_request_action.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.name", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.action.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.event.last_seen")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.action.aws_api_call_action.service_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.severity") {
                    if let Some(val) = event.get("json.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severity".into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_severity_to_event_severity_649b84d8",
                )?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("aws.guardduty.service.event.first_seen")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = {
                event.get("aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names",
                    |event| {
                        if event.has_value("_ingest._value.file_paths") {
                            foreach_array(event, "_ingest._value.file_paths", |event| {
                                event.append_unique(
                                    "file.hash.sha256",
                                    json!(
                                        event
                                            .get("_ingest._value.hash")
                                            .map_or_else(String::new, painless_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names",
                    |event| {
                        if event.has_value("_ingest._value.file_paths") {
                            foreach_array(event, "_ingest._value.file_paths", |event| {
                                event.append_unique(
                                    "file.name",
                                    json!(
                                        event
                                            .get("_ingest._value.file.name")
                                            .map_or_else(String::new, painless_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names",
                    |event| {
                        if event.has_value("_ingest._value.file_paths") {
                            foreach_array(event, "_ingest._value.file_paths", |event| {
                                event.append_unique(
                                    "file.path",
                                    json!(
                                        event
                                            .get("_ingest._value.file.path")
                                            .map_or_else(String::new, painless_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.instance_details.instance.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.instance_details.instance.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.instance_details.platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.platform", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.instance_details.instance.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.machine.type", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.instance_details.instance.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.runtime_details.context.iana_protocol_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.iana_number", v)?;
            }

            if event.has_value(
                "aws.guardduty.service.action.network_connection_action.connection_direction",
            ) {
                if let Some(s) = event.get_string(
                    "aws.guardduty.service.action.network_connection_action.connection_direction",
                ) {
                    let lowered = s.to_lowercase();
                    event.set("network.direction", lowered)?;
                }
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.transport")
            };
            if _cond {
                event.append_unique(
                    "network.transport",
                    json!(
                        event
                            .get("aws.guardduty.service.action.network_connection_action.transport")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("aws.guardduty.service.action.dns_request_action.protocol") };
            if _cond {
                event.append_unique(
                    "network.transport",
                    json!(
                        event
                            .get("aws.guardduty.service.action.dns_request_action.protocol")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has_value("network.transport") {
                if let Some(s) = event.get_string("network.transport") {
                    let lowered = s.to_lowercase();
                    event.set("network.transport", lowered)?;
                }
            }

            if let Some(v) = event.get("aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.name_space").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("orchestrator.namespace", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.name", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.type", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.runtime_details.process.executable_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.runtime_details.process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.runtime_details.process.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.runtime_details.process.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.start", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.runtime_details.process.pwd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.working_directory", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("rule.name") {
                if let Some(input) = event.get_string("rule.name") {
                    // Grok pattern: (?P<rule_ruleset>(?:%{WORD:rule.category}:%{WORD}))
                    let _ = cached_grok_mapped!(
                        "(?P<rule_ruleset>(?:%{WORD:rule.category}:%{WORD}))",
                        [("rule_ruleset", "rule.ruleset")]
                    )
                    .extract_into(&input, event)?;
                }
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.append_unique(
                            "source.address",
                            json!(
                                event
                                    .get("_ingest._value.local_ip_details.ip_address_v4")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.kubernetes_api_call_action.source_ips")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.kubernetes_api_call_action.source_ips",
                    |event| {
                        event.append_unique(
                            "source.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("aws.guardduty.service.action.network_connection_action.local_port_details.port.value")
            };
            if _cond {
                event.append_unique("source.port", json!(event.get("aws.guardduty.service.action.network_connection_action.local_port_details.port.value").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.append_unique(
                            "source.port",
                            json!(
                                event
                                    .get("_ingest._value.local_port_details.port.value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = { event.get("source.port").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "source.port", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.runtime_details.context.threat_file_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.file.path", v)?;
            }

            if let Some(v) = event
                .get("aws.guardduty.service.runtime_details.context.tool_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.software.name", v)?;
            }

            let _cond =
                { event.has_value("aws.guardduty.resource.access_key_details.principal_id") };
            if _cond {
                event.append_unique(
                    "user.id",
                    json!(
                        event
                            .get("aws.guardduty.resource.access_key_details.principal_id")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.uid",
                )
            };
            if _cond {
                event.append_unique("user.id", json!(event.get("aws.guardduty.resource.kubernetes_details.kubernetes_user_details.uid").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("aws.guardduty.resource.access_key_details.user.name") };
            if _cond {
                event.append_unique(
                    "user.name",
                    json!(
                        event
                            .get("aws.guardduty.resource.access_key_details.user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.user_name",
                )
            };
            if _cond {
                event.append_unique("user.name", json!(event.get("aws.guardduty.resource.kubernetes_details.kubernetes_user_details.user_name").map_or_else(String::new, painless_to_string)))?;
            }

            let _cond = { event.has_value("aws.guardduty.resource.rdsdb_user_details.user") };
            if _cond {
                event.append_unique(
                    "user.name",
                    json!(
                        event
                            .get("aws.guardduty.resource.rdsdb_user_details.user")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("aws.guardduty.resource.kubernetes_details.kubernetes_user_details.groups")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.roles", v)?;
            }

            event.remove("json");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("aws.guardduty.account_id");
                event.remove("aws.guardduty.created_at");
                event.remove("aws.guardduty.description");
                event.remove("aws.guardduty.id");
                event.remove("aws.guardduty.partition");
                event.remove("aws.guardduty.region");
                event.remove("aws.guardduty.resource.access_key_details.principal_id");
                event.remove("aws.guardduty.resource.access_key_details.user.name");
                event.remove("aws.guardduty.resource.instance_details.instance.id");
                event.remove("aws.guardduty.resource.instance_details.instance.type");
                event.remove("aws.guardduty.resource.instance_details.platform");
                event.remove(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.groups",
                );
                event.remove(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.uid",
                );
                event.remove(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_user_details.user_name",
                );
                event.remove(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.name",
                );
                event.remove("aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.name_space");
                event.remove(
                    "aws.guardduty.resource.kubernetes_details.kubernetes_workload_details.type",
                );
                event.remove("aws.guardduty.resource.rdsdb_user_details.user");
                event.remove(
                    "aws.guardduty.service.action.aws_api_call_action.remote_ip_details.city.name",
                );
                event.remove("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.country.code");
                event.remove("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.country.name");
                event.remove("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.geo_location");
                event.remove("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.ip_address_v4");
                event.remove("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.asn");
                event.remove("aws.guardduty.service.action.aws_api_call_action.remote_ip_details.organization.asnorg");
                event.remove("aws.guardduty.service.action.aws_api_call_action.service_name");
                event.remove("aws.guardduty.service.action.dns_request_action.domain");
                event.remove("aws.guardduty.service.action.dns_request_action.protocol");
                event.remove("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.city.name");
                event.remove("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.country.code");
                event.remove("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.country.name");
                event.remove("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.geo_location");
                event.remove("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.ip_address_v4");
                event.remove("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.asn");
                event.remove("aws.guardduty.service.action.kubernetes_api_call_action.remote_ip_details.organization.asnorg");
                event.remove("aws.guardduty.service.action.kubernetes_api_call_action.source_ips");
                event.remove(
                    "aws.guardduty.service.action.network_connection_action.connection_direction",
                );
                event.remove("aws.guardduty.service.action.network_connection_action.local_ip_details.ip_address_v4");
                event.remove("aws.guardduty.service.action.network_connection_action.local_port_details.port.value");
                event.remove("aws.guardduty.service.action.network_connection_action.remote_ip_details.city.name");
                event.remove("aws.guardduty.service.action.network_connection_action.remote_ip_details.country.code");
                event.remove("aws.guardduty.service.action.network_connection_action.remote_ip_details.country.name");
                event.remove("aws.guardduty.service.action.network_connection_action.remote_ip_details.geo_location");
                event.remove("aws.guardduty.service.action.network_connection_action.remote_ip_details.ip_address_v4");
                event.remove("aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.asn");
                event.remove("aws.guardduty.service.action.network_connection_action.remote_ip_details.organization.asnorg");
                event.remove("aws.guardduty.service.action.network_connection_action.transport");
                event.remove("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.ip_address_v4");
                event.remove("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.asn");
                event.remove("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.organization.asnorg");
                event.remove("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.country.code");
                event.remove("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.country.name");
                event.remove("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.city.name");
                event.remove("aws.guardduty.service.action.rds_login_attempt_action.remote_ip_details.geo_location");
                event.remove("aws.guardduty.service.action.type");
                event.remove("aws.guardduty.service.event.first_seen");
                event.remove("aws.guardduty.service.event.last_seen");
                event.remove("aws.guardduty.service.runtime_details.context.iana_protocol_number");
                event.remove("aws.guardduty.service.runtime_details.context.threat_file_path");
                event.remove("aws.guardduty.service.runtime_details.context.tool_name");
                event.remove("aws.guardduty.service.runtime_details.process.executable_path");
                event.remove("aws.guardduty.service.runtime_details.process.name");
                event.remove("aws.guardduty.service.runtime_details.process.pid");
                event.remove("aws.guardduty.service.runtime_details.process.pwd");
                event.remove("aws.guardduty.service.runtime_details.process.start_time");
                event.remove("aws.guardduty.service.service_name");
                event.remove("aws.guardduty.type");
                event.remove("aws.guardduty.updated_at");
            }

            let _cond = {
                event
                    .get("aws.guardduty.resource.ecs_cluster_details.task_details.containers")
                    .is_some_and(|v| v.is_array())
                    && (!event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        })))
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.resource.ecs_cluster_details.task_details.containers",
                    |event| {
                        event.remove("_ingest._value.container_runtime");
                        event.remove("_ingest._value.id");
                        event.remove("_ingest._value.name");
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("aws.guardduty.service.action.port_probe_action.port_probe_details")
                    .is_some_and(|v| v.is_array())
                    && (!event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        })))
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.action.port_probe_action.port_probe_details",
                    |event| {
                        event.remove("_ingest._value.local_ip_details.ip_address_v4");
                        event.remove("_ingest._value.local_port_details.port.value");
                        event.remove("_ingest._value.remote_ip_details.city.name");
                        event.remove("_ingest._value.remote_ip_details.country.code");
                        event.remove("_ingest._value.remote_ip_details.country.name");
                        event.remove("_ingest._value.remote_ip_details.geo_location");
                        event.remove("_ingest._value.remote_ip_details.ip_address_v4");
                        event.remove("_ingest._value.remote_ip_details.organization.asn");
                        event.remove("_ingest._value.remote_ip_details.organization.asnorg");
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names").is_some_and(|v| v.is_array()) && (!event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })))
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.guardduty.service.ebs_volume_scan_details.scan.detections.threat_detected_by_name.threat_names",
                    |event| {
                        if event.has_value("_ingest._value.file_paths") {
                            foreach_array(event, "_ingest._value.file_paths", |event| {
                                event.remove("_ingest._value.file.name");
                                event.remove("_ingest._value.file.path");
                                event.remove("_ingest._value.hash");
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
