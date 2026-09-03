// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Grok captures must land on their DOTTED field path.
//!
//! Regex capture names cannot contain a dot, so `%{IPV4:_temp.src_ip}` is
//! captured as `_temp_src_ip` and `grok_to_regex_with_map` returns a map back
//! to the original. A block that sets the capture NAME instead of the mapped
//! path writes a flat `_temp_src_ip` key that every later processor -- all of
//! which read the dotted path -- then misses.
//!
//! The failure is silent: the grok matches, a field is set, and the event
//! simply never gains the value anything downstream looks for.

use dfe_runtime::{Event, Transform};

/// Every key in `value`, flattened to dotted paths.
fn paths(value: &serde_json::Value, prefix: &str, out: &mut Vec<String>) {
    let Some(map) = value.as_object() else {
        return;
    };
    for (key, child) in map {
        let path = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };
        if child.is_object() {
            paths(child, &path, out);
        } else {
            out.push(path);
        }
    }
}

/// A capture named for a dotted path must never appear as a flat underscored
/// key on the event.
fn assert_no_underscored_captures(event: &Event, context: &str) {
    let mut found = Vec::new();
    paths(event.as_value(), "", &mut found);

    let leaked: Vec<&String> = found
        .iter()
        .filter(|path| {
            // The capture-name form of a dotted path: a top-level key that
            // carries the underscore-joined shape of one.
            !path.contains('.') && path.starts_with("_temp_")
        })
        .collect();

    assert!(
        leaked.is_empty(),
        "{context}: grok captures were written under their regex capture name \
         instead of their dotted field path: {leaked:?}",
    );
}

/// `src` is what the address-and-port grok block reads, and it is set by an
/// earlier step in the real pipeline. Set directly so the block under test
/// actually runs.
#[test]
fn cisco_meraki_urls_writes_dotted_paths() {
    let mut event = Event::new(serde_json::json!({
        "message": "1620085725.180187 MX84 urls src=10.0.0.7:443 dst=8.8.8.8:53",
        "src": "10.0.0.7:443",
        "dst": "8.8.8.8:53",
    }));

    let _ = dfe_transforms::filebeat::cisco_meraki::urls::Urls.transform(&mut event);
    assert_no_underscored_captures(&event, "cisco_meraki::urls");
    assert_eq!(
        event.get_str("source.ip"),
        Some("10.0.0.7"),
        "the address the grok captured must reach source.ip",
    );
}

/// The dissect reads `event.original` and needs the whole key sequence, so a
/// shortened line breaks at the first missing pair and the transform errors.
///
/// This test used to pass a message carrying `signature=` alone. Nothing
/// matched, so no capture leaked and the negative assertion held -- a
/// transform that did nothing looks identical to one that did it right. The
/// line here is from `tests/fixtures/cisco/meraki/logs/test-security-events.log`
/// and the expected values are that fixture's own.
#[test]
fn cisco_meraki_idsalerts_writes_dotted_paths() {
    const LINE: &str = "<134>1 1637783891.345984502 MX84 ids-alerts \
        signature=129:4:1 priority=3 timestamp=1637783891.512569 \
        direction=ingress protocol=tcp/ip src=67.43.156.15:80";

    let mut event = Event::new(serde_json::json!({
        "message": LINE,
        "event": { "original": LINE },
    }));

    let _ = dfe_transforms::filebeat::cisco_meraki::idsalerts::Idsalerts.transform(&mut event);
    assert_no_underscored_captures(&event, "cisco_meraki::idsalerts");
    assert_eq!(
        event.get_str("cisco_meraki.security.signature"),
        Some("129:4:1"),
        "the signature must reach its dotted path",
    );
    assert_eq!(
        event.get_str("source.ip"),
        Some("67.43.156.15"),
        "the address the grok captured must reach source.ip",
    );
}

/// A pipeline's own grok definition captured into a dotted path becomes a raw
/// `(?P<a_b>...)` group once inlined, which the expander never sees as
/// `%{NAME:a.b}`. Without the explicit pairs the value lands on `a_b` and every
/// later processor reading the dotted path misses it.
#[test]
fn an_inlined_group_reaches_its_dotted_path_through_the_supplied_map() {
    let compiled = dfe_runtime::grok_cache::grok_mapped(
        r"^(?P<temp_timestamp>(?:\d{4}))$",
        &[("temp_timestamp", "temp.timestamp")],
    );

    let mut event = Event::new(serde_json::json!({}));
    assert!(
        compiled
            .extract_into("2026", &mut event)
            .expect("extraction succeeds")
    );
    assert_eq!(event.get_str("temp.timestamp"), Some("2026"));
    assert!(
        event.as_value().get("temp_timestamp").is_none(),
        "the capture name must not survive as a flat key: {}",
        event.as_value()
    );
}

/// The mapped path is what the block must use, checked directly against the
/// runtime rather than through a transform.
#[test]
fn the_field_map_is_what_a_grok_block_must_set() {
    let compiled = dfe_runtime::grok_cache::grok("^%{IPV4:_temp.src_ip}:%{PORT:sport}$");

    let capture_names: Vec<&str> = compiled
        .regex
        .capture_names()
        .into_iter()
        .flatten()
        .collect();
    assert!(
        capture_names.contains(&"_temp_src_ip"),
        "the dotted path is captured underscored: {capture_names:?}",
    );
    assert_eq!(
        compiled.field_map.get("_temp_src_ip").map(String::as_str),
        Some("_temp.src_ip"),
        "and the map is the only way back to the dotted path",
    );

    let mut event = Event::new(serde_json::json!({}));
    let matched = compiled
        .extract_into("10.0.0.7:443", &mut event)
        .expect("extraction succeeds");

    assert!(matched);
    assert_eq!(
        event.get_str("_temp.src_ip"),
        Some("10.0.0.7"),
        "extract_into must write the dotted path",
    );
}
