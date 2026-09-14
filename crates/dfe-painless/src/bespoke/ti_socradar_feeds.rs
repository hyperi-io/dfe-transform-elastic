// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_socradar_feeds`'s indicator scripts, transcribed.
//!
//! The feed ships one indicator per line with its kind in a free-text
//! `feed_type`, so where the value lands -- an IP, a domain, a file hash, a
//! URL, a mailbox -- is decided at run time rather than by the mapping. The
//! rest is the confidence the vendor score maps to, and the expiry the
//! integration's own configuration sets.

use chrono::{DateTime, Days, FixedOffset, SecondsFormat, TimeDelta};
use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_f64;

/// The subtree the feed's own fields live under.
const FEED: &str = "ti_socradar_feeds.feed";

/// `ti_socradar_feeds/feed`, `script_resolve_indicator_type`: the indicator
/// value written under whichever `threat.indicator` field its kind calls for,
/// and mirrored into the matching `related` list.
fn resolve_indicator_type(event: &mut Event, _params: &Value) {
    let Some(feed_type) = event.get_string(&format!("{FEED}.type")) else {
        return;
    };
    let Some(value) = event.get(&format!("{FEED}.value")).cloned() else {
        return;
    };
    let feed_type = feed_type.to_lowercase();

    // `ctx.threat` and `ctx.threat.indicator` are built before the branch, so
    // an unrecognised kind still leaves the map behind.
    ensure_map(event, "threat");
    ensure_map(event, "threat.indicator");

    match feed_type.as_str() {
        "ipv6" | "ip" | "ipv4-addr" | "ipv4" => {
            let kind = if feed_type == "ipv6" {
                "ipv6-addr"
            } else {
                "ipv4-addr"
            };
            let _ = event.set("threat.indicator.type", kind);
            let _ = event.set("threat.indicator.ip", value.clone());
            relate(event, "related.ip", &value);
        }
        "hash" | "file" => {
            let _ = event.set("threat.indicator.type", "file");
            ensure_map(event, "threat.indicator.file");
            ensure_map(event, "threat.indicator.file.hash");
            // The length alone chooses the algorithm, so a value that is not a
            // string writes no hash and is still related.
            if let Some(text) = value.as_str() {
                let algorithm = match text.chars().count() {
                    32 => Some("md5"),
                    40 => Some("sha1"),
                    64 => Some("sha256"),
                    128 => Some("sha512"),
                    _ => None,
                };
                if let Some(algorithm) = algorithm {
                    let _ = event.set(
                        &format!("threat.indicator.file.hash.{algorithm}"),
                        value.clone(),
                    );
                }
            }
            relate(event, "related.hash", &value);
        }
        "url" => {
            let _ = event.set("threat.indicator.type", "url");
            ensure_map(event, "threat.indicator.url");
            let _ = event.set("threat.indicator.url.full", value.clone());
            let _ = event.set("threat.indicator.url.original", value);
        }
        "email" | "email-addr" => {
            let _ = event.set("threat.indicator.type", "email-addr");
            ensure_map(event, "threat.indicator.email");
            let _ = event.set("threat.indicator.email.address", value);
        }
        // `domain`, `hostname`, `domain-name` and anything unrecognised.
        _ => {
            let _ = event.set("threat.indicator.type", "domain-name");
            ensure_map(event, "threat.indicator.url");
            let _ = event.set("threat.indicator.url.domain", value.clone());
            relate(event, "related.hosts", &value);
        }
    }
}

/// Build the map at the path where the script finds none, the way its
/// `if (x == null) x = new HashMap()` does.
fn ensure_map(event: &mut Event, path: &str) {
    if !event.has_value(path) {
        let _ = event.set(path, Value::Object(Map::new()));
    }
}

/// Append the indicator to one of the `related` lists, once.
fn relate(event: &mut Event, path: &str, value: &Value) {
    ensure_map(event, "related");
    let _ = event.append_unique(path, value.clone());
}

/// `ti_socradar_feeds/feed`, `script_set_confidence_from_score`: the vendor's
/// 0-100 reputation score as the ECS confidence band.
///
/// An absent score is -1, which is the band BELOW zero rather than the top one
/// -- a feed that ships no `extra_info` is the least confident indicator here,
/// not the most.
fn confidence_from_score(event: &mut Event, _params: &Value) {
    let score = event
        .get(&format!("{FEED}.extra_info.score"))
        .filter(|value| value.is_number())
        .map_or(-1.0, painless_to_f64);

    let confidence = if score < 0.0 {
        "Low"
    } else if score == 0.0 {
        "None"
    } else if score <= 25.0 {
        "Low"
    } else if score <= 50.0 {
        "Medium"
    } else {
        "High"
    };
    let _ = event.set("threat.indicator.confidence", confidence);
}

/// `ti_socradar_feeds/feed`, `script_ioc_expiration`: when the indicator
/// stops being worth matching, from the duration the integration is
/// configured with.
///
/// The script dates the expiry off `new Date()`, so the value depends on when
/// the pipeline ran and nothing here can reproduce the capture's. The clock is
/// read as the ingest instant, which is the same reading Elasticsearch stamps
/// the document with.
fn ioc_expiration(event: &mut Event, _params: &Value) {
    let Some(duration) = event.get_string("_conf.ioc_expiration_duration") else {
        return;
    };
    let duration = duration.trim().to_owned();
    let Some(unit) = duration.chars().last() else {
        return;
    };
    let Ok(amount) = duration[..duration.len() - unit.len_utf8()].parse::<i64>() else {
        return;
    };
    let Some(now) = event
        .get_str("_ingest.timestamp")
        .and_then(|stamp| DateTime::parse_from_rfc3339(stamp).ok())
    else {
        return;
    };

    let expiry: Option<DateTime<FixedOffset>> = match unit {
        'h' => now.checked_add_signed(TimeDelta::hours(amount)),
        'm' => now.checked_add_signed(TimeDelta::minutes(amount)),
        // 'd', and every unit the script does not name, which it defaults to
        // ninety days rather than rejecting.
        _ => {
            let days = if unit == 'd' { amount } else { 90 };
            u64::try_from(days)
                .ok()
                .and_then(|days| now.checked_add_days(Days::new(days)))
        }
    };
    let Some(expiry) = expiry else {
        return;
    };

    ensure_map(event, "ti_socradar_feeds");
    ensure_map(event, FEED);
    let _ = event.set(
        &format!("{FEED}.ioc_expiration_date"),
        expiry.to_rfc3339_opts(SecondsFormat::Millis, true),
    );
    let _ = event.set(&format!("{FEED}.ioc_expiration_duration"), duration);
    let _ = event.set(
        &format!("{FEED}.ioc_expiration_reason"),
        "Expiration set by configuration",
    );
}

/// Every `ti_socradar_feeds` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "c7a8e8cfcc7c6a1c44fe6edb44b5a0679acacb786a901a939fc730582fdd2685",
        source: "ti_socradar_feeds",
        name: "resolve_indicator_type",
        run: resolve_indicator_type,
    },
    Entry {
        hash: "c32cc74e45d7cce966d2fe6c3d78a2acecd635eb50841f13681f65b59cfb1021",
        source: "ti_socradar_feeds",
        name: "confidence_from_score",
        run: confidence_from_score,
    },
    Entry {
        hash: "daba4304391af0936d260eedf4719c3e36b44c4493e57d7f412b777c0aaa30cd",
        source: "ti_socradar_feeds",
        name: "ioc_expiration",
        run: ioc_expiration,
    },
];
