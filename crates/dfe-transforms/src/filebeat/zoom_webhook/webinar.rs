// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `webinar` pipeline.
pub struct Webinar;

impl Transform for Webinar {
    fn name(&self) -> &str {
        "webinar"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("event.action") != Some("webinar.alert") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("event.action") == Some("webinar.alert") };
            if _cond {
                event.append("event.type", json!("error"))?;
            }

            let _cond = { ["webinar.created", "webinar.registration_created"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("event.action") == Some("webinar.deleted") };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.get_str("event.action") == Some("webinar.registration_approved") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_str("event.action") == Some("webinar.registration_denied") };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { ["webinar.updated", "webinar.registration_approved", "webinar.registration_denied", "webinar.registration_cancelled"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { ["webinar.started", "webinar.sharing_started"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { ["webinar.ended", "webinar.sharing_ended"].contains(&event.get_str("event.action").unwrap_or("")) };
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
            let v = json!(event.get("zoom.participant.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
            }
            }

            let _cond = { event.has_value("zoom.participant") };
            if _cond {
            let v = json!(event.get("zoom.participant.user_name").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.full_name", v)?;
            }
            }

            if let Some(v) = event.get("zoom.participant.email").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("zoom.registrant") };
            if _cond {
            let v = json!(event.get("zoom.registrant.id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
            }
            }

            let _cond = { event.has_value("zoom.registrant") };
            if _cond {
            let v = json!(event.get("zoom.registrant.email").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.email", v)?;
            }
            }

            let _cond = { event.has_value("zoom.registrant") };
            if _cond {
            let v = json!(format!("{} {}", event.get("zoom.registrant.first_name").map_or_else(String::new, template_to_string), event.get("zoom.registrant.last_name").map_or_else(String::new, template_to_string)));
            if !painless_is_empty_value(&v) {
                    event.set("user.full_name", v)?;
            }
            }

            let _cond = { !event.has_value("zoom.registrant") && !event.has_value("zoom.participant") };
            if _cond {
            let v = json!(event.get("zoom.operator_id").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.id", v)?;
            }
            }

            let _cond = { !event.has_value("zoom.registrant") && !event.has_value("zoom.participant") };
            if _cond {
            let v = json!(event.get("zoom.operator").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.email", v)?;
            }
            }

            let _cond = { event.has_value("zoom.webinar.host_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.webinar.host_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.registrant.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.registrant.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.participant.id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.participant.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.participant.user_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.participant.user_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.participant.participant_uuid") };
            if _cond {
                event.append_unique("related.user", json!(event.get("zoom.participant.participant_uuid").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("zoom.participant.public_ip") && event.get_str("zoom.participant.public_ip") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("zoom.participant.public_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "zoom.participant.public_ip".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
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
