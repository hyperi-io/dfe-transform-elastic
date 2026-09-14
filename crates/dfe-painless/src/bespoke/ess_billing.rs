// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ess_billing`'s region-to-geo lookup, transcribed.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// Every region the billing pipeline knows a location for: the key, then the
/// longitude, latitude and name it writes.
///
/// `aws-us-east-1` answering to the NAME `aws-us-east-2` is the vendor's own
/// table, reproduced rather than corrected -- the two entries also disagree on
/// longitude, so it is a copy-paste in one field and not a merged row.
const REGION_GEO: [(&str, f64, f64, &str); 59] = [
    ("gcp-us-central1", -93.2650, 44.9778, "gcp-us-central1"),
    ("gcp-us-east4", -80.1918, 25.7617, "gcp-us-east4"),
    ("gcp-us-west1", -122.4194, 37.7749, "gcp-us-west1"),
    ("gcp-us-west2", -118.2437, 34.0522, "gcp-us-west2"),
    ("gcp-us-east1", -77.0369, 38.9072, "gcp-us-east1"),
    (
        "gcp-northamerica-northeast1",
        -73.6148,
        45.5059,
        "gcp-northamerica-northeast1",
    ),
    (
        "gcp-southamerica-east1",
        -46.6333,
        -23.5505,
        "gcp-southamerica-east1",
    ),
    ("gcp-europe-west1", -6.2603, 53.3498, "gcp-europe-west1"),
    ("gcp-europe-west2", -0.1276, 51.5074, "gcp-europe-west2"),
    ("gcp-europe-west3", 11.5820, 48.1351, "gcp-europe-west3"),
    ("gcp-europe-west4", 19.0402, 47.4979, "gcp-europe-west4"),
    ("gcp-europe-north1", 24.9458, 60.1921, "gcp-europe-north1"),
    ("gcp-asia-east1", 121.5654, 25.0330, "gcp-asia-east1"),
    (
        "gcp-asia-northeast1",
        139.6917,
        35.6895,
        "gcp-asia-northeast1",
    ),
    ("gcp-asia-south1", 72.8777, 19.0760, "gcp-asia-south1"),
    (
        "gcp-asia-southeast1",
        103.8198,
        1.3521,
        "gcp-asia-southeast1",
    ),
    (
        "gcp-australia-southeast1",
        151.2093,
        -33.8688,
        "gcp-australia-southeast1",
    ),
    (
        "gcp-asia-northeast3",
        126.9780,
        37.5665,
        "gcp-asia-northeast3",
    ),
    (
        "gcp-asia-southeast2",
        106.8650,
        -6.2088,
        "gcp-asia-southeast2",
    ),
    ("gcp-europe-west9", 4.9041, 52.3676, "gcp-europe-west9"),
    ("gcp-me-west1", 34.8516, 31.0461, "gcp-me-west1"),
    ("aws-us-east-2", -82.9988, 39.9612, "aws-us-east-2"),
    ("aws-us-east-1", -77.0369, 39.9612, "aws-us-east-2"),
    ("us-east-1", -77.0369, 38.9072, "us-east-1"),
    ("eu-west-1", -6.2603, 53.3498, "eu-west-1"),
    ("us-west-1", -122.4194, 37.7749, "us-west-1"),
    ("us-west-2", -122.6765, 45.5235, "us-west-2"),
    ("aws-ca-central-1", -75.6972, 45.4215, "aws-ca-central-1"),
    ("sa-east-1", -46.6333, -23.5505, "sa-east-1"),
    ("aws-eu-west-2", -0.1276, 51.5074, "aws-eu-west-2"),
    ("aws-eu-west-3", 2.3522, 48.8566, "aws-eu-west-3"),
    ("aws-eu-central-1", 8.6821, 50.1109, "aws-eu-central-1"),
    ("aws-ap-south-1", 72.8777, 19.0760, "aws-ap-south-1"),
    ("ap-northeast-1", 139.6917, 35.6895, "ap-northeast-1"),
    (
        "aws-ap-northeast-2",
        126.9780,
        37.5665,
        "aws-ap-northeast-2",
    ),
    ("aws-ap-east-1", 114.1095, 22.3964, "aws-ap-east-1"),
    ("ap-southeast-1", 103.8198, 1.3521, "ap-southeast-1"),
    ("ap-southeast-2", 151.2093, -33.8688, "ap-southeast-2"),
    ("aws-af-south-1", 18.4241, -33.9249, "aws-af-south-1"),
    ("aws-me-south-1", 55.2708, 25.2048, "aws-me-south-1"),
    ("aws-eu-south-1", 9.1900, 45.4642, "aws-eu-south-1"),
    ("aws-eu-north-1", 18.0686, 59.3293, "aws-eu-north-1"),
    ("aws-eu-central-2", 16.3738, 48.2082, "aws-eu-central-2"),
    ("azure-eastus2", -80.1918, 25.7617, "azure-eastus2"),
    ("azure-eastus", -77.0369, 38.9072, "azure-eastus"),
    ("azure-centralus", -93.2650, 44.9778, "azure-centralus"),
    (
        "azure-southcentralus",
        -97.7431,
        30.2672,
        "azure-southcentralus",
    ),
    ("azure-westus2", -122.6765, 45.5235, "azure-westus2"),
    ("azure-northeurope", -6.2603, 53.3498, "azure-northeurope"),
    ("azure-uksouth", -0.1276, 51.5074, "azure-uksouth"),
    ("azure-westeurope", 4.9041, 52.3676, "azure-westeurope"),
    (
        "azure-francecentral",
        2.3522,
        48.8566,
        "azure-francecentral",
    ),
    ("azure-japaneast", 139.6917, 35.6895, "azure-japaneast"),
    (
        "azure-southeastasia",
        103.8198,
        1.3521,
        "azure-southeastasia",
    ),
    (
        "azure-australiaeast",
        151.2093,
        -33.8688,
        "azure-australiaeast",
    ),
    (
        "azure-canadacentral",
        -75.6972,
        45.4215,
        "azure-canadacentral",
    ),
    ("azure-brazilsouth", -46.6333, -23.5505, "azure-brazilsouth"),
    (
        "azure-southafricanorth",
        18.4241,
        -33.9249,
        "azure-southafricanorth",
    ),
    ("azure-centralindia", 77.5946, 12.9716, "azure-centralindia"),
];

/// `geo_script` in `ess_billing/billing`: `cloud.geo` for a region the table
/// names, and nothing at all for one it does not.
///
/// The `set_unknown_geo` processor behind this fills in the placeholder
/// location wherever the lookup missed, so declining here is the whole answer.
fn region_geo(event: &mut Event, _params: &Value) {
    let Some(region) = event.get_str("cloud.region") else {
        return;
    };
    let Some(&(_, lon, lat, name)) = REGION_GEO.iter().find(|(key, ..)| *key == region) else {
        return;
    };
    let mut location = Map::new();
    location.insert("lon".to_owned(), Value::from(lon));
    location.insert("lat".to_owned(), Value::from(lat));
    let mut geo = Map::new();
    geo.insert("location".to_owned(), Value::Object(location));
    geo.insert("name".to_owned(), Value::from(name));
    let _ = event.set("cloud.geo", Value::Object(geo));
}

/// Every `ess_billing` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "fd78808f86da6362137b5fe4698b42eab1a6c9638b0cec3d335575818bb4d0b7",
    source: "ess_billing",
    name: "region_geo",
    run: region_geo,
}];
