// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Runners transcribed by hand, keyed by the script each stands in for.
//!
//! The ladder recognises a script's TEXT and dispatches to a runner written
//! for that pattern. A script that appears once in the catalogue costs a whole
//! matcher and pays back one event, and a matcher gated on a keyword can claim
//! a script it cannot run. For those the Painless is transcribed into one
//! function local to its source and registered here against the hash of the
//! script's text. [`crate::plan::PainlessPlan`] consults this table before
//! the params matcher and the ladder, once per call site, so a registered
//! script bypasses whatever claimed it and the hot path pays nothing.
//!
//! One module per source, one `fn` per script, every entry in that module's
//! `ENTRIES`. A function is promoted to a pattern the day a second source
//! spells the same script.
//!
//! The key is [`script_hash`]. `crates/dfe-transforms/tests/bespoke_registry.rs`
//! holds every registered hash to a script in the generated tree, so a vendor
//! change fails loudly instead of falling back to the ladder in silence.
//!
//! Transcribing Painless onto [`Event`]: `ctx.a.b` reads as `event.get("a.b")`
//! and writes as `event.set`; a path the script reads back after writing goes
//! through `Event::update`, because `set` splits a dotted key that `get`
//! honours flat. `x != null` is `has_value`, `containsKey` is `has`, and a
//! removal is `Event::remove` -- never `Map::remove`, which reorders the
//! document under `preserve_order`. `new HashSet()` iterates in Java bucket
//! order, which `crate::helpers::java_bucket` reproduces. `params.x` is the
//! `params` value handed to the runner. The processor's `if:` guard is not
//! part of the script and stays where the generator put it.

use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use serde_json::Value;
use sha2::{Digest, Sha256};

use dfe_core::event::Event;

mod abnormal_security;
mod airlock_digital;
mod akamai;
mod anthropic_metrics;
mod arista_ngfw;
mod atlassian_jira;
mod auditd;
mod aws;
mod aws_billing;
mod axonius;
mod azure_ai_foundry;
mod azure_openai;
mod backstage;
mod beelzebub;
mod beyondtrust_epm;
mod bitwarden;
mod box_events;
mod cato_networks;
mod checkpoint;
mod cisco_ise;
mod cisco_secure_endpoint;
mod citrix_adc;
mod cyberark_pta;
mod cyera;
mod darktrace;
mod dataminr_pulse;
mod elastic_agent;
mod elastic_package_registry;
mod entityanalytics_ad;
mod entityanalytics_okta;
mod ess_billing;
mod extrahop;
mod f5_bigip;
mod github;
mod gitlab;
mod google_workspace;
mod grafana;
mod island_browser;
mod jamf_compliance_reporter;
mod jamf_pro;
mod kolide;
mod kubernetes;
mod microsoft_intune;
mod modsecurity;
mod mongodb_atlas;
mod netbox;
mod nginx;
mod nozomi_networks;
mod prisma_access;
mod qualys_vmdr;
mod rubrik;
mod sentinel_one_cloud_funnel;
mod snyk;
mod splunk;
mod sublime_security;
mod suricata;
mod symantec_endpoint_security;
mod sysdig;
mod tenable_sc;
mod ti_crowdstrike;
mod ti_mandiant_advantage;
mod ti_misp;
mod ti_opencti;
mod ti_rapid7_threat_command;
mod ti_recordedfuture;
mod ti_ticura;
mod trend_micro_vision_one;
mod trendmicro;
mod vsphere;
mod wiz;
mod xm_cyber;
mod zoom;
mod zscaler_zpa;

/// A transcribed script's effect on one event.
///
/// `params` is the pipeline's `params` block verbatim, [`Value::Null`] when
/// the processor carries none.
pub type Runner = fn(&mut Event, &Value);

/// One transcribed script.
pub struct Entry {
    /// [`script_hash`] of the script this stands in for.
    pub hash: &'static str,
    /// The source the transcription lives under, for the census.
    pub source: &'static str,
    /// The function's name, for the census.
    pub name: &'static str,
    /// The transcription.
    pub run: Runner,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bespoke({}::{})", self.source, self.name)
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for Entry {}

/// Every source module's entries, in registration order.
const ENTRIES: &[&[Entry]] = &[
    abnormal_security::ENTRIES,
    airlock_digital::ENTRIES,
    akamai::ENTRIES,
    anthropic_metrics::ENTRIES,
    arista_ngfw::ENTRIES,
    atlassian_jira::ENTRIES,
    auditd::ENTRIES,
    aws::ENTRIES,
    aws_billing::ENTRIES,
    axonius::ENTRIES,
    azure_ai_foundry::ENTRIES,
    azure_openai::ENTRIES,
    backstage::ENTRIES,
    beelzebub::ENTRIES,
    beyondtrust_epm::ENTRIES,
    bitwarden::ENTRIES,
    box_events::ENTRIES,
    cato_networks::ENTRIES,
    checkpoint::ENTRIES,
    cisco_ise::ENTRIES,
    cisco_secure_endpoint::ENTRIES,
    citrix_adc::ENTRIES,
    cyberark_pta::ENTRIES,
    cyera::ENTRIES,
    darktrace::ENTRIES,
    dataminr_pulse::ENTRIES,
    elastic_agent::ENTRIES,
    elastic_package_registry::ENTRIES,
    entityanalytics_ad::ENTRIES,
    entityanalytics_okta::ENTRIES,
    ess_billing::ENTRIES,
    extrahop::ENTRIES,
    f5_bigip::ENTRIES,
    github::ENTRIES,
    gitlab::ENTRIES,
    google_workspace::ENTRIES,
    grafana::ENTRIES,
    island_browser::ENTRIES,
    jamf_compliance_reporter::ENTRIES,
    jamf_pro::ENTRIES,
    kolide::ENTRIES,
    kubernetes::ENTRIES,
    microsoft_intune::ENTRIES,
    modsecurity::ENTRIES,
    mongodb_atlas::ENTRIES,
    netbox::ENTRIES,
    nginx::ENTRIES,
    nozomi_networks::ENTRIES,
    prisma_access::ENTRIES,
    qualys_vmdr::ENTRIES,
    rubrik::ENTRIES,
    sentinel_one_cloud_funnel::ENTRIES,
    snyk::ENTRIES,
    splunk::ENTRIES,
    sublime_security::ENTRIES,
    suricata::ENTRIES,
    symantec_endpoint_security::ENTRIES,
    sysdig::ENTRIES,
    tenable_sc::ENTRIES,
    ti_crowdstrike::ENTRIES,
    ti_mandiant_advantage::ENTRIES,
    ti_misp::ENTRIES,
    ti_opencti::ENTRIES,
    ti_rapid7_threat_command::ENTRIES,
    ti_recordedfuture::ENTRIES,
    ti_ticura::ENTRIES,
    trend_micro_vision_one::ENTRIES,
    trendmicro::ENTRIES,
    vsphere::ENTRIES,
    wiz::ENTRIES,
    xm_cyber::ENTRIES,
    zoom::ENTRIES,
    zscaler_zpa::ENTRIES,
];

/// The key a script is registered under.
///
/// The script with its escapes resolved, every run of whitespace collapsed to
/// one space, both ends trimmed, hashed with SHA-256 and rendered as lowercase
/// hex. Whitespace is normalised so the YAML source, the generated literal and
/// a hand-pasted copy all key the same.
#[must_use]
pub fn script_hash(script: &str) -> String {
    let mut hasher = Sha256::new();
    let mut started = false;
    let mut pending_space = false;
    let mut buf = [0u8; 4];
    for ch in script.chars() {
        if ch.is_whitespace() {
            pending_space = started;
            continue;
        }
        if pending_space {
            hasher.update(b" ");
            pending_space = false;
        }
        hasher.update(ch.encode_utf8(&mut buf).as_bytes());
        started = true;
    }
    format!("{:x}", hasher.finalize())
}

/// The transcription registered for a script, if any.
#[must_use]
pub fn lookup(script: &str) -> Option<&'static Entry> {
    table().get(script_hash(script).as_str()).copied()
}

/// Every registered transcription.
pub fn entries() -> impl Iterator<Item = &'static Entry> {
    ENTRIES.iter().flat_map(|module| module.iter())
}

fn table() -> &'static HashMap<&'static str, &'static Entry> {
    static TABLE: OnceLock<HashMap<&'static str, &'static Entry>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut map = HashMap::new();
        for entry in entries() {
            assert!(
                map.insert(entry.hash, entry).is_none(),
                "two bespoke runners are registered under {}",
                entry.hash
            );
        }
        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_ignores_whitespace_layout_but_not_content() {
        let one = script_hash("ctx.a = 1;\n  ctx.b = 2;  ");
        assert_eq!(one, script_hash("ctx.a = 1; ctx.b = 2;"));
        assert_eq!(one.len(), 64);
        assert_ne!(one, script_hash("ctx.a = 1;ctx.b = 2;"));
        assert_ne!(one, script_hash("ctx.a = 1; ctx.b = 3;"));
    }

    #[test]
    fn unregistered_script_has_no_entry() {
        assert!(lookup("def nothing() { return 1; }").is_none());
    }
}
