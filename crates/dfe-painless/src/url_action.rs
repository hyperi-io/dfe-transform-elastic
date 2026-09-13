// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! An action named from the tail of a request URL.
//!
//! ece classifies every admin-console call by what follows `/api/v1/`: a
//! two-level params table keyed by method and then by that tail, a special
//! name for a call proxied through to Elasticsearch, and a computed
//! `<method>_<first segment>` where neither answers.
//!
//! ```painless
//! def temp = params.get(ctx.http.request.method.toLowerCase());
//! String url_parts = ctx.url.original.splitOnToken("/api/v1/")[1];
//! if (url_parts.contains('elasticsearch/elasticsearch/proxy/')){ ... }
//! else if (temp != null){ if (temp.get(url_parts) != null){ ... } }
//! if (ctx.event?.action == null){ ... }
//! ```
//!
//! All three arms are in the corpus: `create_deployment` from the table,
//! `elasticsearch_api_through_ece-_cat` from the proxy arm, and `post_users`,
//! `get_regions` and `put_deployments` from the fallback. Unclaimed it is 21
//! of ece's 36 failing events, and what claimed it before was `FirstElement`
//! reading two hundred characters of this body as a `ctx.` path.

use crate::params::{clean_path, ctx_path_before, is_ctx_path};
use dfe_core::Event;
use serde_json::{Map, Value};

/// The proxied-call arm: the marker that identifies one, and the name its
/// answer is prefixed with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyArm {
    marker: String,
    prefix: String,
}

/// Where the classifier reads its URL and method, and what it writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlTailAction {
    url: String,
    method: String,
    target: String,
    /// The separator whose TAIL is classified.
    after: String,
    proxy: Option<ProxyArm>,
}

/// Read the paths and the two literals, or decline.
pub fn parse_url_tail_action(script: &str) -> Option<UrlTailAction> {
    // The FIRST cut is the one that takes the tail; the proxy arm cuts the
    // tail again three more times.
    let (head, _) = script.split_once(".splitOnToken(")?;
    let after = quoted_at(&script[head.len() + ".splitOnToken(".len()..])?;
    let url = clean_path(&ctx_path_before(script, ".splitOnToken(")?);

    let method = clean_path(&ctx_path_before(script, ".toLowerCase())")?);
    let target = clean_path(&ctx_path_before(script, ".action = ")?) + ".action";

    if !is_ctx_path(&url) || !is_ctx_path(&method) || !is_ctx_path(&target) {
        return None;
    }

    Some(UrlTailAction {
        url,
        method,
        target,
        after,
        proxy: parse_proxy_arm(script),
    })
}

/// The `if (<local>.contains('<marker>')) { ... = "<prefix>" + "-" + ...` arm.
fn parse_proxy_arm(script: &str) -> Option<ProxyArm> {
    let marker = quoted_at(script.split_once(".contains(")?.1)?;
    let arm = script.split_once(&marker)?.1;
    let prefix = quoted_at(arm.split_once(".action = ")?.1)?;
    (!marker.is_empty() && !prefix.is_empty()).then_some(ProxyArm { marker, prefix })
}

/// The literal a quote opens at the head of `text`.
fn quoted_at(text: &str) -> Option<String> {
    let text = text.trim_start();
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let rest = &text[quote.len_utf8()..];
    rest.find(quote).map(|end| rest[..end].to_string())
}

/// Name the action from the URL's tail.
pub fn url_tail_action(
    event: &mut Event,
    pattern: &UrlTailAction,
    params: &Map<String, Value>,
) -> bool {
    let (Some(url), Some(method)) = (
        event.get_string(&pattern.url),
        event.get_string(&pattern.method),
    ) else {
        return true;
    };
    // `splitOnToken(x)[1]` is what lies between the FIRST and SECOND
    // occurrence, which `split_once` would widen to everything after the first.
    let Some(tail) = url.split(pattern.after.as_str()).nth(1) else {
        return true;
    };
    let method = method.to_lowercase();

    if let Some(proxy) = &pattern.proxy
        && let Some(rest) = tail.split(proxy.marker.as_str()).nth(1)
    {
        let cut = rest.split('?').next().unwrap_or(rest);
        let cut = cut.split('/').next().unwrap_or(cut);
        let named = format!("{}-{}", proxy.prefix, cut.to_lowercase());
        let _ = event.set(&pattern.target, Value::String(named));
        return true;
    }

    if let Some(row) = params.get(&method).and_then(|table| table.get(tail)) {
        let _ = event.set(&pattern.target, row.clone());
        return true;
    }

    // The script's own `if (ctx.event?.action == null)`, so an action an
    // earlier processor already wrote survives.
    if !event.has_value(&pattern.target) {
        let head = tail.split('/').next().unwrap_or(tail);
        let _ = event.set(&pattern.target, Value::String(format!("{method}_{head}")));
    }
    true
}

#[cfg(test)]
#[path = "url_action_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
