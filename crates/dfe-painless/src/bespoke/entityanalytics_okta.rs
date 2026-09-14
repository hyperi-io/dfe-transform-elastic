// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `entityanalytics_okta`'s entity scripts: the account-status flags, the
//! permissions gathered off the user's custom roles, the MFA flag, and the two
//! relationship blocks built from the supervised-user and device lists.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::java_set_order;

/// The flags the status script always writes, in the order it puts them.
const STATUS_FLAGS: [&str; 5] = [
    "recovery",
    "locked_out",
    "suspended",
    "password_expired",
    "deprovisioned",
];

/// `painless_set_user_account_status` in `entityanalytics_okta/entity`: the
/// five `user.account.status` flags, all false but the one the vendor's own
/// status names.
fn set_user_account_status(event: &mut Event, _params: &Value) {
    if !event.has_value("user.account.status") {
        let _ = event.set("user.account.status", Value::Object(Map::new()));
    }
    for flag in STATUS_FLAGS {
        let _ = event.set(&format!("user.account.status.{flag}"), Value::Bool(false));
    }
    let Some(status) = event.get_str("okta.status").map(str::to_lowercase) else {
        return;
    };
    if STATUS_FLAGS.contains(&status.as_str()) {
        let _ = event.set(&format!("user.account.status.{status}"), Value::Bool(true));
    }
}

/// `painless_extract_role_permissions` in `entityanalytics_okta/entity`: every
/// permission label the user's custom roles carry, as
/// `user.entity.attributes.permissions`.
///
/// The script collects into a `HashSet`, so the list it leaves is in Java's
/// bucket order rather than the order the roles spell them.
fn extract_role_permissions(event: &mut Event, _params: &Value) {
    let Some(roles) = event.get_array("entityanalytics_okta.roles").cloned() else {
        return;
    };
    let mut labels: Vec<Value> = Vec::new();
    for role in &roles {
        let Some(permissions) = role.get("permissions").and_then(Value::as_array) else {
            continue;
        };
        for permission in permissions {
            let label = permission.get("label").cloned().unwrap_or(Value::Null);
            if label.is_null() || label.as_str() == Some("") {
                continue;
            }
            if !labels.contains(&label) {
                labels.push(label);
            }
        }
    }
    if labels.is_empty() {
        return;
    }
    let _ = event.set(
        "user.entity.attributes.permissions",
        Value::Array(java_set_order(labels)),
    );
}

/// `painless_set_mfa_enabled` in `entityanalytics_okta/entity`:
/// `user.entity.attributes.mfa_enabled`, set once the user has any active
/// factor.
fn set_mfa_enabled(event: &mut Event, _params: &Value) {
    let Some(factors) = event.get_array("entityanalytics_okta.user.factors") else {
        return;
    };
    let active = factors
        .iter()
        .any(|factor| factor.get("status").and_then(Value::as_str) == Some("ACTIVE"));
    if active {
        let _ = event.set("user.entity.attributes.mfa_enabled", Value::Bool(true));
    }
}

/// `painless_set_user_entity_relationships_owns` in
/// `entityanalytics_okta/entity`: the enrolled devices' ids and display names
/// as `user.entity.relationships.owns.host`.
fn set_relationships_owns(event: &mut Event, _params: &Value) {
    let Some(devices) = event
        .get_array("entityanalytics_okta.user.devices")
        .cloned()
    else {
        return;
    };
    let mut ids = Vec::new();
    let mut names = Vec::new();
    for device in &devices {
        if device.is_null() {
            continue;
        }
        if let Some(id) = present(device.get("id")) {
            ids.push(id);
        }
        if let Some(name) = present(
            device
                .get("profile")
                .and_then(|profile| profile.get("displayName")),
        ) {
            names.push(name);
        }
    }
    let mut host = Map::new();
    if !ids.is_empty() {
        host.insert("id".to_owned(), Value::Array(ids));
    }
    if !names.is_empty() {
        host.insert("name".to_owned(), Value::Array(names));
    }
    if host.is_empty() {
        return;
    }
    let mut owns = Map::new();
    owns.insert("host".to_owned(), Value::Object(host));
    let _ = event.set("user.entity.relationships.owns", Value::Object(owns));
}

/// `painless_set_user_entity_relationships_supervises` in
/// `entityanalytics_okta/entity`: the supervised users rewritten onto ECS key
/// names, and the same three columns gathered as
/// `user.entity.relationships.supervises`.
///
/// The rewritten list keeps `user.id`, `user.name` and `user.email` as FLAT
/// keys holding dots, which is what the vendor pipeline leaves behind.
fn set_relationships_supervises(event: &mut Event, _params: &Value) {
    let Some(supervised) = event
        .get_array("entityanalytics_okta.user.supervises")
        .cloned()
    else {
        return;
    };

    let mut ids = Vec::new();
    let mut names = Vec::new();
    let mut emails = Vec::new();
    let mut rewritten = Vec::with_capacity(supervised.len());
    for entry in &supervised {
        if entry.is_null() {
            continue;
        }
        let mut raw = Map::new();
        if let Some(id) = present(entry.get("user_id")) {
            ids.push(id.clone());
            raw.insert("user.id".to_owned(), id);
        }
        if let Some(name) = present(entry.get("username")) {
            names.push(name.clone());
            raw.insert("user.name".to_owned(), name);
        }
        if let Some(email) = present(entry.get("email")) {
            emails.push(email.clone());
            raw.insert("user.email".to_owned(), email);
        }
        rewritten.push(Value::Object(raw));
    }
    let _ = event.update(
        "entityanalytics_okta.user.supervises",
        Value::Array(rewritten),
    );

    let mut columns = Map::new();
    if !ids.is_empty() {
        columns.insert("id".to_owned(), Value::Array(ids));
    }
    if !names.is_empty() {
        columns.insert("name".to_owned(), Value::Array(names));
    }
    if !emails.is_empty() {
        columns.insert("email".to_owned(), Value::Array(emails));
    }
    if columns.is_empty() {
        return;
    }
    let mut block = Map::new();
    block.insert("user".to_owned(), Value::Object(columns));
    let _ = event.set("user.entity.relationships.supervises", Value::Object(block));
}

/// A member the script's `!= null` check keeps.
fn present(value: Option<&Value>) -> Option<Value> {
    match value {
        None | Some(Value::Null) => None,
        Some(held) => Some(held.clone()),
    }
}

/// Every `entityanalytics_okta` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "54265bdd0b03b7d3039a2a8dd1ea510e09a8c730379998cab11425f4cf212d68",
        source: "entityanalytics_okta",
        name: "set_user_account_status",
        run: set_user_account_status,
    },
    Entry {
        hash: "03cffbfa8676cd72f69d2e5023ef6361b99fdc63b70a2e7bd2422b7e197581c1",
        source: "entityanalytics_okta",
        name: "extract_role_permissions",
        run: extract_role_permissions,
    },
    Entry {
        hash: "de6c4eed97324600ded83cb3a9567784cfd2f2475d6f448157f0e5b110899158",
        source: "entityanalytics_okta",
        name: "set_mfa_enabled",
        run: set_mfa_enabled,
    },
    Entry {
        hash: "c50a798107a859e3bc56491471ece6c340b3ffa1d460e1d3ab86d64d8ec434b5",
        source: "entityanalytics_okta",
        name: "set_relationships_owns",
        run: set_relationships_owns,
    },
    Entry {
        hash: "7f310b95c5d56f0bf6a39cd58d0ba7395666cc785824dd5a199a080e2b544037",
        source: "entityanalytics_okta",
        name: "set_relationships_supervises",
        run: set_relationships_supervises,
    },
];
