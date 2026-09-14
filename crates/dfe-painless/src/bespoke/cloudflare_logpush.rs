// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `cloudflare_logpush`'s email security alerts: the vendor's attachment
//! records turned into ECS files, and the hostnames of an alert gathered from
//! the three places it names one.
//!
//! Both collect into a Java `HashSet`, so what lands in the document is in
//! bucket order rather than the order the script added the members -- and the
//! hashes are gathered by walking a `HashMap` whose own order decides which
//! digest is added first.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{java_map_values, java_set_order, java_table_size};

/// The vendor's attachment list.
const ATTACHMENTS: &str = "cloudflare_logpush.email_security_alerts.attachments";

/// The hostname list the second script rewrites.
const RELATED_HOSTS: &str = "related.hosts";

/// Each digest the vendor may send, against the ECS name it takes, in the order
/// the script tests them.
const DIGESTS: [(&str, &str); 6] = [
    ("Md5", "md5"),
    ("Sha1", "sha1"),
    ("Sha256", "sha256"),
    ("Sha384", "sha384"),
    ("Sha512", "sha512"),
    ("Ssdeep", "ssdeep"),
];

/// `script_218079c2` in `cloudflare_logpush/email_security_alerts`: `file`,
/// `email.attachments` and `related.hash`, out of one walk of the attachments.
fn attachments_to_ecs(event: &mut Event, _params: &Value) {
    let Some(attachments) = event.get_array(ATTACHMENTS).cloned() else {
        return;
    };

    let mut files = Vec::with_capacity(attachments.len());
    let mut email_attachments = Vec::with_capacity(attachments.len());
    let mut hashes: Vec<Value> = Vec::new();

    for attachment in &attachments {
        let mut file = Map::new();
        if let Some(name) = member(attachment, "Name") {
            file.insert("name".to_owned(), name);
        }
        if let Some(mime) = member(attachment, "ContentTypeComputed") {
            file.insert("mime_type".to_owned(), mime);
        }

        let mut digests = Map::new();
        for (sent, ecs) in DIGESTS {
            if let Some(digest) = member(attachment, sent) {
                digests.insert(ecs.to_owned(), digest);
            }
        }
        if !digests.is_empty() {
            file.insert("hash".to_owned(), Value::Object(digests.clone()));
        }

        files.push(Value::Object(file.clone()));
        let mut wrapper = Map::new();
        wrapper.insert("file".to_owned(), Value::Object(file));
        email_attachments.push(Value::Object(wrapper));

        for digest in java_map_values(&digests, java_table_size(digests.len())) {
            if !hashes.contains(&digest) {
                hashes.push(digest);
            }
        }
    }

    let _ = event.set("file", Value::Array(files));
    if !email_attachments.is_empty() {
        if !event.has_value("email") {
            let _ = event.set("email", Value::Object(Map::new()));
        }
        let _ = event.set("email.attachments", Value::Array(email_attachments));
    }
    if !event.has_value("related") {
        let _ = event.set("related", Value::Object(Map::new()));
    }
    let _ = event.set("related.hash", Value::Array(java_set_order(hashes)));
}

/// `script_45dae815` in `cloudflare_logpush/email_security_alerts`: the
/// hostnames already collected, the server's own domain, and the domain half of
/// every address in `related.user`.
fn collect_related_hosts(event: &mut Event, _params: &Value) {
    // The script dereferences `ctx.related` unguarded on its last line, so a
    // document without it raises there and writes nothing.
    if !event.has_value("related") {
        return;
    }

    let mut domains: Vec<Value> = Vec::new();
    if let Some(hosts) = event.get_array(RELATED_HOSTS) {
        for host in hosts {
            if !domains.contains(host) {
                domains.push(host.clone());
            }
        }
    }
    if let Some(domain) = event.get("server.domain")
        && !domain.is_null()
        && !domains.contains(domain)
    {
        domains.push(domain.clone());
    }
    if let Some(users) = event.get_array("related.user") {
        for user in users {
            // A member the script cannot call `length()` on raises, so the rest
            // of the walk never happens.
            let Some(address) = user.as_str() else {
                return;
            };
            if address.encode_utf16().count() < 3 {
                continue;
            }
            let parts: Vec<&str> = address.split('@').collect();
            if parts.len() != 2 {
                continue;
            }
            let domain = Value::from(parts[1]);
            if !domains.contains(&domain) {
                domains.push(domain);
            }
        }
    }

    let ordered = Value::Array(java_set_order(domains));
    if event.has(RELATED_HOSTS) {
        let _ = event.update(RELATED_HOSTS, ordered);
    } else {
        let _ = event.set(RELATED_HOSTS, ordered);
    }
}

/// One member of an attachment record, absent where the vendor sent null.
fn member(attachment: &Value, key: &str) -> Option<Value> {
    let held = attachment.get(key)?;
    if held.is_null() {
        return None;
    }
    Some(held.clone())
}

/// Every `cloudflare_logpush` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "6b50189aab95c0da5a782726b220c9bd3be2b4ba431920c3222823745c0d07c6",
        source: "cloudflare_logpush",
        name: "attachments_to_ecs",
        run: attachments_to_ecs,
    },
    Entry {
        hash: "5ab0f0b894000579816b01e70acd9c772e0685ec2331ec264d8b2338c10f6bcb",
        source: "cloudflare_logpush",
        name: "collect_related_hosts",
        run: collect_related_hosts,
    },
];
