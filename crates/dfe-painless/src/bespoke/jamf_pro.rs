// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `jamf_pro`'s inventory pass: a macOS version padded to three parts, and the
//! release name that version belongs to.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// The inventory field the version is read from and written back to.
const VERSION: &str = "jamf_pro.inventory.operating_system.version";

/// The macOS release each version prefix names, in the order the vendor's own
/// table tests them.
const RELEASES: [(&str, &str); 12] = [
    ("15.", "sequoia"),
    ("14.", "sonoma"),
    ("13.", "ventura"),
    ("12.", "monterey"),
    ("11.", "big sur"),
    ("10.15.", "catalina"),
    ("10.14.", "mojave"),
    ("10.13.", "high sierra"),
    ("10.12.", "sierra"),
    ("10.11.", "el capitan"),
    ("10.10.", "yosemite"),
    ("10.9.", "mavericks"),
];

/// `jamf_pro/inventory`, processor `script_normalize_operating_system_version`:
/// the normalised `jamf_pro.inventory.operating_system.version` and
/// `host.os.full`.
///
/// `host.os.version` is copied off the normalised value by the next processor,
/// so the padding reaches it too.
fn normalize_operating_system_version(event: &mut Event, _params: &Value) {
    let Some(version) = event.get_string(VERSION) else {
        return;
    };
    let normalised = pad_to_three_parts(&version);
    let release = release_name(&normalised);
    event.update(VERSION, normalised);
    if release.is_empty() {
        return;
    }
    for path in ["host", "host.os"] {
        if !event.has_value(path) {
            let _ = event.set(path, Value::Object(Map::new()));
        }
    }
    let _ = event.set("host.os.full", release);
}

/// A version of digits and dots padded out to `major.minor.patch`.
///
/// Anything else -- a build suffix, a beta letter -- is left exactly as the
/// vendor sent it.
fn pad_to_three_parts(version: &str) -> String {
    let mut dots = 0;
    for ch in version.chars() {
        if ch == '.' {
            dots += 1;
            continue;
        }
        if !ch.is_ascii_digit() {
            return version.to_owned();
        }
    }
    match dots {
        0 => format!("{version}.0.0"),
        1 => format!("{version}.0"),
        _ => version.to_owned(),
    }
}

/// The release a version belongs to, or the empty string for one the vendor's
/// own table does not name.
fn release_name(version: &str) -> String {
    for (prefix, name) in RELEASES {
        if version.starts_with(prefix) {
            return name.to_owned();
        }
    }
    String::new()
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "2c867fd4910e58d254f6e845ae740f548706b9f45654a772ee895fbfdf919e89",
    source: "jamf_pro",
    name: "normalize_operating_system_version",
    run: normalize_operating_system_version,
}];
