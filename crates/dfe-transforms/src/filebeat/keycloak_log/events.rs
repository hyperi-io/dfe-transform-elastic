// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `events` pipeline.
pub struct Events;

impl Transform for Events {
    fn name(&self) -> &str {
        "events"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if event.has_value("message") {
                if let Some(kv_str) = event.get_string("message") {
                    for pair in kv_str.split(", ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = value.trim_matches(|c| "\\\\\"".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("json.{}", key), value)?;
                            }
                        }
                    }
                }
            }

                if event.has_value("json.type") {
                    event.rename("json.type", "keycloak.login.type")?;
                }

            let _cond = { event.has_value("keycloak.login.type") && event.get_str("keycloak.login.type").is_some_and(|s| s.eq_ignore_ascii_case("login")) };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.has_value("keycloak.login.type") && event.get_str("keycloak.login.type").is_some_and(|s| s.eq_ignore_ascii_case("login_error")) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
            event.set("event.outcome", json!("unknown"))?;
            }

                if event.has_value("json.operationType") {
                    event.rename("json.operationType", "keycloak.admin.operation")?;
                }

                if event.has_value("json.resourceType") {
                    event.rename("json.resourceType", "keycloak.admin.resource.type")?;
                }

                if event.has_value("json.resourcePath") {
                    event.rename("json.resourcePath", "keycloak.admin.resource.path")?;
                }

            let _cond = { event.has_value("keycloak.login") };
            if _cond {
            event.set("keycloak.event_type", json!("login"))?;
            }

            let _cond = { event.has_value("keycloak.admin") };
            if _cond {
            event.set("keycloak.event_type", json!("admin"))?;
            }

            let _cond = { event.has_value("keycloak.admin") };
            if _cond {
            event.set("event.code", json!(format!("{}-{}", event.get("keycloak.admin.operation").map_or_else(String::new, template_to_string), event.get("keycloak.admin.resource.type").map_or_else(String::new, template_to_string))))?;
            }

            let _cond = { event.has_value("keycloak.admin") };
            if _cond {
            if let Some(v) = event.get("event.code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }
            }

            let _cond = { event.has_value("keycloak.login") && !event.has_value("event.code") };
            if _cond {
                if event.has_value("json.error") {
                    event.rename("json.error", "event.code")?;
                }
            }

            if let Some(v) = event.get("keycloak.login.type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

                if event.has_value("json.realmId") {
                    event.rename("json.realmId", "keycloak.realm.id")?;
                }

            let _cond = { event.get_str("json.clientId") != Some("null") };
            if _cond {
                if event.has_value("json.clientId") {
                    event.rename("json.clientId", "keycloak.client.id")?;
                }
            }

            let _cond = { event.get_str("json.userId") != Some("null") };
            if _cond {
                if event.has_value("json.userId") {
                    event.rename("json.userId", "user.id")?;
                }
            }

                if event.has_value("json.ipAddress") {
                    event.rename("json.ipAddress", "source.address")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })();

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

                if event.has_value("json.redirect_uri") {
                    event.rename("json.redirect_uri", "keycloak.login.redirect_uri")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "keycloak.login.redirect_uri", "url", true, false)?;
                Ok(())
            })();

                if event.has_value("json.auth_method") {
                    event.rename("json.auth_method", "keycloak.login.auth_method")?;
                }

                if event.has_value("json.auth_type") {
                    event.rename("json.auth_type", "keycloak.login.auth_type")?;
                }

                if event.has_value("json.code_id") {
                    event.rename("json.code_id", "keycloak.login.code_id")?;
                }

                if event.has_value("json.username") {
                    event.rename("json.username", "user.name")?;
                }

                if event.has_value("json.authSessionParentId") {
                    event.rename("json.authSessionParentId", "keycloak.login.auth_session_parent_id")?;
                }

                if event.has_value("json.authSessionTabId") {
                    event.rename("json.authSessionTabId", "keycloak.login.auth_session_tab_id")?;
                }

            let _cond = { event.get_str("json.impersonator_realm") != Some("null") };
            if _cond {
                if event.has_value("json.impersonator_realm") {
                    event.rename("json.impersonator_realm", "keycloak.impersonator_realm")?;
                }
            }

            let _cond = { event.get_str("json.impersonator") != Some("null") };
            if _cond {
                if event.has_value("json.impersonator") {
                    event.rename("json.impersonator", "keycloak.impersonator")?;
                }
            }

            let _cond = { event.get_str("json.sessionId") != Some("null") };
            if _cond {
                if event.has_value("json.sessionId") {
                    event.rename("json.sessionId", "keycloak.session.id")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("keycloak.admin.resource.path") {
                if let Some(input) = event.get_string("keycloak.admin.resource.path") {
                    // Grok pattern: users/%{UUID:user.target.id}
                    // Grok pattern: groups/%{UUID:group.id}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("users/%{UUID:user.target.id}"),
                            cached_grok!("groups/%{UUID:group.id}"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("keycloak.login") };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

                event.append("event.type", json!("info"))?;

            let _cond = { event.get_str("keycloak.login.type") == Some("LOGIN") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("keycloak.login.type") == Some("LOGOUT") };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = { event.has_value("keycloak.admin") };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = { event.has_value("keycloak.admin") };
            if _cond {
                event.append("event.type", json!("admin"))?;
            }

            let _cond = { event.get_str("keycloak.admin.operation") == Some("CREATE") };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("keycloak.admin.operation") == Some("UPDATE") };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("keycloak.admin.operation") == Some("DELETE") };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.get_str("keycloak.admin.resource") == Some("GROUP") };
            if _cond {
                event.append("event.type", json!("group"))?;
            }

            let _cond = { event.get_str("keycloak.admin.resource") == Some("USER") };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append("related.user", json!(event.get("user.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.target.id") };
            if _cond {
                event.append("related.user", json!(event.get("user.target.id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append("related.hosts", json!(event.get("url.domain").map_or_else(String::new, template_to_string)))?;
            }

                event.remove("message");
                event.remove("json");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
