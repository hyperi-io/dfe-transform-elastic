// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `sublime_security`'s audit categorisation, and the scripts that lift a
//! message's links, attachments and header hops onto ECS.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// `sublime_security/audit`, processor `painless_ecs_categorization`:
/// `event.category` and `event.type`.
///
/// The audit type is `<resource>.<verb>`, so the resource before the first dot
/// picks the category and the verb inside the whole string picks the lifecycle.
fn ecs_categorization(event: &mut Event, params: &Value) {
    let Some(audit_type) = event.get_string("sublime_security.audit.type") else {
        return;
    };
    let resource = audit_type
        .split_once('.')
        .map_or(audit_type.as_str(), |(head, _)| head);
    let listed = |name: &str, wanted: &str| {
        params
            .get(name)
            .and_then(Value::as_array)
            .is_some_and(|list| list.iter().any(|item| item.as_str() == Some(wanted)))
    };
    let lifecycle = || {
        if ["create", "connect", "upload"]
            .iter()
            .any(|verb| audit_type.contains(verb))
        {
            "creation"
        } else if audit_type.contains("delete") {
            "deletion"
        } else {
            "change"
        }
    };

    let mut categories = Vec::new();
    let mut types = Vec::new();
    if listed("email_resources", resource) {
        categories.push(Value::from("email"));
        types.push(Value::from("info"));
    } else if resource == "auth" {
        categories.push(Value::from("authentication"));
        let kind = if audit_type.starts_with("auth.login") {
            "start"
        } else if audit_type == "auth.logout" {
            "end"
        } else {
            "info"
        };
        types.push(Value::from(kind));
    } else if listed("iam_resources", resource) {
        categories.push(Value::from("iam"));
        if resource == "user" {
            types.push(Value::from("user"));
        }
        if resource == "rbac" {
            types.push(Value::from("group"));
        }
        types.push(Value::from(lifecycle()));
    } else if listed("configuration_resources", resource)
        || listed("configuration_types", &audit_type)
    {
        categories.push(Value::from("configuration"));
        let kind = if audit_type == "inline_signed_headers_retrieved" {
            "info"
        } else {
            lifecycle()
        };
        types.push(Value::from(kind));
    }

    if categories.is_empty() {
        return;
    }
    let _ = event.set("event.category", Value::Array(categories));
    let _ = event.set("event.type", Value::Array(types));
}

/// `sublime_security/email_message`, processor `script_to_set_url_field`: the
/// whole `url` field, as one entry per link the body carries an href for.
///
/// A link the vendor recorded only a DISPLAY url for is skipped, which is what
/// keeps a rewritten tracking link out of `url`.
fn set_url(event: &mut Event, _params: &Value) {
    /// Each ECS name and the path under `href_url` it is read from.
    const MEMBERS: [(&str, &str); 11] = [
        ("domain", "domain.domain"),
        ("subdomain", "domain.subdomain"),
        ("top_level_domain", "domain.tld"),
        ("fragment", "fragment"),
        ("password", "password"),
        ("path", "path"),
        ("port", "port"),
        ("query", "query_params"),
        ("scheme", "scheme"),
        ("full", "url"),
        ("username", "username"),
    ];

    let Some(links) = event.get_array("json.body.links") else {
        return;
    };
    let urls: Vec<Value> = links
        .iter()
        .filter_map(|link| {
            let href = link.get("href_url").filter(|value| !value.is_null())?;
            let mut out = Map::new();
            for (name, path) in MEMBERS {
                out.insert(name.to_owned(), at(href, path));
            }
            Some(Value::Object(out))
        })
        .collect();
    let _ = event.set("url", Value::Array(urls));
}

/// `sublime_security/email_message`, processor
/// `script_to_set_email_attachments_field`: `email.attachments`.
///
/// The script replaces `email` wholesale with a fresh map, which is harmless
/// because every other `email.*` field is written after it.
fn set_email_attachments(event: &mut Event, _params: &Value) {
    let Some(attachments) = event.get_array("sublime_security.email_message.attachments") else {
        return;
    };
    let out: Vec<Value> = attachments
        .iter()
        .map(|attachment| {
            let mut hash = Map::new();
            for name in ["md5", "sha1", "sha256"] {
                hash.insert(name.to_owned(), at(attachment, name));
            }
            let mut file = Map::new();
            file.insert("hash".to_owned(), Value::Object(hash));
            file.insert("mime_type".to_owned(), at(attachment, "content.type"));
            file.insert("extension".to_owned(), at(attachment, "file.extension"));
            file.insert("name".to_owned(), at(attachment, "file.name"));
            file.insert("size".to_owned(), at(attachment, "size"));
            let mut entry = Map::new();
            entry.insert("file".to_owned(), Value::Object(file));
            Value::Object(entry)
        })
        .collect();
    let _ = event.set("email", Value::Object(Map::new()));
    let _ = event.set("email.attachments", Value::Array(out));
}

/// `sublime_security/email_message`, the untagged extension processor: the
/// leading dot taken off every `email.attachments` file extension.
fn strip_attachment_extension(event: &mut Event, _params: &Value) {
    let Some(mut attachments) = event.take_array("email.attachments") else {
        return;
    };
    for attachment in &mut attachments {
        let Some(file) = attachment.get_mut("file") else {
            continue;
        };
        let trimmed = file
            .get("extension")
            .and_then(Value::as_str)
            .and_then(|extension| extension.strip_prefix('.'))
            .map(str::to_owned);
        if let Some(trimmed) = trimmed
            && let Some(slot) = file.get_mut("extension")
        {
            *slot = Value::from(trimmed);
        }
    }
    event.update("email.attachments", Value::Array(attachments));
}

/// `sublime_security/email_message`, the untagged hops processor: each header
/// field copied to a member named after the header itself.
///
/// A hop's fields are a LIST of `{name, value}` pairs, and the copy is what
/// lets a mapping address one header by name rather than by position.
fn name_hop_fields(event: &mut Event, _params: &Value) {
    let Some(mut hops) = event.take_array("json.headers.hops") else {
        return;
    };
    for hop in &mut hops {
        let Some(fields) = hop.get_mut("fields").and_then(Value::as_array_mut) else {
            continue;
        };
        for field in fields {
            let Some(members) = field.as_object_mut() else {
                continue;
            };
            let Some(name) = members
                .get("name")
                .and_then(Value::as_str)
                .map(str::to_lowercase)
            else {
                continue;
            };
            let value = members.get("value").cloned().unwrap_or(Value::Null);
            members.insert(name, value);
        }
    }
    event.update("json.headers.hops", Value::Array(hops));
}

/// One dotted path inside a value, read as the null Painless reads an absent
/// member as.
fn at(value: &Value, path: &str) -> Value {
    let mut current = value;
    for segment in path.split('.') {
        match current.get(segment) {
            Some(next) => current = next,
            None => return Value::Null,
        }
    }
    current.clone()
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "97b190dba1e498e83bfa24ce6702e83ff90c93afb2021e8cfca24234136b4258",
        source: "sublime_security",
        name: "ecs_categorization",
        run: ecs_categorization,
    },
    Entry {
        hash: "0e077699fc5fe7da655be519e41c8fb49a60ceed30fdec106af35fa3601bf7f0",
        source: "sublime_security",
        name: "set_url",
        run: set_url,
    },
    Entry {
        hash: "a5258d66d76ce6a9fbf3bd258483892d1fe36bb8a77f51647907b3f188acdc5a",
        source: "sublime_security",
        name: "set_email_attachments",
        run: set_email_attachments,
    },
    Entry {
        hash: "caeba5bfcd1056c009ee0e0963eadafa59dbb832b51254b3500bb20980ec8db7",
        source: "sublime_security",
        name: "strip_attachment_extension",
        run: strip_attachment_extension,
    },
    Entry {
        hash: "1d43c059430ba0d25a028de8b8d20383a5918090f673f6997cdf5e517b4e84d4",
        source: "sublime_security",
        name: "name_hop_fields",
        run: name_hop_fields,
    },
];
