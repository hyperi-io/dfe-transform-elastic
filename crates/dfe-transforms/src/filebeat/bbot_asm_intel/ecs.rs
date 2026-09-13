// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ecs` pipeline.
pub struct Ecs;

impl Transform for Ecs {
    fn name(&self) -> &str {
        "ecs"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            event.append_unique("event.category", json!("network"))?;

            event.append_unique("event.type", json!("info"))?;

        let _cond = { event.has_value("bbot.host") };
        if _cond {
        if let Some(v) = event.get("bbot.host").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.name", v)?;
        }
        }

        let _cond = { event.has_value("bbot.data.DNS_NAME") };
        if _cond {
        if let Some(v) = event.get("bbot.data.DNS_NAME").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.name", v)?;
        }
        }

        let _cond = { event.has_value("bbot.data.DNS_NAME") };
        if _cond {
        if let Some(v) = event.get("bbot.data.DNS_NAME").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("_tmp.host", v)?;
        }
        }

        let _cond = { event.has_value("bbot.data.IP_ADDRESS") };
        if _cond {
            event.append_unique("host.ip", json!(event.get("bbot.data.IP_ADDRESS").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("bbot.data.IP_ADDRESS") {
            if let Some(ip_str) = event.get_string("bbot.data.IP_ADDRESS") {
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

        if event.has_value("bbot.data.IP_ADDRESS") {
            if let Some(ip_str) = event.get_string("bbot.data.IP_ADDRESS") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("host.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("host.as.organization_name", v.clone())?;
                    }
                }
            }
        }

            if event.has_value("host.as.asn") {
                event.rename("host.as.asn", "host.as.number")?;
            }

            if event.has_value("host.as.organization_name") {
                event.rename("host.as.organization_name", "host.as.organization.name")?;
            }

        let _cond = { event.has_value("bbot.resolved_hosts") };
        if _cond {
        if let Some(v) = event.get("bbot.resolved_hosts").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("related.hosts", v)?;
        }
        }

        let _cond = { event.has_value("bbot.host") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("bbot.host").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.DNS_NAME") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("bbot.data.DNS_NAME").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.IP_ADDRESS") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("bbot.data.IP_ADDRESS").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("bbot.dns_children.A") {
            foreach_array(event, "bbot.dns_children.A", |event| {
                event.append_unique("related.ip", json!(event.get("_ingest.value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if event.has_value("bbot.dns_children.AAAA") {
            foreach_array(event, "bbot.dns_children.AAAA", |event| {
                event.append_unique("related.ip", json!(event.get("_ingest.value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if event.has_value("bbot.dns_children.CNAME") {
            foreach_array(event, "bbot.dns_children.CNAME", |event| {
                event.append_unique("related.hosts", json!(event.get("_ingest.value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if event.has_value("bbot.dns_children.MX") {
            foreach_array(event, "bbot.dns_children.MX", |event| {
                event.append_unique("related.hosts", json!(event.get("_ingest.value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if event.has_value("bbot.dns_children.NS") {
            foreach_array(event, "bbot.dns_children.NS", |event| {
                event.append_unique("related.hosts", json!(event.get("_ingest.value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if event.has_value("bbot.dns_children.PTR") {
            foreach_array(event, "bbot.dns_children.PTR", |event| {
                event.append_unique("related.hosts", json!(event.get("_ingest.value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if event.has_value("bbot.dns_children.SOA") {
            foreach_array(event, "bbot.dns_children.SOA", |event| {
                event.append_unique("related.hosts", json!(event.get("_ingest.value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.has_value("bbot.data.PROTOCOL.port") };
        if _cond {
        if let Some(v) = event.get("bbot.data.PROTOCOL.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.port", v)?;
        }
        }

        let _cond = { event.has_value("bbot.data.URL") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.URL").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.URL_UNVERIFIED") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.URL_UNVERIFIED").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.FINDING.url") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.FINDING.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.SOCIAL.url") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.SOCIAL.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.STORAGE_BUCKET.url") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.STORAGE_BUCKET.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.TECHNOLOGY.url") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.TECHNOLOGY.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.VULNERABILITY.url") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.VULNERABILITY.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.WAF.url") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.WAF.url").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.WEBSCREENSHOT.url") };
        if _cond {
        event.set("_tmp.url", json!(event.get("bbot.data.WEBSCREENSHOT.url").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("_tmp.url") {
            uri_parts(event, "_tmp.url", "url", true, false)?;
        }

        let _cond = { event.has_value("url.original") };
        if _cond {
            event.append_unique("url.full", json!(event.get("url.original").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.FINDING.host") };
        if _cond {
        event.set("_tmp.host", json!(event.get("bbot.data.FINDING.host").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.PROTOCOL.host") };
        if _cond {
        event.set("_tmp.host", json!(event.get("bbot.data.PROTOCOL.host").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.TECHNOLOGY.host") };
        if _cond {
        event.set("_tmp.host", json!(event.get("bbot.data.TECHNOLOGY.host").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.VULNERABILITY.host") };
        if _cond {
        event.set("_tmp.host", json!(event.get("bbot.data.VULNERABILITY.host").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("bbot.data.WAF.host") };
        if _cond {
        event.set("_tmp.host", json!(event.get("bbot.data.WAF.host").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("_tmp.host") };
        if _cond {
        event.set("url.domain", json!(event.get("_tmp.host").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("url.domain") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("url.domain").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("url.domain") {
            if let Some(domain) = event.get_string("url.domain") {
                // Public suffix list lookup for registered domain extraction.
                // A failed lookup writes NO target field, which is what
                // Elasticsearch does.
                if let Some(rd) = registered_domain_lookup(&domain) {
                    event.set("url.domain", json!(domain))?;
                    if let Some(registered) = rd.registered_domain {
                        event.set("url.registered_domain", json!(registered))?;
                    }
                    event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("url.subdomain", json!(sub))?;
                    }
                }
            }
        }

        let _cond = { event.has_value("bbot.data.VULNERABILITY.severity") };
        if _cond {
        if let Some(v) = event.get("bbot.data.VULNERABILITY.severity").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.severity", v)?;
        }
        }

        let _cond = { event.has_value("bbot.data") };
        if _cond {
        if event.has_value("bbot.data") {
            foreach_array(event, "bbot.data", |event| {
                map_strings(event, "_ingest._key", "_ingest._key", str::to_lowercase)?;
                Ok(())
            })?;
        }
        }

        Ok(TransformResult::Continue)
    }
}
