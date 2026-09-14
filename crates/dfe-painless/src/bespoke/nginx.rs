// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `nginx`'s choice of source address out of the forwarded-for chain,
//! transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The forwarded-for chain the access grok splits out.
const REMOTE_IP_LIST: &str = "nginx.access.remote_ip_list";

/// The access pipeline's untagged address script: `source.address` is the
/// FIRST PUBLIC address in the chain, falling back to the first of any kind.
///
/// Taking the head of the list instead reports the reverse proxy, which is the
/// one address that is never the client.
fn first_public_remote_ip(event: &mut Event, params: &Value) {
    // Painless raises on the assignment where the parent map is absent, and
    // the script's own catch then raises again on the same line.
    if !event.has("source") {
        return;
    }
    let _ = event.set("source.address", Value::Null);
    let Some(chain) = event.get_array(REMOTE_IP_LIST).cloned() else {
        return;
    };
    let delimiters = params.get("dot").and_then(Value::as_str).unwrap_or(".");
    let chosen = chain
        .iter()
        .find(|address| !is_private(delimiters, address))
        .or_else(|| chain.first());
    // An empty chain leaves the null behind, which the remove behind this
    // script then takes out -- Painless gets there by raising on `[0]`.
    if let Some(address) = chosen.cloned() {
        let _ = event.set("source.address", address);
    }
}

/// Whether an address is in one of the four ranges the script calls private.
///
/// Anything the script's own `Integer.parseInt` cannot read -- an IPv6
/// address, a host name, a short chain member -- is NOT private, because the
/// exception it raises is caught and answered `false`.
fn is_private(delimiters: &str, address: &Value) -> bool {
    let Some(text) = address.as_str() else {
        return false;
    };
    let mut tokens = text
        .split(|ch| delimiters.contains(ch))
        .filter(|token| !token.is_empty());
    let (Some(first), Some(second)) = (tokens.next(), tokens.next()) else {
        return false;
    };
    let (Ok(first), Ok(second)) = (first.parse::<i32>(), second.parse::<i32>()) else {
        return false;
    };
    first == 10
        || (first == 192 && second == 168)
        || (first == 172 && (16..=31).contains(&second))
        || first == 127
}

/// Every nginx script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "8077e0c5103759c21e7acb0af6d1acc72cb2b62a6d5a1315d0d56ddf47c21092",
    source: "nginx",
    name: "first_public_remote_ip",
    run: first_public_remote_ip,
}];
