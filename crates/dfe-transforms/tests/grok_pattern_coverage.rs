// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Which grok pattern names the transforms use that the registry does not know.
//!
//! `grok_pattern_regex` answers an unknown name with `.+?`. That is a
//! catch-all: the capture SUCCEEDS and takes arbitrary text, so the field ends
//! up holding the wrong value and nothing reports it. It is indistinguishable
//! from a correct extraction, which is what makes it worth a test.
//!
//! `%{SPACE}` was the worst of them -- 152 sites expecting whitespace and
//! matching any text instead.
//!
//! Lower the ceiling as patterns are defined. Never raise it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Total uses of pattern names the registry does not define. Only goes down.
///
/// Was 391 before `SPACE`, `PORT`, `TIME`, `QS` and the syslog names were
/// defined; `%{SPACE}` alone accounted for 152 of them.
const CEILING: usize = 0;

/// Names that may still fall through, because their definitions live in the
/// upstream pipelines' `pattern_definitions` and have not been carried across.
/// A name NOT on this list falling through is a regression.
///
/// Empty: the generator inlines every `pattern_definitions` entry, so a
/// vendor name never reaches the emitted Rust.
const ALLOWED: &[&str] = &[];

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Every `%{NAME}` and `%{NAME:field}` used, counted.
fn used_pattern_names() -> BTreeMap<String, usize> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&root, &mut files);

    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for path in files {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let mut rest = text.as_str();
        while let Some(start) = rest.find("%{") {
            rest = &rest[start + 2..];
            let Some(end) = rest.find('}') else { break };
            let body = &rest[..end];
            let name = body.split(':').next().unwrap_or(body);
            if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                *counts.entry(name.to_string()).or_default() += 1;
            }
            rest = &rest[end + 1..];
        }
    }
    counts
}

#[test]
fn unknown_grok_patterns_do_not_increase() {
    let used = used_pattern_names();
    let unknown: BTreeMap<&String, usize> = used
        .iter()
        .filter(|(name, _)| !dfe_runtime::codegen_api::is_known_grok_pattern(name))
        .map(|(name, count)| (name, *count))
        .collect();

    let total: usize = unknown.values().sum();
    for (name, count) in &unknown {
        println!("{count:5}  %{{{name}}}");
    }
    println!(
        "{total} use(s) of {} undefined pattern name(s)",
        unknown.len()
    );

    assert_eq!(
        total, CEILING,
        "uses of undefined grok patterns moved off {CEILING} to {total} -- \
         a pattern name was used that `grok_pattern_regex` does not define, so \
         it silently captures arbitrary text. Lower the ceiling if it FELL."
    );
}

/// A name falling through that is not on the allow-list means someone used a
/// pattern nobody has defined, and the capture is quietly wrong.
#[test]
fn only_known_gaps_fall_through_to_the_catch_all() {
    let allowed: BTreeSet<&str> = ALLOWED.iter().copied().collect();

    let unexpected: Vec<String> = used_pattern_names()
        .into_keys()
        .filter(|name| !dfe_runtime::codegen_api::is_known_grok_pattern(name))
        .filter(|name| !allowed.contains(name.as_str()))
        .collect();

    assert!(
        unexpected.is_empty(),
        "these pattern names are undefined and not on the allow-list, so they \
         capture arbitrary text: {unexpected:?}"
    );
}

/// The definitions added for the common names have to actually be right, or
/// they are worse than the catch-all -- a wrong pattern fails to match where
/// the catch-all at least matched something.
#[test]
fn the_defined_patterns_match_what_they_name() {
    for (name, should_match, should_not) in [
        ("SPACE", vec!["", " ", "   "], vec![]),
        ("PORT", vec!["443", "0", "65535"], vec!["", "http"]),
        (
            "TIME",
            vec!["10:30", "10:30:00", "10:30:00.5"],
            vec!["1030"],
        ),
        ("SYSLOGPRI", vec!["<134>"], vec!["134", "<>"]),
        (
            "CISCOMAC",
            vec!["0011.2233.4455"],
            vec!["00:11:22:33:44:55"],
        ),
        ("QS", vec!["\"quoted\"", "\"\""], vec!["unquoted"]),
    ] {
        let pattern = dfe_runtime::codegen_api::grok_to_regex(&format!("^%{{{name}}}$"));
        let re = regex::Regex::new(&pattern)
            .unwrap_or_else(|e| panic!("{name} expands to an invalid regex: {e}"));

        for input in should_match {
            assert!(re.is_match(input), "%{{{name}}} should match {input:?}");
        }
        for input in should_not {
            assert!(
                !re.is_match(input),
                "%{{{name}}} should NOT match {input:?}"
            );
        }
    }
}
