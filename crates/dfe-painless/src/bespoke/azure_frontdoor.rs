// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `azure_frontdoor`'s identity claims: the claim URIs turned into keys a
//! document can hold, and the user claims gathered under one object.
//!
//! Azure names each claim by its schema URI, so `.` sits inside the KEY
//! rather than between path segments. The first script rewrites those dots to
//! underscores, and only then can the second address a claim by name -- its
//! params table spells the underscored form, so the pair runs in that order
//! and neither half works alone.
//!
//! Both are spelled once per data stream, over `access` and `waf`.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// The schema every gathered user claim is stamped with.
const CLAIMS_SCHEMA: &str = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims";

/// `script_claims_cleanup` in `compat-azure_frontdoor-access-default`.
fn access_claims_cleanup(event: &mut Event, _params: &Value) {
    underscore_claim_keys(event, "azure.frontdoor.access.identity.claims");
}

/// `script_claims_cleanup` in `compat-azure_frontdoor-waf-default`.
fn waf_claims_cleanup(event: &mut Event, _params: &Value) {
    underscore_claim_keys(event, "azure.frontdoor.waf.identity.claims");
}

/// `script_claims_user` in `compat-azure_frontdoor-access-default`.
fn access_claims_user(event: &mut Event, params: &Value) {
    gather_user_claims(event, params, "azure.frontdoor.access.identity");
}

/// `script_claims_user` in `compat-azure_frontdoor-waf-default`.
fn waf_claims_user(event: &mut Event, params: &Value) {
    gather_user_claims(event, params, "azure.frontdoor.waf.identity");
}

/// Rewrite every dot in a claim's KEY to an underscore.
///
/// A `claims` that is not a map raises in Painless, which a runner cannot, so
/// it writes nothing instead.
fn underscore_claim_keys(event: &mut Event, path: &str) {
    let Some(claims) = event.get_object(path) else {
        return;
    };

    let mut rewritten = Map::with_capacity(claims.len());
    for (key, value) in claims {
        rewritten.insert(key.replace('.', "_"), value.clone());
    }

    let _ = event.update(path, Value::Object(rewritten));
}

/// Gather the claims the params table names into one object beside them.
///
/// The table is keyed by the NAME the gathered claim takes and holds the
/// claim's own underscored key, so the lookup runs in that direction.
fn gather_user_claims(event: &mut Event, params: &Value, root: &str) {
    let Some(claims) = event.get_object(&format!("{root}.claims")).cloned() else {
        return;
    };

    let mut gathered = Map::new();
    if let Some(name) = claims.get("name").filter(|held| !held.is_null()) {
        gathered.insert("fullname".to_owned(), name.clone());
    }
    if let Some(table) = params.as_object() {
        for (name, claim) in table {
            let Some(claim) = claim.as_str() else {
                continue;
            };
            if let Some(value) = claims.get(claim).filter(|held| !held.is_null()) {
                gathered.insert(name.clone(), value.clone());
            }
        }
    }

    if gathered.is_empty() {
        return;
    }
    gathered.insert("schema".to_owned(), Value::from(CLAIMS_SCHEMA));
    let _ = event.set(
        &format!("{root}.claims_initiated_by_user"),
        Value::Object(gathered),
    );
}

/// Every `azure_frontdoor` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "d7a036120ea438e3e1e880b7ea8b78213cee967af6cf3d82541fd3f416307d7b",
        source: "azure_frontdoor",
        name: "access_claims_cleanup",
        run: access_claims_cleanup,
    },
    Entry {
        hash: "d2e37e9291561784a6703d564763ad37cc8c417b91884ee7d7f344083fd5da91",
        source: "azure_frontdoor",
        name: "waf_claims_cleanup",
        run: waf_claims_cleanup,
    },
    Entry {
        hash: "c7f6d3fa8fdbd2190ea36506e703dbc5f73bc3bb81c7bddbf65a39ef71df1571",
        source: "azure_frontdoor",
        name: "access_claims_user",
        run: access_claims_user,
    },
    Entry {
        hash: "4cc72b37368e6420efad78b8f8fb881d85bff9811b21b3fac776b18e837efe9f",
        source: "azure_frontdoor",
        name: "waf_claims_user",
        run: waf_claims_user,
    },
];
