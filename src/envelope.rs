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
use serde_json::{Map, Value, json};

use crate::registry::{Framing, Intake};

/// The receiver's own field names, removed once they have been lifted.
///
/// `_source` is deliberately NOT here. It carries the receiver's variant id,
/// and dfe-loader routes a transformed row to its source's table by matching
/// that field, so stripping it put every row in the catch-all `dfe.main` with
/// nothing to attribute it by (hyperi-io/dfe-transform-elastic#67).
///
/// Public so a test asserts against THIS list rather than a copy of it. The
/// integration suite held a hand-written six of them, so `procid`, `msgid`,
/// `timestamp`, `structured_data`, `facility`, `severity`, `hostname` and
/// `appname` were checked by nothing outside this file.
pub const RECEIVER_KEYS: &[&str] = &[
    "_raw",
    // OTLP and Vector's gRPC tag with these instead of `_source`, so a record
    // from either reaches the loader with no field to route on.
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
/// A transport with no entry keeps its payload and loses only `_raw`, which is
/// the conservative reading: the transform's grok gets what arrived rather than
/// a shape invented for it.
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
/// Stripped from the PAYLOAD before it is handed to the transform, the same way
/// the receiver's are: they describe the delivery, not the event. `_source_fetcher`
/// is put back at the top level afterwards, because that is the field dfe-engine
/// compiles a fetched source's routing rule against.
///
/// Public for the same reason as [`RECEIVER_KEYS`].
pub const FETCHER_KEYS: &[&str] = &[
    "_timestamp_fetcher",
    "_timestamp_received",
    "_source_fetcher",
];

/// The fetcher's marker. dfe-engine's routing rule for a fetched source matches
/// on it, so it survives the unwrap. dfe-loader reads only `_source`
/// (`metadata.source_fields`), so nothing in the loader keys on this field.
const FETCHER_SOURCE: &str = "_source_fetcher";

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
            Self::Fetcher => unwrap_fetcher(event),
        }
    }

    /// What wrote the wrapper, as `agent.type` where the payload names no
    /// agent of its own.
    ///
    /// Beats stamps the beat's own name here, so the DFE producers stamp
    /// theirs -- an operator reading `agent.type` learns what shipped the
    /// event either way.
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

    if keys.contains_key(FETCHER_SOURCE) {
        return owned(Envelope::Fetcher, FETCHER_SOURCE);
    }
    if let Some(source) = keys.get("_source") {
        // A routing rule may have replaced the transport's name with a catalogue
        // label, so what the event carries says which transport it was.
        let named = source.as_str().is_some_and(is_transport_name);
        if let Some(transport) = infer_transport(keys).filter(|_| !named) {
            return Detected {
                family: Envelope::Receiver,
                variant: Cow::Borrowed(transport),
            };
        }
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
    /// Which of that producer's shapes, and what `input.type` becomes where
    /// the payload names none.
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

    /// Remove the wrapper, then put back the metadata Beats would have set
    /// where the payload does not carry it already.
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

        if self.envelope == Envelope::Fetcher {
            dress_graph_record(self.dataset, event);
        }
        self.envelope
            .unwrap_into_beats(event, self.framing, &self.variant)?;

        // A Beats or Agent document the receiver forwards keeps its own agent,
        // input and namespace, as Elasticsearch keeps them. The data stream's
        // type and dataset name the pipeline that runs here, so they are ours.
        // This build's version only ever describes our own agent, never a
        // shipper's type.
        if !event.has_value("agent.type") {
            event.set_resolved("agent.type", json!(self.envelope.producer()))?;
            event.set_if_absent("agent.version", json!(env!("CARGO_PKG_VERSION")))?;
        }
        event.set_if_absent("input.type", json!(self.variant.as_ref()))?;
        event.set_resolved("data_stream.type", json!("logs"))?;
        event.set_resolved("data_stream.dataset", json!(self.dataset))?;
        event.set_if_absent("data_stream.namespace", json!("default"))?;
        if let Some(ingested) = ingested {
            event.set_resolved("event.ingested", ingested)?;
        }
        Ok(())
    }
}

/// The Microsoft Graph audit logs dfe-fetcher pulls, by the dataset of the
/// Elastic pipeline that reads them.
const GRAPH_AUDIT_DATASETS: &[&str] =
    &["azure.auditlogs", "azure.signinlogs", "azure.provisioning"];

/// The nested objects of a Graph provisioning record that the Event Hub form
/// spells with a capital: `id` is `Id` and `displayName` is `Name` there.
const PROVISIONING_OBJECTS: &[&str] = &[
    "servicePrincipal",
    "sourceSystem",
    "targetSystem",
    "initiatedBy",
    "sourceIdentity",
    "targetIdentity",
];

/// Wrap a Microsoft Graph audit record as the Azure Monitor diagnostic-log
/// record the `azure` pipelines were written for.
///
/// Elastic reads these streams from an Event Hub, where each record is a
/// diagnostic-log envelope (`time`, `category`, `operationName`, ...) around
/// the Graph resource under `properties`. dfe-fetcher delivers the Graph
/// resource itself, whose own `category` is a resource category such as
/// `UserManagement`, so the audit-logs pipeline, which keeps only
/// `category: AuditLogs`, drops every record. The envelope fields are derived
/// from the record, and the delivery keys stay at the top level for
/// [`unwrap_fetcher`] to strip.
///
/// A record that already has `properties` or `message` is in the Event Hub
/// form and is left alone.
fn dress_graph_record(dataset: &str, event: &mut Event) {
    if !GRAPH_AUDIT_DATASETS.contains(&dataset) {
        return;
    }
    let Some(record) = event.as_value_mut().as_object_mut() else {
        return;
    };
    if record.contains_key("properties") || record.contains_key("message") {
        return;
    }

    let (category, time_field, operation, address): (_, _, _, &[&str]) = match dataset {
        "azure.auditlogs" => (
            "AuditLogs",
            "activityDateTime",
            record.get("activityDisplayName").cloned(),
            &["initiatedBy", "user", "ipAddress"],
        ),
        "azure.signinlogs" => (
            "SignInLogs",
            "createdDateTime",
            Some(json!("Sign-in activity")),
            &["ipAddress"],
        ),
        _ => (
            "ProvisioningLogs",
            "activityDateTime",
            Some(json!("Provisioning activity")),
            &[],
        ),
    };
    if dataset == "azure.provisioning" {
        capitalise_provisioning_keys(record);
    }

    // dfe-fetcher copies the event time to `timestamp` for the common header.
    // It is a delivery key here, so it must not reach the Graph resource.
    if record.get("timestamp").is_some() && record.get("timestamp") == record.get(time_field) {
        record.remove("timestamp");
    }

    let mut dressed = Map::new();
    if let Some(time) = record.get(time_field) {
        dressed.insert("time".to_owned(), time.clone());
    }
    dressed.insert("category".to_owned(), json!(category));
    if let Some(operation) = operation.filter(Value::is_string) {
        dressed.insert("operationName".to_owned(), operation);
    }
    // A provisioning record is correlated by its change, and carries the
    // outcome, duration and tenant the Event Hub envelope repeats.
    let correlation = if dataset == "azure.provisioning" {
        "changeId"
    } else {
        "correlationId"
    };
    if let Some(correlation) = record.get(correlation) {
        dressed.insert("correlationId".to_owned(), correlation.clone());
    }
    if dataset == "azure.provisioning" {
        let copies = [
            ("tenantId", record.get("tenantId")),
            ("durationMs", record.get("durationInMilliseconds")),
            (
                "resultType",
                nested(record, &["provisioningStatusInfo", "status"])
                    .filter(|status| status.is_string()),
            ),
        ];
        for (key, value) in copies {
            if let Some(value) = value {
                dressed.insert(key.to_owned(), value.clone());
            }
        }
    }
    let caller = nested(record, address).filter(|ip| ip.as_str().is_some_and(|ip| !ip.is_empty()));
    if let Some(caller) = caller {
        dressed.insert("callerIpAddress".to_owned(), caller.clone());
    }
    for key in FETCHER_KEYS {
        if let Some(value) = record.remove(*key) {
            dressed.insert((*key).to_owned(), value);
        }
    }
    dressed.insert(
        "properties".to_owned(),
        Value::Object(std::mem::take(record)),
    );
    *record = dressed;
}

/// The value at `path` below `record`, if every step is an object.
fn nested<'a>(record: &'a Map<String, Value>, path: &[&str]) -> Option<&'a Value> {
    let (first, rest) = path.split_first()?;
    rest.iter()
        .try_fold(record.get(*first)?, |node, key| node.get(*key))
}

/// Spell the nested objects of a Graph provisioning record the way the Event
/// Hub form does, which is what the pipeline renames from: `id` becomes `Id`,
/// `displayName` becomes `Name`, and the initiator's `initiatingType` or
/// `initiatorType` becomes `Type`. A key the record already spells that way is
/// kept.
fn capitalise_provisioning_keys(record: &mut Map<String, Value>) {
    for object in PROVISIONING_OBJECTS {
        let Some(Value::Object(inner)) = record.get_mut(*object) else {
            continue;
        };
        for (graph, hub) in [
            ("id", "Id"),
            ("displayName", "Name"),
            ("initiatingType", "Type"),
            ("initiatorType", "Type"),
        ] {
            if inner.contains_key(hub) {
                continue;
            }
            if let Some(value) = inner.remove(graph) {
                inner.insert(hub.to_owned(), value);
            }
        }
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
/// expects, drop the fetcher's delivery keys, and put its marker back.
///
/// Beats hands an API source its payload as a SERIALISED string in `message`,
/// which is what every one of these pipelines parses first. dfe-fetcher
/// delivers the same payload as a real object at the top level, so the
/// conversion is a re-serialise rather than a reshape.
///
/// `_source_fetcher` is lifted out before that re-serialise and written back
/// after it. The delivery keys must not reach the vendor payload, but this one
/// is what dfe-engine compiles a fetched source's routing rule against, so an
/// output without it lands in the loader's catch-all table
/// (hyperi-io/dfe-transform-elastic#67).
///
/// # Errors
///
/// Returns [`crate::Error`] if the marker cannot be written back.
fn unwrap_fetcher(event: &mut Event) -> crate::Result<()> {
    let marker = event.remove(FETCHER_SOURCE);
    for key in FETCHER_KEYS {
        event.remove(key);
    }

    // Already in the Beats shape: a fetcher whose provider hands back a bare
    // line rather than an object leaves `message` where it is.
    let wrapped = event.get_string("message").is_some()
        && event.as_value().as_object().is_some_and(|o| o.len() == 1);

    if !wrapped {
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

    if let Some(marker) = marker {
        event.set(FETCHER_SOURCE, marker)?;
    }
    Ok(())
}

/// Dispatch on the transport the receiver took it from.
///
/// `variant` is what detection returned, which is `_source` for the six
/// transports that write one and the marker key for the ones that do not.
/// syslog is the only transport that rebuilds a line; the rest pass their
/// payload through with their own names moved out of ECS's way.
///
/// `_source` itself is READ here and left on the event -- see [`RECEIVER_KEYS`]
/// for why it is the one receiver key that survives.
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
/// that write one, unless a routing rule relabelled it, in which case the
/// event's own fields do. The detected variant covers the transports that write
/// none, chiefly Splunk HEC. With neither, syslog is the only transport a bare
/// payload can still be.
fn variant_or_source<'a>(event: &'a Event, variant: &'a str) -> &'a str {
    if let Some(source) = event.get_str("_source") {
        if is_transport_name(source) {
            return source;
        }
        let inferred = event.as_value().as_object().and_then(infer_transport);
        return inferred.unwrap_or(source);
    }
    if TRANSPORTS.iter().any(|t| t.name == variant) {
        return variant;
    }
    "syslog"
}

/// Whether `name` is a transport this module unwraps.
fn is_transport_name(name: &str) -> bool {
    name == "syslog" || TRANSPORTS.iter().any(|t| t.name == name)
}

/// The fields dfe-receiver's syslog converter writes, each only when the line
/// carried it. `severity` and `timestamp` are left out because gelf writes them
/// too.
const SYSLOG_FIELDS: &[&str] = &[
    "hostname",
    "appname",
    "procid",
    "msgid",
    "structured_data",
    "facility",
];

/// Which transport an event came from when `_source` no longer says.
///
/// dfe-receiver writes `_source` with the transport's name, and a routing rule
/// then overwrites it with the catalogue label (`cisco-asa`). Syslog is the
/// transport with syslog header fields. Any other is the [`TRANSPORTS`] entry
/// with the most of its own source fields present, at least two, so one shared
/// name such as `timestamp` never decides it alone. Entries with the same
/// fields (netflow and sflow) unwrap identically, so the first of them will do.
fn infer_transport(keys: &serde_json::Map<String, Value>) -> Option<&'static str> {
    if SYSLOG_FIELDS.iter().any(|f| keys.contains_key(*f)) {
        return Some("syslog");
    }
    let mut best: Option<(&'static str, usize)> = None;
    for transport in TRANSPORTS {
        let present = transport
            .lift
            .iter()
            .filter(|(from, _)| keys.contains_key(*from))
            .count();
        if present >= 2 && best.is_none_or(|(_, n)| present > n) {
            best = Some((transport.name, present));
        }
    }
    best.map(|(name, _)| name)
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
/// [`Framing::Body`] wants the body alone, because a prefixed header would
/// corrupt the first field of a CSV or key-value pipeline. It comes from the
/// receiver's parsed fields with a wrong tag split repaired (`repaired_body`).
/// `_raw`, which is opt-in, is read only when those fields hold no message at
/// all. [`Framing::Line`] prefers `_raw` when present, because a rebuilt line
/// loses what the parse dropped (`cisco_nexus` sequence numbers, the `cisco_ios`
/// source IP), and otherwise rebuilds from the same repaired fields.
fn unwrap_syslog(event: &mut Event, framing: Framing) -> crate::Result<()> {
    let message = match framing {
        Framing::Body if event.get_str("message").is_some_and(|m| !m.is_empty()) => {
            repaired_body(event)
        }
        Framing::Body => {
            raw_line(event).and_then(|raw| strip_syslog_header(&raw).map(str::to_owned))
        }
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

/// Whether `app` can be a real RFC 3164 tag: at most 32 of letters, digits and
/// `._-`, with a letter in it. `1,2015/01/05` is a CSV field, and a bare `1` a
/// version number, neither of them a tag.
fn is_tag(app: &str) -> bool {
    app.len() <= 32
        && app
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
        && app.bytes().any(|b| b.is_ascii_alphabetic())
}

/// The body the receiver's parsed fields describe, when its split was wrong.
///
/// dfe-receiver parses with `syslog_loose`, which takes the first token of the
/// message as the tag even when the device wrote none, ending it at a space or
/// a colon and dropping that one separator. A tagless PAN-OS line therefore
/// arrives with `1,2015/01/05` as `appname`. When `appname` is not a tag, the
/// body is `appname[procid]`, a space and `message`. The space is exact when
/// the token ended at one, which is how every tagless device line seen ends it.
///
/// With no `hostname` or `appname` the parse found no header at all: either
/// `message` is the whole line, or it starts with the tail of a Cisco stamp
/// (`.838 EST: `) the parse left behind.
///
/// `None` leaves `message` as it is.
fn repaired_body(event: &Event) -> Option<String> {
    let message = event.get_str("message").unwrap_or_default();
    match event.get_str("appname") {
        Some(app) if is_tag(app) => None,
        Some(app) => {
            let mut body = String::with_capacity(app.len() + message.len() + 8);
            body.push_str(app);
            if let Some(pid) = event.get_str("procid") {
                body.push('[');
                body.push_str(pid);
                body.push(']');
            }
            if !message.is_empty() {
                body.push(' ');
                body.push_str(message);
            }
            Some(body)
        }
        None if event.get_str("hostname").is_some() => None,
        None => strip_syslog_header(message)
            .or_else(|| strip_stamp_tail(message))
            .map(str::to_owned),
    }
}

/// `.838 EST: body` or `.838: body`, the tail of a Cisco stamp.
fn strip_stamp_tail(message: &str) -> Option<&str> {
    let fraction = message.strip_prefix('.')?;
    let digits = fraction.bytes().take_while(u8::is_ascii_digit).count();
    if !(1..=9).contains(&digits) {
        return None;
    }
    cisco_body(&fraction[digits..]).filter(|body| !body.is_empty())
}

/// The body of a syslog line, or `None` when its header does not read in full.
///
/// Optional `<PRI>`, then RFC 5424 (`1 TIMESTAMP HOST APP PROCID MSGID SD`) or
/// RFC 3164 (`Mmm dd hh:mm:ss`, or an RFC 3339 stamp, then HOST and an optional
/// tag). A Cisco stamp that ends `[ TZ]:` has no host. A tag is taken only when
/// the first token is `name:` or `name[pid]:` with the colon followed by a space
/// or the end, so `1,2015/01/05 05:38:58,...` stays in the body.
fn strip_syslog_header(line: &str) -> Option<&str> {
    let rest = line.get(pri_len(line.as_bytes())..)?;
    let body = rfc5424_body(rest).or_else(|| rfc3164_body(rest))?;
    let body = body.trim_end_matches(['\r', '\n']);
    (!body.is_empty()).then_some(body)
}

/// Bytes taken by a leading `<1-3 digits>`, or zero.
fn pri_len(b: &[u8]) -> usize {
    if b.first() != Some(&b'<') {
        return 0;
    }
    let digits = b[1..].iter().take_while(|d| d.is_ascii_digit()).count();
    if (1..=3).contains(&digits) && b.get(1 + digits) == Some(&b'>') {
        digits + 2
    } else {
        0
    }
}

/// The first space-delimited token and what follows its space.
fn token(s: &str) -> Option<(&str, &str)> {
    let (tok, rest) = s.split_once(' ')?;
    (!tok.is_empty()).then_some((tok, rest))
}

/// `dddd-dd-ddTdd:dd:dd`, an optional fraction, then `Z` or `+dd:dd`.
fn is_rfc3339(t: &str) -> bool {
    let b = t.as_bytes();
    let shape = |at: usize, pat: &[u8]| {
        b.get(at..at + pat.len()).is_some_and(|w| {
            w.iter().zip(pat).all(|(&c, &p)| {
                if p == b'd' {
                    c.is_ascii_digit()
                } else {
                    c == p
                }
            })
        })
    };
    if !shape(0, b"dddd-dd-ddTdd:dd:dd") {
        return false;
    }
    let mut i = 19;
    if b.get(i) == Some(&b'.') {
        let frac = b[i + 1..].iter().take_while(|d| d.is_ascii_digit()).count();
        if !(1..=9).contains(&frac) {
            return false;
        }
        i += 1 + frac;
    }
    match b.get(i) {
        Some(b'Z') => i + 1 == b.len(),
        Some(b'+' | b'-') => shape(i + 1, b"dd:dd") && i + 6 == b.len(),
        _ => false,
    }
}

/// RFC 5424: version, five header tokens, then structured data.
fn rfc5424_body(s: &str) -> Option<&str> {
    let digits = s.bytes().take_while(u8::is_ascii_digit).count();
    if !(1..=3).contains(&digits) || s.as_bytes().get(digits) != Some(&b' ') {
        return None;
    }
    let (stamp, s) = token(&s[digits + 1..])?;
    if stamp != "-" && !is_rfc3339(stamp) {
        return None;
    }
    // HOST, APP-NAME, PROCID, MSGID.
    let (_, s) = token(s)?;
    let (_, s) = token(s)?;
    let (_, s) = token(s)?;
    let (_, s) = token(s)?;

    if let Some(after) = s.strip_prefix('-') {
        return match after.as_bytes().first() {
            None => Some(after),
            Some(b' ') => Some(&after[1..]),
            Some(_) => None,
        };
    }
    let mut rest = s;
    let mut groups = 0;
    while rest.starts_with('[') {
        let b = rest.as_bytes();
        let mut j = 1;
        loop {
            match b.get(j)? {
                b'\\' => j += 2,
                b']' => break,
                _ => j += 1,
            }
        }
        rest = &rest[j + 1..];
        groups += 1;
    }
    (groups > 0).then(|| rest.strip_prefix(' ').unwrap_or(rest))
}

/// RFC 3164, with a BSD, Cisco or RFC 3339 stamp.
fn rfc3164_body(s: &str) -> Option<&str> {
    if let Some(after) = bsd_stamp(s) {
        return cisco_body(after).or_else(|| host_and_tag(after.strip_prefix(' ')?));
    }
    let (stamp, rest) = token(s)?;
    is_rfc3339(stamp).then(|| host_and_tag(rest)).flatten()
}

/// What follows `Mmm dd hh:mm:ss[.fff]`, or `None` when the stamp does not read.
/// A leading `*` or `.` is Cisco's unsynchronised-clock mark.
fn bsd_stamp(s: &str) -> Option<&str> {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let b = s.as_bytes();
    let mut i = usize::from(matches!(b.first(), Some(b'*' | b'.')));
    if !MONTHS.contains(&s.get(i..i + 3)?) {
        return None;
    }
    i += 3;
    let spaces = b[i..].iter().take_while(|&&c| c == b' ').count();
    if !(1..=2).contains(&spaces) {
        return None;
    }
    i += spaces;
    let day = b[i..].iter().take_while(|c| c.is_ascii_digit()).count();
    if !(1..=2).contains(&day) || b.get(i + day) != Some(&b' ') {
        return None;
    }
    i += day + 1;
    let clock = b.get(i..i + 8)?;
    let ok = clock.iter().enumerate().all(|(n, &c)| match n {
        2 | 5 => c == b':',
        _ => c.is_ascii_digit(),
    });
    if !ok {
        return None;
    }
    i += 8;
    if b.get(i) == Some(&b'.') {
        let frac = b[i + 1..].iter().take_while(|d| d.is_ascii_digit()).count();
        if !(1..=9).contains(&frac) {
            return None;
        }
        i += 1 + frac;
    }
    s.get(i..)
}

/// Cisco's `: body` or ` TZ: body` straight after the stamp, which has no host.
fn cisco_body(after: &str) -> Option<&str> {
    let rest = if let Some(r) = after.strip_prefix(':') {
        r
    } else {
        let (tz, r) = after.strip_prefix(' ')?.split_once(':')?;
        if !(2..=5).contains(&tz.len()) || !tz.bytes().all(|c| c.is_ascii_uppercase()) {
            return None;
        }
        r
    };
    Some(rest.strip_prefix(' ').unwrap_or(rest))
}

/// HOST, then the body with a leading `name:` or `name[pid]:` tag removed.
fn host_and_tag(s: &str) -> Option<&str> {
    let (host, rest) = token(s)?;
    if host.ends_with(':') {
        return None;
    }
    let b = rest.as_bytes();
    let mut j = b
        .iter()
        .take_while(|c| !matches!(c, b' ' | b':' | b'[' | b']'))
        .count();
    if j == 0 {
        return Some(rest);
    }
    if b.get(j) == Some(&b'[') {
        let pid = b[j + 1..]
            .iter()
            .take_while(|c| !matches!(c, b' ' | b'[' | b']'))
            .count();
        if pid == 0 || b.get(j + 1 + pid) != Some(&b']') {
            return Some(rest);
        }
        j += pid + 2;
    }
    if b.get(j) == Some(&b':') && matches!(b.get(j + 1), None | Some(b' ')) {
        let body = &rest[j + 1..];
        return Some(body.strip_prefix(' ').unwrap_or(body));
    }
    Some(rest)
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

    // A split the receiver got wrong has no real tag, so the repaired body
    // stands alone and nothing is written in a tag's place.
    if let Some(body) = repaired_body(event) {
        line.push_str(&body);
        return line;
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
        assert!(!event.has("message"), "no line may be reconstructed");
    }

    /// dfe-loader routes a transformed row to its source's table by matching
    /// `_source`, so an unwrap that strips it costs the row its table.
    #[test]
    fn the_receivers_variant_id_survives_every_arm() {
        let arms: [(&str, Event); 3] = [
            ("syslog", receiver_event()),
            (
                "gelf",
                Event::new(
                    json!({ "_source": "gelf", "message": "m", "host": "web01", "level": 6 }),
                ),
            ),
            (
                "a_transport_from_the_future",
                Event::new(json!({ "_source": "a_transport_from_the_future", "whatever": "x" })),
            ),
        ];

        for (variant, mut event) in arms {
            Envelope::Receiver
                .unwrap_into_beats(&mut event, Some(Framing::Line), variant)
                .unwrap();
            assert_eq!(
                event.get_str("_source"),
                Some(variant),
                "{variant}: the variant id was stripped"
            );
        }
    }

    /// Splunk HEC writes no `_source`, so there is nothing to carry through --
    /// and inventing one would put a value into a routing field nothing owns.
    #[test]
    fn a_transport_that_wrote_no_variant_id_gains_none() {
        let mut event = Event::new(json!({
            "message": "a line",
            "sourcetype": "pan:traffic",
        }));

        Envelope::Receiver
            .unwrap_into_beats(&mut event, Some(Framing::Body), "splunk_hec")
            .unwrap();

        assert!(!event.has("_source"));
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
        for gone in ["version", "short_message", "severity"] {
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
    /// them may reach the payload the transform parses.
    ///
    /// `_source_fetcher` is the exception on the EVENT: it is what dfe-engine
    /// compiles a fetched source's routing rule against, so it has to come out
    /// the far side.
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

        let message = event.get_string("message").expect("payload in message");
        for key in FETCHER_KEYS {
            assert!(!message.contains(key), "{key} survived into {message}");
        }
        assert!(!event.has("_timestamp_fetcher"));
        assert!(!event.has("_timestamp_received"));
        assert_eq!(event.get_str("_source_fetcher"), Some("okta"));
    }

    /// A provider that hands back a bare line is already in the Beats shape,
    /// so re-serialising it would wrap the string in a second layer of JSON.
    ///
    /// The marker has to survive this branch of the unwrap as well as the
    /// re-serialising one.
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
        assert_eq!(event.get_str("_source_fetcher"), Some("somewhere"));
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

    /// With no parsed message at all, body framing falls back to the body cut
    /// from `_raw`, and leaves `message` unset when that header does not read.
    #[test]
    fn body_framing_cuts_the_body_from_raw_only_without_a_message() {
        let rows: &[(&str, Option<&str>, Option<&str>)] = &[
            (
                "PAN-OS has no tag, so the colon in the stamp is not one",
                Some(
                    "<11>Jan  5 05:38:59 PAN1.exampleCustomer.com 1,2015/01/05 05:38:58,0006C110285,THREAT,vulnerability",
                ),
                Some("1,2015/01/05 05:38:58,0006C110285,THREAT,vulnerability"),
            ),
            (
                "a 3164 tag with a pid",
                Some("<134>Oct 26 16:27:11 fw01 filterlog[123]: 5,,,1000000103,igb0,match"),
                Some("5,,,1000000103,igb0,match"),
            ),
            (
                "a 3164 tag without a pid",
                Some("<134>Oct 26 16:27:11 fw01 sshd: Accepted publickey"),
                Some("Accepted publickey"),
            ),
            (
                "5424 with structured data",
                Some(
                    r#"<165>1 2026-03-03T10:30:00.003+11:00 web01 app 1234 ID47 [exampleSDID@32473 iut="3" note="a \] b"][x@1 k="v"] An application event"#,
                ),
                Some("An application event"),
            ),
            (
                "5424 with nil structured data",
                Some("<165>1 2026-03-03T10:30:00Z web01 app - - - the 5424 body"),
                Some("the 5424 body"),
            ),
            (
                "Cisco stamp with a fraction and zone has no host",
                Some("<189>Jun  5 05:09:30.838 EST: %IP-4-DUPADDR: Duplicate address 172.16.1.216"),
                Some("%IP-4-DUPADDR: Duplicate address 172.16.1.216"),
            ),
            (
                "Cisco unsynchronised-clock mark",
                Some("<189>*Nov 14 08:30:56.699: %LINK-5-CHANGED: Interface Gi2, changed state"),
                Some("%LINK-5-CHANGED: Interface Gi2, changed state"),
            ),
            (
                "meraki has a space before any colon, so no tag",
                Some(
                    "<134>Oct  2 21:09:28 192.168.128.1 1 1696306168.009553886 satosugu events x url='https://a.b/'",
                ),
                Some("1 1696306168.009553886 satosugu events x url='https://a.b/'"),
            ),
            (
                "an RFC 3339 stamp with a 3164 host",
                Some("<134>2026-03-03T10:30:00+11:00 web01 nginx: GET /"),
                Some("GET /"),
            ),
            (
                "a colon with no space after it is not a tag",
                Some("<134>Oct 26 16:27:11 h tag:value rest"),
                Some("tag:value rest"),
            ),
            (
                "no PRI",
                Some("Oct 26 16:27:11 h body here"),
                Some("body here"),
            ),
            (
                "garbage leaves message unset",
                Some("<165>a full line"),
                None,
            ),
            (
                "a header with no body leaves it unset",
                Some("<134>Oct 26 16:27:11 host"),
                None,
            ),
            (
                "a 5424 header short of tokens",
                Some("<165>1 2026-03-03T10:30:00Z web01 app"),
                None,
            ),
            (
                "a bad 5424 stamp",
                Some("<165>1 yesterday web01 app - - - x"),
                None,
            ),
            (
                "an unterminated structured-data group",
                Some(r#"<165>1 - h a - - [id k="v" body"#),
                None,
            ),
            ("no _raw", None, None),
            ("a blank _raw", Some("  "), None),
        ];
        for (name, raw, want) in rows {
            let mut event = receiver_event();
            event.remove("message");
            if let Some(raw) = raw {
                event.set("_raw", json!(raw)).unwrap();
            }
            Envelope::Receiver
                .unwrap_into_beats(&mut event, Some(Framing::Body), "syslog")
                .unwrap();
            assert_eq!(event.get_str("message"), *want, "{name}");
            assert!(!event.has("_raw"), "{name}: _raw is a receiver key");
        }
    }

    /// The event dfe-receiver builds from `line`, parsed by the same
    /// `syslog_loose` it uses. Only the fields it writes when they are set.
    fn parsed_event(line: &str) -> Event {
        use syslog_loose::{ProcId, Variant};
        let m = syslog_loose::parse_message(line, Variant::Either);
        let mut fields = serde_json::Map::new();
        fields.insert("message".into(), json!(m.msg));
        fields.insert("_source".into(), json!("syslog"));
        for (key, value) in [
            ("hostname", m.hostname),
            ("appname", m.appname),
            ("msgid", m.msgid),
        ] {
            if let Some(value) = value {
                fields.insert(key.into(), json!(value));
            }
        }
        match m.procid {
            Some(ProcId::PID(pid)) => fields.insert("procid".into(), json!(pid.to_string())),
            Some(ProcId::Name(name)) => fields.insert("procid".into(), json!(name)),
            None => None,
        };
        Event::new(Value::Object(fields))
    }

    /// Body framing is fed from the parsed fields, with a wrong tag split put
    /// back, whether or not `_raw` is there. Line framing prefers `_raw` and
    /// otherwise rebuilds from the same repaired fields. The separator rule is
    /// the one `syslog_loose` was measured to leave: it drops one space or colon
    /// after the first token, so a tagless body rejoins with a single space.
    #[test]
    fn a_wrong_tag_split_is_repaired_from_the_parsed_fields() {
        let rows: &[(&str, &str, &str)] = &[
            (
                "tagless PAN-OS",
                "<11>Jan  5 05:38:59 PAN1.exampleCustomer.com 1,2015/01/05 05:38:58,0006C110285,THREAT,vuln,1",
                "1,2015/01/05 05:38:58,0006C110285,THREAT,vuln,1",
            ),
            (
                "PAN-OS with a colon later in the body",
                "<134>Jan  5 05:38:59 PAN1 1,2015/01/05 05:38:58,x,y: z",
                "1,2015/01/05 05:38:58,x,y: z",
            ),
            (
                "Cisco stamp tail left in the message",
                "<189>Jun  5 05:09:30.838 EST: %IP-4-DUPADDR: Duplicate address 172.16.1.216",
                "%IP-4-DUPADDR: Duplicate address 172.16.1.216",
            ),
            (
                "Cisco unsynchronised-clock line the parse gave up on",
                "<189>*Nov 14 08:30:56.699: %LINK-5-CHANGED: Interface Gi2, changed",
                "%LINK-5-CHANGED: Interface Gi2, changed",
            ),
            (
                "meraki",
                "<134>Oct  2 21:09:28 192.168.128.1 1 1696306168.009553886 satosugu events x url='https://a.b/'",
                "1 1696306168.009553886 satosugu events x url='https://a.b/'",
            ),
            (
                "a real tag with a pid is left alone",
                "<134>Oct 26 16:27:11 fw01 filterlog[123]: 5,,,1000000103,igb0",
                "5,,,1000000103,igb0",
            ),
            (
                "a real tag with a colon in the body",
                "<134>Oct 26 16:27:11 h sshd[1]: Accepted: ok",
                "Accepted: ok",
            ),
            (
                "a tag with no space after the colon",
                "<134>Oct 26 16:27:11 h tag:value rest",
                "value rest",
            ),
            (
                "a tagless first word that reads as a tag is left alone",
                "<134>Oct 26 16:27:11 h plain body with no colon",
                "body with no colon",
            ),
            (
                "a colon-ended CSV field rejoins with a space",
                "<134>Oct 26 16:27:11 h 1,2:3,4",
                "1,2 3,4",
            ),
        ];
        for (name, line, body) in rows {
            for raw in [None, Some(*line)] {
                let mut event = parsed_event(line);
                if let Some(raw) = raw {
                    event.set("_raw", json!(raw)).unwrap();
                }
                Envelope::Receiver
                    .unwrap_into_beats(&mut event, Some(Framing::Body), "syslog")
                    .unwrap();
                assert_eq!(event.get_str("message"), Some(*body), "{name}, raw {raw:?}");

                let mut event = parsed_event(line);
                if let Some(raw) = raw {
                    event.set("_raw", json!(raw)).unwrap();
                }
                Envelope::Receiver
                    .unwrap_into_beats(&mut event, Some(Framing::Line), "syslog")
                    .unwrap();
                let rebuilt = event.get_str("message").unwrap();
                match raw {
                    Some(raw) => assert_eq!(rebuilt, raw, "{name}: _raw wins on a line"),
                    None => assert!(rebuilt.ends_with(body), "{name}: {rebuilt}"),
                }
            }
        }
    }

    /// A routing rule overwrites `_source` with a catalogue label, so the
    /// transport is read off the event's own fields, and the label survives.
    #[test]
    fn a_relabelled_source_is_unwrapped_by_its_fields() {
        let syslog = |label: &str| {
            let mut event = parsed_event(
                "<11>Jan  5 05:38:59 PAN1 1,2015/01/05 05:38:58,0006C110285,THREAT,vuln,1",
            );
            event.set("_source", json!(label)).unwrap();
            event
        };
        let rows: Vec<(&str, Event, &str, &str, Option<&str>)> = vec![
            // name, event, path that must hold, its value, message afterwards
            (
                "cisco-asa syslog",
                syslog("cisco-asa"),
                "log.syslog.hostname",
                "PAN1",
                Some("1,2015/01/05 05:38:58,0006C110285,THREAT,vuln,1"),
            ),
            (
                "demo-syslog syslog",
                syslog("demo-syslog"),
                "log.syslog.hostname",
                "PAN1",
                Some("1,2015/01/05 05:38:58,0006C110285,THREAT,vuln,1"),
            ),
            (
                "relabelled gelf",
                Event::new(
                    json!({"_source": "cisco-asa", "message": "m", "host": "web01", "level": 6, "severity": "info"}),
                ),
                "host.name",
                "web01",
                Some("m"),
            ),
            (
                "relabelled otlp",
                Event::new(
                    json!({"_source": "cisco-asa", "body": "the line", "severity_text": "INFO", "trace_id": "abc"}),
                ),
                "log.level",
                "INFO",
                Some("the line"),
            ),
            (
                "an unknown label with no transport fields is left as it was",
                Event::new(json!({"_source": "mystery", "whatever": "x"})),
                "whatever",
                "x",
                None,
            ),
            (
                "one shared field does not pick a transport",
                Event::new(json!({"_source": "mystery", "job": "j"})),
                "job",
                "j",
                None,
            ),
        ];
        for (name, mut event, path, value, message) in rows {
            let label = event.get_str("_source").unwrap().to_owned();
            let detected = detect(&event);
            assert_eq!(detected.family, Envelope::Receiver, "{name}");
            Envelope::Receiver
                .unwrap_into_beats(&mut event, Some(Framing::Body), &detected.variant)
                .unwrap();
            assert_eq!(event.get_str(path), Some(value), "{name}");
            assert_eq!(event.get_str("message"), message, "{name}");
            assert_eq!(event.get_str("_source"), Some(label.as_str()), "{name}");
        }
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

    /// A Filebeat document as the receiver routes it: the shipper's whole
    /// envelope, plus the `_source` its routing rule stamped.
    fn shipped_through_the_receiver() -> Event {
        Event::new(json!({
            "agent": { "type": "filebeat", "version": "9.5.5", "name": "edge01" },
            "data_stream": { "dataset": "cisco_ios.log", "namespace": "branch01", "type": "logs" },
            "input": { "type": "filestream" },
            "log": { "file": { "path": "/var/log/cisco_ios.log" } },
            "message": "<189>29: Mar  3 10:30:00: %SYS-5-CONFIG_I: Configured from console",
            "_source": "el0a1b2c3d",
        }))
    }

    fn receiver_delivery(event: &Event, dataset: &'static str) -> Delivery {
        Delivery {
            envelope: Envelope::Receiver,
            variant: detect(event).variant,
            framing: Some(Framing::Line),
            dataset,
        }
    }

    /// Elasticsearch keeps what the shipper stamped, so a Beats or Agent
    /// document forwarded by the receiver must too: a replaced `agent.type`
    /// fills the meta schema's `agent_type` column with `dfe-receiver`.
    #[test]
    fn a_receiver_delivery_keeps_the_shippers_agent_input_and_namespace() {
        let mut event = shipped_through_the_receiver();
        receiver_delivery(&event, "cisco_ios.log")
            .apply(&mut event)
            .unwrap();

        assert_eq!(event.get_str("agent.type"), Some("filebeat"));
        assert_eq!(event.get_str("agent.version"), Some("9.5.5"));
        assert_eq!(event.get_str("input.type"), Some("filestream"));
        assert_eq!(event.get_str("data_stream.namespace"), Some("branch01"));
        assert_eq!(event.get_str("data_stream.dataset"), Some("cisco_ios.log"));
        assert_eq!(event.get_str("data_stream.type"), Some("logs"));
    }

    /// The dataset names the pipeline this transform runs, which a shipper's
    /// value cannot change, so it is written whatever arrived.
    #[test]
    fn a_receiver_delivery_writes_the_dataset_of_the_pipeline_that_ran() {
        let mut event = shipped_through_the_receiver();
        receiver_delivery(&event, "cisco_ios.other")
            .apply(&mut event)
            .unwrap();

        assert_eq!(
            event.get_str("data_stream.dataset"),
            Some("cisco_ios.other")
        );
    }

    /// A raw line the receiver wrapped carries no shipper metadata, so it still
    /// gets the delivery's own: the producer, this build, the transport, and
    /// Elasticsearch's default namespace.
    #[test]
    fn a_bare_line_through_the_receiver_still_gets_the_delivery_labels() {
        let mut event = receiver_event();
        receiver_delivery(&event, "panw.panos")
            .apply(&mut event)
            .unwrap();

        assert_eq!(event.get_str("agent.type"), Some("dfe-receiver"));
        assert_eq!(
            event.get_str("agent.version"),
            Some(env!("CARGO_PKG_VERSION"))
        );
        assert_eq!(event.get_str("input.type"), Some("syslog"));
        assert_eq!(event.get_str("data_stream.namespace"), Some("default"));
        assert_eq!(event.get_str("data_stream.dataset"), Some("panw.panos"));
        assert_eq!(event.get_str("data_stream.type"), Some("logs"));
    }

    /// A shipper that names its agent without a version gets none of ours:
    /// this build's version beside a shipper's type would describe neither.
    #[test]
    fn a_shipper_agent_without_a_version_gains_none_of_ours() {
        let mut event = shipped_through_the_receiver();
        event.remove("agent.version");
        receiver_delivery(&event, "cisco_ios.log")
            .apply(&mut event)
            .unwrap();

        assert_eq!(event.get_str("agent.type"), Some("filebeat"));
        assert!(!event.has("agent.version"), "{:?}", event.get("agent"));
    }

    /// A shipper field holding null is no value, which is how an ingest `set`
    /// with `override: false` reads it.
    #[test]
    fn a_null_shipper_field_is_filled() {
        let mut event = shipped_through_the_receiver();
        event.set("data_stream.namespace", Value::Null).unwrap();
        receiver_delivery(&event, "cisco_ios.log")
            .apply(&mut event)
            .unwrap();

        assert_eq!(event.get_str("data_stream.namespace"), Some("default"));
    }

    /// Run `record` through a fetcher delivery for `dataset` and return the
    /// payload the transform will parse.
    fn dressed_payload(dataset: &'static str, record: Value) -> Value {
        let mut event = Event::new(record);
        Delivery {
            envelope: Envelope::Fetcher,
            variant: detect(&event).variant,
            framing: None,
            dataset,
        }
        .apply(&mut event)
        .unwrap();
        serde_json::from_str(event.get_str("message").expect("payload in message")).unwrap()
    }

    /// A Graph provisioning record names its nested objects `id` and
    /// `displayName`, and the Event Hub form the pipeline renames from spells
    /// them `Id` and `Name`. The envelope fields are derived from the record.
    #[test]
    fn a_graph_provisioning_record_gets_the_event_hub_spelling() {
        let payload = dressed_payload(
            "azure.provisioning",
            json!({
                "id": "75b5b0ae",
                "activityDateTime": "2019-05-04T03:00:54Z",
                "tenantId": "74beb175",
                "changeId": "b6502552",
                "durationInMilliseconds": 3236,
                "provisioningStatusInfo": { "status": "success" },
                "servicePrincipal": { "id": "6cc35b93", "displayName": "Contoso" },
                "initiatedBy": { "initiatingType": "system", "id": "", "displayName": "Service" },
                "sourceIdentity": { "Id": "kept", "id": "dropped-by-the-existing-key" },
                "_source_fetcher": "azure.entra_provisioning",
                "_timestamp_received": 1_771_459_200_000_u64,
            }),
        );
        assert_eq!(payload["category"], "ProvisioningLogs");
        assert_eq!(payload["operationName"], "Provisioning activity");
        assert_eq!(payload["time"], "2019-05-04T03:00:54Z");
        assert_eq!(payload["correlationId"], "b6502552");
        assert_eq!(payload["tenantId"], "74beb175");
        assert_eq!(payload["durationMs"], 3236);
        assert_eq!(payload["resultType"], "success");
        let properties = &payload["properties"];
        assert_eq!(properties["servicePrincipal"]["Id"], "6cc35b93");
        assert_eq!(properties["servicePrincipal"]["Name"], "Contoso");
        assert!(properties["servicePrincipal"].get("id").is_none());
        assert_eq!(properties["initiatedBy"]["Type"], "system");
        assert_eq!(properties["initiatedBy"]["Name"], "Service");
        assert_eq!(properties["sourceIdentity"]["Id"], "kept");
    }

    /// dfe-fetcher copies an Entra event's time to `timestamp` for the common
    /// header. The Graph resource has no such field, so it stays out of
    /// `properties`, but a `timestamp` that is not that copy is the
    /// provider's and stays.
    #[test]
    fn the_fetchers_timestamp_copy_stays_out_of_the_graph_resource() {
        let stamped = dressed_payload(
            "azure.auditlogs",
            json!({
                "activityDateTime": "2018-01-09T21:20:02Z",
                "timestamp": "2018-01-09T21:20:02Z",
            }),
        );
        assert!(stamped["properties"].get("timestamp").is_none());

        let provider = dressed_payload(
            "azure.auditlogs",
            json!({
                "activityDateTime": "2018-01-09T21:20:02Z",
                "timestamp": "provider's own",
            }),
        );
        assert_eq!(provider["properties"]["timestamp"], "provider's own");
    }

    /// The fetcher moves the provider's whole payload into `message`, so a
    /// provider field that happens to be called `agent` or `data_stream` never
    /// stands in for the delivery's labels.
    #[test]
    fn a_fetcher_delivery_still_gets_its_own_labels() {
        let mut event = Event::new(json!({
            "agent": { "type": "provider-agent" },
            "data_stream": { "namespace": "provider-ns" },
            "eventType": "user.session.start",
            "_timestamp_fetcher": 1_771_459_200_000_u64,
            "_timestamp_received": 1_771_459_200_000_u64,
            "_source_fetcher": "okta.system_log",
        }));
        Delivery {
            envelope: Envelope::Fetcher,
            variant: detect(&event).variant,
            framing: None,
            dataset: "okta.system",
        }
        .apply(&mut event)
        .unwrap();

        assert_eq!(event.get_str("agent.type"), Some("dfe-fetcher"));
        assert_eq!(
            event.get_str("agent.version"),
            Some(env!("CARGO_PKG_VERSION"))
        );
        assert_eq!(event.get_str("input.type"), Some("okta.system_log"));
        assert_eq!(event.get_str("data_stream.namespace"), Some("default"));
        assert_eq!(event.get_str("data_stream.dataset"), Some("okta.system"));
        assert_eq!(
            event.get_str("event.ingested"),
            Some("2026-02-19T00:00:00.000Z")
        );
        let payload: Value =
            serde_json::from_str(event.get_str("message").expect("payload in message")).unwrap();
        assert_eq!(
            payload.pointer("/agent/type").and_then(Value::as_str),
            Some("provider-agent")
        );
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
