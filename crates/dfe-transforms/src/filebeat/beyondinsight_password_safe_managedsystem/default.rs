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
            event.set("ecs.version", json!("9.1.0"))?;

            let _cond = { event.has_value("error.message") && !event.has_value("event.original") };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "event.original".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            parse_json_field(
                event,
                "event.original",
                "beyondinsight_password_safe.managedsystem",
            )?;

            // Painless script
            // Source: ctx.beyondinsight_password_safe.managedsystem.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.beyondinsight_password_safe.managedsystem.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n"#
                ),
            )?;

            // Painless script
            // Source: for (field in params.numeric_ids) {\n  def value = ctx.beyondinsight_password_safe.managedsystem[field];\n  if (value instanceof Number) {\n    ctx.beyondinsight_password_safe.managedsystem[field] =\n      Integer.toString(value.intValue());\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"for (field in params.numeric_ids) {\n  def value = ctx.beyondinsight_password_safe.managedsystem[field];\n  if (value instanceof Number) {\n    ctx.beyondinsight_password_safe.managedsystem[field] =\n      Integer.toString(value.intValue());\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"numeric_ids\":[\"ApplicationHostID\",\"AssetID\",\"CloudID\",\"DatabaseID\",\"DirectoryID\",\"DSSKeyRuleID\",\"EntityTypeID\",\"FunctionalAccountID\",\"LoginAccountID\",\"ManagedSystemID\",\"PasswordRuleID\",\"PlatformID\",\"SshKeyEnforcementMode\",\"WorkgroupID\"]}"
                ),
            )?;

            // Painless script
            // Source: Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.managedsystem.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.managedsystem = renamedFields;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.managedsystem.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.managedsystem = renamedFields;\n"#
                ),
                cached_params!(
                    "{\"field_mappings\":{\"AccessURL\":\"access_url\",\"AccountNameFormat\":\"account_name_format\",\"ApplicationHostID\":\"application_host_id\",\"AssetID\":\"asset_id\",\"AutoManagementFlag\":\"auto_management_flag\",\"ChangeFrequencyDays\":\"change_frequency_days\",\"ChangeFrequencyType\":\"change_frequency_type\",\"ChangePasswordAfterAnyReleaseFlag\":\"change_password_after_any_release_flag\",\"ChangeTime\":\"change_time\",\"CheckPasswordFlag\":\"check_password_flag\",\"CloudID\":\"cloud_id\",\"ContactEmail\":\"contact_email\",\"DNSName\":\"dns_name\",\"DSSKeyRuleID\":\"dss_key_rule_id\",\"DatabaseID\":\"database_id\",\"Description\":\"description\",\"DirectoryID\":\"directory_id\",\"ElevationCommand\":\"elevation_command\",\"EntityTypeID\":\"entity_type_id\",\"ForestName\":\"forest_name\",\"FunctionalAccountID\":\"functional_account_id\",\"HostName\":\"host_name\",\"IPAddress\":\"ip_address\",\"ISAReleaseDuration\":\"isa_release_duration\",\"InstanceName\":\"instance_name\",\"IsApplicationHost\":\"is_application_host\",\"IsDefaultInstance\":\"is_default_instance\",\"LoginAccountID\":\"login_account_id\",\"ManagedSystemID\":\"managed_system_id\",\"MaxReleaseDuration\":\"max_release_duration\",\"NetBiosName\":\"net_bios_name\",\"OracleInternetDirectoryID\":\"oracle_internet_directory_id\",\"OracleInternetDirectoryServiceName\":\"oracle_internet_directory_service_name\",\"PasswordRuleID\":\"password_rule_id\",\"PlatformID\":\"platform_id\",\"Port\":\"port\",\"ReleaseDuration\":\"release_duration\",\"RemoteClientType\":\"remote_client_type\",\"ResetPasswordOnMismatchFlag\":\"reset_password_on_mismatch_flag\",\"SshKeyEnforcementMode\":\"ssh_key_enforcement_mode\",\"SystemName\":\"system_name\",\"Template\":\"template\",\"Timeout\":\"timeout\",\"UseSSL\":\"use_ssl\",\"WorkgroupID\":\"workgroup_id\"}}"
                ),
            )?;

            event.set("event.kind", json!("asset"))?;

            event.append("event.category", json!("iam"))?;

            event.append("event.type", json!("info"))?;

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.host_name") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.managedsystem.host_name")
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.dns_name") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.managedsystem.dns_name")
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.dns_name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    gsub_field(
                        event,
                        "beyondinsight_password_safe.managedsystem.dns_name",
                        "host.domain",
                        cached_regex!("^[^.]+\\."),
                        "",
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("beyondinsight_password_safe.managedsystem.contact_email") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.managedsystem.contact_email")
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.access_url") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.managedsystem.access_url")
                    .cloned()
                {
                    event.set("url.full", v)?;
                }
            }

            let _cond = {
                event
                    .has_value("beyondinsight_password_safe.managedsystem.ssh_key_enforcement_mode")
            };
            if _cond {
                // Painless script
                // Source: def description = params.descriptions.get(ctx.beyondinsight_password_safe.managedsystem.ssh_key_enforcement_mode);\nif (description != null) {\n  ctx.beyondinsight_password_safe.managedsystem.ssh_key_enforcement_mode = description;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def description = params.descriptions.get(ctx.beyondinsight_password_safe.managedsystem.ssh_key_enforcement_mode);\nif (description != null) {\n  ctx.beyondinsight_password_safe.managedsystem.ssh_key_enforcement_mode = description;\n}\n"#
                    ),
                    cached_params!(
                        "{\"descriptions\":{\"0\":\"None\",\"1\":\"Auto\",\"2\":\"Strict\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.ip_address") };
            if _cond {
                gsub_field(
                    event,
                    "beyondinsight_password_safe.managedsystem.ip_address",
                    "beyondinsight_password_safe.managedsystem.ip_address",
                    cached_regex!("\\b0+(\\d)"),
                    "$1",
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.ip_address") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) =
                        event.get("beyondinsight_password_safe.managedsystem.ip_address")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondinsight_password_safe.managedsystem.ip_address".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondinsight_password_safe.managedsystem.ip_address",
                            converted,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    if event
                        .remove("beyondinsight_password_safe.managedsystem.ip_address")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "beyondinsight_password_safe.managedsystem.ip_address".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.ip_address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedsystem.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("host.ip") {
                if let Some(ip_str) = event.get_string("host.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedsystem.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedsystem.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedsystem.dns_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedsystem.dns_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("beyondinsight_password_safe.managedsystem.contact_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedsystem.contact_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
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
                                "with tag '{}' ",
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
