// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `zscaler_zpa`'s four vendor scripts: the browser-access URL, the audit
//! event classification, the per-object-type ECS mapping of an audit record's
//! changed values, and the user-activity transport lookup.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// `(no tag)` in `zscaler_zpa/browser_access`: `url.full`, which the
/// `uri_parts` after it takes apart.
///
/// All three parts are lowercased, so the path and query come out lowercase
/// too.
fn build_browser_access_url(event: &mut Event, _params: &Value) {
    let _ = event.set("url", Value::Object(Map::new()));
    let protocol = lowered(event, "json.Protocol");
    let domain = lowered(event, "json.Host");
    let endpoint = lowered(event, "json.URL");
    let (Some(protocol), Some(domain), Some(endpoint)) = (protocol, domain, endpoint) else {
        return;
    };
    let _ = event.set(
        "url.full",
        Value::from(format!("{protocol}://{domain}{endpoint}")),
    );
}

/// `(no tag)` in `zscaler_zpa/audit`: `event.type`, `event.category` and
/// `event.outcome` from the pipeline's own operation-type table.
fn classify_audit_operation(event: &mut Event, params: &Value) {
    let Some(operation) = lowered(event, "json.AuditOperationType") else {
        return;
    };
    let Some(class) = params
        .get("event_classification")
        .and_then(|table| table.get(&operation))
    else {
        return;
    };
    for (target, member) in [
        ("event.type", "type"),
        ("event.category", "category"),
        ("event.outcome", "outcome"),
    ] {
        let _ = event.set(target, class.get(member).cloned().unwrap_or(Value::Null));
    }
}

/// `(no tag)` in `zscaler_zpa/audit`: the changed values mapped onto ECS, one
/// arm per object type.
///
/// Which side of the change is read depends on the operation: a removal reads
/// the OLD values, everything else the new ones.
fn map_audit_values(event: &mut Event, _params: &Value) {
    let object_type = lowered(event, "json.ObjectType");
    let operation = lowered(event, "json.AuditOperationType");
    let values = match operation.as_deref() {
        Some("delete" | "sign out") => event.get("json.AuditOldValue").cloned(),
        Some("create" | "sign in" | "update") => event.get("json.AuditNewValue").cloned(),
        _ => None,
    }
    .unwrap_or(Value::Null);

    match object_type.as_deref() {
        Some("administrator") => {
            let _ = event.set("user.target", Value::Object(Map::new()));
            let roles: Vec<Value> = member(&values, "roles")
                .as_array()
                .map(|listed| listed.iter().map(|role| member(role, "name")).collect())
                .unwrap_or_default();
            let _ = event.set("user.target.roles", Value::Array(roles));
            let _ = event.set("user.target.email", member(&values, "email"));
        }
        Some("app connector group") => {
            let _ = event.set("group", Value::Object(Map::new()));
            let _ = event.set("group.id", member(&values, "id"));
            let _ = event.set("group.name", member(&values, "name"));
            let _ = event.set("observer", Value::Object(Map::new()));
            let _ = event.set("observer.geo.location.lat", member(&values, "latitude"));
            let _ = event.set("observer.geo.location.lon", member(&values, "longitude"));
            let _ = event.set("observer.geo.city_name", member(&values, "cityCountry"));
            let _ = event.set("observer.geo.country_name", member(&values, "location"));
        }
        Some("browser access") => {
            let _ = event.set("network", Value::Object(Map::new()));
            let protocol = match member(&values, "applicationProtocol") {
                Value::String(text) => Value::from(text.to_lowercase()),
                _ => Value::Null,
            };
            let _ = event.set("network.protocol", protocol);
        }
        Some("authentication") => {
            let _ = event.set("client", Value::Object(Map::new()));
            let _ = event.set("client.ip", member(&values, "remoteIP"));
        }
        Some("certificate") => {
            let _ = event.set("x509", Value::Object(Map::new()));
            let _ = event.set(
                "x509.alternative_names",
                member(&values, "subjectAlternateNames"),
            );
            let _ = event.set("x509.issuer.common_name", member(&values, "commonName"));
            let _ = event.set(
                "x509.issuer.distinguished_name",
                member(&values, "issuedTo"),
            );
        }
        Some("executive insights user") => {
            let _ = event.set("user", Value::Object(Map::new()));
            let _ = event.set("user.target.id", member(&values, "id"));
            let _ = event.set("user.target.email", member(&values, "email"));
            let _ = event.set("user.target.name", member(&values, "name"));
        }
        Some("idp certificate") => {
            let _ = event.set("x509", Value::Object(Map::new()));
            if let Some(seconds) = epoch_seconds(&values, "creationTimeInSeconds") {
                let _ = event.set("x509.not_before", Value::from(seconds));
            }
            if let Some(seconds) = epoch_seconds(&values, "expirationTimeInSeconds") {
                let _ = event.set("x509.not_after", Value::from(seconds));
            }
            let _ = event.set("x509.issuer.common_name", member(&values, "commonName"));
        }
        Some("server") => {
            let _ = event.set("server", Value::Object(Map::new()));
            let _ = event.set("server.address", member(&values, "domainOrIpAddress"));
        }
        _ => {}
    }
}

/// `(no tag)` in `zscaler_zpa/user_activity`: `network.transport` from the
/// pipeline's IANA protocol-number table.
fn map_transport(event: &mut Event, params: &Value) {
    if !event.has_value("network") {
        let _ = event.set("network", Value::Object(Map::new()));
    }
    let Some(number) = event.get("json.IPProtocol").map(painless_to_string) else {
        return;
    };
    let named = params
        .get("iana_numbers")
        .and_then(|table| table.get(&number))
        .cloned()
        .unwrap_or(Value::Null);
    let _ = event.set("network.transport", named);
}

/// A string field lowercased, or nothing where the field is absent or is not a
/// string.
fn lowered(event: &Event, path: &str) -> Option<String> {
    event.get_str(path).map(str::to_lowercase)
}

/// One member of the changed-values map, null where the vendor sent none.
fn member(values: &Value, key: &str) -> Value {
    values.get(key).cloned().unwrap_or(Value::Null)
}

/// A seconds member as `Long.parseLong` reads it, or nothing where the member
/// is absent.
fn epoch_seconds(values: &Value, key: &str) -> Option<i64> {
    match values.get(key) {
        Some(Value::Number(number)) => number.as_i64(),
        Some(Value::String(text)) => text.trim().parse::<i64>().ok(),
        _ => None,
    }
}

/// Every `zscaler_zpa` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "0a324fa4ada08d867c433aa7a9e578ace10c1cbc0c36a4c684f57ede9fc5b572",
        source: "zscaler_zpa",
        name: "build_browser_access_url",
        run: build_browser_access_url,
    },
    Entry {
        hash: "e477ecad55fce1cf23ba29d6bf27c493bc5b9fb00c2b306c3cc25abd4f856eff",
        source: "zscaler_zpa",
        name: "classify_audit_operation",
        run: classify_audit_operation,
    },
    Entry {
        hash: "338a53f456db7e25b767107dffec5b0f10241be078e9c8a42c293a1fdafb4815",
        source: "zscaler_zpa",
        name: "map_audit_values",
        run: map_audit_values,
    },
    Entry {
        hash: "b03c09d6946dee3702e00e7fd618365c12bccef7c929f90fcd86ddd68e2cf675",
        source: "zscaler_zpa",
        name: "map_transport",
        run: map_transport,
    },
];
