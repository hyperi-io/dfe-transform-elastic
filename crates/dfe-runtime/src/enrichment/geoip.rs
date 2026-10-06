// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `GeoIP` enrichment using `MaxMind` MMDB.
//!
//! Provides IP-to-geography lookups using memory-mapped `MaxMind` `GeoLite2`
//! databases (City, Country, ASN).

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::Path;

use maxminddb::Reader;
use serde_json::{Value, json};

use crate::error::{Result, TransformError};
use crate::event::Event;

/// `GeoIP` lookup result as a flat map of property → value.
pub type GeoIpResult = HashMap<String, Value>;

/// `GeoIP` enrichment engine wrapping a `MaxMind` MMDB reader.
pub struct GeoIpEnrichment {
    reader: Reader<Vec<u8>>,
    db_type: GeoIpDbType,
}

/// Database type determines which fields are extracted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoIpDbType {
    City,
    Country,
    Asn,
}

impl GeoIpEnrichment {
    /// Open a `MaxMind` MMDB file.
    pub fn open(path: &Path) -> std::result::Result<Self, String> {
        let reader =
            Reader::open_readfile(path).map_err(|e| format!("failed to open MMDB: {e}"))?;

        let db_type = match reader.metadata().database_type.as_str() {
            t if t.contains("City") => GeoIpDbType::City,
            t if t.contains("Country") => GeoIpDbType::Country,
            t if t.contains("ASN") => GeoIpDbType::Asn,
            other => return Err(format!("unsupported MMDB type: {other}")),
        };

        Ok(Self { reader, db_type })
    }

    /// Look up an IP address and return matching properties.
    pub fn lookup(&self, ip_str: &str) -> std::result::Result<GeoIpResult, String> {
        let ip: IpAddr = ip_str
            .parse()
            .map_err(|e| format!("invalid IP '{ip_str}': {e}"))?;
        self.lookup_addr(ip)
    }

    /// Look up an address that is already parsed.
    ///
    /// The caller on the hot path has to parse before it can decide whether the
    /// address is private, so this is the entry point that does not re-parse.
    pub fn lookup_addr(&self, ip: IpAddr) -> std::result::Result<GeoIpResult, String> {
        let lookup_result = self
            .reader
            .lookup(ip)
            .map_err(|e| format!("MMDB lookup failed: {e}"))?;

        let record: Value = match lookup_result
            .decode()
            .map_err(|e| format!("MMDB decode failed: {e}"))?
        {
            Some(v) => v,
            None => return Ok(GeoIpResult::new()),
        };

        let mut result = GeoIpResult::new();

        match self.db_type {
            GeoIpDbType::City => extract_city_fields(&record, &mut result),
            GeoIpDbType::Country => extract_country_fields(&record, &mut result),
            GeoIpDbType::Asn => extract_asn_fields(&record, &mut result),
        }

        Ok(result)
    }

    /// Enrich an event with `GeoIP` data.
    pub fn enrich(
        &self,
        event: &mut Event,
        ip_field: &str,
        target_prefix: &str,
        properties: Option<&[String]>,
        ignore_missing: bool,
    ) -> Result<()> {
        let ip_str = match event.get_str(ip_field) {
            Some(v) => v.to_string(),
            None if ignore_missing => return Ok(()),
            None => {
                return Err(TransformError::FieldNotFound {
                    path: ip_field.into(),
                });
            }
        };

        match self.lookup(&ip_str) {
            Ok(geo) => {
                for (key, value) in &geo {
                    // Filter by properties if specified
                    if let Some(props) = properties
                        && !props.iter().any(|p| p == key)
                    {
                        continue;
                    }
                    let field_path = format!("{target_prefix}.{key}");
                    event.set(&field_path, value.clone())?;
                }
                Ok(())
            }
            Err(msg) => Err(TransformError::EnrichmentError {
                enrichment: "geoip".into(),
                message: msg,
            }),
        }
    }
}

fn extract_city_fields(record: &Value, result: &mut GeoIpResult) {
    extract_country_fields(record, result);

    if let Some(city) = record.pointer("/city/names/en") {
        result.insert("city_name".into(), city.clone());
    }
    if let Some(region_code) = record.pointer("/subdivisions/0/iso_code")
        && let Some(country_code) = result.get("country_iso_code")
    {
        result.insert(
            "region_iso_code".into(),
            json!(format!(
                "{}-{}",
                country_code.as_str().unwrap_or(""),
                region_code.as_str().unwrap_or("")
            )),
        );
    }
    if let Some(region_name) = record.pointer("/subdivisions/0/names/en") {
        result.insert("region_name".into(), region_name.clone());
    }
    if let Some(tz) = record.pointer("/location/time_zone") {
        result.insert("timezone".into(), tz.clone());
    }
    if let (Some(lat), Some(lon)) = (
        record.pointer("/location/latitude"),
        record.pointer("/location/longitude"),
    ) {
        result.insert("location".into(), json!({ "lat": lat, "lon": lon }));
    }
}

fn extract_country_fields(record: &Value, result: &mut GeoIpResult) {
    if let Some(code) = record.pointer("/country/iso_code") {
        result.insert("country_iso_code".into(), code.clone());
    }
    if let Some(name) = record.pointer("/country/names/en") {
        result.insert("country_name".into(), name.clone());
    }
    if let Some(continent) = record.pointer("/continent/names/en") {
        result.insert("continent_name".into(), continent.clone());
    }
}

fn extract_asn_fields(record: &Value, result: &mut GeoIpResult) {
    if let Some(asn) = record.get("autonomous_system_number") {
        result.insert("asn".into(), asn.clone());
    }
    if let Some(org) = record.get("autonomous_system_organization") {
        result.insert("organization_name".into(), org.clone());
    }
    if let Some(network) = record.get("network") {
        result.insert("network".into(), network.clone());
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn db_type_detection() {
        // Test the enum variants
        assert_eq!(GeoIpDbType::City, GeoIpDbType::City);
        assert_ne!(GeoIpDbType::City, GeoIpDbType::Asn);
    }

    #[test]
    fn extract_city_from_json() {
        let record = json!({
            "country": {
                "iso_code": "SE",
                "names": { "en": "Sweden" }
            },
            "continent": {
                "names": { "en": "Europe" }
            },
            "city": {
                "names": { "en": "Stockholm" }
            },
            "subdivisions": [{
                "iso_code": "AB",
                "names": { "en": "Stockholm County" }
            }],
            "location": {
                "latitude": 59.3293,
                "longitude": 18.0686,
                "time_zone": "Europe/Stockholm"
            }
        });

        let mut result = GeoIpResult::new();
        extract_city_fields(&record, &mut result);

        assert_eq!(result["country_iso_code"], json!("SE"));
        assert_eq!(result["country_name"], json!("Sweden"));
        assert_eq!(result["continent_name"], json!("Europe"));
        assert_eq!(result["city_name"], json!("Stockholm"));
        assert_eq!(result["region_iso_code"], json!("SE-AB"));
        assert_eq!(result["region_name"], json!("Stockholm County"));
        assert_eq!(result["timezone"], json!("Europe/Stockholm"));
        assert!(result.contains_key("location"));
    }

    #[test]
    fn extract_asn_from_json() {
        let record = json!({
            "autonomous_system_number": 29518,
            "autonomous_system_organization": "Bredband2 AB",
            "network": "89.160.20.0/24"
        });

        let mut result = GeoIpResult::new();
        extract_asn_fields(&record, &mut result);

        assert_eq!(result["asn"], json!(29518));
        assert_eq!(result["organization_name"], json!("Bredband2 AB"));
        assert_eq!(result["network"], json!("89.160.20.0/24"));
    }

    #[test]
    fn extract_country_from_json() {
        let record = json!({
            "country": {
                "iso_code": "AU",
                "names": { "en": "Australia" }
            },
            "continent": {
                "names": { "en": "Oceania" }
            }
        });

        let mut result = GeoIpResult::new();
        extract_country_fields(&record, &mut result);

        assert_eq!(result["country_iso_code"], json!("AU"));
        assert_eq!(result["country_name"], json!("Australia"));
        assert_eq!(result["continent_name"], json!("Oceania"));
    }
}
