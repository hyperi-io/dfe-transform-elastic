// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How the vendor payload is wrapped on the way in.
//!
//! The transform pipeline is the same whichever way an event arrives. Only the
//! wrapper differs, and this module unwraps it, so a source that a device can
//! emit over syslog can be fed from dfe-receiver instead of from Beats without
//! a second pipeline.
//!
//! [`Envelope::Beats`] is the shape the transforms were generated against: the
//! raw vendor payload as a string in `message`.
//!
//! [`Envelope::Syslog`] is what dfe-receiver produces: the syslog MSG body in
//! `message`, with the header already parsed into siblings. Unwrapping it means
//! putting a syslog-shaped line back in `message`, because that is what the
//! transforms grok.

use dfe_runtime::Event;
use serde_json::{Value, json};

use crate::registry::Framing;

/// The receiver's own field names, removed once they have been lifted.
const RECEIVER_KEYS: &[&str] = &[
    "_source",
    "_raw",
    "facility",
    "severity",
    "hostname",
    "appname",
    "procid",
    "msgid",
    "timestamp",
    "structured_data",
];

/// Facility name to its RFC 5424 number.
///
/// Pinned to `syslog_loose::SyslogFacility::as_str`, which is what dfe-receiver
/// serialises. The crate offers no public reverse, so the table lives here.
const FACILITIES: [(&str, u8); 24] = [
    ("kern", 0),
    ("user", 1),
    ("mail", 2),
    ("daemon", 3),
    ("auth", 4),
    ("syslog", 5),
    ("lpr", 6),
    ("news", 7),
    ("uucp", 8),
    ("cron", 9),
    ("authpriv", 10),
    ("ftp", 11),
    ("ntp", 12),
    ("audit", 13),
    ("alert", 14),
    ("clockd", 15),
    ("local0", 16),
    ("local1", 17),
    ("local2", 18),
    ("local3", 19),
    ("local4", 20),
    ("local5", 21),
    ("local6", 22),
    ("local7", 23),
];

/// Severity name to its RFC 5424 number.
const SEVERITIES: [(&str, u8); 8] = [
    ("emerg", 0),
    ("alert", 1),
    ("crit", 2),
    ("err", 3),
    ("warning", 4),
    ("notice", 5),
    ("info", 6),
    ("debug", 7),
];

/// The facility used when the receiver saw no PRI. 1 (user) is what a syslogd
/// assumes for an unprefixed line.
const DEFAULT_FACILITY: u8 = 1;

/// The severity used when the receiver saw no PRI.
const DEFAULT_SEVERITY: u8 = 5;

/// How the payload is wrapped on the way in.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Envelope {
    /// Beats or Elastic Agent: the raw vendor payload is a string in `message`.
    #[default]
    Beats,
    /// dfe-receiver syslog JSON: header parsed into siblings, body in
    /// `message`.
    Syslog,
}

impl Envelope {
    /// Rewrite `event` into the shape `framing` expects in `message`.
    ///
    /// [`Envelope::Beats`] is already that shape and is a no-op.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] if a field cannot be set on the event.
    pub fn unwrap_into_beats(
        self,
        event: &mut Event,
        framing: Option<Framing>,
    ) -> crate::Result<()> {
        match self {
            Self::Beats => Ok(()),
            Self::Syslog => unwrap_syslog(event, framing.unwrap_or(Framing::Line)),
        }
    }
}

/// Leave `message` holding what the pipeline groks, and lift the parsed header
/// onto ECS `log.syslog.*` either way.
///
/// [`Framing::Body`] keeps the receiver's `message` untouched: those pipelines
/// read it as CSV or key-value, so a prefixed header would corrupt the first
/// field.
fn unwrap_syslog(event: &mut Event, framing: Framing) -> crate::Result<()> {
    let message = match framing {
        Framing::Body => None,
        Framing::Line => Some(raw_line(event).unwrap_or_else(|| reconstruct(event))),
    };

    lift_to_ecs(event)?;

    for key in RECEIVER_KEYS {
        event.remove(key);
    }

    if let Some(line) = message {
        event.set("message", json!(line))?;
    }
    Ok(())
}

/// The original line, when the receiver kept one.
///
/// Preferred over reconstruction: a device's exact spelling is not always
/// recoverable from the parsed fields. `cisco_nexus` carries a sequence number
/// and `cisco_ios` a source IP, and neither survives the receiver's parse.
fn raw_line(event: &Event) -> Option<String> {
    let raw = event.get_str("_raw")?;
    (!raw.trim().is_empty()).then(|| raw.to_string())
}

/// Rebuild an RFC 3164 line from the receiver's parsed fields.
///
/// `<PRI>TIMESTAMP HOSTNAME APPNAME[PROCID]: BODY`, which is what a syslogd
/// would have written and what the vendor groks are built against.
fn reconstruct(event: &Event) -> String {
    let mut line = String::with_capacity(128);

    let facility = event
        .get_str("facility")
        .and_then(facility_number)
        .unwrap_or(DEFAULT_FACILITY);
    let severity = event
        .get_str("severity")
        .and_then(severity_number)
        .unwrap_or(DEFAULT_SEVERITY);
    line.push('<');
    line.push_str(&(u16::from(facility) * 8 + u16::from(severity)).to_string());
    line.push('>');

    if let Some(ts) = event.get_str("timestamp").and_then(rfc3164_timestamp) {
        line.push_str(&ts);
        line.push(' ');
    }

    if let Some(host) = event.get_str("hostname") {
        line.push_str(host);
        line.push(' ');
    }

    if let Some(app) = event.get_str("appname") {
        line.push_str(app);
        if let Some(pid) = event.get_str("procid") {
            line.push('[');
            line.push_str(pid);
            line.push(']');
        }
        line.push_str(": ");
    }

    if let Some(body) = event.get_str("message") {
        line.push_str(body);
    }

    line
}

/// RFC 3339 from the receiver to the `MMM dd HH:mm:ss` the vendor groks match.
fn rfc3164_timestamp(rfc3339: &str) -> Option<String> {
    chrono::DateTime::parse_from_rfc3339(rfc3339)
        .ok()
        .map(|dt| dt.format("%b %e %H:%M:%S").to_string())
}

fn facility_number(name: &str) -> Option<u8> {
    FACILITIES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, code)| *code)
}

fn severity_number(name: &str) -> Option<u8> {
    SEVERITIES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, code)| *code)
}

/// Copy the parsed header onto ECS `log.syslog.*` and `host.hostname`.
///
/// The transform's own grok sets the same fields when it matches. Setting them
/// here as well means the header survives a grok that does not.
fn lift_to_ecs(event: &mut Event) -> crate::Result<()> {
    let pairs: Vec<(&str, Value)> = [
        ("hostname", "log.syslog.hostname"),
        ("appname", "log.syslog.appname"),
        ("procid", "log.syslog.procid"),
        ("msgid", "log.syslog.msgid"),
        ("facility", "log.syslog.facility.name"),
        ("severity", "log.syslog.severity.name"),
    ]
    .iter()
    .filter_map(|(from, to)| event.get(from).cloned().map(|v| (*to, v)))
    .collect();

    for (path, value) in pairs {
        event.set(path, value)?;
    }

    if let Some(host) = event.get("hostname").cloned() {
        event.set("host.hostname", host)?;
    }
    if let Some(ts) = event.get("timestamp").cloned() {
        event.set("@timestamp", ts)?;
    }

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// The receiver's output for `<165>1 2026-03-03T10:30:00+11:00 web01 nginx
    /// 1234 ID47 - the body`.
    fn receiver_event() -> Event {
        Event::new(json!({
            "message": "the body",
            "facility": "local4",
            "severity": "notice",
            "timestamp": "2026-03-03T10:30:00+11:00",
            "hostname": "web01",
            "appname": "nginx",
            "procid": "1234",
            "msgid": "ID47",
            "_source": "syslog"
        }))
    }

    #[test]
    fn beats_envelope_leaves_the_event_alone() {
        let mut event = Event::new(json!({ "message": "raw payload" }));
        Envelope::Beats
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();
        assert_eq!(event.get_str("message"), Some("raw payload"));
    }

    #[test]
    fn syslog_reconstructs_a_line_into_message() {
        let mut event = receiver_event();
        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        // local4 = 20, notice = 5 -> 20*8+5 = 165.
        assert_eq!(
            event.get_str("message"),
            Some("<165>Mar  3 10:30:00 web01 nginx[1234]: the body")
        );
    }

    /// A body-framed pipeline reads `message` as CSV or key-value, so a
    /// prefixed header corrupts its first field. The body must come through
    /// untouched while the header still reaches the ECS fields.
    #[test]
    fn body_framing_leaves_message_alone() {
        let mut event = receiver_event();
        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Body))
            .unwrap();

        assert_eq!(event.get_str("message"), Some("the body"));
        assert_eq!(event.get_str("log.syslog.hostname"), Some("web01"));
    }

    /// `_raw` must not override `message` under body framing either -- the
    /// whole point of body framing is that a line would corrupt the parse.
    #[test]
    fn body_framing_ignores_raw() {
        let mut event = receiver_event();
        event.set("_raw", json!("<165>a full line")).unwrap();

        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Body))
            .unwrap();
        assert_eq!(event.get_str("message"), Some("the body"));
    }

    /// `_raw` is the device's own spelling, and beats any reconstruction.
    #[test]
    fn raw_wins_over_reconstruction() {
        let mut event = receiver_event();
        event
            .set("_raw", json!("<165>original spelling from the device"))
            .unwrap();

        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();
        assert_eq!(
            event.get_str("message"),
            Some("<165>original spelling from the device")
        );
    }

    /// An empty `_raw` is not a line -- fall back rather than emit nothing.
    #[test]
    fn empty_raw_falls_back_to_reconstruction() {
        let mut event = receiver_event();
        event.set("_raw", json!("   ")).unwrap();

        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();
        assert!(event.get_str("message").unwrap().contains("the body"));
    }

    #[test]
    fn parsed_header_survives_on_ecs_fields() {
        let mut event = receiver_event();
        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        assert_eq!(event.get_str("log.syslog.hostname"), Some("web01"));
        assert_eq!(event.get_str("log.syslog.appname"), Some("nginx"));
        assert_eq!(event.get_str("log.syslog.procid"), Some("1234"));
        assert_eq!(event.get_str("log.syslog.msgid"), Some("ID47"));
        assert_eq!(event.get_str("log.syslog.facility.name"), Some("local4"));
        assert_eq!(event.get_str("log.syslog.severity.name"), Some("notice"));
        assert_eq!(event.get_str("host.hostname"), Some("web01"));
        assert_eq!(
            event.get_str("@timestamp"),
            Some("2026-03-03T10:30:00+11:00")
        );
    }

    /// The receiver's own field names must not reach the output -- the
    /// contract is that both envelopes produce the same shape.
    #[test]
    fn receiver_fields_are_removed() {
        let mut event = receiver_event();
        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        for key in RECEIVER_KEYS {
            assert!(!event.has(key), "{key} leaked into the output");
        }
    }

    /// A line the receiver could not parse a PRI from still gets one, or the
    /// vendor groks that require `<PRI>` would all miss.
    #[test]
    fn a_missing_pri_defaults_rather_than_being_omitted() {
        let mut event = Event::new(json!({ "message": "plain line", "_source": "syslog" }));
        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        // user(1)*8 + notice(5) = 13.
        assert_eq!(event.get_str("message"), Some("<13>plain line"));
    }

    #[test]
    fn every_facility_and_severity_name_maps_to_its_rfc_number() {
        assert_eq!(facility_number("kern"), Some(0));
        assert_eq!(facility_number("local7"), Some(23));
        assert_eq!(severity_number("emerg"), Some(0));
        assert_eq!(severity_number("debug"), Some(7));
        assert_eq!(facility_number("nonsense"), None);
        assert_eq!(severity_number("nonsense"), None);

        // Codes must be dense, unique and in range, or a PRI is wrong.
        let codes: Vec<u8> = FACILITIES.iter().map(|(_, c)| *c).collect();
        assert_eq!(codes, (0..24).collect::<Vec<u8>>());
        let codes: Vec<u8> = SEVERITIES.iter().map(|(_, c)| *c).collect();
        assert_eq!(codes, (0..8).collect::<Vec<u8>>());
    }

    /// The PRI arithmetic must round-trip against the RFC 5424 definition for
    /// every combination, not just the one in the fixture.
    #[test]
    fn pri_is_facility_times_eight_plus_severity() {
        for (fname, fcode) in FACILITIES {
            for (sname, scode) in SEVERITIES {
                let mut event = Event::new(json!({
                    "message": "b",
                    "facility": fname,
                    "severity": sname,
                }));
                Envelope::Syslog
                    .unwrap_into_beats(&mut event, Some(Framing::Line))
                    .unwrap();

                let expected = format!("<{}>", u16::from(fcode) * 8 + u16::from(scode));
                let line = event.get_str("message").unwrap();
                assert!(
                    line.starts_with(&expected),
                    "{fname}/{sname}: expected {expected}, got {line}"
                );
            }
        }
    }

    /// An event with nothing but a body must still come out as a line rather
    /// than a panic or an empty string.
    #[test]
    fn a_bare_body_still_produces_a_line() {
        let mut event = Event::new(json!({ "message": "" }));
        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();
        assert_eq!(event.get_str("message"), Some("<13>"));
    }

    #[test]
    fn a_malformed_timestamp_is_dropped_rather_than_emitted_raw() {
        let mut event = receiver_event();
        event.set("timestamp", json!("not a timestamp")).unwrap();
        Envelope::Syslog
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        let line = event.get_str("message").unwrap();
        assert!(!line.contains("not a timestamp"), "{line}");
        assert!(line.starts_with("<165>web01"), "{line}");
    }
}
