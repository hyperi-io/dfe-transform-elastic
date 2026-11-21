// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Per-source settings the integration's pipeline reads from `_conf`.
//!
//! Elastic Agent sets an integration's settings on every event as root fields
//! under `_conf` (`fields_under_root: true`), and the pipelines read them there:
//! `cisco_nexus` maps a zone abbreviation through `_conf.tz_map`, and most read
//! `_conf.tz_offset`. `source.conf` is that map. It is checked once at start,
//! then set as `_conf` on each event after the envelope unwraps and before the
//! transform runs.

use std::collections::BTreeSet;

use dfe_runtime::Event;
use serde_json::{Map, Value};

use crate::conf_keys::CONF_KEYS;

/// The most `source.conf` may serialise to.
pub const MAX_CONF_BYTES: usize = 16 * 1024;

/// How deep `source.conf` may nest. The integrations' own settings go three
/// levels, a list of `{tz_short, tz_long}` maps.
const MAX_DEPTH: usize = 16;

/// Key names that hold a credential. A plain value in `source.conf` is written
/// into the deployment's config, which is not where a credential belongs, so
/// these are refused at any depth.
const SECRET_NAMES: [&str; 5] = ["secret", "password", "token", "api_key", "client_secret"];

/// The `_conf` keys `variant`'s transform reads. Empty when it reads none.
#[must_use]
pub fn keys_read(variant: &str) -> &'static [&'static str] {
    CONF_KEYS
        .binary_search_by_key(&variant, |(name, _)| *name)
        .map_or(&[], |at| CONF_KEYS[at].1)
}

/// Refuse a `source.conf` that is too big, nests too deep or carries a secret.
///
/// # Errors
///
/// [`crate::Error::Config`] naming the offending key.
pub fn check_shape(settings: &Map<String, Value>) -> crate::Result<()> {
    if settings.is_empty() {
        return Ok(());
    }
    let bytes = serde_json::to_vec(settings)
        .map_err(|e| crate::Error::Config(format!("source.conf does not serialise: {e}")))?
        .len();
    if bytes > MAX_CONF_BYTES {
        return Err(crate::Error::Config(format!(
            "source.conf is {bytes} bytes, over the {MAX_CONF_BYTES} byte limit"
        )));
    }
    let mut path = String::from("source.conf");
    check_object(settings, &mut path, 1)
}

fn check_object(map: &Map<String, Value>, path: &mut String, depth: usize) -> crate::Result<()> {
    for (key, value) in map {
        let mark = path.len();
        path.push('.');
        path.push_str(key);
        if SECRET_NAMES
            .iter()
            .any(|name| key.eq_ignore_ascii_case(name))
        {
            return Err(crate::Error::Config(format!(
                "{path} names a secret, and source.conf is written into the deployment's config \
                 in plain text. Secrets are not supported here"
            )));
        }
        check_value(value, path, depth)?;
        path.truncate(mark);
    }
    Ok(())
}

fn check_value(value: &Value, path: &mut String, depth: usize) -> crate::Result<()> {
    if !matches!(value, Value::Object(_) | Value::Array(_)) {
        return Ok(());
    }
    if depth >= MAX_DEPTH {
        return Err(crate::Error::Config(format!(
            "{path} nests deeper than {MAX_DEPTH} levels"
        )));
    }
    match value {
        Value::Object(map) => check_object(map, path, depth + 1),
        Value::Array(items) => {
            for (at, item) in items.iter().enumerate() {
                let mark = path.len();
                path.push('[');
                path.push_str(&at.to_string());
                path.push(']');
                check_value(item, path, depth + 1)?;
                path.truncate(mark);
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Refuse a top-level `source.conf` key that none of `variants` reads.
///
/// A key a transform never reads does nothing, so a typo would pass silently.
/// `scope` names the variants in the refusal.
///
/// # Errors
///
/// [`crate::Error::Config`] naming the key and what could be set instead.
pub fn check_readers(
    settings: &Map<String, Value>,
    variants: &[&str],
    scope: &str,
) -> crate::Result<()> {
    let readable: BTreeSet<&str> = variants
        .iter()
        .flat_map(|variant| keys_read(variant).iter().copied())
        .collect();
    for key in settings.keys() {
        if readable.contains(key.as_str()) {
            continue;
        }
        let instead = if readable.is_empty() {
            format!("{scope} reads no source.conf keys")
        } else {
            let names: Vec<&str> = readable.iter().copied().collect();
            format!("{scope} reads: {}", names.join(", "))
        };
        return Err(crate::Error::Config(format!(
            "source.conf.{key} is read by no part of {scope}, so it would do nothing. {instead}"
        )));
    }
    Ok(())
}

/// The settings to set as `_conf`, or nothing.
///
/// Built once at start. With no settings it holds no map, and
/// [`apply`](Self::apply) returns before it reads the event.
#[derive(Debug, Clone, Default)]
pub struct SourceConf(Option<Map<String, Value>>);

impl SourceConf {
    /// The settings `settings` holds, none when it is empty.
    #[must_use]
    pub fn new(settings: &Map<String, Value>) -> Self {
        Self((!settings.is_empty()).then(|| settings.clone()))
    }

    /// Set the settings as `_conf` on `event`.
    ///
    /// A `_conf` the event already carries, from an agent that set its own,
    /// keeps the keys these do not name. Where both name a key, these win.
    #[inline]
    pub fn apply(&self, event: &mut Event) {
        if let Some(settings) = &self.0 {
            Self::set(settings, event);
        }
    }

    fn set(settings: &Map<String, Value>, event: &mut Event) {
        let Some(fields) = event.as_value_mut().as_object_mut() else {
            return;
        };
        match fields.get_mut("_conf") {
            Some(Value::Object(existing)) => {
                existing.extend(settings.iter().map(|(k, v)| (k.clone(), v.clone())));
            }
            _ => {
                fields.insert("_conf".into(), Value::Object(settings.clone()));
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    fn settings(value: Value) -> Map<String, Value> {
        let Value::Object(map) = value else {
            panic!("a test settings value is an object");
        };
        map
    }

    fn message(result: crate::Result<()>) -> String {
        result.expect_err("refused").to_string()
    }

    /// The table is bisected, so an unsorted row would make a present variant
    /// read as one with no keys. Every row also names a registered variant.
    #[test]
    fn the_key_table_is_sorted_and_names_registered_variants() {
        let names: Vec<&str> = CONF_KEYS.iter().map(|(name, _)| *name).collect();
        assert!(
            names.windows(2).all(|pair| pair[0] < pair[1]),
            "unsorted or repeated"
        );
        for name in names {
            assert!(
                crate::registry::lookup(name).is_some(),
                "{name} is not registered"
            );
        }
    }

    #[test]
    fn a_variant_reports_the_keys_its_transform_reads() {
        assert_eq!(
            keys_read("filebeat.cisco_nexus.default"),
            ["tz_map", "tz_offset"]
        );
        assert_eq!(keys_read("filebeat.no_such_source.default"), [] as [&str; 0]);
    }

    #[test]
    fn shape_accepts_what_the_integrations_document() {
        let tz = json!({
            "tz_offset": "+10:00",
            "tz_map": [{"tz_short": "AEST", "tz_long": "Australia/Sydney"}]
        });
        check_shape(&settings(tz)).unwrap();
        check_shape(&Map::new()).unwrap();
    }

    /// Every secret name, at the top, in a map, in a list of maps, in any case.
    #[test]
    fn shape_refuses_a_secret_at_any_depth() {
        for (value, path) in [
            (json!({"secret": "x"}), "source.conf.secret"),
            (json!({"Password": "x"}), "source.conf.Password"),
            (json!({"a": {"token": "x"}}), "source.conf.a.token"),
            (
                json!({"a": [{"ok": 1}, {"api_key": "x"}]}),
                "source.conf.a[1].api_key",
            ),
            (json!({"client_secret": "x"}), "source.conf.client_secret"),
        ] {
            let text = message(check_shape(&settings(value.clone())));
            assert!(text.contains(path), "{value}: {text}");
            assert!(text.contains("names a secret"), "{text}");
        }
    }

    #[test]
    fn shape_refuses_more_than_the_size_limit() {
        let big = json!({"tz_offset": "x".repeat(MAX_CONF_BYTES)});
        assert!(message(check_shape(&settings(big))).contains("byte limit"));
        let just_under = json!({"tz_offset": "x".repeat(MAX_CONF_BYTES - 20)});
        check_shape(&settings(just_under)).unwrap();
    }

    /// `levels` maps inside one another, the outermost being `source.conf`'s own.
    fn nested(levels: usize) -> Map<String, Value> {
        let mut deep = json!("leaf");
        for _ in 0..levels {
            deep = json!({ "k": deep });
        }
        settings(deep)
    }

    #[test]
    fn shape_refuses_nesting_past_the_limit() {
        check_shape(&nested(MAX_DEPTH)).unwrap();
        let text = message(check_shape(&nested(MAX_DEPTH + 1)));
        assert!(text.contains("nests deeper"), "{text}");
    }

    #[test]
    fn readers_accept_a_key_the_variant_reads() {
        let tz = settings(json!({"tz_map": [], "tz_offset": "+10:00"}));
        check_readers(&tz, &["filebeat.cisco_nexus.default"], "cisco_nexus").unwrap();
    }

    #[test]
    fn readers_refuse_a_typo_and_name_what_is_read() {
        let typo = settings(json!({"tz_mapp": []}));
        let text = message(check_readers(
            &typo,
            &["filebeat.cisco_nexus.default"],
            "cisco_nexus",
        ));
        assert!(text.contains("source.conf.tz_mapp"), "{text}");
        assert!(text.contains("tz_map, tz_offset"), "{text}");
    }

    #[test]
    fn readers_refuse_any_key_for_a_variant_that_reads_none() {
        let any = settings(json!({"tz_offset": "+10:00"}));
        let text = message(check_readers(
            &any,
            &["filebeat.no_such_source.default"],
            "that source",
        ));
        assert!(text.contains("reads no source.conf keys"), "{text}");
    }

    /// A key read by any of the candidates passes, which is how agentless mode
    /// serves many sources at once.
    #[test]
    fn readers_accept_a_key_any_candidate_reads() {
        let tz_map = settings(json!({"tz_map": []}));
        let both = ["filebeat.okta.default", "filebeat.cisco_nexus.default"];
        check_readers(&tz_map, &both, "any candidate").unwrap();
        assert!(check_readers(&tz_map, &["filebeat.okta.default"], "okta").is_err());
    }

    #[test]
    fn empty_settings_leave_the_event_untouched() {
        let conf = SourceConf::new(&Map::new());
        assert!(conf.0.is_none(), "empty settings hold no map");
        let mut event = Event::new(json!({"message": "m"}));
        conf.apply(&mut event);
        assert_eq!(event.as_value(), &json!({"message": "m"}));
    }

    #[test]
    fn settings_become_a_conf_object_on_the_event() {
        let conf = SourceConf::new(&settings(json!({"tz_offset": "+10:00"})));
        let mut event = Event::new(json!({"message": "m"}));
        conf.apply(&mut event);
        assert_eq!(
            event.as_value(),
            &json!({"message": "m", "_conf": {"tz_offset": "+10:00"}})
        );
        assert_eq!(event.get_str("_conf.tz_offset"), Some("+10:00"));
    }

    /// An agent that set its own `_conf` keeps what these do not name.
    #[test]
    fn an_existing_conf_is_merged_with_these_winning() {
        let conf = SourceConf::new(&settings(json!({"tz_offset": "+10:00"})));
        let mut event = Event::new(json!({"_conf": {"tz_offset": "+00:00", "keep_json": true}}));
        conf.apply(&mut event);
        assert_eq!(
            event.as_value(),
            &json!({"_conf": {"tz_offset": "+10:00", "keep_json": true}})
        );
    }

    #[test]
    fn an_event_that_is_not_an_object_is_left_alone() {
        let conf = SourceConf::new(&settings(json!({"tz_offset": "+10:00"})));
        let mut event = Event::new(json!([1, 2]));
        conf.apply(&mut event);
        assert_eq!(event.as_value(), &json!([1, 2]));
    }
}
