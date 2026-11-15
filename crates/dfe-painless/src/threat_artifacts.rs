// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A list of threat artifacts classified into the ECS indicator lists.
//!
//! `proofpoint_essentials` walks `threats_info_map` and sorts each artifact by
//! what it LOOKS like rather than by any field the vendor sets: a 64-character
//! lowercase-hex string is a file hash, a string shaped like an address is an
//! impostor sender, and a `threat_type` of `URL` is a malicious link. Every
//! artifact also joins `threat.indicator.name` whatever it turned out to be.
//!
//! Each branch is a native test rather than the vendor's regex. `^[0-9a-f]{64}$`
//! over a string already known to be 64 characters is an is-hex scan, and the
//! address pattern is three character classes around one `@` -- both cheaper
//! than a compiled regex, and the regex crate has no business on this path.
//!
//! Every list is written only where the walk put something in it. The script
//! seeds them all with `?: []`, but an empty list is a field Elasticsearch
//! prunes, so seeding one here would be an extra on every event with no
//! artifacts of that kind.

use serde_json::Value;

use crate::params::clean_path;
use dfe_core::Event;

/// Where each classification lands.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ThreatArtifacts {
    /// The list of artifacts walked.
    source: String,
    /// The member carrying the artifact's value.
    value_member: String,
    /// The member naming the artifact's kind, and the literal meaning a URL.
    kind_member: String,
    url_literal: String,
    /// Every artifact's value joins this list.
    name_target: String,
    /// One tag per classified artifact.
    type_target: String,
    email_target: String,
    url_target: String,
    related_hash: String,
    related_user: String,
}

/// Every `ctx.<path>.add(<argument>)` in `text`, in order.
///
/// The target is read back to the statement's own start, so the `ctx.` path is
/// this call's and not one from the line above it.
fn appends(text: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for (at, marker) in text.match_indices(".add(") {
        let Some(argument) = text[at + marker.len()..].split(')').next() else {
            continue;
        };
        let statement = text[..at]
            .rsplit(['\n', ';', '{', '}'])
            .next()
            .unwrap_or_default()
            .trim();
        let Some(path) = statement.strip_prefix("ctx.") else {
            continue;
        };
        found.push((clean_path(path), argument.trim().to_string()));
    }
    found
}

/// The target of the first append of `wanted` in `text`.
fn target_adding(text: &str, wanted: &str) -> Option<String> {
    appends(text)
        .into_iter()
        .find(|(_, argument)| argument == wanted)
        .map(|(path, _)| path)
}

/// Read the whole classifier, or `None` where any branch is spelled otherwise.
///
/// Every branch must be present: a script missing one is a different script,
/// and writing three of its four lists makes the source read as needing polish
/// rather than a matcher.
#[must_use]
pub fn parse_threat_artifacts(script: &str) -> Option<ThreatArtifacts> {
    // The walk, and the member each artifact's value sits on.
    let (_, rest) = script.split_once("for (")?;
    let (head, body) = rest.split_once(')')?;
    let (var, source) = head.split_once(" in ")?;
    let var = var.trim().rsplit(' ').next()?.trim();
    let source = clean_path(source.trim().strip_prefix("ctx.")?);

    // `if (<var>.<member> != null)` opens the loop and names the value member.
    let value_member = body
        .split_once(&format!("if ({var}."))?
        .1
        .split_once(" != null")?
        .0
        .trim()
        .to_string();
    let value = format!("{var}.{value_member}");

    // The hash branch is the only one that lowercases before testing, and the
    // local it binds is what the branch appends.
    let hash_local = body
        .split_once(&format!("= {value}.toLowerCase()"))?
        .0
        .trim_end()
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()?
        .to_string();
    if hash_local.is_empty() || !body.contains(".length() == 64") {
        return None;
    }

    // The kind guard is the one testing a member against a LITERAL; the two
    // before it test for null and for a length.
    let (kind_member, url_literal) = kind_test(body, var)?;

    // The branches run in a fixed order, so an append is attributed to the one
    // it sits between rather than to its argument, which two of them share.
    let hash_add = body.find(&format!(".add({hash_local})"))?;
    let url_guard = body.find(&format!("== '{url_literal}'"))?;
    let email_branch = body.get(hash_add..url_guard)?;
    let url_branch = body.get(url_guard..)?;

    Some(ThreatArtifacts {
        // `str` is the hash branch's own local, so it is appended once.
        related_hash: target_adding(body, &hash_local)?,
        type_target: target_adding(body, "'email-addr'")?,
        email_target: target_adding(email_branch, &value)?,
        related_user: last_target_adding(email_branch, &value)?,
        url_target: target_adding(url_branch, &value)?,
        // Outside every branch, and therefore the last append of them all.
        name_target: last_target_adding(body, &value)?,
        source,
        value_member,
        kind_member,
        url_literal,
    })
}

/// The `if (<var>.<member> != null && <var>.<member> == '<literal>')` guard,
/// as the member it reads and the literal it wants.
fn kind_test(body: &str, var: &str) -> Option<(String, String)> {
    let opener = format!("if ({var}.");
    for (at, marker) in body.match_indices(&opener) {
        let Some(condition) = body[at + marker.len()..].split(')').next() else {
            continue;
        };
        let Some((member, literal)) = condition.split_once("== ") else {
            continue;
        };
        let Some(literal) = quoted(literal) else {
            continue;
        };
        // The member is whatever the LAST `<var>.` in the condition names, so
        // the null half of the conjunction is stepped over.
        let member = member
            .rsplit(&format!("{var}."))
            .next()?
            .trim()
            .trim_end_matches(['&', ' '])
            .trim();
        if !member.is_empty() {
            return Some((member.to_string(), literal));
        }
    }
    None
}

/// The quoted literal a fragment opens with.
fn quoted(text: &str) -> Option<String> {
    let text = text.trim_start();
    let quote = text.chars().next().filter(|c| matches!(c, '\'' | '"'))?;
    let inner = &text[quote.len_utf8()..];
    let end = inner.find(quote)?;
    Some(inner[..end].to_string())
}

/// The target of the LAST append of `wanted` in `text`.
fn last_target_adding(text: &str, wanted: &str) -> Option<String> {
    appends(text)
        .into_iter()
        .rfind(|(_, argument)| argument == wanted)
        .map(|(path, _)| path)
}

/// Is this the lowercase-hex digest the vendor's `^[0-9a-f]{64}$` accepts?
fn is_sha256(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Does this look like the address the vendor's pattern accepts --
/// `^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$`?
fn is_email(text: &str) -> bool {
    let Some((local, domain)) = text.split_once('@') else {
        return false;
    };
    if local.is_empty()
        || !local
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._%+-".contains(&b))
    {
        return false;
    }
    // One `@` only, and the domain needs a dot with a two-letter-or-longer
    // alphabetic tail after it.
    if domain.contains('@')
        || !domain
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
    {
        return false;
    }
    let Some((host, tld)) = domain.rsplit_once('.') else {
        return false;
    };
    !host.is_empty() && tld.len() >= 2 && tld.bytes().all(|b| b.is_ascii_alphabetic())
}

/// The list at `path`, as the strings already in it.
fn existing(event: &Event, path: &str) -> Vec<String> {
    match event.get(path) {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|item| item.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}

/// Append `added` to the list at `path`, writing nothing where it is empty.
fn extend_list(event: &mut Event, path: &str, added: Vec<String>) {
    if added.is_empty() {
        return;
    }
    let mut list = match event.get(path) {
        Some(Value::Array(items)) => items.clone(),
        _ => Vec::new(),
    };
    list.extend(added.into_iter().map(Value::from));
    let _ = event.set(path, Value::Array(list));
}

/// Classify every artifact into the indicator lists.
pub fn run_threat_artifacts(event: &mut Event, pattern: &ThreatArtifacts) -> bool {
    let Some(Value::Array(artifacts)) = event.get(&pattern.source).cloned() else {
        return true;
    };

    // Read once: the script tests membership against the list AS IT GROWS, so
    // each branch checks what it has already added too.
    let mut hashes = existing(event, &pattern.related_hash);
    let mut users = existing(event, &pattern.related_user);
    let (mut new_hashes, mut new_users) = (Vec::new(), Vec::new());
    let (mut names, mut types) = (Vec::new(), Vec::new());
    let (mut addresses, mut urls) = (Vec::new(), Vec::new());

    for artifact in &artifacts {
        let Some(value) = artifact
            .get(&pattern.value_member)
            .and_then(Value::as_str)
            .map(str::to_string)
        else {
            continue;
        };

        if value.len() == 64 {
            let lowered = value.to_lowercase();
            if is_sha256(&lowered) && !hashes.contains(&lowered) {
                hashes.push(lowered.clone());
                new_hashes.push(lowered);
            }
        }

        if is_email(&value) && !users.contains(&value) {
            addresses.push(value.clone());
            types.push("email-addr".to_string());
            users.push(value.clone());
            new_users.push(value.clone());
        }

        if artifact.get(&pattern.kind_member).and_then(Value::as_str)
            == Some(pattern.url_literal.as_str())
        {
            urls.push(value.clone());
            types.push(pattern.url_literal.to_lowercase());
        }

        names.push(value);
    }

    extend_list(event, &pattern.related_hash, new_hashes);
    extend_list(event, &pattern.related_user, new_users);
    extend_list(event, &pattern.email_target, addresses);
    extend_list(event, &pattern.url_target, urls);
    extend_list(event, &pattern.type_target, types);
    extend_list(event, &pattern.name_target, names);
    true
}

// The script constant is quoted verbatim from a generated call site, which
// spells it `r#"..."#`. Keeping it character-identical is what lets a script be
// copied straight from a module into a test.
#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::expect_used, clippy::needless_raw_string_hashes)]
#[path = "threat_artifacts_tests.rs"]
mod tests;
