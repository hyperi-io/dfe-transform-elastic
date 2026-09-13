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
            event.set("ecs.version", json!("9.3.0"))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.category", json!("host"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.properties.CreatedDate") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.properties.DeviceId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.properties.LastContact") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.tenantId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.time") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.properties.AADTenantId") {
                event.rename(
                    "json.properties.AADTenantId",
                    "microsoft_intune.managed_device.properties.aad_tenant_id",
                )?;
            }

            if event.has_value("json.properties.AndroidPatchLevel") {
                event.rename(
                    "json.properties.AndroidPatchLevel",
                    "microsoft_intune.managed_device.properties.android_patch_level",
                )?;
            }

            if event.has_value("json.properties.BatchId") {
                event.rename(
                    "json.properties.BatchId",
                    "microsoft_intune.managed_device.properties.batch_id",
                )?;
            }

            if event.has_value("json.properties.CategoryName") {
                event.rename(
                    "json.properties.CategoryName",
                    "microsoft_intune.managed_device.properties.category_name",
                )?;
            }

            if event.has_value("json.properties.CompliantState") {
                event.rename(
                    "json.properties.CompliantState",
                    "microsoft_intune.managed_device.properties.compliant_state",
                )?;
            }

            let _cond = {
                event.has_value("json.properties.CreatedDate")
                    && event.get_str("json.properties.CreatedDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.CreatedDate") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSSSSSS",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSSS",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "microsoft_intune.managed_device.properties.created_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.CreatedDate".into(),
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
                        "date_properties_CreatedDate",
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

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.created_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has_value("json.properties.DeviceId") {
                event.rename(
                    "json.properties.DeviceId",
                    "microsoft_intune.managed_device.properties.device_id",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.device_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if event.has_value("json.properties.DeviceName") {
                event.rename(
                    "json.properties.DeviceName",
                    "microsoft_intune.managed_device.properties.device_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.device_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond =
                { event.has_value("microsoft_intune.managed_device.properties.device_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("microsoft_intune.managed_device.properties.device_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.DeviceRegistrationState") {
                event.rename(
                    "json.properties.DeviceRegistrationState",
                    "microsoft_intune.managed_device.properties.device_registration_state",
                )?;
            }

            if event.has_value("json.properties.DeviceState") {
                event.rename(
                    "json.properties.DeviceState",
                    "microsoft_intune.managed_device.properties.device_state",
                )?;
            }

            if event.has_value("json.properties.EasID") {
                event.rename(
                    "json.properties.EasID",
                    "microsoft_intune.managed_device.properties.eas_id",
                )?;
            }

            if event.has_value("json.properties.EncryptionStatusString") {
                event.rename(
                    "json.properties.EncryptionStatusString",
                    "microsoft_intune.managed_device.properties.encryption_status_string",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.properties.GraphDeviceIsManaged") {
                    if let Some(val) = event.get("json.properties.GraphDeviceIsManaged") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.properties.GraphDeviceIsManaged".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_intune.managed_device.properties.graph_device_is_managed",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_properties_GraphDeviceIsManaged_to_boolean",
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

            if event.has_value("json.properties.IMEI") {
                event.rename(
                    "json.properties.IMEI",
                    "microsoft_intune.managed_device.properties.imei",
                )?;
            }

            let _cond = {
                event.has_value("json.properties.InGracePeriodUntil")
                    && event.get_str("json.properties.InGracePeriodUntil") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.properties.InGracePeriodUntil")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSSSSSS",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSSS",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "microsoft_intune.managed_device.properties.in_grace_period_until",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.InGracePeriodUntil".into(),
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
                        "date_properties_InGracePeriodUntil",
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

            if event.has_value("json.properties.IntuneAccountId") {
                event.rename(
                    "json.properties.IntuneAccountId",
                    "microsoft_intune.managed_device.properties.intune_account_id",
                )?;
            }

            if event.has_value("json.properties.JailBroken") {
                event.rename(
                    "json.properties.JailBroken",
                    "microsoft_intune.managed_device.properties.jail_broken",
                )?;
            }

            if event.has_value("json.properties.JoinType") {
                event.rename(
                    "json.properties.JoinType",
                    "microsoft_intune.managed_device.properties.join_type",
                )?;
            }

            let _cond = {
                event.has_value("json.properties.LastContact")
                    && event.get_str("json.properties.LastContact") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.LastContact") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSSSSSS",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSSSSS",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "microsoft_intune.managed_device.properties.last_contact",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.LastContact".into(),
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
                        "date_properties_LastContact",
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

            if event.has_value("json.properties.MEID") {
                event.rename(
                    "json.properties.MEID",
                    "microsoft_intune.managed_device.properties.meid",
                )?;
            }

            if event.has_value("json.properties.ManagedBy") {
                event.rename(
                    "json.properties.ManagedBy",
                    "microsoft_intune.managed_device.properties.managed_by",
                )?;
            }

            if event.has_value("json.properties.ManagedDeviceName") {
                event.rename(
                    "json.properties.ManagedDeviceName",
                    "microsoft_intune.managed_device.properties.managed_device_name",
                )?;
            }

            if event.has_value("json.properties.Manufacturer") {
                event.rename(
                    "json.properties.Manufacturer",
                    "microsoft_intune.managed_device.properties.manufacturer",
                )?;
            }

            if event.has_value("json.properties.Model") {
                event.rename(
                    "json.properties.Model",
                    "microsoft_intune.managed_device.properties.model",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.name", v)?;
            }

            if event.has_value("json.properties.OS") {
                event.rename(
                    "json.properties.OS",
                    "microsoft_intune.managed_device.properties.os",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if event.has_value("json.properties.OSVersion") {
                event.rename(
                    "json.properties.OSVersion",
                    "microsoft_intune.managed_device.properties.os_version",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.os_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has_value("json.properties.Ownership") {
                event.rename(
                    "json.properties.Ownership",
                    "microsoft_intune.managed_device.properties.ownership",
                )?;
            }

            if event.has_value("json.properties.PhoneNumber") {
                event.rename(
                    "json.properties.PhoneNumber",
                    "microsoft_intune.managed_device.properties.phone_number",
                )?;
            }

            if event.has_value("json.properties.PrimaryUser") {
                event.rename(
                    "json.properties.PrimaryUser",
                    "microsoft_intune.managed_device.properties.primary_user",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.primary_user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond =
                { event.has_value("microsoft_intune.managed_device.properties.primary_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_intune.managed_device.properties.primary_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.ReferenceId") {
                event.rename(
                    "json.properties.ReferenceId",
                    "microsoft_intune.managed_device.properties.reference_id",
                )?;
            }

            if event.has_value("json.properties.SerialNumber") {
                event.rename(
                    "json.properties.SerialNumber",
                    "microsoft_intune.managed_device.properties.serial_number",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.serial_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.serial_number", v)?;
            }

            if event.has_value("json.properties.SkuFamily") {
                event.rename(
                    "json.properties.SkuFamily",
                    "microsoft_intune.managed_device.properties.sku_family",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.properties.StorageFree") {
                    if let Some(val) = event.get("json.properties.StorageFree") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.properties.StorageFree".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_intune.managed_device.properties.storage_free",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_properties_StorageFree_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.properties.StorageTotal") {
                    if let Some(val) = event.get("json.properties.StorageTotal") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.properties.StorageTotal".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_intune.managed_device.properties.storage_total",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_properties_StorageTotal_to_long",
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

            if event.has_value("json.properties.SubscriberCarrierNetwork") {
                event.rename(
                    "json.properties.SubscriberCarrierNetwork",
                    "microsoft_intune.managed_device.properties.subscriber_carrier_network",
                )?;
            }

            if event.has_value("json.properties.SupervisedStatusString") {
                event.rename(
                    "json.properties.SupervisedStatusString",
                    "microsoft_intune.managed_device.properties.supervised_status_string",
                )?;
            }

            if event.has_value("json.properties.UPN") {
                event.rename(
                    "json.properties.UPN",
                    "microsoft_intune.managed_device.properties.upn",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.upn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("microsoft_intune.managed_device.properties.upn") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_intune.managed_device.properties.upn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.UserEmail") {
                event.rename(
                    "json.properties.UserEmail",
                    "microsoft_intune.managed_device.properties.user_email",
                )?;
            }

            let _cond =
                { event.has_value("microsoft_intune.managed_device.properties.user_email") };
            if _cond {
                event.append_unique(
                    "user.email",
                    json!(
                        event
                            .get("microsoft_intune.managed_device.properties.user_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("microsoft_intune.managed_device.properties.user_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_intune.managed_device.properties.user_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.UserName") {
                event.rename(
                    "json.properties.UserName",
                    "microsoft_intune.managed_device.properties.user_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.properties.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("microsoft_intune.managed_device.properties.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_intune.managed_device.properties.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.WifiMacAddress") {
                event.rename(
                    "json.properties.WifiMacAddress",
                    "microsoft_intune.managed_device.properties.wifi_mac_address",
                )?;
            }

            let _cond =
                { event.has_value("microsoft_intune.managed_device.properties.wifi_mac_address") };
            if _cond {
                event.append_unique(
                    "host.mac",
                    json!(
                        event
                            .get("microsoft_intune.managed_device.properties.wifi_mac_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.mac") {
                    gsub_field(
                        event,
                        "host.mac",
                        "host.mac",
                        cached_regex!(
                            "^([A-F0-9]{2})([A-F0-9]{2})([A-F0-9]{2})([A-F0-9]{2})([A-F0-9]{2})([A-F0-9]{2})$"
                        ),
                        "$1-$2-$3-$4-$5-$6",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "format_mac_address_without_separators",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.mac") {
                    map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set("_ingest.on_failure_processor_tag", "uppercase_host_mac")?;
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

            let _cond =
                { event.has_value("microsoft_intune.managed_device.properties.wifi_mac_address") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("microsoft_intune.managed_device.properties.wifi_mac_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.category") {
                event.rename("json.category", "microsoft_intune.managed_device.category")?;
            }

            if event.has_value("json.operationName") {
                event.rename(
                    "json.operationName",
                    "microsoft_intune.managed_device.operation_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.operation_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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

            if event.has_value("json.resultType") {
                event.rename(
                    "json.resultType",
                    "microsoft_intune.managed_device.result_type",
                )?;
            }

            if event.has_value("json.tenantId") {
                event.rename("json.tenantId", "microsoft_intune.managed_device.tenant_id")?;
            }

            if let Some(v) = event
                .get("microsoft_intune.managed_device.tenant_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("microsoft_intune.managed_device.time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time")?;
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

            if let Some(v) = event
                .get("microsoft_intune.managed_device.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

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
                event.remove("microsoft_intune.managed_device.properties.created_date");
                event.remove("microsoft_intune.managed_device.properties.device_id");
                event.remove("microsoft_intune.managed_device.properties.device_name");
                event.remove("microsoft_intune.managed_device.properties.model");
                event.remove("microsoft_intune.managed_device.properties.os");
                event.remove("microsoft_intune.managed_device.properties.os_version");
                event.remove("microsoft_intune.managed_device.properties.primary_user");
                event.remove("microsoft_intune.managed_device.properties.serial_number");
                event.remove("microsoft_intune.managed_device.properties.upn");
                event.remove("microsoft_intune.managed_device.properties.user_email");
                event.remove("microsoft_intune.managed_device.properties.user_name");
                event.remove("microsoft_intune.managed_device.operation_name");
                event.remove("microsoft_intune.managed_device.tenant_id");
                event.remove("microsoft_intune.managed_device.time");
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                    json!(format!(
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
