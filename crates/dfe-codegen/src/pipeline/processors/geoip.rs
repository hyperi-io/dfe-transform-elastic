use anyhow::bail;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::{Validate, conditional::Conditional, unsupported_fields};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum GeoIPDB {
    #[serde(rename = "GeoLite2-City.mmdb")]
    #[default]
    CITY,
    #[serde(rename = "GeoLite2-Country.mmdb")]
    COUNTRY,
    #[serde(rename = "GeoLite2-ASN.mmdb")]
    ASN,
}

impl GeoIPDB {
    /// Return the enrichment table name for this database.
    pub fn table_name(&self) -> &'static str {
        match self {
            GeoIPDB::CITY => "geoip_city",
            GeoIPDB::COUNTRY => "geoip_country",
            GeoIPDB::ASN => "geoip_asn",
        }
    }
}

impl std::fmt::Display for GeoIPDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoIPDB::CITY => write!(f, "GeoLite2-City.mmdb"),
            GeoIPDB::COUNTRY => write!(f, "GeoLite2-Country.mmdb"),
            GeoIPDB::ASN => write!(f, "GeoLite2-ASN.mmdb"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Geoip {
    pub field: String,
    pub database_file: Option<GeoIPDB>,
    pub target_field: Option<String>,
    pub properties: Option<Vec<String>>,
    pub ignore_missing: Option<bool>,

    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,

    // Unsupported Fields
    pub first_only: Option<bool>,
    pub download_database_on_pipeline_creation: Option<bool>,
}

const CITY_FIELDS: &[&str] = &[
    "ip",
    "country_iso_code",
    "country_name",
    "continent_name",
    "region_iso_code",
    "region_name",
    "city_name",
    "timezone",
    "latitude",
    "longitude",
    "location",
];

const COUNTRY_FIELDS: &[&str] = &["ip", "country_iso_code", "country_name", "continent_name"];

const ASN_FIELDS: &[&str] = &["ip", "asn", "organization_name", "network"];

impl Validate for Geoip {
    #[instrument(name = "Geoip::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!(
            "geoip",
            self,
            first_only,
            download_database_on_pipeline_creation
        );

        if let Some(properties) = &self.properties {
            match self.database_file.unwrap_or_default() {
                GeoIPDB::CITY => properties.iter().try_for_each(|property| {
                    if CITY_FIELDS.contains(&property.as_str()) {
                        Ok(())
                    } else {
                        bail!("{property} is not among the supported fields for GeoLite2-City.mmdb")
                    }
                }),
                GeoIPDB::ASN => properties.iter().try_for_each(|property| {
                    if ASN_FIELDS.contains(&property.as_str()) {
                        Ok(())
                    } else {
                        bail!("{property} is not among the supported fields for GeoLite2-ASN.mmdb")
                    }
                }),
                GeoIPDB::COUNTRY => properties.iter().try_for_each(|property| {
                    if COUNTRY_FIELDS.contains(&property.as_str()) {
                        Ok(())
                    } else {
                        bail!(
                            "{property} is not among the supported fields for GeoLite2-Country.mmdb"
                        )
                    }
                }),
            }?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::{
            Pipeline, Processor, processors::geoip::GeoIPDB, unsupported_fields_tests,
        };
        use pretty_assertions::assert_eq;

        #[test]
        fn field() {
            let configuration = r"
                processors:
                    - geoip:
                        field: source.ip
            ";

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::Geoip(geoip)] => {
                    assert_eq!(geoip.field, "source.ip");
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn database_file() {
            for database_file in [GeoIPDB::ASN, GeoIPDB::CITY, GeoIPDB::COUNTRY] {
                let configuration = format!(
                    r"
                    processors:
                        - geoip:
                            field: source.ip
                            database_file: {database_file}
                "
                );

                match &Pipeline::parse(&configuration).unwrap().processors[..] {
                    [Processor::Geoip(geoip)] => {
                        assert_eq!(geoip.field, "source.ip");
                        assert_eq!(geoip.database_file.unwrap(), database_file);
                    }
                    _ => panic!("unexpected pipeline structure"),
                }
            }
        }

        #[test]
        fn unsupported_database_file() {
            let database_file = "unsupported.mmdb";

            let configuration = format!(
                r"
                    processors:
                        - geoip:
                            field: source.ip
                            database_file: {database_file}
                "
            );

            assert_eq!(
                Pipeline::parse(&configuration)
                    .unwrap_err()
                    .root_cause()
                    .to_string(),
                "processors[0].geoip.database_file: unknown variant `unsupported.mmdb`, expected one of \
                 `GeoLite2-City.mmdb`, `GeoLite2-Country.mmdb`, `GeoLite2-ASN.mmdb` at line 5 column 44"
            );
        }

        #[test]
        fn target_field() {
            let configuration = r"
                processors:
                    - geoip:
                        field: source.ip
                        target_field: source.geo
            ";

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::Geoip(geoip)] => {
                    assert_eq!(geoip.field, "source.ip");
                    assert_eq!(geoip.target_field.as_ref().unwrap(), "source.geo");
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn properties() {
            let configuration = r"
                processors:
                    - geoip:
                        field: source.ip
                        properties:
                            - continent_name
                            - country_name
            ";

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::Geoip(geoip)] => {
                    assert_eq!(geoip.field, "source.ip");
                    assert_eq!(
                        *geoip.properties.as_ref().unwrap(),
                        vec!["continent_name".to_string(), "country_name".to_string()]
                    );
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn ignore_missing() {
            let configuration = r"
                processors:
                    - geoip:
                        field: source.ip
                        ignore_missing: true
            ";

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::Geoip(geoip)] => {
                    assert_eq!(geoip.field, "source.ip");
                    assert_eq!(geoip.ignore_missing.unwrap(), true);
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        unsupported_fields_tests!(
            "geoip",
            r"
                processors:
                    - geoip:
                        field: source.ip
                        {}: {}
            ",
            first_only => "false",
            download_database_on_pipeline_creation => "false"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use pretty_assertions::assert_eq;
    //
    // use crate::pipeline::Pipeline;
    //
    // #[test]
    // fn happy_path() {
    // let configuration = r#"
    // processors:
    // - geoip:
    // field: ip
    // "#;
    //
    // let lat = NotNan::new(58.4167).unwrap();
    // let long = NotNan::new(15.6167).unwrap();
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ ip: "89.160.20.128" }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // ip: "89.160.20.128",
    // geoip: {
    // "continent_code": "EU",
    // "country_name": "Sweden",
    // "country_iso_code": "SE",
    // "city_name": "Linköping",
    // "region_iso_code": "SE-E",
    // "region_name": "Östergötland County",
    // "location": { "lat": lat, "lon": long }
    // }
    // }),
    // );
    // }
    //
    // #[test]
    // fn target_field() {
    // let configuration = r#"
    // processors:
    // - geoip:
    // field: ip
    // target_field: ip
    // "#;
    //
    // let lat = NotNan::new(58.4167).unwrap();
    // let long = NotNan::new(15.6167).unwrap();
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ ip: "89.160.20.128" }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // ip: {
    // "continent_code": "EU",
    // "country_name": "Sweden",
    // "country_iso_code": "SE",
    // "city_name": "Linköping",
    // "region_iso_code": "SE-E",
    // "region_name": "Östergötland County",
    // "location": { "lat": lat, "lon": long }
    // }
    // }),
    // );
    // }
    //
    // #[test]
    // fn properties_city() {
    // let configuration = r#"
    // processors:
    // - geoip:
    // field: ip
    // properties:
    // - country_iso_code
    // - region_iso_code
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ ip: "89.160.20.128" }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // ip: "89.160.20.128",
    // geoip: {
    // "country_iso_code": "SE",
    // "region_iso_code": "SE-E"
    // }
    // }),
    // );
    // }
    //
    // // Need to find a valid IP in the test DB
    // #[test]
    // fn properties_asn() {
    // let configuration = r#"
    // processors:
    // - geoip:
    // field: ip
    // database_file: GeoLite2-ASN.mmdb
    // properties:
    // - asn
    // - organization_name
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ ip: "89.160.20.112" }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // ip: "89.160.20.112",
    // geoip: {
    // "asn": 29518,
    // "organization_name": "Bredband2 AB"
    // }
    // }),
    // );
    // }
    //
    // #[test]
    // fn ignore_missing_failure() {
    // let configuration = r#"
    // processors:
    // - geoip:
    // field: source.ip
    // ignore_missing: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ ip: "89.160.20.128" }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("missing source.ip for geoip processing"));
    // }
    //
    // #[test]
    // fn ignore_missing() {
    // let configuration = r#"
    // processors:
    // - geoip:
    // field: source.ip
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ ip: "89.160.20.128" }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // ip: "89.160.20.128",
    // }),
    // );
    // }
    //
    // #[test]
    // fn conditional() {
    // let configuration = r#"
    // processors:
    // - geoip:
    // field: source.ip
    // if: ctx?.source?.ip != null
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // ip: "89.160.20.128",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ ip: "89.160.20.128" }))
    // .unwrap()
    // .target,
    // );
    // }
    // }
}
