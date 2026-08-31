// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `event_enrich` pipeline.
pub struct EventEnrich;

impl Transform for EventEnrich {
    fn name(&self) -> &str {
        "event_enrich"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if event.has_value("client.address") {
            if let Some(input) = event.get_string("client.address") {
                // Grok pattern: ^(%{IP:client.ip}|%{HOSTNAME:client.domain})$
                if !cached_grok!("^(%{IP:client.ip}|%{HOSTNAME:client.domain})$").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
        }

        if event.has_value("client.ip") {
            if let Some(ip_str) = event.get_string("client.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-City.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                    if let Some(v) = geo.get("country_iso_code") {
                        event.set("client.geo.country_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("country_name") {
                        event.set("client.geo.country_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("continent_name") {
                        event.set("client.geo.continent_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_iso_code") {
                        event.set("client.geo.region_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_name") {
                        event.set("client.geo.region_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("city_name") {
                        event.set("client.geo.city_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("timezone") {
                        event.set("client.geo.timezone", v.clone())?;
                    }
                    if let Some(v) = geo.get("location") {
                        event.set("client.geo.location", v.clone())?;
                    }
                }
            }
        }

        if event.has_value("client.ip") {
            if let Some(ip_str) = event.get_string("client.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("client.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("client.as.organization_name", v.clone())?;
                    }
                }
            }
        }

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

        if event.has_value("server.address") {
            if let Some(input) = event.get_string("server.address") {
                // Grok pattern: ^(%{IP:server.ip}|%{HOSTNAME:server.domain})$
                if !cached_grok!("^(%{IP:server.ip}|%{HOSTNAME:server.domain})$").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
        }

        if event.has_value("server.ip") {
            if let Some(ip_str) = event.get_string("server.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-City.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                    if let Some(v) = geo.get("country_iso_code") {
                        event.set("server.geo.country_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("country_name") {
                        event.set("server.geo.country_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("continent_name") {
                        event.set("server.geo.continent_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_iso_code") {
                        event.set("server.geo.region_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_name") {
                        event.set("server.geo.region_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("city_name") {
                        event.set("server.geo.city_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("timezone") {
                        event.set("server.geo.timezone", v.clone())?;
                    }
                    if let Some(v) = geo.get("location") {
                        event.set("server.geo.location", v.clone())?;
                    }
                }
            }
        }

        if event.has_value("server.ip") {
            if let Some(ip_str) = event.get_string("server.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("server.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("server.as.organization_name", v.clone())?;
                    }
                }
            }
        }

            if event.has_value("server.as.asn") {
                event.rename("server.as.asn", "server.as.number")?;
            }

            if event.has_value("server.as.organization_name") {
                event.rename("server.as.organization_name", "server.as.organization.name")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("source.address") {
            if let Some(input) = event.get_string("source.address") {
                // Grok pattern: ^(%{IP:source.ip}|%{HOSTNAME:source.domain})$
                if !cached_grok!("^(%{IP:source.ip}|%{HOSTNAME:source.domain})$").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
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

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("destination.address") {
            if let Some(input) = event.get_string("destination.address") {
                // Grok pattern: ^(%{IP:destination.ip}|%{HOSTNAME:destination.domain})$
                if !cached_grok!("^(%{IP:destination.ip}|%{HOSTNAME:destination.domain})$").extract_into(&input, event)? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }
        }
            Ok(())
        })();

        if event.has_value("destination.ip") {
            if let Some(ip_str) = event.get_string("destination.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-City.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                    if let Some(v) = geo.get("country_iso_code") {
                        event.set("destination.geo.country_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("country_name") {
                        event.set("destination.geo.country_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("continent_name") {
                        event.set("destination.geo.continent_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_iso_code") {
                        event.set("destination.geo.region_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_name") {
                        event.set("destination.geo.region_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("city_name") {
                        event.set("destination.geo.city_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("timezone") {
                        event.set("destination.geo.timezone", v.clone())?;
                    }
                    if let Some(v) = geo.get("location") {
                        event.set("destination.geo.location", v.clone())?;
                    }
                }
            }
        }

        if event.has_value("destination.ip") {
            if let Some(ip_str) = event.get_string("destination.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("destination.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("destination.as.organization_name", v.clone())?;
                    }
                }
            }
        }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename("destination.as.organization_name", "destination.as.organization.name")?;
            }

        let _cond = { !event.has_value("user.email") && event.has_value("user.name") && event.get_str("user.name").map(|s| s.find("@").map(|b| s[..b].chars().count())).is_some_and(|i| i.is_some_and(|i| i > 0)) };
        if _cond {
            event.rename("user.name", "user.email")?;
        }

        let _cond = { !event.has_value("user.name") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("user.email") {
            if let Some(input) = event.get_string("user.email") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find("@") else { break 'dissect false };
                    captured.push(("user.name", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("@") else { break 'dissect false };
                    remaining = rest;
                    captured.push(("user.domain", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                }
            }
        }
            Ok(())
        })();
        }

        let _cond = { event.has_value("client.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("destination.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("server.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("server.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("source.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("teleport.audit.certificate.identity.client_ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("teleport.audit.certificate.identity.client_ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("host.hostname") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("host.hostname").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("client.domain") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("client.domain").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("destination.domain") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("destination.domain").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("server.domain") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("server.domain").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("source.domain") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("source.domain").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("url.domain") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("url.domain").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("teleport.audit.app.public_address") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("teleport.audit.app.public_address").map_or_else(String::new, template_to_string)))?;
        }

        event.set("related.user", json!(""))?;

        let _cond = { event.has_value("user.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("user.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("user.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("teleport.audit.resource.name") && event.get_str("event.action").is_some_and(|s| s.starts_with("user.")) };
        if _cond {
            event.append_unique("related.user", json!(event.get("teleport.audit.resource.name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("process.user.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("process.user.name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("teleport.audit.database.user") };
        if _cond {
            event.append_unique("related.user", json!(event.get("teleport.audit.database.user").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("teleport.audit.database.user_change.username") };
        if _cond {
            event.append_unique("related.user", json!(event.get("teleport.audit.database.user_change.username").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("teleport.audit.certificate.identity.user") };
        if _cond {
            event.append_unique("related.user", json!(event.get("teleport.audit.certificate.identity.user").map_or_else(String::new, template_to_string)))?;
        }

            // Painless script
            // Source: if (ctx.teleport?.audit?.certificate?.identity?.logins != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.logins);\n}\nif (ctx.teleport?.audit?.certificate?.identity?.participants != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.participants);\n}\nif (ctx.teleport?.audit?.certificate?.identity?.database_users != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.database_users);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.teleport?.audit?.certificate?.identity?.logins != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.logins);\n}\nif (ctx.teleport?.audit?.certificate?.identity?.participants != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.participants);\n}\nif (ctx.teleport?.audit?.certificate?.identity?.database_users != null) {\n  ctx.related.user.addAll(ctx.teleport.audit.certificate.identity.database_users);\n}\n"#))?;

        let _cond = { event.has_value("teleport.audit.certificate.identity.impersonator") };
        if _cond {
            event.append_unique("related.user", json!(event.get("teleport.audit.certificate.identity.impersonator").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("teleport.audit.server.labels.hostname") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("teleport.audit.server.labels.hostname").map_or_else(String::new, template_to_string)))?;
        }

        Ok(TransformResult::Continue)
    }
}
