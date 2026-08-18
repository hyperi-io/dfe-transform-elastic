// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Syslog priority: the RFC 5424 facility and severity tables, both ways.
//!
//! `PRI = facility * 8 + severity`. Several Elastic pipelines decompose it in
//! Painless, and the service's syslog envelope composes it back, so the tables
//! live here rather than in two copies that can disagree.
//!
//! Names match `syslog_loose`'s `as_str`, which is what dfe-receiver puts on
//! the wire.

/// Facility names, indexed by their RFC 5424 code.
pub const FACILITIES: [&str; 24] = [
    "kern", "user", "mail", "daemon", "auth", "syslog", "lpr", "news", "uucp", "cron", "authpriv",
    "ftp", "ntp", "audit", "alert", "clockd", "local0", "local1", "local2", "local3", "local4",
    "local5", "local6", "local7",
];

/// Severity names, indexed by their RFC 5424 code.
pub const SEVERITIES: [&str; 8] = [
    "emerg", "alert", "crit", "err", "warning", "notice", "info", "debug",
];

/// Facility code for a name.
#[must_use]
pub fn facility_code(name: &str) -> Option<u8> {
    #[allow(clippy::cast_possible_truncation)]
    FACILITIES.iter().position(|f| *f == name).map(|i| i as u8)
}

/// Severity code for a name.
#[must_use]
pub fn severity_code(name: &str) -> Option<u8> {
    #[allow(clippy::cast_possible_truncation)]
    SEVERITIES.iter().position(|s| *s == name).map(|i| i as u8)
}

/// Facility name for a code.
#[must_use]
pub fn facility_name(code: u8) -> Option<&'static str> {
    FACILITIES.get(code as usize).copied()
}

/// Severity name for a code.
#[must_use]
pub fn severity_name(code: u8) -> Option<&'static str> {
    SEVERITIES.get(code as usize).copied()
}

/// Split a PRI into its facility and severity codes.
#[must_use]
pub const fn decompose(pri: u16) -> (u8, u8) {
    #[allow(clippy::cast_possible_truncation)]
    (((pri >> 3) & 0x1F) as u8, (pri & 0x7) as u8)
}

/// Combine a facility and severity code into a PRI.
#[must_use]
pub const fn compose(facility: u8, severity: u8) -> u16 {
    (facility as u16) * 8 + (severity as u16)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn names_and_codes_round_trip() {
        for (code, name) in FACILITIES.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let code = code as u8;
            assert_eq!(facility_code(name), Some(code));
            assert_eq!(facility_name(code), Some(*name));
        }
        for (code, name) in SEVERITIES.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let code = code as u8;
            assert_eq!(severity_code(name), Some(code));
            assert_eq!(severity_name(code), Some(*name));
        }
    }

    #[test]
    fn unknown_names_have_no_code() {
        assert_eq!(facility_code("nonsense"), None);
        assert_eq!(severity_code("nonsense"), None);
        assert_eq!(facility_name(24), None);
        assert_eq!(severity_name(8), None);
    }

    /// Every valid PRI must survive a decompose/compose round trip.
    #[test]
    fn pri_round_trips_for_every_valid_value() {
        for facility in 0..24_u8 {
            for severity in 0..8_u8 {
                let pri = compose(facility, severity);
                assert_eq!(decompose(pri), (facility, severity), "pri {pri}");
            }
        }
    }

    /// The worked example from RFC 5424: local4 + notice is 165.
    #[test]
    fn the_rfc_example_holds() {
        assert_eq!(compose(20, 5), 165);
        assert_eq!(decompose(165), (20, 5));
        assert_eq!(facility_name(20), Some("local4"));
        assert_eq!(severity_name(5), Some("notice"));
    }
}
