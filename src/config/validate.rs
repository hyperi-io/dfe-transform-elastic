// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Refusing a configuration that cannot work.
//!
//! Empty is not invalid. An instance nothing has configured yet idles, which
//! [`Config::work_state`] decides, so the checks here split in two: a value
//! someone SET out of range is refused whether or not there is work, and the
//! rest run only once the configuration names work.

use super::{Config, PIPELINE_MESSAGE_MAX_BYTES};

/// The smallest inbound fetch budget librdkafka can honour here.
const MIN_BATCH_BYTES: usize = 4 * 1024 * 1024;

/// The largest inbound fetch budget, bounded by what `i32` can carry into
/// librdkafka.
const MAX_BATCH_BYTES: usize = 256 * 1024 * 1024;

impl Config {
    /// Reject a configuration that is structurally wrong.
    ///
    /// Empty is NOT wrong: an instance nothing has configured yet is valid and
    /// idles, which [`Config::work_state`] decides. This refuses only what is
    /// present and cannot work.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Config`] when a numeric value is out of range,
    /// or -- once the configuration names work -- when a value that work needs
    /// is missing or names a source this build does not carry.
    pub fn validate(&self) -> crate::Result<()> {
        if self.source.batch_size == 0 {
            return Err(crate::Error::Config("source.batch_size is zero".into()));
        }
        if self.source.max_batch_bytes < MIN_BATCH_BYTES
            || self.source.max_batch_bytes > MAX_BATCH_BYTES
        {
            return Err(crate::Error::Config(format!(
                "source.max_batch_bytes must be between {MIN_BATCH_BYTES} and {MAX_BATCH_BYTES}, \
                 got {}",
                self.source.max_batch_bytes
            )));
        }
        // A budget under one event's worth drops every event as oversize, and
        // one over the stack's record ceiling admits records no layer carries.
        if self.sink.max_message_bytes < 4096
            || self.sink.max_message_bytes > PIPELINE_MESSAGE_MAX_BYTES
        {
            return Err(crate::Error::Config(format!(
                "sink.max_message_bytes must be between 4096 and {PIPELINE_MESSAGE_MAX_BYTES}, \
                 got {}",
                self.sink.max_message_bytes
            )));
        }
        // Emptiness is not invalidity. An instance deployed before anything
        // names a source idles instead of refusing, so every check below runs
        // only once the configuration actually gives the transform work.
        if self.work_state().is_idle() {
            return Ok(());
        }

        // Each transport needs different things, so requiring the bus's fields
        // on the direct path would refuse a deployment that can work.
        if self.source.transport.is_direct() {
            if self.source.listen.parse::<std::net::SocketAddr>().is_err() {
                return Err(crate::Error::Config(format!(
                    "source.listen must be a bind address on the direct transport, got '{}'",
                    self.source.listen
                )));
            }
        } else {
            if self.source.brokers.is_empty() {
                return Err(crate::Error::Config("source.brokers is empty".into()));
            }
            if self.source.group_id.trim().is_empty() {
                return Err(crate::Error::Config("source.group_id is empty".into()));
            }
        }

        // On direct the topic is the routing key every push carries, and a push
        // without one lands under the downstream loader's default topic.
        if self.sink.topic.trim().is_empty() {
            return Err(crate::Error::Config(
                "sink.topic is empty; it names where the events land on both transports".into(),
            ));
        }
        if self.sink.transport.is_direct() && self.sink.endpoint.trim().is_empty() {
            return Err(crate::Error::Config(
                "sink.endpoint is empty on the direct transport".into(),
            ));
        }

        if self.source.mode == super::Mode::Agentless {
            return self.validate_agentless();
        }

        let intake = crate::registry::intake(&self.source.name)
            .ok_or_else(|| crate::Error::UnknownSource(self.source.name.clone()))?;

        // An envelope this source never arrives in would unwrap a shape that
        // is not there and quietly emit nothing useful. Refuse it at startup
        // rather than at the first batch. Only a PINNED envelope can be checked
        // here: under `auto` there is no event yet to detect from, so the same
        // mismatch is counted per batch instead.
        if let Some(pinned) = self.source.envelope.pinned()
            && !intake.accepts(pinned)
        {
            let name = |e: &crate::envelope::Envelope| format!("{e:?}").to_lowercase();
            let accepts: Vec<String> = intake.envelopes.iter().map(name).collect();
            return Err(crate::Error::Config(format!(
                "source '{}' cannot arrive over {}; it accepts {}, and the {} \
                 envelope applies to: {}",
                self.source.name,
                name(&pinned),
                accepts.join(" and "),
                name(&pinned),
                crate::registry::sources_accepting(pinned)
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        }

        Ok(())
    }

    /// The checks agentless mode adds: the source is detected, so none is named.
    fn validate_agentless(&self) -> crate::Result<()> {
        use super::Unmatched;
        let source = &self.source;
        let refuse = |message: String| Err(crate::Error::Config(message));

        if !source.name.trim().is_empty() {
            return refuse(format!(
                "source.name is '{}' but source.mode is agentless, which picks the source per \
                 event. Clear one of them",
                source.name
            ));
        }
        if let Some(pinned) = source.envelope.pinned()
            && pinned != crate::envelope::Envelope::Receiver
        {
            return refuse(
                "source.envelope must be auto or receiver in agentless mode, which reads \
                 dfe-receiver syslog"
                    .into(),
            );
        }
        crate::detect::Detector::new(&source.detect.candidates)
            .map_err(|e| crate::Error::Config(e.to_string()))?;

        let fallback = source.detect.fallback.as_str();
        match (source.detect.unmatched, fallback.is_empty()) {
            (Unmatched::Fallback, true) => {
                refuse("source.detect.fallback is required when unmatched is fallback".into())
            }
            (Unmatched::Fallback, false) => {
                crate::registry::lookup(fallback)
                    .ok_or_else(|| crate::Error::UnknownSource(fallback.to_owned()))?;
                if crate::detect::label(fallback).is_none() {
                    return refuse(format!("source.detect.fallback '{fallback}' has no catalogue label"));
                }
                Ok(())
            }
            (_, false) => refuse(
                "source.detect.fallback is set but unmatched is not fallback, so it would never run"
                    .into(),
            ),
            (_, true) => Ok(()),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::super::valid;
    use super::PIPELINE_MESSAGE_MAX_BYTES;

    #[test]
    fn accepts_a_complete_config() {
        assert!(valid().validate().is_ok());
    }

    /// Empty is not invalid. A transform with no topics has nothing to consume,
    /// so it idles until a configuration names work instead of refusing.
    #[test]
    fn empty_topics_idle_rather_than_refuse() {
        let mut c = valid();
        c.source.topics.clear();
        assert!(c.validate().is_ok(), "empty topics must not refuse");
        assert!(c.work_state().is_idle());
    }

    /// The same for a source nothing has named yet: the engine writes the
    /// variant only once the deployment carries one.
    #[test]
    fn an_unnamed_source_idles() {
        let mut c = valid();
        c.source.name.clear();
        assert!(c.validate().is_ok());
        assert!(c.work_state().is_idle());
    }

    /// A value someone SET being out of range is structurally wrong whether or
    /// not there is work yet, so the range checks run ahead of the idle gate.
    #[test]
    fn a_structural_fault_refuses_even_when_idle() {
        let mut c = valid();
        c.source.topics.clear();
        c.sink.max_message_bytes = PIPELINE_MESSAGE_MAX_BYTES + 1;
        assert!(c.validate().is_err());
    }

    /// The bus's fields are meaningless on the direct transport, so requiring
    /// them would refuse a deployment that works.
    #[test]
    fn a_direct_source_needs_no_brokers_or_group() {
        let mut c = valid();
        c.source.transport = crate::config::Transport::Direct;
        c.source.brokers.clear();
        c.source.group_id.clear();
        assert!(c.validate().is_ok());
    }

    #[test]
    fn a_direct_source_refuses_a_listen_that_is_not_an_address() {
        let mut c = valid();
        c.source.transport = crate::config::Transport::Direct;
        c.source.listen = "not-an-address".into();
        let message = c
            .validate()
            .expect_err("a bad bind address must refuse")
            .to_string();
        assert!(message.contains("source.listen"), "{message}");
    }

    #[test]
    fn a_direct_sink_refuses_an_empty_endpoint() {
        let mut c = valid();
        c.sink.transport = crate::config::Transport::Direct;
        c.sink.endpoint.clear();
        let message = c
            .validate()
            .expect_err("an empty endpoint must refuse")
            .to_string();
        assert!(message.contains("sink.endpoint"), "{message}");
    }

    /// A direct push carries `sink.topic` as its routing key, so an empty one
    /// would land every event under the loader's default topic.
    #[test]
    fn a_direct_sink_refuses_an_empty_topic() {
        let mut c = valid();
        c.sink.transport = crate::config::Transport::Direct;
        c.sink.topic.clear();
        let message = c
            .validate()
            .expect_err("an empty topic must refuse on direct")
            .to_string();
        assert!(message.contains("sink.topic"), "{message}");
    }

    #[test]
    fn a_bus_sink_refuses_an_empty_topic() {
        let mut c = valid();
        c.sink.topic = "  ".into();
        assert!(c.validate().is_err());
    }

    /// The bus still needs its own fields, so the split must not relax them.
    #[test]
    fn the_bus_still_refuses_missing_brokers() {
        let mut c = valid();
        c.source.brokers.clear();
        assert!(c.validate().is_err());
    }

    #[test]
    fn rejects_zero_batch_size() {
        let mut c = valid();
        c.source.batch_size = 0;
        assert!(c.validate().is_err());
    }

    /// The budget bounds one record, so a value above the stack's record
    /// ceiling admits records no broker, topic or consumer carries.
    #[test]
    fn rejects_a_message_budget_the_stack_cannot_carry() {
        let mut c = valid();
        c.sink.max_message_bytes = PIPELINE_MESSAGE_MAX_BYTES + 1;
        assert!(c.validate().is_err());

        c.sink.max_message_bytes = 512;
        assert!(c.validate().is_err());
    }

    /// Anything up to the stack's ceiling is a budget the chain carries, so a
    /// multi-megabyte value an operator sets is honoured.
    #[test]
    fn accepts_a_message_budget_up_to_the_stack_ceiling() {
        let mut c = valid();
        for budget in [4_000_000, PIPELINE_MESSAGE_MAX_BYTES] {
            c.sink.max_message_bytes = budget;
            assert!(c.validate().is_ok(), "{budget} was refused");
        }
    }

    #[test]
    fn rejects_an_unregistered_source() {
        let mut c = valid();
        c.source.name = "filebeat.nosuchthing".into();
        assert!(c.validate().is_err());
    }

    /// `auto` cannot be checked against the intake at startup, so a source that
    /// refuses a pinned envelope must still validate when nothing is pinned.
    #[test]
    fn auto_validates_on_a_source_that_refuses_a_pinned_envelope() {
        let mut c = valid();
        c.source.envelope = crate::envelope::EnvelopeSetting::Receiver;
        assert!(c.validate().is_err(), "okta pinned to receiver is refused");

        c.source.envelope = crate::envelope::EnvelopeSetting::Auto;
        assert!(c.validate().is_ok());
    }

    #[test]
    fn accepts_the_receiver_envelope_on_a_pushed_source() {
        let mut c = valid();
        c.source.name = "filebeat.cisco_ios.default".into();
        c.source.envelope = crate::envelope::EnvelopeSetting::Receiver;
        assert!(c.validate().is_ok());
    }

    /// okta is pulled from an API. Asking for it over syslog is a config
    /// error, not a silent no-op at the first batch.
    #[test]
    fn rejects_the_receiver_envelope_on_a_fetched_source() {
        let mut c = valid();
        c.source.envelope = crate::envelope::EnvelopeSetting::Receiver;

        let err = c.validate().expect_err("okta over syslog must be rejected");
        let message = err.to_string();
        assert!(message.contains("cannot arrive over receiver"), "{message}");
        // The error must name both what this source DOES take and what would.
        assert!(message.contains("beats and fetcher"), "{message}");
        assert!(message.contains("filebeat.cisco_ios.default"), "{message}");
    }

    /// The other direction: a device pushes `cisco_ios`, so there is nothing for
    /// dfe-fetcher to pull and the fetcher envelope is refused.
    #[test]
    fn rejects_the_fetcher_envelope_on_a_pushed_source() {
        let mut c = valid();
        c.source.name = "filebeat.cisco_ios.default".into();
        c.source.envelope = crate::envelope::EnvelopeSetting::Fetcher;

        let message = c
            .validate()
            .expect_err("cisco_ios over fetcher must be rejected")
            .to_string();
        assert!(message.contains("cannot arrive over fetcher"), "{message}");
        assert!(message.contains("beats and receiver"), "{message}");
        assert!(message.contains("filebeat.okta.default"), "{message}");
    }

    /// dfe-fetcher can obtain what Elastic's `httpjson` input obtains, so okta
    /// accepts it and the config is valid.
    #[test]
    fn accepts_the_fetcher_envelope_on_a_pulled_source() {
        let mut c = valid();
        c.source.envelope = crate::envelope::EnvelopeSetting::Fetcher;
        assert!(c.validate().is_ok());
    }

    /// Each row edits an agentless config and names the text its refusal must
    /// carry, or `None` where it must pass.
    #[test]
    fn agentless_mode_is_checked_as_a_whole() {
        use crate::config::{Mode, Unmatched};
        type Edit = fn(&mut crate::config::Config);
        let rows: &[(&str, Edit, Option<&str>)] = &[
            ("no name, defaults", |_| {}, None),
            (
                "a name is refused",
                |c| c.source.name = "filebeat.okta.default".into(),
                Some("source.name"),
            ),
            (
                "a pinned beats envelope",
                |c| c.source.envelope = crate::envelope::EnvelopeSetting::Beats,
                Some("source.envelope"),
            ),
            (
                "a pinned receiver envelope",
                |c| c.source.envelope = crate::envelope::EnvelopeSetting::Receiver,
                None,
            ),
            (
                "an unknown candidate",
                |c| c.source.detect.candidates = vec!["filebeat.nope.default".into()],
                Some("candidates"),
            ),
            (
                "a known candidate",
                |c| c.source.detect.candidates = vec!["filebeat.cisco_asa.default".into()],
                None,
            ),
            (
                "fallback needs a source",
                |c| c.source.detect.unmatched = Unmatched::Fallback,
                Some("source.detect.fallback is required"),
            ),
            (
                "fallback names an unregistered source",
                |c| {
                    c.source.detect.unmatched = Unmatched::Fallback;
                    c.source.detect.fallback = "filebeat.nope.default".into();
                },
                Some("unknown source"),
            ),
            (
                "fallback names a source",
                |c| {
                    c.source.detect.unmatched = Unmatched::Fallback;
                    c.source.detect.fallback = "filebeat.cisco_ios.default".into();
                },
                None,
            ),
            (
                "a fallback that would never run",
                |c| c.source.detect.fallback = "filebeat.cisco_ios.default".into(),
                Some("would never run"),
            ),
            (
                "stop needs nothing else",
                |c| c.source.detect.unmatched = Unmatched::Stop,
                None,
            ),
        ];
        for (name, edit, refusal) in rows {
            let mut c = valid();
            c.source.mode = Mode::Agentless;
            c.source.name.clear();
            c.source.envelope = crate::envelope::EnvelopeSetting::Auto;
            edit(&mut c);
            assert!(
                !c.work_state().is_idle(),
                "{name}: agentless with topics is work"
            );
            match (c.validate(), refusal) {
                (Ok(()), None) => {}
                (Err(e), Some(text)) => assert!(e.to_string().contains(text), "{name}: {e}"),
                (got, want) => panic!("{name}: got {got:?}, wanted {want:?}"),
            }
        }
    }

    #[test]
    fn agentless_keys_load_from_yaml() {
        let parsed: crate::config::Config = scalo::yaml::from_str(
            "source:\n  mode: agentless\n  topics: [in]\n  group_id: g\n  brokers: [b:9092]\n  \
             detect:\n    unmatched: fallback\n    fallback: filebeat.cisco_ios.default\n    \
             candidates: [filebeat.cisco_asa.default]\nsink:\n  topic: out\n",
            "config.yaml",
            scalo::yaml::Profile::Config,
        )
        .expect("agentless keys parse");
        assert_eq!(parsed.source.mode, crate::config::Mode::Agentless);
        assert_eq!(
            parsed.source.detect.unmatched,
            crate::config::Unmatched::Fallback
        );
        assert_eq!(
            parsed.source.detect.candidates,
            ["filebeat.cisco_asa.default"]
        );
        assert!(parsed.validate().is_ok());
    }
}
