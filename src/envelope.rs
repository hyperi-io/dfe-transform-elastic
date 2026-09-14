// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How the vendor payload is wrapped on the way in.
//!
//! The transform pipeline is the same whichever way an event arrives. Only the
//! wrapper differs, and this module unwraps it, so the parsers are reused
//! across every transport that can carry the same vendor payload rather than
//! being tied to the one Elastic happens to ship.
//!
//! [`Envelope::Beats`] is the shape the transforms expect: the raw vendor
//! payload as a string in `message`. It covers Elastic Agent too, and anything
//! that passed a Beats document through unaltered.
//!
//! [`Envelope::Receiver`] is what dfe-receiver produces, on any of its
//! transports -- syslog, gelf, fluent, splunk-hec, otlp, grpc, prometheus,
//! netflow, sflow. The syslog arm puts a syslog-shaped line back in `message`,
//! because that is what the transforms grok; the rest pass their payload
//! through with their own field names moved onto the ECS paths those names
//! would otherwise shadow.
//!
//! [`Envelope::Fetcher`] is what dfe-fetcher produces: the provider's own JSON
//! at the top level with three keys of its own added. It applies wherever
//! Elastic's agent input is a pure transport -- `httpjson`, `cel`, `aws-s3`,
//! `azure-eventhub` -- because there the ingest pipeline does all the parsing
//! and fetching the data ourselves loses nothing.
//!
//! Which one it is comes from [`detect`], off the first event of each batch.
//! `source.envelope` in the config pins it instead, for the shapes that carry
//! no marker at all.

use std::borrow::Cow;

use dfe_runtime::{Event, syslog_pri};
use serde_json::{Value, json};

use crate::registry::{Framing, Intake};

/// The receiver's own field names, removed once they have been lifted.
///
/// Public so a test asserts against THIS list rather than a copy of it. The
/// integration suite held a hand-written six of these twelve, so `procid`,
/// `msgid`, `timestamp`, `structured_data`, `facility`, `severity`, `hostname`
/// and `appname` were checked by nothing outside this file.
pub const RECEIVER_KEYS: &[&str] = &[
    "_source",
    "_raw",
    // OTLP and Vector's gRPC tag with these instead of `_source`; both are
    // routing marks rather than event data.
    "_signal",
    "_vector_type",
    "facility",
    "severity",
    "hostname",
    "appname",
    "procid",
    "msgid",
    "timestamp",
    "structured_data",
];

/// What one dfe-receiver transport calls things, and what ECS calls them.
///
/// A transport with no entry keeps its payload and loses only `_source` and
/// `_raw`, which is the conservative reading: the transform's grok gets what
/// arrived rather than a shape invented for it.
struct Transport {
    /// The value the converter writes into `_source`.
    name: &'static str,
    /// Copied onto the ECS path, then removed.
    ///
    /// The removal is not tidiness. `host` and `source` are bare STRINGS on
    /// these transports and ECS objects everywhere else, so leaving one in
    /// place shadows the whole `source.*` subtree and the pipeline's writes
    /// under it go nowhere.
    lift: &'static [(&'static str, &'static str)],
    /// Removed without being lifted: describes the delivery, not the event.
    drop: &'static [&'static str],
}

/// Epoch seconds rather than a string, so the lift has to convert.
const EPOCH_SECONDS: &str = "@timestamp:epoch_seconds";

/// Epoch milliseconds rather than a string. Prometheus remote-write is the only
/// transport that sends this resolution.
const EPOCH_MILLIS: &str = "@timestamp:epoch_millis";

/// What netflow and sflow share: one envelope head, written by the same
/// `write_envelope_head`, with the protocol name substituted.
///
/// The exporter is an observer in ECS's sense and the packet sequence is the
/// event's. `protocol`, `version`, `observation_domain`, `record_count` and
/// `flows` are the packet's own structure, and ECS has no home for them.
const FLOW_LIFT: &[(&str, &str)] = &[
    ("exporter_ip", "observer.ip"),
    ("packet_seq", "event.sequence"),
    ("t_collected", "event.created"),
];

/// Every transport whose own field names need moving out of ECS's way.
///
/// syslog is absent because it has an arm of its own -- it is the only one that
/// rebuilds a line rather than passing the payload through.
const TRANSPORTS: &[Transport] = &[
    Transport {
        name: "gelf",
        lift: &[
            ("host", "host.name"),
            ("level", "log.syslog.severity.code"),
            ("severity", "log.syslog.severity.name"),
        ],
        // `short_message` was copied into `message` by the converter.
        drop: &["version", "short_message", "timestamp"],
    },
    Transport {
        name: "fluent",
        lift: &[("timestamp", EPOCH_SECONDS)],
        drop: &[],
    },
    Transport {
        name: "splunk_hec",
        lift: &[
            ("host", "host.name"),
            ("source", "log.file.path"),
            ("sourcetype", "event.provider"),
            ("_time", EPOCH_SECONDS),
        ],
        drop: &["index"],
    },
    // OTLP's generic mode. `body` is the log line and the tracing pair is ECS's
    // own. `attributes`, `resource` and the scope pair stay put: they carry the
    // sender's data, and inventing a home for them would be a guess.
    Transport {
        name: "otlp",
        lift: &[
            ("body", "message"),
            ("severity_text", "log.level"),
            ("severity_number", "event.severity"),
            ("trace_id", "trace.id"),
            ("span_id", "span.id"),
            ("_timestamp", "@timestamp"),
            ("_observed_timestamp", "event.created"),
        ],
        drop: &[],
    },
    // Vector's gRPC transport tags metrics and traces with `_vector_type` and
    // adds nothing else. A log event is the sender's own map, untagged, and is
    // read as a bare payload instead -- so this arm only ever sees a metric or
    // a trace, whose stamp is the one field with an ECS home.
    Transport {
        name: "grpc",
        lift: &[("timestamp", "@timestamp")],
        drop: &[],
    },
    // Prometheus remote-write, native mode: the series labels arrive as
    // top-level fields, so `__name__` and the rest are the sender's own naming.
    // `job` and `instance` are the two with an exact ECS home.
    Transport {
        name: "prometheus",
        lift: &[
            ("job", "service.name"),
            ("instance", "service.address"),
            ("timestamp", EPOCH_MILLIS),
        ],
        drop: &[],
    },
    Transport {
        name: "netflow",
        lift: FLOW_LIFT,
        drop: &[],
    },
    Transport {
        name: "sflow",
        lift: FLOW_LIFT,
        drop: &[],
    },
];

/// The fetcher's own field names, added to every record it delivers.
///
/// Stripped before the payload is handed to the transform, the same way the
/// receiver's are: they describe the delivery, not the event.
///
/// Public for the same reason as [`RECEIVER_KEYS`].
pub const FETCHER_KEYS: &[&str] = &[
    "_timestamp_fetcher",
    "_timestamp_received",
    "_source_fetcher",
];

/// The facility used when the receiver saw no PRI. 1 (user) is what a syslogd
/// assumes for an unprefixed line.
const DEFAULT_FACILITY: u8 = 1;

/// The severity used when the receiver saw no PRI.
const DEFAULT_SEVERITY: u8 = 5;

/// What the operator said about the envelope.
///
/// `auto` is the default: the family is read off the first event of each batch,
/// so a deployment that changes producer needs no config change. Naming one
/// pins it, which is the way through for a shape that carries no marker.
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
pub enum EnvelopeSetting {
    /// Detect the family from each batch's first event.
    #[default]
    Auto,
    /// Beats, Elastic Agent, or anything passed through unaltered.
    #[serde(alias = "elastic")]
    Beats,
    /// dfe-receiver, on any of its transports.
    #[serde(alias = "syslog")]
    Receiver,
    /// dfe-fetcher.
    Fetcher,
}

impl EnvelopeSetting {
    /// The family the operator pinned, or `None` when they left it to detection.
    #[must_use]
    pub const fn pinned(self) -> Option<Envelope> {
        match self {
            Self::Auto => None,
            Self::Beats => Some(Envelope::Beats),
            Self::Receiver => Some(Envelope::Receiver),
            Self::Fetcher => Some(Envelope::Fetcher),
        }
    }
}

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
    /// `variant` names which of the producer's transports it came from, since
    /// they do not all agree on where the body goes. It is what detection
    /// returned, not `_source`: Splunk HEC writes no `_source` at all.
    ///
    /// [`Envelope::Beats`] is already the target shape and is a no-op.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] if a field cannot be set on the event.
    pub fn unwrap_into_beats(
        self,
        event: &mut Event,
        framing: Option<Framing>,
        variant: &str,
    ) -> crate::Result<()> {
        match self {
            Self::Beats => Ok(()),
            Self::Receiver => unwrap_receiver(event, framing.unwrap_or(Framing::Line), variant),
            Self::Fetcher => {
                unwrap_fetcher(event);
                Ok(())
            }
        }
    }

    /// What wrote the wrapper, as `agent.type`.
    ///
    /// Beats stamps the beat's own name here, so the DFE producers stamp
    /// theirs -- an operator reading `agent.type` learns what actually
    /// delivered the event either way.
    #[must_use]
    pub const fn producer(self) -> &'static str {
        match self {
            Self::Beats => "filebeat",
            Self::Receiver => "dfe-receiver",
            Self::Fetcher => "dfe-fetcher",
        }
    }
}

/// What one event's wrapper turned out to be.
///
/// The family selects the unwrap; the variant names which of that producer's
/// shapes it is, and is carried so the logs and metrics can report it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detected {
    /// The producer whose wrapper is around the payload.
    pub family: Envelope,
    /// Which of that producer's shapes. Borrowed for the Elastic variants,
    /// which are a closed set, and owned for the other two, whose variant is
    /// whatever string the producer wrote.
    pub variant: Cow<'static, str>,
}

/// Name the family from the marker keys, most specific first.
///
/// Top-level `contains_key` and nothing else -- no path walking, no parse, and
/// no allocation on the Elastic arms. Called once per EVENT through
/// [`Resolver::resolve`]: a scalo `WorkBatch` spans partitions, so one batch
/// can carry two producers' wrappers.
///
/// `_source_fetcher` is tested before `_source` because a fetched payload can
/// carry a field of that name itself.
#[must_use]
pub fn detect(event: &Event) -> Detected {
    let Some(keys) = event.as_value().as_object() else {
        return Detected {
            family: Envelope::Beats,
            variant: Cow::Borrowed(BARE),
        };
    };

    let owned = |family, key: &str| Detected {
        family,
        variant: keys
            .get(key)
            .and_then(Value::as_str)
            .map_or(Cow::Borrowed(UNNAMED), safe_variant),
    };

    if keys.contains_key("_source_fetcher") {
        return owned(Envelope::Fetcher, "_source_fetcher");
    }
    if keys.contains_key("_source") {
        return owned(Envelope::Receiver, "_source");
    }
    // OTLP's generic mode tags `_signal` and the Vector gRPC source tags
    // metrics and traces `_vector_type`; neither writes `_source`.
    if keys.contains_key("_signal") {
        return Detected {
            family: Envelope::Receiver,
            variant: Cow::Borrowed("otlp"),
        };
    }
    if keys.contains_key("_vector_type") {
        return Detected {
            family: Envelope::Receiver,
            variant: Cow::Borrowed("grpc"),
        };
    }
    // Splunk HEC writes no marker, so `sourcetype` stands in: the one key in
    // its metadata no other shape uses. A HEC event whose sender sent no
    // metadata is indistinguishable from a bare one and falls through.
    if keys.contains_key("sourcetype") {
        return Detected {
            family: Envelope::Receiver,
            variant: Cow::Borrowed("splunk_hec"),
        };
    }
    if keys.contains_key("data_stream") && keys.contains_key("elastic_agent") {
        return Detected {
            family: Envelope::Beats,
            variant: Cow::Borrowed("agent"),
        };
    }
    if keys.contains_key("fileset") {
        return Detected {
            family: Envelope::Beats,
            variant: Cow::Borrowed("module"),
        };
    }

    Detected {
        family: Envelope::Beats,
        variant: Cow::Borrowed(BARE),
    }
}

/// The Elastic variant with no wrapper at all: `message` and maybe `tags`.
///
/// The fallback rather than a detected shape, since it carries no marker.
pub const BARE: &str = "bare";

/// How a batch arrived, worked out once and applied to every event in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    /// The family to unwrap with.
    pub envelope: Envelope,
    /// Which of that producer's shapes, and what `input.type` becomes.
    pub variant: Cow<'static, str>,
    /// What a syslog delivery must leave in `message`.
    pub framing: Option<Framing>,
    /// `<package>.<data_stream>` for the configured source.
    pub dataset: &'static str,
}

impl Delivery {
    /// A Beats delivery, which needs no unwrap and no metadata put back.
    ///
    /// For callers with no source to hand -- offline tools and tests -- where
    /// the dataset would go unused anyway.
    #[must_use]
    pub const fn beats() -> Self {
        Self {
            envelope: Envelope::Beats,
            variant: Cow::Borrowed(BARE),
            framing: None,
            dataset: "",
        }
    }

    /// Remove the wrapper, then put back the metadata Beats would have set.
    ///
    /// A Beats delivery already carries all of it, so it is left alone.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] if a field cannot be set on the event.
    pub fn apply(&self, event: &mut Event) -> crate::Result<()> {
        if self.envelope == Envelope::Beats {
            return Ok(());
        }

        // Read before the unwrap: `unwrap_fetcher` re-serialises the payload
        // into `message`, taking the delivery keys with it.
        let ingested = ingest_time(event);

        self.envelope
            .unwrap_into_beats(event, self.framing, &self.variant)?;

        // `set_resolved`, not `set`: a payload already spelling one of these as
        // a single dotted key keeps it and gains a nested twin beside it, and
        // the transform reads past both.
        event.set_resolved("agent.type", json!(self.envelope.producer()))?;
        event.set_resolved("agent.version", json!(env!("CARGO_PKG_VERSION")))?;
        event.set_resolved("input.type", json!(self.variant.as_ref()))?;
        event.set_resolved("data_stream.type", json!("logs"))?;
        event.set_resolved("data_stream.dataset", json!(self.dataset))?;
        event.set_resolved("data_stream.namespace", json!("default"))?;
        if let Some(ingested) = ingested {
            event.set_resolved("event.ingested", ingested)?;
        }
        Ok(())
    }
}

/// When the producer says it took delivery, if it says at all.
///
/// dfe-fetcher stamps `_timestamp_received` in epoch milliseconds and the flow
/// transports stamp `t_collected` in RFC 3339. Nothing else records one, and
/// this service's own clock is no substitute: it would read as the receive time
/// while actually being the time the batch was transformed, which for a replay
/// is a different day.
fn ingest_time(event: &Event) -> Option<Value> {
    if let Some(collected) = event.get_str("t_collected") {
        return Some(json!(collected));
    }
    let millis = event.get("_timestamp_received")?.as_i64()?;
    let stamped = chrono::DateTime::from_timestamp_millis(millis)?;
    Some(json!(
        stamped.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    ))
}

/// Which envelope a batch will be unwrapped with, and what was odd about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    /// How to unwrap, and what to stamp back on.
    pub delivery: Delivery,
    /// What the batch's first event looked like, when there was one.
    pub detected: Option<Detected>,
    /// The operator pinned a family the events do not look like.
    pub contradicted: bool,
    /// The detected family is not one this source can arrive in.
    pub unaccepted: bool,
}

/// Decide how a batch arrived, from its first event.
///
/// A pinned setting wins over what the events look like, because the whole
/// point of pinning is to name a shape detection cannot see. A contradiction is
/// reported rather than fatal: one odd event must not take the pod down, and
/// the counters make it visible.
///
/// Detection that lands on a family the source cannot arrive in falls back to
/// `Beats`, which every source accepts.
#[must_use]
pub fn resolve(
    setting: EnvelopeSetting,
    first: Option<&Event>,
    intake: Intake,
    dataset: &'static str,
) -> Resolution {
    let detected = first.map(detect);
    let variant = detected
        .as_ref()
        .map_or(Cow::Borrowed(BARE), |d| d.variant.clone());

    let (envelope, contradicted, unaccepted) = if let Some(pinned) = setting.pinned() {
        let contradicted = detected.as_ref().is_some_and(|d| d.family != pinned);
        (pinned, contradicted, false)
    } else {
        let family = detected.as_ref().map_or(Envelope::Beats, |d| d.family);
        let unaccepted = !intake.accepts(family);
        let envelope = if unaccepted { Envelope::Beats } else { family };
        (envelope, false, unaccepted)
    };

    Resolution {
        delivery: Delivery {
            envelope,
            variant,
            framing: intake.framing,
            dataset,
        },
        detected,
        contradicted,
        unaccepted,
    }
}

/// The variant of a producer that tagged its family but wrote a non-string
/// where its own name should be.
const UNNAMED: &str = "unnamed";

/// The longest a producer-supplied variant name may be.
///
/// Every transport name dfe-receiver and dfe-fetcher use is under a dozen
/// characters, so this is a ceiling rather than a limit anything real reaches.
const VARIANT_MAX: usize = 64;

/// A producer's own name for its transport, trimmed to something safe to log
/// and to stamp into `input.type`.
///
/// The value comes off the wire in `_source` or `_source_fetcher` with no
/// length and no character restriction, and it reaches a log line and an output
/// field unaltered. Non-graphic bytes are dropped rather than escaped, because
/// a transport name is ASCII; a value left with nothing after that is reported
/// as [`UNNAMED`], the same as a non-string.
fn safe_variant(raw: &str) -> Cow<'static, str> {
    let trimmed: String = raw
        .chars()
        .filter(char::is_ascii_graphic)
        .take(VARIANT_MAX)
        .collect();
    if trimmed.is_empty() {
        Cow::Borrowed(UNNAMED)
    } else {
        Cow::Owned(trimmed)
    }
}

/// Decides each event's envelope, with the source's intake and dataset fixed.
///
/// Detection is per EVENT rather than per batch. A scalo `WorkBatch` spans
/// partitions -- a `Record` carries no partition -- so a topic fed by two
/// producers has both wrappers in the same batch, and reading only the first
/// event unwrapped every one of them as whatever that first event looked like.
/// The cost is the handful of top-level `contains_key` calls in [`detect`],
/// against an unwrap measured at 2,740 ns an event.
#[derive(Debug, Clone, Copy)]
pub struct Resolver {
    setting: EnvelopeSetting,
    intake: Intake,
    dataset: &'static str,
}

impl Resolver {
    /// A resolver for one configured source.
    #[must_use]
    pub const fn new(setting: EnvelopeSetting, intake: Intake, dataset: &'static str) -> Self {
        Self {
            setting,
            intake,
            dataset,
        }
    }

    /// How this event arrived, and what was odd about it.
    #[must_use]
    pub fn resolve(&self, event: &Event) -> Resolution {
        resolve(self.setting, Some(event), self.intake, self.dataset)
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

    let payload = match serde_json::to_string(event.as_value()) {
        Ok(payload) => payload,
        // A parsed document cannot hold a non-string key or a non-finite
        // number, so this is unreachable today -- but writing an empty
        // `message` in silence would look like a vendor that sent nothing.
        Err(e) => {
            tracing::warn!(error = %e, "fetcher payload would not re-serialise; message left empty");
            String::new()
        }
    };
    *event.as_value_mut() = json!({ "message": payload });
}

/// Dispatch on the transport the receiver took it from.
///
/// `variant` is what detection returned, which is `_source` for the six
/// transports that write one and the marker key for the ones that do not.
/// syslog is the only transport that rebuilds a line; the rest pass their
/// payload through with their own names moved out of ECS's way.
fn unwrap_receiver(event: &mut Event, framing: Framing, variant: &str) -> crate::Result<()> {
    let transport = variant_or_source(event, variant);
    if transport == "syslog" {
        return unwrap_syslog(event, framing);
    }

    if let Some(spec) = TRANSPORTS.iter().find(|t| t.name == transport) {
        apply_transport(event, spec)?;
    }
    for key in RECEIVER_KEYS {
        event.remove(key);
    }
    Ok(())
}

/// The transport's name, from the producer first and detection second.
///
/// `_source` is what the converter wrote and settles it for the six transports
/// that write one. The detected variant covers the ones that do not, chiefly
/// Splunk HEC. With neither, syslog is the only transport a bare payload can
/// still be.
fn variant_or_source<'a>(event: &'a Event, variant: &'a str) -> &'a str {
    if let Some(source) = event.get_str("_source") {
        return source;
    }
    if TRANSPORTS.iter().any(|t| t.name == variant) {
        return variant;
    }
    "syslog"
}

/// Move a transport's own field names onto their ECS homes.
///
/// Lifted before dropped, and the source key is removed as it is lifted: a bare
/// `host` string left in place makes `host.name` unwritable, since ECS wants an
/// object where the transport put a scalar.
fn apply_transport(event: &mut Event, spec: &Transport) -> crate::Result<()> {
    for (from, to) in spec.lift {
        let Some(value) = event.remove(from) else {
            continue;
        };
        if is_unset(&value) {
            continue;
        }
        // `set_resolved` throughout: a transport's payload can spell an ECS
        // target as one dotted key of its own, and splitting the path leaves
        // that key holding the value the pipeline then fails to find.
        match *to {
            EPOCH_SECONDS => {
                if let Some(stamped) = epoch_stamp(&value, 1000.0) {
                    event.set_resolved("@timestamp", stamped)?;
                }
            }
            EPOCH_MILLIS => {
                if let Some(stamped) = epoch_stamp(&value, 1.0) {
                    event.set_resolved("@timestamp", stamped)?;
                }
            }
            path => event.set_resolved(path, value)?,
        }
    }
    for key in spec.drop {
        event.remove(key);
    }
    Ok(())
}

/// Whether a transport wrote this to mean "I did not have one".
///
/// OTLP fills `trace_id`, `span_id` and `_observed_timestamp` in every log,
/// with an empty string or the Unix epoch where it had no value, and lifting
/// those writes an ECS field that says something false.
///
/// The trade is deliberate: a sender that genuinely means midnight on
/// 1970-01-01 loses that stamp. Nothing this service ingests does -- the epoch
/// is what a zeroed `Timestamp` renders as -- and treating it as real puts
/// every OTLP log's `event.created` fifty-six years in the past.
fn is_unset(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.is_empty() || s.starts_with("1970-01-01T00:00:00"),
        _ => false,
    }
}

/// An epoch stamp, whole or fractional, as the RFC 3339 string ECS wants.
///
/// `per_milli` is what one unit of the incoming number is worth in
/// milliseconds: 1000 for seconds, 1 for milliseconds.
fn epoch_stamp(value: &Value, per_milli: f64) -> Option<Value> {
    let count = value.as_f64()?;
    #[allow(clippy::cast_possible_truncation)]
    let millis = (count * per_milli) as i64;
    let stamped = chrono::DateTime::from_timestamp_millis(millis)?;
    Some(json!(
        stamped.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    ))
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

/// The receiver's field names against the ECS ones they belong under.
///
/// A list rather than a hand-written arm per field, so a transport that names
/// its header differently is an entry. Walked directly: building a `Vec` of the
/// matches first cost an allocation on every event for nothing.
const SYSLOG_TO_ECS: &[(&str, &str)] = &[
    ("hostname", "log.syslog.hostname"),
    ("appname", "log.syslog.appname"),
    ("procid", "log.syslog.procid"),
    ("msgid", "log.syslog.msgid"),
    ("facility", "log.syslog.facility.name"),
    ("severity", "log.syslog.severity.name"),
    ("hostname", "host.hostname"),
    ("timestamp", "@timestamp"),
];

/// Copy the parsed header onto ECS `log.syslog.*`, `host.hostname` and
/// `@timestamp`.
///
/// The transform's own grok sets the same fields when it matches. Setting them
/// here as well means the header survives a grok that does not.
fn lift_to_ecs(event: &mut Event) -> crate::Result<()> {
    apply_renames(event, SYSLOG_TO_ECS)
}

/// Move each `from` onto its `to`, skipping the fields this event does not
/// carry.
///
/// A source that feeds a LATER destination as well is cloned; the last
/// destination for a source takes the value itself. Of the eight pairs only
/// `hostname` has two destinations, so seven events' worth of clone per
/// receiver event goes away, and every source here is in [`RECEIVER_KEYS`] and
/// would have been removed a few lines later regardless.
fn apply_renames(event: &mut Event, renames: &[(&str, &str)]) -> crate::Result<()> {
    for (index, (from, to)) in renames.iter().enumerate() {
        let reused = renames
            .iter()
            .skip(index + 1)
            .any(|(later, _)| later == from);
        // `set` needs the event mutably, so a value fed to a later destination
        // has to be cloned out first.
        let value = if reused {
            event.get(from).cloned()
        } else {
            event.remove(from)
        };
        if let Some(value) = value {
            event.set_resolved(to, value)?;
        }
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

    /// A fetched payload can carry a field called `_source` of its own, so the
    /// fetcher's key has to be tested first or its events read as receiver ones.
    #[test]
    fn the_fetcher_marker_wins_over_a_payload_field_of_the_same_name() {
        let event = Event::new(json!({
            "_source": "s3://bucket/key",
            "_source_fetcher": "aws.cloudtrail",
        }));
        let detected = detect(&event);
        assert_eq!(detected.family, Envelope::Fetcher);
        assert_eq!(detected.variant, "aws.cloudtrail");
    }

    /// A marker whose value is not a string still names its family, so the
    /// unwrap is right even though the variant cannot be reported.
    #[test]
    fn a_marker_with_a_non_string_value_still_names_its_family() {
        let event = Event::new(json!({ "_source": 7 }));
        let detected = detect(&event);
        assert_eq!(detected.family, Envelope::Receiver);
        assert_eq!(detected.variant, "unnamed");
    }

    /// A JSON line that is an array or a scalar has no keys to read, and must
    /// fall through rather than panic.
    #[test]
    fn an_event_that_is_not_an_object_detects_as_bare() {
        let detected = detect(&Event::new(json!([1, 2, 3])));
        assert_eq!(detected.family, Envelope::Beats);
        assert_eq!(detected.variant, BARE);
    }

    #[test]
    fn beats_envelope_leaves_the_event_alone() {
        let mut event = Event::new(json!({ "message": "raw payload" }));
        Envelope::Beats
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
            .unwrap();
        assert_eq!(event.get_str("message"), Some("raw payload"));
    }

    /// A transport this table has never heard of must not be run through the
    /// syslog reconstruction, which would build a line out of fields it never
    /// set. The payload survives and only `_source` goes.
    ///
    /// Every transport dfe-receiver ships today has an arm, so the case this
    /// guards is the next one it grows.
    #[test]
    fn a_receiver_transport_without_an_arm_keeps_its_payload() {
        let mut event = Event::new(json!({
            "_source": "a_transport_from_the_future",
            "whatever": "the sender called it",
            "value": 1234.0,
        }));

        Envelope::Receiver
            .unwrap_into_beats(
                &mut event,
                Some(Framing::Line),
                "a_transport_from_the_future",
            )
            .unwrap();

        assert_eq!(event.get_str("whatever"), Some("the sender called it"));
        assert_eq!(event.get("value"), Some(&json!(1234.0)));
        assert!(!event.has("_source"));
        assert!(!event.has("message"), "no line may be reconstructed");
    }

    /// A bare `host` string is an ECS OBJECT everywhere else, so leaving one
    /// where gelf and Splunk HEC put it shadows the whole `host.*` subtree and
    /// every write the pipeline makes under it goes nowhere.
    #[test]
    fn a_transports_own_names_move_off_the_ecs_paths_they_shadow() {
        let mut event = Event::new(json!({
            "_source": "gelf",
            "message": "short message",
            "host": "web01",
            "version": "1.1",
            "short_message": "short message",
            "level": 6,
            "severity": "informational",
        }));

        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line), "gelf")
            .unwrap();

        assert_eq!(event.get_str("message"), Some("short message"));
        assert_eq!(event.get_str("host.name"), Some("web01"));
        assert_eq!(
            event.get_str("log.syslog.severity.name"),
            Some("informational")
        );
        // `host` is present, but as the ECS object -- what must not survive is
        // the bare string that was standing where the object goes.
        assert_eq!(event.get_str("host"), None);
        for gone in ["version", "short_message", "severity", "_source"] {
            assert!(!event.has(gone), "{gone} survived");
        }
    }

    /// Splunk HEC writes no `_source`, so only the detected variant says which
    /// transport it is -- and without that its `source` string shadows ECS.
    #[test]
    fn the_hec_arm_is_selected_by_the_detected_variant_alone() {
        let mut event = Event::new(json!({
            "message": "a line",
            "host": "fw01",
            "source": "/var/log/panw",
            "sourcetype": "pan:traffic",
            "index": "main",
            "_time": 1_771_459_200_u64,
        }));

        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Body), "splunk_hec")
            .unwrap();

        assert_eq!(event.get_str("host.name"), Some("fw01"));
        assert_eq!(event.get_str("log.file.path"), Some("/var/log/panw"));
        assert_eq!(event.get_str("event.provider"), Some("pan:traffic"));
        assert_eq!(
            event.get_str("@timestamp"),
            Some("2026-02-19T00:00:00.000Z")
        );
        // Each is now the ECS object its name belongs to, never the bare
        // string that was shadowing it.
        for shadowed in ["host", "source", "sourcetype"] {
            assert_eq!(
                event.get_str(shadowed),
                None,
                "{shadowed} is still a bare string and shadows ECS"
            );
        }
        for gone in ["index", "_time"] {
            assert!(!event.has(gone), "{gone} survived");
        }
    }

    /// The syslog arm is selected by `_source`, not by being the only one.
    #[test]
    fn the_syslog_arm_is_selected_by_its_transport_name() {
        let mut event = receiver_event();
        assert_eq!(event.get_str("_source"), Some("syslog"));

        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
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
            .unwrap_into_beats(&mut event, None, "syslog")
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
            .unwrap_into_beats(&mut event, None, "syslog")
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
            .unwrap_into_beats(&mut event, None, "syslog")
            .unwrap();
        assert_eq!(event.get_str("message"), Some("<134>1 raw syslog line"));
    }

    #[test]
    fn syslog_reconstructs_a_line_into_message() {
        let mut event = receiver_event();
        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
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
            .unwrap_into_beats(&mut event, Some(Framing::Body), "syslog")
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
            .unwrap_into_beats(&mut event, Some(Framing::Body), "syslog")
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
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
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
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
            .unwrap();
        assert!(event.get_str("message").unwrap().contains("the body"));
    }

    #[test]
    fn parsed_header_survives_on_ecs_fields() {
        let mut event = receiver_event();
        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
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
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
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
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
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
                    .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
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
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
            .unwrap();
        assert_eq!(event.get_str("message"), Some("<13>"));
    }

    #[test]
    fn a_malformed_timestamp_is_dropped_rather_than_emitted_raw() {
        let mut event = receiver_event();
        event.set("timestamp", json!("not a timestamp")).unwrap();
        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
            .unwrap();

        let line = event.get_str("message").unwrap();
        assert!(!line.contains("not a timestamp"), "{line}");
        assert!(line.starts_with("<165>web01"), "{line}");
    }
}
