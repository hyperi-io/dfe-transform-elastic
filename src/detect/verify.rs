// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The few markers no literal pins down, each checked by a handful of byte
//! comparisons at the start of a literal hit.
//!
//! `at` is where the literal matched, so a check reads forward from it, or
//! back from it where a marker is only meaningful after something else.

/// Whether `h` at `p` has the shape `pattern`: `d` is an ASCII digit and any
/// other byte stands for itself.
fn shape(h: &[u8], p: usize, pattern: &[u8]) -> bool {
    h.get(p..p + pattern.len()).is_some_and(|window| {
        window.iter().zip(pattern).all(|(&b, &want)| {
            if want == b'd' {
                b.is_ascii_digit()
            } else {
                b == want
            }
        })
    })
}

/// How many bytes from `p` satisfy `keep`.
fn run(h: &[u8], p: usize, keep: fn(u8) -> bool) -> usize {
    h.get(p..)
        .map_or(0, |rest| rest.iter().take_while(|&&b| keep(b)).count())
}

fn upper_or_digit(b: u8) -> bool {
    b.is_ascii_uppercase() || b.is_ascii_digit()
}

fn mnemonic_byte(b: u8) -> bool {
    upper_or_digit(b) || b == b'_'
}

/// Carbon Black's event forwarder opens with a 10-digit epoch and a fraction,
/// where Jamf's webhook writes epoch milliseconds under the same key.
pub(super) fn cb_timestamp(h: &[u8], at: usize) -> bool {
    let digits = at + br#"{"event_timestamp":"#.len();
    shape(h, digits, b"dddddddddd.")
}

/// An IOS `%FACILITY[-SUB]-n-MNEMONIC:` message id, from the percent sign.
pub(super) fn ios_mnemonic(h: &[u8], at: usize) -> bool {
    // `-n-MNEMONIC`, an optional space, then the colon.
    let tail = |p: usize| {
        if !shape(h, p, b"-d-") {
            return false;
        }
        let name = run(h, p + 3, mnemonic_byte);
        let end = p + 3 + name;
        let end = end + usize::from(h.get(end).is_some_and(u8::is_ascii_whitespace));
        name > 0 && h.get(end) == Some(&b':')
    };

    let facility = at + 1;
    let facility_end = facility + run(h, facility, mnemonic_byte);
    if facility_end == facility {
        return false;
    }
    // The facility may carry a `-SUB` part, or the `-n-` may follow it directly.
    let sub = (h.get(facility_end) == Some(&b'-'))
        .then(|| facility_end + 1 + run(h, facility_end + 1, upper_or_digit));
    tail(facility_end) || sub.is_some_and(|end| end > facility_end + 1 && tail(end))
}

/// NX-OS writes the year before the month, which IOS does not: a month that
/// follows four digits and a day and time is a Nexus stamp. `at` is the space
/// before the month.
pub(super) fn nexus_stamp(h: &[u8], at: usize) -> bool {
    let year = at.checked_sub(4).and_then(|from| h.get(from..at));
    year.is_some_and(|y| y.iter().all(u8::is_ascii_digit))
        && h.get(at + 1).is_some_and(u8::is_ascii_uppercase)
        && h.get(at + 4) == Some(&b' ')
        && (shape(h, at + 5, b" d") || shape(h, at + 5, b"dd"))
        && shape(h, at + 7, b" dd:dd:dd")
}

/// A PAN-OS CSV log type is followed by the subtype, a sequence number and the
/// generated-time field, which is a date. `at` is the comma before the type.
pub(super) fn panos_date(h: &[u8], at: usize) -> bool {
    let mut comma = at;
    for _ in 0..3 {
        let Some(next) = memchr::memchr(b',', h.get(comma + 1..).unwrap_or_default()) else {
            return false;
        };
        comma += 1 + next;
    }
    shape(h, comma + 1, b"dddd/dd/dd") || shape(h, comma + 1, b"dddd-dd-dd")
}

/// Snort's `alert_csv`: a time, then ` ,gid,sid,rev,"message"`. `at` is the
/// space before the first comma.
pub(super) fn snort_csv(h: &[u8], at: usize) -> bool {
    let mut p = at + 2;
    for _ in 0..3 {
        let digits = run(h, p, |b| b.is_ascii_digit());
        if digits == 0 || h.get(p + digits) != Some(&b',') {
            return false;
        }
        p += digits + 1;
    }
    h.get(p) == Some(&b'"')
}

/// Squid writes `ACTION/status`: `TCP_MISS/200 `. `ProxySG` spells the same
/// action with a space instead of a slash. `at` is the start of the action.
pub(super) fn squid_status(h: &[u8], at: usize) -> bool {
    let name = if h.get(at..).is_some_and(|rest| rest.starts_with(b"NONE")) {
        4
    } else {
        3
    };
    let slash = at + name + run(h, at + name, |b| b.is_ascii_uppercase() || b == b'_');
    h.get(slash) == Some(&b'/') && shape(h, slash + 1, b"ddd ")
}

/// Sophos UTM writes `YYYY:MM:DD-HH:MM:SS `. `at` is the start of the year.
pub(super) fn utm_stamp(h: &[u8], at: usize) -> bool {
    shape(h, at, b"20dd:dd:dd-dd:dd:dd ")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// Each row is a check, the line, where its literal hit, and the verdict.
    /// The rows that should be refused are the near misses the literal alone
    /// would have accepted.
    #[test]
    fn checks_accept_their_shape_and_refuse_the_near_miss() {
        type Check = fn(&[u8], usize) -> bool;
        let rows: &[(&str, Check, &str, &str, bool)] = &[
            (
                "cb epoch",
                cb_timestamp,
                r#"{"event_timestamp":1397248033.914,"md5""#,
                "{",
                true,
            ),
            (
                "cb millis",
                cb_timestamp,
                r#"{"event_timestamp":1725443872001,"id""#,
                "{",
                false,
            ),
            ("ios plain", ios_mnemonic, "x %SYS-5-CONFIG_I: y", "%", true),
            (
                "ios sub",
                ios_mnemonic,
                "x %ETHPORT-SPEED-5-X: y",
                "%",
                true,
            ),
            ("ios space", ios_mnemonic, "x %LINK-3-UPDOWN : y", "%", true),
            ("ios no sev", ios_mnemonic, "x %SYS-CONFIG_I: y", "%", false),
            ("ios bare", ios_mnemonic, "100% done", "%", false),
            (
                "nexus",
                nexus_stamp,
                ": 2023 May  2 12:55:19 UTC:",
                " May",
                true,
            ),
            (
                "nexus no year",
                nexus_stamp,
                "SGT: Jun  2 15:15:19 SGT",
                " Jun",
                false,
            ),
            (
                "nexus no time",
                nexus_stamp,
                "in 2023 May the",
                " May",
                false,
            ),
            (
                "panos",
                panos_date,
                "x,TRAFFIC,drop,2562,2024/12/02 18:26:37,y",
                ",TRAFFIC",
                true,
            ),
            (
                "panos dashes",
                panos_date,
                "x,SYSTEM,gen,1,2024-12-02 18:26:37",
                ",SYSTEM",
                true,
            ),
            ("panos csv", panos_date, "a,START,b,c,d", ",START", false),
            ("panos short", panos_date, "a,END,b", ",END", false),
            (
                "snort csv",
                snort_csv,
                r#"09/04 ,1,1000006,0,"TCP""#,
                " ,",
                true,
            ),
            ("snort csv text", snort_csv, "a ,b,c", " ,", false),
            ("squid", squid_status, "0 TCP_MISS/200 12", "TCP_", true),
            ("squid none", squid_status, "0 NONE/503 0", "NONE", true),
            (
                "squid space",
                squid_status,
                "200 TCP_NC_MISS GET",
                "TCP_",
                false,
            ),
            ("utm", utm_stamp, "2023:04:28-00:00:04 host", "2023", true),
            ("utm year only", utm_stamp, "2023:04 host", "2023", false),
        ];
        for (name, check, line, hit, want) in rows {
            let at = line.find(hit).expect("the literal is in the line");
            assert_eq!(check(line.as_bytes(), at), *want, "{name}");
        }
    }
}
