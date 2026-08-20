// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How the vendor payload is wrapped on the way in.
//!
//! The transform pipeline is the same whichever way an event arrives. Only the
//! wrapper differs, and this module unwraps it, so the parsers are reused
//! across every transport that can carry the same vendor payload rather than
//! being tied to the one Elastic happens to ship.
//!
//! [`Envelope::Beats`] is the shape the transforms expect: the
//! raw vendor payload as a string in `message`.
//!
//! [`Envelope::Receiver`] is what dfe-receiver produces, on any of its
//! transports -- syslog, gelf, fluent, splunk-hec, otlp, prometheus, netflow.
//! Every converter tags its output with `_source`, which is what selects the
//! unwrap. The syslog arm puts a syslog-shaped line back in `message`, because
//! that is what the transforms grok.
//!
//! [`Envelope::Fetcher`] is what dfe-fetcher produces: the provider's own JSON
//! at the top level with three keys of its own added. It applies wherever
//! Elastic's agent input is a pure transport -- `httpjson`, `cel`, `aws-s3`,
//! `azure-eventhub` -- because there the ingest pipeline does all the parsing
//! and fetching the data ourselves loses nothing.

use dfe_runtime::{Event, syslog_pri};
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

/// The fetcher's own field names, added to every record it delivers.
///
/// Stripped before the payload is handed to the transform, the same way the
/// receiver's are: they describe the delivery, not the event.
const FETCHER_KEYS: &[&str] = &[
    "_timestamp_fetcher",
    "_timestamp_received",
    "_source_fetcher",
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
    /// dfe-receiver JSON, from any of its transports. `_source` names which,
    /// and the unwrap for that transport lifts it to the Elastic-equivalent
    /// fields. Accepts `syslog` as a name for the era when that was the only
    /// one.
    #[serde(alias = "syslog")]
    Receiver,
    /// dfe-fetcher JSON: the provider's own payload at the top level, with the
    /// fetcher's delivery keys alongside it.
    Fetcher,
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
            Self::Receiver => unwrap_receiver(event, framing.unwrap_or(Framing::Line)),
            Self::Fetcher => {
                unwrap_fetcher(event);
                Ok(())
            }
        }
    }
}

/// Move the provider's payload into `message` as the string the transform
/// expects, and drop the fetcher's delivery keys.
///
/// Beats hands an API source its payload as a SERIALISED string in `message`,
/// which is what every one of these pipelines parses first. dfe-fetcher
/// delivers the same payload as a real object at the top level, so the
/// conversion is a re-serialise rather than a reshape.
fn unwrap_fetcher(event: &mut Event) {
    for key in FETCHER_KEYS {
        event.remove(key);
    }

    // Already in the Beats shape: a fetcher whose provider hands back a bare
    // line rather than an object leaves `message` where it is.
    if event.get_string("message").is_some()
        && event.as_value().as_object().is_some_and(|o| o.len() == 1)
    {
        return;
    }

    let payload = serde_json::to_string(event.as_value()).unwrap_or_default();
    *event.as_value_mut() = json!({ "message": payload });
}

/// Dispatch on the transport the receiver took it from.
///
/// Every converter tags its output with `_source` -- `syslog`, `gelf`,
/// `fluent`, `prometheus` and the rest -- so that is what selects the unwrap.
/// A transport with no arm of its own keeps its payload and loses only the
/// receiver's own keys, which is the conservative reading: the transform's
/// grok gets what arrived rather than a shape invented for it.
fn unwrap_receiver(event: &mut Event, framing: Framing) -> crate::Result<()> {
    match event.get_str("_source") {
        Some("syslog") | None => unwrap_syslog(event, framing),
        Some(_) => {
            for key in RECEIVER_KEYS {
                event.remove(key);
            }
            Ok(())
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
        .and_then(syslog_pri::facility_code)
        .unwrap_or(DEFAULT_FACILITY);
    let severity = event
        .get_str("severity")
        .and_then(syslog_pri::severity_code)
        .unwrap_or(DEFAULT_SEVERITY);
    line.push('<');
    line.push_str(&syslog_pri::compose(facility, severity).to_string());
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

    /// A receiver transport with no arm of its own must not be run through the
    /// syslog reconstruction, which would build a line out of fields it never
    /// set. The payload survives and only the receiver's own keys go.
    #[test]
    fn a_receiver_transport_without_an_arm_keeps_its_payload() {
        let mut event = Event::new(json!({
            "_source": "gelf",
            "message": "short message",
            "host": "web01",
            "level": 6,
        }));

        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        assert_eq!(event.get_str("message"), Some("short message"));
        assert_eq!(event.get_str("host"), Some("web01"));
        assert!(!event.has("_source"));
    }

    /// The syslog arm is selected by `_source`, not by being the only one.
    #[test]
    fn the_syslog_arm_is_selected_by_its_transport_name() {
        let mut event = receiver_event();
        assert_eq!(event.get_str("_source"), Some("syslog"));

        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        assert_eq!(
            event.get_str("message"),
            Some("<165>Mar  3 10:30:00 web01 nginx[1234]: the body")
        );
    }

    /// Beats hands an API source its payload SERIALISED in `message`, so a
    /// fetcher record -- the same payload as a real object -- is re-serialised
    /// into the shape the transform parses.
    #[test]
    fn fetcher_serialises_the_payload_into_message() {
        let mut event = Event::new(json!({
            "actor": { "id": "00u1", "type": "User" },
            "eventType": "user.session.start",
            "_timestamp_fetcher": 1_700_000_000_000_u64,
            "_timestamp_received": 1_700_000_000_000_u64,
            "_source_fetcher": "okta",
        }));

        Envelope::Fetcher
            .unwrap_into_beats(&mut event, None)
            .unwrap();

        let message = event.get_str("message").expect("payload in message");
        let payload: Value = serde_json::from_str(message).expect("message is the payload");
        assert_eq!(
            payload.pointer("/actor/id").and_then(Value::as_str),
            Some("00u1")
        );
        assert_eq!(
            payload.get("eventType").and_then(Value::as_str),
            Some("user.session.start")
        );
    }

    /// The fetcher's own keys describe the delivery, not the event, so none of
    /// them may reach the transform.
    #[test]
    fn fetcher_keys_never_reach_the_payload() {
        let mut event = Event::new(json!({
            "eventType": "x",
            "_timestamp_fetcher": 1_u64,
            "_timestamp_received": 2_u64,
            "_source_fetcher": "okta",
        }));

        Envelope::Fetcher
            .unwrap_into_beats(&mut event, None)
            .unwrap();

        let message = event.get_str("message").expect("payload in message");
        for key in FETCHER_KEYS {
            assert!(!message.contains(key), "{key} survived into {message}");
            assert!(!event.has(key), "{key} survived on the event");
        }
    }

    /// A provider that hands back a bare line is already in the Beats shape,
    /// so re-serialising it would wrap the string in a second layer of JSON.
    #[test]
    fn fetcher_leaves_an_already_wrapped_line_alone() {
        let mut event = Event::new(json!({
            "message": "<134>1 raw syslog line",
            "_source_fetcher": "somewhere",
        }));

        Envelope::Fetcher
            .unwrap_into_beats(&mut event, None)
            .unwrap();
        assert_eq!(event.get_str("message"), Some("<134>1 raw syslog line"));
    }

    #[test]
    fn syslog_reconstructs_a_line_into_message() {
        let mut event = receiver_event();
        Envelope::Receiver
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
        Envelope::Receiver
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

        Envelope::Receiver
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

        Envelope::Receiver
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

        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();
        assert!(event.get_str("message").unwrap().contains("the body"));
    }

    #[test]
    fn parsed_header_survives_on_ecs_fields() {
        let mut event = receiver_event();
        Envelope::Receiver
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
        Envelope::Receiver
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
        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        // user(1)*8 + notice(5) = 13.
        assert_eq!(event.get_str("message"), Some("<13>plain line"));
    }

    /// The PRI the envelope writes must match the RFC for every combination,
    /// not just the one in the fixture.
    #[test]
    fn pri_is_facility_times_eight_plus_severity() {
        for (fcode, fname) in syslog_pri::FACILITIES.iter().enumerate() {
            for (scode, sname) in syslog_pri::SEVERITIES.iter().enumerate() {
                let mut event = Event::new(json!({
                    "message": "b",
                    "facility": fname,
                    "severity": sname,
                }));
                Envelope::Receiver
                    .unwrap_into_beats(&mut event, Some(Framing::Line))
                    .unwrap();

                let expected = format!("<{}>", fcode * 8 + scode);
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
        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();
        assert_eq!(event.get_str("message"), Some("<13>"));
    }

    #[test]
    fn a_malformed_timestamp_is_dropped_rather_than_emitted_raw() {
        let mut event = receiver_event();
        event.set("timestamp", json!("not a timestamp")).unwrap();
        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line))
            .unwrap();

        let line = event.get_str("message").unwrap();
        assert!(!line.contains("not a timestamp"), "{line}");
        assert!(line.starts_with("<165>web01"), "{line}");
    }
}
