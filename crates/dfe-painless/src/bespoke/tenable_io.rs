// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `tenable_io`'s plugin-output scripts, transcribed.
//!
//! A vulnerability's evidence arrives as the scanner plugin's own printed
//! text, so the package list, the versions and the fix are parsed back out of
//! a block that was written for a human. The scan stream has the same problem
//! one layer up: its targets are a comma-separated line mixing addresses,
//! ranges, CIDR and hostnames.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// Where the scanner's printed evidence sits.
const OUTPUT: &str = "tenable_io.vulnerability.output";

/// The nested copy of the package list, beside the flat ECS arrays.
const NESTED: &str = "tenable_io.vulnerability.package_nested";

/// `tenable_io/vulnerability`, `script_to_extract_package_fields`: the
/// packages a plugin reported, as the four flat `package.*` arrays and one
/// nested list.
///
/// Only the two block layouts the script names are parsed -- a `Path` listing
/// and a `Remote package installed` listing. Anything else is left alone,
/// which is how a Windows KB block passes through untouched.
fn extract_package_fields(event: &mut Event, _params: &Value) {
    let Some(output) = event.get_string(OUTPUT) else {
        return;
    };
    let results = strip_leading_blank(&output);
    if !(results.starts_with("Path") || results.starts_with("Remote package installed")) {
        return;
    }
    let results = format_output_for_split(results);

    let mut names: Vec<Value> = Vec::new();
    let mut versions: Vec<Value> = Vec::new();
    let mut fixed: Vec<Value> = Vec::new();
    let mut paths: Vec<Value> = Vec::new();
    let mut nested: Vec<Value> = Vec::new();

    for block in results.split(";;") {
        let mut entry = Map::new();
        for line in block.split("||") {
            if line.is_empty() {
                continue;
            }
            // Exactly two parts, so a value carrying a colon of its own is
            // passed over rather than truncated.
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                continue;
            }
            let key = parts[0].trim();
            let value = parts[1].trim();
            match key {
                "Path" => {
                    entry.insert("path".to_owned(), Value::from(value));
                    paths.push(Value::from(value));
                    let name = name_from_path(value);
                    entry.insert("name".to_owned(), Value::from(name));
                    names.push(Value::from(name));
                }
                "Installed version" => {
                    entry.insert("version".to_owned(), Value::from(value));
                    versions.push(Value::from(value));
                }
                "Fixed version" | "Should be" => {
                    entry.insert("fixed_version".to_owned(), Value::from(value));
                    fixed.push(Value::from(value));
                }
                "Remote package installed" => {
                    if let Some((name, version)) = split_name_version(value) {
                        entry.insert("name".to_owned(), Value::from(name));
                        entry.insert("version".to_owned(), Value::from(version));
                        names.push(Value::from(name));
                        versions.push(Value::from(version));
                    } else {
                        entry.insert("name".to_owned(), Value::from(value));
                        names.push(Value::from(value));
                    }
                }
                _ => {}
            }
        }
        nested.push(Value::Object(entry));
    }

    // `ctx.package = [:]` replaces whatever the pipeline had put there.
    let mut package = Map::with_capacity(4);
    package.insert("name".to_owned(), Value::Array(names));
    package.insert("version".to_owned(), Value::Array(versions));
    package.insert("fixed_version".to_owned(), Value::Array(fixed));
    package.insert("path".to_owned(), Value::Array(paths));
    let _ = event.set("package", Value::Object(package));
    let _ = event.set(NESTED, Value::Array(nested));
}

/// `^\n+\s*` removed, which is the blank the plugin opens its block with.
fn strip_leading_blank(output: &str) -> &str {
    if !output.starts_with('\n') {
        return output;
    }
    output.trim_start_matches(|c: char| c.is_whitespace())
}

/// The two separators the script rewrites the block with: a run of blank
/// lines between packages, and a single line break within one.
///
/// Two passes, in the script's own order, because they are not one rule --
/// the first needs two ADJACENT breaks and the second takes whatever it
/// leaves behind.
fn format_output_for_split(results: &str) -> String {
    let packages = replace_breaks(results, 2, ";;");
    replace_breaks(&packages, 1, "||")
}

/// `\n{min,}\s*` replaced by `separator`, everywhere it occurs.
///
/// `\s` covers a line break, so the greedy tail takes any whitespace that
/// follows the run -- the indent the plugin lays each line out with included.
fn replace_breaks(text: &str, min: usize, separator: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut at = 0usize;
    while at < bytes.len() {
        if bytes[at] != b'\n' {
            let next = text[at..]
                .find('\n')
                .map_or(bytes.len(), |offset| at + offset);
            out.push_str(&text[at..next]);
            at = next;
            continue;
        }
        let mut end = at;
        while end < bytes.len() && bytes[end] == b'\n' {
            end += 1;
        }
        if end - at < min {
            out.push('\n');
            at += 1;
            continue;
        }
        while end < bytes.len() && bytes[end].is_ascii_whitespace() {
            end += 1;
        }
        out.push_str(separator);
        at = end;
    }
    out
}

/// The package name a path spells: its last non-empty segment.
fn name_from_path(path: &str) -> &str {
    let segments: Vec<&str> = path.split('/').collect();
    let last = segments.len() - 1;
    if last > 0 && !segments[last].is_empty() {
        return segments[last];
    }
    segments[last.saturating_sub(1)]
}

/// `^([a-zA-Z-_]+)-(\d.*)`: the name and version an RPM-style string carries.
///
/// The name is the LONGEST leading run of letters, dashes and underscores that
/// still leaves a dash in front of a digit, which is what keeps
/// `kernel-debug-devel` whole ahead of `3.10.0-1160`.
fn split_name_version(value: &str) -> Option<(&str, &str)> {
    let bytes = value.as_bytes();
    let mut run = 0usize;
    while run < bytes.len()
        && (bytes[run].is_ascii_alphabetic() || matches!(bytes[run], b'-' | b'_'))
    {
        run += 1;
    }
    for at in (1..=run).rev() {
        if bytes.get(at) == Some(&b'-') && bytes.get(at + 1).is_some_and(u8::is_ascii_digit) {
            return Some((&value[..at], &value[at + 1..]));
        }
    }
    None
}

/// `tenable_io/vulnerability`, the linux `host.os.platform` script: the
/// distribution name, which is the first word of the reported OS.
fn os_platform(event: &mut Event, _params: &Value) {
    let Some(full) = event
        .get_array("host.os.full")
        .and_then(|items| items.first())
        .and_then(Value::as_str)
    else {
        return;
    };
    let platform = full.split(' ').next().unwrap_or_default().to_lowercase();
    let _ = event.set("host.os.platform", platform);
}

/// Where the scan stream keeps the console's own report of the run.
const SCAN_DETAILS: &str = "tenable_io.scan.scan_details";

/// `tenable_io/scan`, `populate_related_from_scan_details`: every address and
/// hostname the scan touched, split between `related.ip` and `related.hosts`.
fn related_from_scan_details(event: &mut Event, _params: &Value) {
    let targets = event
        .get_str(&format!("{SCAN_DETAILS}.info.targets"))
        .map(str::to_owned);
    if let Some(targets) = targets {
        for target in targets.split(',') {
            let value = target.trim().to_owned();
            if value.is_empty() {
                continue;
            }
            relate(event, &value);
        }
    }

    let hostnames: Vec<String> = event
        .get_array(&format!("{SCAN_DETAILS}.hosts"))
        .map(|hosts| {
            hosts
                .iter()
                .filter_map(|host| host.get("hostname").and_then(Value::as_str))
                .filter(|hostname| !hostname.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    for hostname in hostnames {
        relate(event, &hostname);
    }
}

/// One target onto whichever `related` list its own text says it belongs on.
///
/// An address list is related by the BASE -- a CIDR loses its prefix and a
/// range its upper bound -- while a hostname is related as written.
fn relate(event: &mut Event, value: &str) {
    let mut base = value;
    if let Some(slash) = base.find('/') {
        base = &base[..slash];
    }
    // A dash at position zero is a leading sign rather than a range, and an
    // address holding a colon is IPv6, whose own text carries no range.
    if let Some(dash) = base.find('-')
        && dash > 0
        && !base.contains(':')
    {
        base = &base[..dash];
    }
    let base = base.trim();
    if is_ipv4(base) || is_ipv6ish(base) {
        let _ = event.append_unique("related.ip", base);
    } else {
        let _ = event.append_unique("related.hosts", value);
    }
}

/// The script's own IPv4 pattern: four decimal octets, no leading zero on any
/// of them.
fn is_ipv4(text: &str) -> bool {
    let mut octets = 0usize;
    for octet in text.split('.') {
        octets += 1;
        if octets > 4 || octet.is_empty() || octet.len() > 3 {
            return false;
        }
        if !octet.bytes().all(|byte| byte.is_ascii_digit()) {
            return false;
        }
        if octet.len() > 1 && octet.starts_with('0') {
            return false;
        }
        if octet.parse::<u16>().unwrap_or(u16::MAX) > 255 {
            return false;
        }
    }
    octets == 4
}

/// The script's own IPv6 pattern, which is a character test rather than a
/// parse: `^[0-9A-Fa-f:]*:[0-9A-Fa-f:.]*$`.
///
/// Reading it as "a colon somewhere, with no dot ahead of the first one"
/// is the same test -- the earliest colon is the one that leaves the least
/// for the stricter half of the pattern to cover.
fn is_ipv6ish(text: &str) -> bool {
    let Some(first_colon) = text.find(':') else {
        return false;
    };
    if text[..first_colon].bytes().any(|byte| byte == b'.') {
        return false;
    }
    text.bytes()
        .all(|byte| byte.is_ascii_hexdigit() || matches!(byte, b':' | b'.'))
}

/// Every `tenable_io` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "ecfa216d46bf7e0315890933156a61758f5f2fff10d34adaf0c7034c9cf8867b",
        source: "tenable_io",
        name: "extract_package_fields",
        run: extract_package_fields,
    },
    Entry {
        hash: "b2add4c3faec99e88a8d43bbc429dd2708e65d22a0e2f1a4fa1dcd6c815d48e4",
        source: "tenable_io",
        name: "os_platform",
        run: os_platform,
    },
    Entry {
        hash: "865e317932fe09e4e6bad1527e3c68a188d9b62e6aa49ba443e0e4ffc3254fee",
        source: "tenable_io",
        name: "related_from_scan_details",
        run: related_from_scan_details,
    },
];
