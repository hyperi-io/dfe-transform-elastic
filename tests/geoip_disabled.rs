// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `geoip.enabled: false` loads no database, even one sitting where the lookup
//! engine searches. Its own process, because the lookup engine's state is
//! process-wide and set once.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use dfe_runtime::enrichment::geoip_global;

#[tokio::test]
async fn geoip_disabled_loads_no_database_from_the_search_path() {
    let name = "geoip_disabled_loads_no_database_from_the_search_path";
    // One of the directories the lookup engine searches when nothing is provisioned.
    let asn = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/testdata/geoip/dbip-asn-lite.mmdb"
    );
    if !std::path::Path::new(asn).exists() {
        scalo::deployment::test_support::skip(
            "geoip",
            name,
            "no testdata/geoip database to find -- run scripts/download-geoip.sh",
        );
        return;
    }

    let mut config = dfe_transform_elastic::config::default_geoip();
    config.enabled = false;
    dfe_transform_elastic::service::provision_geoip(&config).await;

    assert!(
        !geoip_global::enabled(),
        "geoip.enabled is false, yet {asn} was loaded"
    );
}
