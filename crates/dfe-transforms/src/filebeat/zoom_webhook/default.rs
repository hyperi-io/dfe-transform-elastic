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
            event.set("observer.vendor", json!("Zoom"))?;

            event.set("observer.product", json!("Webhook"))?;

            event.set("ecs.version", json!("8.11.0"))?;

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
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

            event.append("event.kind", json!("event"))?;

            if event.has_value("zoom.event") {
                event.rename("zoom.event", "event.action")?;
            }

            event.rename("zoom.payload", "_temp_.payload")?;

            if event.remove("zoom").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zoom".into(),
                });
            }

            event.rename("_temp_.payload", "zoom")?;

            if event.has_value("zoom.old_object") {
                event.rename("zoom.old_object", "zoom.old_values")?;
            }

            if event.has_value("zoom.object.participant") {
                event.rename("zoom.object.participant", "zoom.participant")?;
            }

            if event.has_value("zoom.object.settings") {
                event.rename("zoom.object.settings", "zoom.settings")?;
            }

            if event.has_value("zoom.object.registrant") {
                event.rename("zoom.object.registrant", "zoom.registrant")?;
            }

            let _cond = { event.has_value("zoom.operator_id") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("zoom.operator_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zoom.operator_id") };
            if _cond {
                event.set(
                    "user.id",
                    json!(
                        event
                            .get("zoom.operator_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zoom.operator_id") };
            if _cond {
                let v = json!(
                    event
                        .get("zoom.operator")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.email", v)?;
                }
            }

            event.remove("message");
            event.remove("_temp_");
            event.remove("zoom.object.occurrences");
            event.remove("zoom.old_values.occurrences");
            event.remove("zoom.object.recurrence");
            event.remove("zoom.old_values.recurrence");
            event.remove("zoom.object.managed_domains");
            event.remove("zoom.old_values.managed_domains");
            event.remove("zoom.registrant.custom_questions");
            event.remove("zoom.old_values.registrant.custom_questions");
            event.remove("zoom.object.call_logs");
            event.remove("zoom.old_values.call_logs");
            event.remove("zoom.object.recording_files");
            event.remove("zoom.old_values.recording_files");
            event.remove("zoom.object.call_logs");

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("meeting"))
            };
            if _cond {
                // Begin nested pipeline: "meeting"
                let _cond = { event.get_str("event.action") != Some("meeting.alert") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = { event.get_str("event.action") == Some("meeting.alert") };
                if _cond {
                    event.append("event.type", json!("error"))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("meeting.registration_approved") };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                }
                let _cond = {
                    ["meeting.registration_created", "meeting.created"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = { event.get_str("event.action") == Some("meeting.deleted") };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = { event.get_str("event.action") == Some("meeting.updated") };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = {
                    ["meeting.started", "meeting.sharing_started"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = {
                    ["meeting.ended", "meeting.sharing_ended"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("end"))?;
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.meeting")?;
                }
                if event.has_value("zoom.meeting.join_url") {
                    event.rename("zoom.meeting.join_url", "url.full")?;
                }
                let _cond = { !event.has_value("url.full") };
                if _cond {
                    if event.has_value("zoom.registrant.join_url") {
                        event.rename("zoom.registrant.join_url", "url.full")?;
                    }
                }
                let _cond = { event.has_value("zoom.participant") };
                if _cond {
                    event.remove("user");
                }
                let v = json!(
                    event
                        .get("zoom.participant.id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
                }
                let v = json!(
                    event
                        .get("zoom.participant.user_name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.full_name", v)?;
                }
                if let Some(v) = event
                    .get("zoom.participant.email")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                let v = json!(
                    event
                        .get("zoom.meeting.host_id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    if !event.has("user.id") {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = { event.has_value("zoom.participant.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.participant.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.participant.user_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.participant.user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.participant.participant_uuid") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.participant.participant_uuid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.meeting.host_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.meeting.host_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("zoom.participant.public_ip")
                        && event.get_str("zoom.participant.public_ip") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(val) = event.get("zoom.participant.public_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "zoom.participant.public_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.remove("zoom.participant.public_ip");
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("source.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("source.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("source.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("source.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("source.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("source.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("source.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("source.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                let _cond = { event.get_str("event.action") == Some("meeting.started") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.meeting.start_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("event.start", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.meeting.start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("meeting.sharing_started") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("zoom.participant.sharing_details.date_time")
                        {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.participant.sharing_details.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    [
                        "meeting.participant_put_in_waiting_room",
                        "meeting.participant_joined_waiting_room",
                        "meeting.participant_left_waiting_room",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.participant.date_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.participant.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("meeting.participant_joined") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.participant.join_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.participant.join_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("meeting.participant_left") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.participant.leave_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.participant.leave_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("meeting.updated") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.time_stamp") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.time_stamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("zoom.meeting.duration") };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx.event.duration = ctx.zoom.meeting.duration * 60L * 1000000000L;
                    scale_field(
                        event,
                        &ScaleField::new(
                            "zoom.meeting.duration",
                            "event.duration",
                            Factor::Long(1000000000),
                        ),
                    );
                }
                let _cond = { event.get_str("event.action") == Some("meeting.started") };
                if _cond {
                    event.remove("zoom.meeting.start_time");
                }
                let _cond = { event.has_value("event.duration") };
                if _cond {
                    event.remove("zoom.meeting.duration");
                }
                let _cond = { event.get_str("event.action") == Some("meeting.sharing_started") };
                if _cond {
                    event.remove("zoom.participant.sharing_details.date_time");
                }
                let _cond = {
                    [
                        "meeting.participant_put_in_waiting_room",
                        "meeting.participant_joined_waiting_room",
                        "meeting.participant_left_waiting_room",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.remove("zoom.participant.date_time");
                }
                let _cond = { event.get_str("event.action") == Some("meeting.participant_joined") };
                if _cond {
                    event.remove("zoom.participant.join_time");
                }
                let _cond = { event.get_str("event.action") == Some("meeting.participant_left") };
                if _cond {
                    event.remove("zoom.participant.leave_time");
                }
                let _cond = { event.get_str("event.action") == Some("meeting.updated") };
                if _cond {
                    event.remove("zoom.time_stamp");
                }
                // End nested pipeline: "meeting"
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("account"))
            };
            if _cond {
                // Begin nested pipeline: "account"
                event.append("event.category", json!("iam"))?;
                let _cond = { event.get_str("event.action") == Some("account.settings_updated") };
                if _cond {
                    event.append("event.category", json!("configuration"))?;
                }
                event.append("event.type", json!("user"))?;
                let _cond = { event.get_str("event.action") == Some("account.created") };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = {
                    [
                        "account.updated",
                        "account.settings_updated",
                        "account.disassociated",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                if event.has_value("zoom.account_id") {
                    event.rename("zoom.account_id", "zoom.master_account_id")?;
                }
                if event.has_value("zoom.object.id") {
                    event.rename("zoom.object.id", "zoom.sub_account_id")?;
                }
                let _cond = { event.has_value("zoom.time_stamp") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.time_stamp") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.time_stamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.account")?;
                }
                let v = json!(
                    event
                        .get("zoom.account.owner_id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.target.id", v)?;
                }
                let v = json!(
                    event
                        .get("zoom.account.owner_email")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.target.email", v)?;
                }
                let _cond = { event.has_value("zoom.old_values.id") };
                if _cond {
                    event.set(
                        "user.target.id",
                        json!(
                            event
                                .get("zoom.old_values.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.old_values.account_email") };
                if _cond {
                    event.set(
                        "user.target.email",
                        json!(
                            event
                                .get("zoom.old_values.account_email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.old_values.account_name") };
                if _cond {
                    event.set(
                        "user.target.full_name",
                        json!(
                            event
                                .get("zoom.old_values.account_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.old_values.account_alias") };
                if _cond {
                    event.set(
                        "user.target.name",
                        json!(
                            event
                                .get("zoom.old_values.account_alias")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("zoom.account.id")
                        && !condition_eq(
                            event.get("zoom.old_values.id"),
                            event.get("zoom.account.id"),
                        )
                };
                if _cond {
                    event.set(
                        "user.changes.id",
                        json!(
                            event
                                .get("zoom.account.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("zoom.account.account_email")
                        && !condition_eq(
                            event.get("zoom.old_values.account_email"),
                            event.get("zoom.account.account_email"),
                        )
                };
                if _cond {
                    event.set(
                        "user.changes.email",
                        json!(
                            event
                                .get("zoom.account.account_email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("zoom.account.account_name")
                        && !condition_eq(
                            event.get("zoom.old_values.account_name"),
                            event.get("zoom.account.account_name"),
                        )
                };
                if _cond {
                    event.set(
                        "user.changes.full_name",
                        json!(
                            event
                                .get("zoom.account.account_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("zoom.account.account_alias")
                        && !condition_eq(
                            event.get("zoom.old_values.account_alias"),
                            event.get("zoom.account.account_alias"),
                        )
                };
                if _cond {
                    event.set(
                        "user.changes.name",
                        json!(
                            event
                                .get("zoom.account.account_alias")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.account.owner_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.account.owner_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("user.target.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.target.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("user.changes.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.changes.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.remove("zoom.time_stamp");
                // End nested pipeline: "account"
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("chat_message"))
            };
            if _cond {
                // Begin nested pipeline: "chat_message"
                event.append("event.type", json!("info"))?;
                let _cond = { event.get_str("event.action") == Some("chat_message.sent") };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = { event.get_str("event.action") == Some("chat_message.deleted") };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = { event.get_str("event.action") == Some("chat_message.updated") };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.chat_message")?;
                }
                let _cond = { event.has_value("zoom.chat_message.contact_id") };
                if _cond {
                    event.append(
                        "related.user",
                        json!(
                            event
                                .get("zoom.chat_message.contact_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.chat_message.timestamp") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.chat_message.timestamp") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.chat_message.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("zoom.chat_message.timestamp") };
                if _cond {
                    event.remove("zoom.chat_message.date_time");
                }
                let _cond = { !event.has_value("zoom.chat_message.timestamp") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.chat_message.date_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.chat_message.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                event.remove("zoom.chat_message.timestamp");
                let _cond = { !event.has_value("zoom.chat_message.message") };
                if _cond {
                    event.remove("zoom.chat_message.message");
                }
                // End nested pipeline: "chat_message"
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("chat_channel"))
            };
            if _cond {
                // Begin nested pipeline: "chat_channel"
                let _cond = {
                    [
                        "chat_channel.member_invited",
                        "chat_channel.member_joined",
                        "chat_channel.member_left",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("user"))?;
                }
                let _cond = { event.get_str("event.action") == Some("chat_channel.created") };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = { event.get_str("event.action") == Some("chat_channel.deleted") };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = { event.get_str("event.action") == Some("chat_channel.updated") };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.chat_channel")?;
                }
                let _cond = { event.has_value("zoom.chat_channel.timestamp") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.chat_channel.timestamp") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.chat_channel.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("zoom.chat_channel.timestamp") };
                if _cond {
                    event.remove("zoom.chat_channel.date_time");
                }
                let _cond = {
                    event.has_value("zoom.chat_channel.date_time")
                        && !event.has_value("zoom.chat_channel.timestamp")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.chat_channel.date_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.chat_channel.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("zoom.chat_channel.timestamp") };
                if _cond {
                    event.remove("zoom.chat_channel.timestamp");
                }
                if event.has_value("zoom.chat_channel.members") {
                    foreach_array(event, "zoom.chat_channel.members", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.display_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.id")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                event.remove("zoom.chat_channel.members");
                // End nested pipeline: "chat_channel"
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("phone"))
            };
            if _cond {
                // Begin nested pipeline: "phone"
                event.append("event.type", json!("info"))?;
                let _cond = {
                    ["phone.caller_ringing", "phone.callee_ringing"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = {
                    ["phone.callee_answered", "phone.caller_connected"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = {
                    [
                        "phone.callee_missed",
                        "phone.callee_ended",
                        "phone.caller_ended",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("end"))?;
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.phone")?;
                }
                if event.has_value("zoom.phone.download_url") {
                    event.rename("zoom.phone.download_url", "url.full")?;
                }
                let _cond = {
                    [
                        "phone.callee_ringing",
                        "phone.caller_ringing",
                        "phone.caller_ended",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.phone.ringing_start_time")
                        {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.phone.ringing_start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("phone.caller_connected") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("zoom.phone.connected_start_time")
                        {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.phone.connected_start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("zoom.phone.answer_start_time")
                        && event.get_str("event.action") == Some("phone.callee_answered")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.phone.answer_start_time")
                        {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.phone.answer_start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    [
                        "phone.callee_missed",
                        "phone.callee_ended",
                        "phone.caller_ended",
                        "phone.callee_rejected",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.phone.call_end_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.phone.call_end_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("phone.voicemail_received") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.phone.date_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.phone.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("zoom.phone.duration") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zoom.phone.duration".into(),
                                message,
                            }
                        })?;
                        event.set("zoom.phone.duration", converted)?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("zoom.phone.ringing_start_time")
                        && !event.has_value("zoom.phone.answer_start_time")
                        && event.has_value("zoom.phone.call_end_time")
                        && !event.has_value("zoom.phone.duration")
                };
                if _cond {
                    // Painless script
                    // Source: ctx.event.start = ctx.zoom.phone.ringing_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event.start = ctx.zoom.phone.ringing_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#
                        ),
                    )?;
                }
                let _cond = {
                    !event.has_value("zoom.phone.ringing_start_time")
                        && event.has_value("zoom.phone.answer_start_time")
                        && event.has_value("zoom.phone.call_end_time")
                        && !event.has_value("zoom.phone.duration")
                };
                if _cond {
                    // Painless script
                    // Source: ctx.event.start = ctx.zoom.phone.answer_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event.start = ctx.zoom.phone.answer_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.duration") };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx.event.duration = ctx.zoom.phone.duration * 60L * 1000000000L;
                    scale_field(
                        event,
                        &ScaleField::new(
                            "zoom.phone.duration",
                            "event.duration",
                            Factor::Long(1000000000),
                        ),
                    );
                }
                if event.has_value("zoom.phone.callee_user_id") {
                    event.rename("zoom.phone.callee_user_id", "zoom.phone.callee.user_id")?;
                }
                if event.has_value("zoom.phone.callee_extension_type") {
                    event.rename(
                        "zoom.phone.callee_extension_type",
                        "zoom.phone.callee.extension_type",
                    )?;
                }
                if event.has_value("zoom.phone.callee_id") {
                    event.rename("zoom.phone.callee_id", "zoom.phone.callee.id")?;
                }
                if event.has_value("zoom.phone.callee_name") {
                    event.rename("zoom.phone.callee_name", "zoom.phone.callee.name")?;
                }
                if event.has_value("zoom.phone.callee_number") {
                    event.rename("zoom.phone.callee_number", "zoom.phone.callee.phone_number")?;
                }
                if event.has_value("zoom.phone.callee_number_type") {
                    event.rename(
                        "zoom.phone.callee_number_type",
                        "zoom.phone.callee.number_type",
                    )?;
                }
                if event.has_value("zoom.phone.callee_user_id") {
                    event.rename("zoom.phone.callee_user_id", "zoom.phone.callee.user_id")?;
                }
                if event.has_value("zoom.phone.callee_extension_type") {
                    event.rename(
                        "zoom.phone.callee_extension_type",
                        "zoom.phone.callee.extension_type",
                    )?;
                }
                if event.has_value("zoom.phone.caller_id") {
                    event.rename("zoom.phone.caller_id", "zoom.phone.caller.id")?;
                }
                if event.has_value("zoom.phone.caller_name") {
                    event.rename("zoom.phone.caller_name", "zoom.phone.caller.name")?;
                }
                if event.has_value("zoom.phone.caller_number") {
                    event.rename("zoom.phone.caller_number", "zoom.phone.caller.phone_number")?;
                }
                if event.has_value("zoom.phone.caller_number_type") {
                    event.rename(
                        "zoom.phone.caller_number_type",
                        "zoom.phone.caller.number_type",
                    )?;
                }
                let _cond = { event.has_value("zoom.phone.callee.user_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.phone.callee.user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.phone.callee_user_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.phone.callee_user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.phone.caller.user_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.phone.caller.user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("phone.voicemail_received") };
                if _cond {
                    event.remove("zoom.phone.date_time");
                }
                let v = json!(
                    event
                        .get("zoom.phone.caller.user_id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.user.id", v)?;
                }
                let v = json!(
                    event
                        .get("zoom.phone.callee.user_id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("destination.user.id", v)?;
                }
                // End nested pipeline: "phone"
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("recording"))
            };
            if _cond {
                // Begin nested pipeline: "recording"
                event.append("event.type", json!("info"))?;
                let _cond =
                    { event.get_str("event.action") == Some("recording.registration_created") };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("recording.registration_approved") };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("recording.registration_denied") };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                }
                let _cond = {
                    ["recording.deleted", "recording.trashed"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = {
                    [
                        "recording.paused",
                        "recording.resumed",
                        "recording.renamed",
                        "recording.recovered",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = { event.get_str("event.action") == Some("recording.started") };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = {
                    [
                        "recording.stopped",
                        "recording.completed",
                        "recording.transcript_completed",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("end"))?;
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.recording")?;
                }
                if event.has_value("zoom.recording.share_url") {
                    event.rename("zoom.recording.share_url", "url.full")?;
                }
                let _cond = { event.get_str("event.action") == Some("recording.renamed") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.time_stamp") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.time_stamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond =
                    { event.get_str("zoom.recording.recording_file.recording_start") == Some("") };
                if _cond {
                    if event
                        .remove("zoom.recording.recording_file.recording_start")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "zoom.recording.recording_file.recording_start".into(),
                        });
                    }
                }
                let _cond =
                    { event.get_str("zoom.recording.recording_file.recording_end") == Some("") };
                if _cond {
                    if event
                        .remove("zoom.recording.recording_file.recording_end")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "zoom.recording.recording_file.recording_end".into(),
                        });
                    }
                }
                let _cond = { event.get_str("event.action") == Some("recording.started") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.recording.recording_file.recording_start")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("event.start", v)?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("recording.stopped") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.recording.recording_file.recording_end")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("event.end", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.end")
                        && event.has_value("event.start")
                        && event.get_str("event.action") == Some("recording.stopped")
                };
                if _cond {
                    // Painless script
                    // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("zoom.recording.recording_file.recording_start")
                        && event.get_str("event.action") == Some("recording.started")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("zoom.recording.recording_file.recording_start")
                        {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.recording.recording_file.recording_start"
                                            .into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("zoom.recording.host_id") };
                if _cond {
                    event.append(
                        "related.user",
                        json!(
                            event
                                .get("zoom.recording.host_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.registrant.id") };
                if _cond {
                    event.append(
                        "related.user",
                        json!(
                            event
                                .get("zoom.registrant.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("event.action") == Some("recording.renamed") };
                if _cond {
                    event.remove("zoom.time_stamp");
                }
                let _cond = { !event.has_value("user.id") && event.has_value("zoom.registrant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.registrant.email")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = { !event.has_value("user.id") && event.has_value("zoom.registrant") };
                if _cond {
                    let v = json!(format!(
                        "{} {}",
                        event
                            .get("zoom.registrant.first_name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("zoom.registrant.last_name")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("user.full_name", v)?;
                    }
                }
                let _cond = { !event.has_value("user.id") && event.has_value("zoom.registrant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.registrant.id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = { !event.has_value("zoom.registrant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.recording.host_id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.id", v)?;
                    }
                }
                // End nested pipeline: "recording"
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("user"))
            };
            if _cond {
                // Begin nested pipeline: "user"
                let _cond = { event.get_str("event.action") == Some("user.settings_updated") };
                if _cond {
                    event.append("event.category", json!("configuration"))?;
                }
                let _cond = {
                    !(["user.signed_in", "user.signed_out"]
                        .contains(&event.get_str("event.action").unwrap_or("")))
                };
                if _cond {
                    event.append("event.category", json!("iam"))?;
                }
                let _cond = {
                    ["user.signed_in", "user.signed_out"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("authentication"))?;
                }
                let _cond = { event.get_str("event.action") == Some("user.created") };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = { event.get_str("event.action") == Some("user.deleted") };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = {
                    [
                        "user.updated",
                        "user.settings_updated",
                        "user.deactivated",
                        "user.activated",
                        "user.disassociated",
                        "user.presence_status_updated",
                        "user.personal_notes_updated",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = { event.get_str("event.action") == Some("user.signed_in") };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = { event.get_str("event.action") == Some("user.signed_out") };
                if _cond {
                    event.append("event.type", json!("end"))?;
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.user")?;
                }
                let _cond = {
                    ["user.updated", "user.settings_updated"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.time_stamp") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.time_stamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    [
                        "user.signed_in",
                        "user.signed_out",
                        "user.personal_notes_updated",
                        "user.presence_status_updated",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.user.date_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.user.date_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("zoom.user.id") };
                if _cond {
                    event.append(
                        "related.user",
                        json!(
                            event
                                .get("zoom.user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.remove("zoom.time_stamp");
                event.remove("zoom.user.date_time");
                let v = json!(
                    event
                        .get("zoom.operator_id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
                }
                let _cond = {
                    event.get("zoom.operator").is_some_and(|v| v.is_string())
                        && event.get("zoom.operator").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
                };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.operator")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.email", v)?;
                    }
                }
                if let Some(v) = event
                    .get("zoom.user.email")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.email") {
                        event.set("user.email", v)?;
                    }
                }
                let _cond =
                    { !event.has_value("zoom.operator") && !event.has_value("zoom.operator_id") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.user.id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.id", v)?;
                    }
                }
                let _cond =
                    { !event.has_value("zoom.operator") && !event.has_value("zoom.operator_id") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.user.email")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("zoom.operator")
                        && !event.has_value("zoom.operator_id")
                        && event.has_value("zoom.user.first_name")
                };
                if _cond {
                    let v = json!(format!(
                        "{} {}",
                        event
                            .get("zoom.user.first_name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("zoom.user.last_name")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("user.full_name", v)?;
                    }
                }
                let v = json!(
                    event
                        .get("zoom.old_values.id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.target.id", v)?;
                }
                let v = json!(
                    event
                        .get("zoom.old_values.id")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.target.id", v)?;
                }
                let v = json!(
                    event
                        .get("zoom.old_values.email")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.target.email", v)?;
                }
                let v = json!(
                    event
                        .get("zoom.old_values.email")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("user.target.email", v)?;
                }
                let _cond = { event.has_value("zoom.old_values.first_name") };
                if _cond {
                    event.set(
                        "user.target.full_name",
                        json!(format!(
                            "{} {}",
                            event
                                .get("zoom.old_values.first_name")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("zoom.old_values.last_name")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = {
                    event.has_value("zoom.old_values")
                        || event.has_value("zoom.operator")
                        || event.has_value("zoom.operator_id")
                };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.user.id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        if !event.has("user.target.id") {
                            event.set("user.target.id", v)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("zoom.old_values")
                        || event.has_value("zoom.operator")
                        || event.has_value("zoom.operator_id")
                };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.user.id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        if !event.has("user.target.id") {
                            event.set("user.target.id", v)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("zoom.old_values")
                        || event.has_value("zoom.operator")
                        || event.has_value("zoom.operator_id")
                };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.user.email")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        if !event.has("user.target.email") {
                            event.set("user.target.email", v)?;
                        }
                    }
                }
                let _cond = { event.has_value("zoom.old_values") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.user.email")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        if !event.has("user.target.email") {
                            event.set("user.target.email", v)?;
                        }
                    }
                }
                let _cond = {
                    (event.has_value("zoom.old_values")
                        || event.has_value("zoom.operator")
                        || event.has_value("zoom.operator_id"))
                        && event.has_value("zoom.user.first_name")
                };
                if _cond {
                    if !event.has("user.target.full_name") {
                        event.set(
                            "user.target.full_name",
                            json!(format!(
                                "{} {}",
                                event
                                    .get("zoom.user.first_name")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("zoom.user.last_name")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                    }
                }
                let _cond = {
                    event.has_value("zoom.old_values.id")
                        && !condition_eq(event.get("zoom.old_values.id"), event.get("zoom.user.id"))
                };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.user.id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.changes.id", v)?;
                    }
                }
                let _cond = {
                    event.has_value("zoom.old_values.email")
                        && !condition_eq(
                            event.get("zoom.old_values.email"),
                            event.get("zoom.user.email"),
                        )
                };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.user.email")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.changes.email", v)?;
                    }
                }
                let _cond = {
                    event.has_value("zoom.old_values.first_name")
                        && event.has_value("zoom.old_values.last_name")
                        && (!condition_eq(
                            event.get("zoom.old_values.last_name"),
                            event.get("zoom.user.last_name"),
                        ) || !condition_eq(
                            event.get("zoom.old_values.first_name"),
                            event.get("zoom.user.first_name"),
                        ))
                };
                if _cond {
                    let v = json!(format!(
                        "{} {}",
                        event
                            .get("zoom.user.first_name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("zoom.user.last_name")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("user.changes.full_name", v)?;
                    }
                }
                let _cond = { event.has_value("zoom.user.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.old_values.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.old_values.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "user"
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("webinar"))
            };
            if _cond {
                // Begin nested pipeline: "webinar"
                let _cond = { event.get_str("event.action") != Some("webinar.alert") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = { event.get_str("event.action") == Some("webinar.alert") };
                if _cond {
                    event.append("event.type", json!("error"))?;
                }
                let _cond = {
                    ["webinar.created", "webinar.registration_created"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = { event.get_str("event.action") == Some("webinar.deleted") };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("webinar.registration_approved") };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                }
                let _cond =
                    { event.get_str("event.action") == Some("webinar.registration_denied") };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                }
                let _cond = {
                    [
                        "webinar.updated",
                        "webinar.registration_approved",
                        "webinar.registration_denied",
                        "webinar.registration_cancelled",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = {
                    ["webinar.started", "webinar.sharing_started"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = {
                    ["webinar.ended", "webinar.sharing_ended"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("end"))?;
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.webinar")?;
                }
                let _cond = { event.get_str("event.action") == Some("webinar.updated") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.time_stamp") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.time_stamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("webinar.started") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.webinar.start_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.webinar.start_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("webinar.participant_joined") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.participant.join_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.participant.join_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("webinar.participant_left") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("zoom.participant.leave_time") {
                            match parse_date_out(&date_str, &["ISO_INSTANT"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "zoom.participant.leave_time".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("zoom.participant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.participant.id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = { event.has_value("zoom.participant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.participant.user_name")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.full_name", v)?;
                    }
                }
                if let Some(v) = event
                    .get("zoom.participant.email")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                let _cond = { event.has_value("zoom.registrant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.registrant.id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = { event.has_value("zoom.registrant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.registrant.email")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = { event.has_value("zoom.registrant") };
                if _cond {
                    let v = json!(format!(
                        "{} {}",
                        event
                            .get("zoom.registrant.first_name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("zoom.registrant.last_name")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("user.full_name", v)?;
                    }
                }
                let _cond =
                    { !event.has_value("zoom.registrant") && !event.has_value("zoom.participant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.operator_id")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.id", v)?;
                    }
                }
                let _cond =
                    { !event.has_value("zoom.registrant") && !event.has_value("zoom.participant") };
                if _cond {
                    let v = json!(
                        event
                            .get("zoom.operator")
                            .map_or_else(String::new, template_to_string)
                    );
                    if !painless_is_empty_value(&v) {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = { event.has_value("zoom.webinar.host_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.webinar.host_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.registrant.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.registrant.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.participant.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.participant.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.participant.user_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.participant.user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("zoom.participant.participant_uuid") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("zoom.participant.participant_uuid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("zoom.participant.public_ip")
                        && event.get_str("zoom.participant.public_ip") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(val) = event.get("zoom.participant.public_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "zoom.participant.public_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.remove("zoom.participant.public_ip");
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("source.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("source.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("source.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("source.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("source.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("source.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("source.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("source.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("source.ip") {
                    if let Some(ip_str) = event.get_string("source.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("source.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("source.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                // End nested pipeline: "webinar"
            }

            let _cond = {
                event
                    .get_str("event.action")
                    .is_some_and(|s| s.starts_with("zoomroom"))
            };
            if _cond {
                // Begin nested pipeline: "zoomroom"
                let _cond = {
                    ["zoomroom.checked_in", "zoomroom.checked_out"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = { event.get_str("event.action") == Some("zoomroom.checked_in") };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = { event.get_str("event.action") == Some("zoomroom.checked_out") };
                if _cond {
                    event.append("event.type", json!("end"))?;
                }
                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.zoomroom")?;
                }
                // End nested pipeline: "zoomroom"
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
