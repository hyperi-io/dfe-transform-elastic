// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Agentless mode: the source is read off each event instead of configured.
//!
//! A batch is split by detected source, each group runs through its source's
//! transform exactly as agent mode would, and every output event is stamped
//! `_source` with the source's catalogue label so dfe-loader routes it to that
//! source's table. Events keep their arrival order within a group, and groups
//! are emitted in source order.

use dfe_runtime::{Event, Transform};
use metrics::Counter;
use serde_json::Value;

use crate::config::{Mode, SourceConfig, Unmatched};
use crate::detect::{Detector, detectable, label};
use crate::envelope::{Envelope, EnvelopeSetting, Resolver, detect as detect_envelope};
use crate::metrics::TransformMetrics;
use crate::pipeline::{BatchOutcome, EnvelopeOutcome, transform_batch_resolved};
use crate::registry::{self, Framing};

/// Unmatched events under `detect.unmatched: stop`.
#[derive(Debug, thiserror::Error)]
#[error("{count} events matched no source marker and source.detect.unmatched is stop: {sample}")]
pub struct Stopped {
    /// How many events in the block matched nothing.
    pub count: usize,
    /// The start of the first one's text.
    pub sample: String,
}

/// One source's transform and how its output is labelled.
struct Target {
    transform: &'static (dyn Transform + Sync),
    resolver: Resolver,
    label: &'static str,
    detected: Counter,
}

impl Target {
    fn new(source: &'static str, label: &'static str) -> crate::Result<Self> {
        let unknown = || crate::Error::UnknownSource(source.to_owned());
        Ok(Self {
            transform: registry::lookup(source).ok_or_else(unknown)?,
            resolver: Resolver::new(
                EnvelopeSetting::Auto,
                registry::intake(source).ok_or_else(unknown)?,
                registry::dataset(source).ok_or_else(unknown)?,
            ),
            label,
            detected: metrics::counter!("events_detected_total", "source" => label),
        })
    }

    /// Run the group through this source's transform and stamp the label.
    fn run(&self, events: Vec<Event>) -> (Vec<Event>, BatchOutcome, EnvelopeOutcome) {
        let (mut out, outcome, envelopes) =
            transform_batch_resolved(self.transform, &self.resolver, events);
        for event in &mut out {
            if let Some(fields) = event.as_value_mut().as_object_mut() {
                fields.insert("_source".into(), Value::from(self.label));
            }
        }
        (out, outcome, envelopes)
    }
}

/// The detector and the transforms it can hand an event to.
pub struct Agentless {
    detector: Detector,
    /// One slot per [`detectable`] source, empty for one `candidates` excludes.
    targets: Vec<Option<Target>>,
    unmatched: Unmatched,
    fallback: Option<Target>,
    events_unmatched: Counter,
    events_fallback: Counter,
}

impl Agentless {
    /// The agentless stage for `config`, or `None` in agent mode.
    ///
    /// # Errors
    ///
    /// When a candidate or the fallback names no registered, labelled source.
    pub fn new(source: &SourceConfig, metrics: &TransformMetrics) -> crate::Result<Option<Self>> {
        if source.mode != Mode::Agentless {
            return Ok(None);
        }
        let detector = Detector::new(&source.detect.candidates)
            .map_err(|e| crate::Error::Config(e.to_string()))?;

        let mut targets = Vec::with_capacity(detectable().len());
        for &name in detectable() {
            let wanted = source.detect.candidates.is_empty()
                || source.detect.candidates.iter().any(|c| c == name);
            targets.push(if wanted {
                let label = label(name).ok_or_else(|| {
                    crate::Error::Config(format!("{name} has no catalogue label"))
                })?;
                Some(Target::new(name, label)?)
            } else {
                None
            });
        }

        let fallback = match source.detect.unmatched {
            Unmatched::Fallback => {
                let name = registry::sources()
                    .find(|s| *s == source.detect.fallback)
                    .ok_or_else(|| crate::Error::UnknownSource(source.detect.fallback.clone()))?;
                let label = label(name).ok_or_else(|| {
                    crate::Error::Config(format!("{name} has no catalogue label"))
                })?;
                Some(Target::new(name, label)?)
            }
            Unmatched::Passthrough | Unmatched::Stop => None,
        };

        Ok(Some(Self {
            detector,
            targets,
            unmatched: source.detect.unmatched,
            fallback,
            events_unmatched: metrics.events_unmatched.clone(),
            events_fallback: metrics.events_fallback.clone(),
        }))
    }

    /// Detect, group, transform and label one batch.
    ///
    /// # Errors
    ///
    /// [`Stopped`] when `unmatched` is `stop` and an event matched nothing.
    pub fn transform(
        &self,
        events: Vec<Event>,
    ) -> Result<(Vec<Event>, BatchOutcome, EnvelopeOutcome), Stopped> {
        let mut scratch = String::new();
        let mut groups: Vec<Vec<Event>> = (0..self.targets.len()).map(|_| Vec::new()).collect();
        let mut unmatched = Vec::new();
        for event in events {
            // A fetched event carries the provider's own payload, which no
            // syslog marker describes.
            let found = if is_fetched(&event) {
                None
            } else {
                self.detector.detect_event(&event, &mut scratch)
            };
            match found {
                Some(i) => groups[i].push(event),
                None => unmatched.push(event),
            }
        }

        let mut out = Vec::new();
        let mut outcome = BatchOutcome::default();
        let mut envelopes = EnvelopeOutcome::default();
        let mut add = |(events, o, e): (Vec<Event>, BatchOutcome, EnvelopeOutcome)| {
            out.extend(events);
            outcome.emitted += o.emitted;
            outcome.dropped += o.dropped;
            outcome.errored += o.errored;
            envelopes.first = envelopes.first.take().or(e.first);
            envelopes.contradicted |= e.contradicted;
            envelopes.unaccepted |= e.unaccepted;
        };

        for (group, target) in groups.into_iter().zip(&self.targets) {
            if let (false, Some(target)) = (group.is_empty(), target) {
                target.detected.increment(group.len() as u64);
                add(target.run(group));
            }
        }
        if !unmatched.is_empty() {
            add(self.unmatched(unmatched)?);
        }
        Ok((out, outcome, envelopes))
    }

    /// Events no marker claimed, handled as `detect.unmatched` says.
    #[cold]
    #[inline(never)]
    fn unmatched(
        &self,
        mut events: Vec<Event>,
    ) -> Result<(Vec<Event>, BatchOutcome, EnvelopeOutcome), Stopped> {
        self.events_unmatched.increment(events.len() as u64);
        match (&self.fallback, self.unmatched) {
            (Some(target), _) => {
                self.events_fallback.increment(events.len() as u64);
                Ok(target.run(events))
            }
            (None, Unmatched::Stop) => Err(Stopped {
                count: events.len(),
                sample: sample(&events[0]),
            }),
            (None, _) => {
                let mut outcome = BatchOutcome::default();
                events.retain_mut(|event| {
                    if unwrap_receiver(event).is_ok() {
                        outcome.emitted += 1;
                        true
                    } else {
                        outcome.errored += 1;
                        false
                    }
                });
                Ok((events, outcome, EnvelopeOutcome::default()))
            }
        }
    }
}

fn is_fetched(event: &Event) -> bool {
    event
        .as_value()
        .as_object()
        .is_some_and(|fields| fields.contains_key("_source_fetcher"))
}

/// Lift a receiver delivery to the Beats layout and drop its `_source`, so the
/// loader files the row under the sink topic's default table. Any other family
/// is left exactly as it arrived.
fn unwrap_receiver(event: &mut Event) -> crate::Result<()> {
    let detected = detect_envelope(event);
    if detected.family != Envelope::Receiver {
        return Ok(());
    }
    Envelope::Receiver.unwrap_into_beats(event, Some(Framing::Body), &detected.variant)?;
    event.remove("_source");
    Ok(())
}

/// The head of what the event says, cut to the bound the log applies to a grok miss.
fn sample(event: &Event) -> String {
    let text = event
        .get_str("_raw")
        .or_else(|| event.get_str("message"))
        .unwrap_or_default();
    crate::pipeline::elided(text).into_owned()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use scalo::metrics::MetricsManager;
    use serde_json::json;

    use super::*;
    use crate::config::DetectConfig;

    const ASA: &str = "filebeat.cisco_asa.default";
    const IOS: &str = "filebeat.cisco_ios.default";

    fn stage(unmatched: Unmatched, fallback: &str, candidates: &[&str]) -> Agentless {
        let config = SourceConfig {
            mode: Mode::Agentless,
            detect: DetectConfig {
                unmatched,
                fallback: fallback.into(),
                candidates: candidates.iter().map(|c| (*c).to_owned()).collect(),
            },
            ..SourceConfig::default()
        };
        let metrics = TransformMetrics::register(&MetricsManager::new("test"), "0", "0");
        Agentless::new(&config, &metrics)
            .expect("the config builds")
            .expect("agentless mode")
    }

    /// dfe-receiver's syslog output for one line.
    fn receiver(body: &str) -> Event {
        Event::new(json!({
            "_source": "syslog",
            "message": body,
            "facility": "local4",
            "severity": "notice",
            "timestamp": "2026-03-03T10:30:00+11:00",
            "hostname": "fw01",
            "appname": null,
            "procid": null,
            "msgid": null,
            "structured_data": [],
        }))
    }

    const ASA_LINE: &str = "%ASA-6-302013: Built inbound TCP connection 1 for outside:10.0.0.1/80 \
                            (10.0.0.1/80) to inside:10.0.0.2/1234 (10.0.0.2/1234)";
    const IOS_LINE: &str = "%SYS-5-CONFIG_I: Configured from console by admin on vty0";
    const JUNK: &str = "nothing here names a device";

    fn sources_of(events: &[Event]) -> Vec<Option<String>> {
        let mut found: Vec<_> = events
            .iter()
            .map(|e| e.get_str("_source").map(str::to_owned))
            .collect();
        found.sort();
        found
    }

    /// Each row is a stage, three receiver lines, and the `_source` each output
    /// event must carry. `None` is an event left unlabelled for the loader's
    /// default table, and `Err` is a stop.
    #[test]
    fn groups_stamps_and_handles_the_unmatched() {
        let asa = label(ASA).map(str::to_owned);
        let ios = label(IOS).map(str::to_owned);
        let rows = [
            (
                "passthrough leaves the stranger unlabelled",
                stage(Unmatched::Passthrough, "", &[]),
                Ok(vec![None, asa.clone(), ios.clone()]),
            ),
            (
                "fallback labels the stranger as the fallback",
                stage(Unmatched::Fallback, IOS, &[]),
                Ok(vec![asa.clone(), ios.clone(), ios.clone()]),
            ),
            (
                "stop refuses the block",
                stage(Unmatched::Stop, "", &[]),
                Err(()),
            ),
            (
                "a candidates list sends ASA lines to the IOS catch-all",
                stage(Unmatched::Passthrough, "", &[IOS]),
                Ok(vec![None, ios.clone(), ios.clone()]),
            ),
        ];
        for (name, agentless, want) in rows {
            let events = vec![receiver(ASA_LINE), receiver(IOS_LINE), receiver(JUNK)];
            let got = agentless.transform(events);
            match (got, want) {
                (Ok((out, outcome, _)), Ok(mut want)) => {
                    want.sort();
                    assert_eq!(sources_of(&out), want, "{name}");
                    assert_eq!(outcome.total(), 3, "{name}");
                }
                (Err(stopped), Err(())) => assert_eq!(stopped.count, 1, "{name}"),
                (got, want) => panic!("{name}: got {:?}, wanted {want:?}", got.map(|g| g.1)),
            }
        }
    }

    #[test]
    fn a_fetched_event_is_unmatched_and_untouched() {
        let agentless = stage(Unmatched::Passthrough, "", &[]);
        let fetched = Event::new(json!({
            "_source": "okta",
            "_source_fetcher": "okta.system",
            "message": ASA_LINE,
        }));
        let (out, outcome, _) = agentless.transform(vec![fetched]).expect("passthrough");
        assert_eq!(outcome.emitted, 1);
        assert_eq!(out[0].get_str("_source"), Some("okta"));
        assert_eq!(out[0].get_str("_source_fetcher"), Some("okta.system"));
    }
}
