// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Names the source of a syslog line by what is written in it.
//!
//! Agentless mode serves one dfe-receiver stream that carries many device
//! types, so each event is matched against the marker table in `signatures`
//! and the winner's transform runs. One Aho-Corasick pass over the line finds
//! every literal at once. No regex runs and nothing is allocated per event.
//!
//! Cost: 2,080 ns an event (481K events a second on one core) over 20,000
//! compat-corpus events of 718 bytes on average, measured on desktop-derek (AMD
//! EPYC 7302) with the bench profile. It is the overlapping DFA walk over every
//! byte, because more than 100 patterns rule out aho-corasick's Teddy prefilter,
//! and not the hit count (`%` hits 0.20 a line and `]: ` 0.17). Re-derive with
//! `cargo bench --bench detect`.

mod labels;
mod signatures;
mod verify;

use std::collections::HashMap;
use std::sync::LazyLock;

use aho_corasick::{AhoCorasick, AhoCorasickKind, MatchKind};
use dfe_runtime::event::Event;
use serde_json::Value;

pub use labels::label;
use signatures::{CEF, MAX_EVERY, SIGNATURES, Signature};

/// The text of one event a marker may appear in.
///
/// The receiver splits a syslog line, so a marker that was in the header on the
/// wire is in `head` here. Both are searched with the same table.
#[derive(Debug, Clone, Copy, Default)]
pub struct Haystacks<'a> {
    /// The message body, or the whole raw line when the receiver kept it.
    pub body: &'a [u8],
    /// ` hostname appname msgid ` then the structured data as `[id key="value"]`.
    /// The spaces keep a marker such as ` vpxd ` as separate from its neighbours
    /// as it was on the line.
    pub head: &'a [u8],
}

/// Why a [`Detector`] could not be built.
#[derive(Debug, thiserror::Error)]
pub enum DetectError {
    /// A `candidates` entry names no detectable source.
    #[error("source.detect.candidates names {0:?}, which no marker detects")]
    UnknownCandidate(String),
    /// The automaton refused the marker table.
    #[error("the marker table does not build: {0}")]
    Build(String),
}

/// Every source the table can name, sorted.
pub fn detectable() -> &'static [&'static str] {
    static NAMES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
        let mut names: Vec<&'static str> = SIGNATURES
            .iter()
            .map(|s| s.source)
            .chain(CEF.iter().map(|c| c.source))
            .collect();
        names.sort_unstable();
        names.dedup();
        names
    });
    &NAMES
}

/// Where `source` sits in [`detectable`].
fn rank(source: &str) -> usize {
    detectable().binary_search(&source).unwrap_or_default()
}

/// One literal as the automaton reports it: which marker slots it counts toward.
struct Pattern {
    entries: Box<[Entry]>,
    /// Length of `CEF:` or `LEEF:` when this literal opens a header, else 0.
    header: u8,
}

/// A literal's part in one signature.
struct Entry {
    slot: u16,
    /// The bit this literal sets, for a signature that needs every literal.
    key: u8,
}

/// A retained signature and what it needs to fire.
struct Slot {
    sig: &'static Signature,
    /// Index into the per-line scratch, for a signature that needs every literal.
    every: usize,
    /// The bits set when every literal has appeared.
    full: u8,
}

/// The marker table, built once from the candidates.
pub struct Detector {
    ac: AhoCorasick,
    patterns: Vec<Pattern>,
    slots: Vec<Slot>,
    /// Position in [`detectable`] of each slot's source, then of each retained CEF entry's.
    ranks: Vec<usize>,
    cef: Vec<&'static signatures::CefMarker>,
}

impl Detector {
    /// Build over `candidates`, or over every detectable source when it is empty.
    ///
    /// # Errors
    ///
    /// [`DetectError::UnknownCandidate`] when a name is not one of [`detectable`].
    pub fn new(candidates: &[String]) -> Result<Self, DetectError> {
        if let Some(bad) = candidates
            .iter()
            .find(|c| !detectable().contains(&c.as_str()))
        {
            return Err(DetectError::UnknownCandidate(bad.clone()));
        }
        let keep = |source: &str| candidates.is_empty() || candidates.iter().any(|c| c == source);

        let mut slots = Vec::new();
        let mut ranks = Vec::new();
        let mut ids: HashMap<&'static str, usize> = HashMap::new();
        let mut patterns: Vec<(&'static str, Vec<Entry>, u8)> = Vec::new();
        let mut every = 0;
        for sig in SIGNATURES.iter().filter(|s| keep(s.source)) {
            let slot = u16::try_from(slots.len()).unwrap_or(u16::MAX);
            let full = if sig.all {
                u8::try_from((1u16 << sig.markers.len()) - 1).unwrap_or(u8::MAX)
            } else {
                0
            };
            slots.push(Slot { sig, every, full });
            every += usize::from(sig.all);
            ranks.push(rank(sig.source));
            for (key, literal) in sig.markers.iter().enumerate() {
                let id = *ids.entry(literal).or_insert_with(|| {
                    patterns.push((literal, Vec::new(), 0));
                    patterns.len() - 1
                });
                patterns[id].1.push(Entry {
                    slot,
                    key: if sig.all { 1 << key } else { 0 },
                });
            }
        }
        debug_assert!(every <= MAX_EVERY);

        let cef: Vec<_> = CEF.iter().filter(|c| keep(c.source)).collect();
        ranks.extend(cef.iter().map(|c| rank(c.source)));
        if !cef.is_empty() {
            for (literal, len) in [("CEF:", 4), ("LEEF:", 5)] {
                let id = *ids.entry(literal).or_insert_with(|| {
                    patterns.push((literal, Vec::new(), 0));
                    patterns.len() - 1
                });
                patterns[id].2 = len;
            }
        }

        let ac = AhoCorasick::builder()
            .match_kind(MatchKind::Standard)
            .kind(Some(AhoCorasickKind::DFA))
            .build(patterns.iter().map(|(literal, _, _)| *literal))
            .map_err(|e| DetectError::Build(e.to_string()))?;
        let patterns = patterns
            .into_iter()
            .map(|(_, entries, header)| Pattern {
                entries: entries.into(),
                header,
            })
            .collect();
        Ok(Self {
            ac,
            patterns,
            slots,
            ranks,
            cef,
        })
    }

    /// The source `h` came from, as a position in [`detectable`], or `None`
    /// when no marker fires.
    ///
    /// The highest priority wins, and the earlier table entry wins a tie.
    #[must_use]
    pub fn detect(&self, h: &Haystacks<'_>) -> Option<usize> {
        let mut seen = [0u8; MAX_EVERY];
        let mut best: Option<(u8, usize)> = None;
        let mut offer = |priority: u8, index: usize| {
            if best.is_none_or(|(p, i)| priority > p || (priority == p && index < i)) {
                best = Some((priority, index));
            }
        };

        for hay in [h.body, h.head] {
            if hay.is_empty() {
                continue;
            }
            for hit in self.ac.find_overlapping_iter(hay) {
                let pattern = &self.patterns[hit.pattern().as_usize()];
                for e in &pattern.entries {
                    let slot = &self.slots[usize::from(e.slot)];
                    let fires = if slot.sig.all {
                        seen[slot.every] |= e.key;
                        seen[slot.every] == slot.full
                    } else {
                        slot.sig.verify.is_none_or(|v| v(hay, hit.start()))
                    };
                    if fires {
                        offer(slot.sig.priority, usize::from(e.slot));
                    }
                }
                if pattern.header != 0 {
                    self.header(hay, hit.start() + usize::from(pattern.header), &mut offer);
                }
            }
        }
        best.map(|(_, index)| self.ranks[index])
    }

    /// The source of a dfe-receiver syslog event.
    ///
    /// The body is `_raw` when the receiver kept the line and `message`
    /// otherwise, and the header fields are searched as well. `scratch` holds
    /// the rendered header and is reused across events.
    #[must_use]
    pub fn detect_event(&self, event: &Event, scratch: &mut String) -> Option<usize> {
        let bytes = |path| event.get_str(path).unwrap_or_default();
        scratch.clear();
        for field in ["hostname", "appname", "msgid"] {
            scratch.push(' ');
            scratch.push_str(bytes(field));
        }
        scratch.push(' ');
        if let Some(data) = event.get("structured_data") {
            render_structured(data, scratch);
        }
        let body = if event.has("_raw") {
            bytes("_raw")
        } else {
            bytes("message")
        };
        self.detect(&Haystacks {
            body: body.as_bytes(),
            head: scratch.as_bytes(),
        })
    }

    /// Offer the CEF entry whose vendor and product follow a `CEF:` or `LEEF:` opener.
    fn header(&self, hay: &[u8], from: usize, offer: &mut impl FnMut(u8, usize)) {
        let Some(rest) = hay.get(from..) else { return };
        let mut fields = rest.splitn(4, |&b| b == b'|');
        let (Some(version), Some(vendor), Some(product), Some(_)) =
            (fields.next(), fields.next(), fields.next(), fields.next())
        else {
            return;
        };
        if version.is_empty() || !version.iter().all(|b| b.is_ascii_digit() || *b == b'.') {
            return;
        }
        for (i, c) in self.cef.iter().enumerate() {
            if vendor.eq_ignore_ascii_case(c.vendor.as_bytes())
                && product.eq_ignore_ascii_case(c.product.as_bytes())
            {
                offer(c.priority, self.slots.len() + i);
            }
        }
    }
}

/// Render RFC 5424 structured data as `[id key="value"]` text.
///
/// The receiver delivers it as an array of `{id, params}` objects.
fn render_structured(data: &Value, out: &mut String) {
    let Some(elements) = data.as_array() else {
        return;
    };
    for element in elements {
        out.push('[');
        out.push_str(
            element
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default(),
        );
        if let Some(params) = element.get("params").and_then(Value::as_object) {
            for (key, value) in params {
                out.push(' ');
                out.push_str(key);
                out.push_str("=\"");
                out.push_str(value.as_str().unwrap_or_default());
                out.push('"');
            }
        }
        out.push(']');
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::envelope::Envelope;
    use crate::registry;

    fn detector(candidates: &[&str]) -> Detector {
        let owned: Vec<String> = candidates.iter().map(|c| (*c).to_owned()).collect();
        Detector::new(&owned).expect("candidates are detectable")
    }

    fn body(line: &str) -> Haystacks<'_> {
        Haystacks {
            body: line.as_bytes(),
            ..Haystacks::default()
        }
    }

    const ASA: &str = "filebeat.cisco_asa.default";
    const FTD: &str = "filebeat.cisco_ftd.default";

    /// Each row is a candidates list, a line and the source it must name, with
    /// `None` for a line nothing in the filtered table claims.
    #[test]
    fn names_the_source_and_honours_candidates() {
        const IOS: &str = "filebeat.cisco_ios.default";
        const ADC: &str = "filebeat.citrix_adc_log.default";
        const WAF: &str = "filebeat.citrix_waf_log.default";
        let asa_line = "%ASA-6-302013: Built inbound TCP connection 1 for outside:1.2.3.4/80";
        let cef = |vendor: &str, product: &str| {
            format!("CEF:0|{vendor}|{product}|1.0|100|x|5|src=1.2.3.4")
        };
        let rows: Vec<(&str, Vec<&str>, String, Option<&str>)> = vec![
            ("asa", vec![], asa_line.into(), Some(ASA)),
            ("asa out, ftd alone", vec![FTD], asa_line.into(), None),
            (
                "asa out, ios catches it",
                vec![FTD, IOS],
                asa_line.into(),
                Some(IOS),
            ),
            (
                "ftd owns its marker",
                vec![],
                "%FTD-6-430003: x".into(),
                Some(FTD),
            ),
            (
                "ftd beats asa on a shared id",
                vec![],
                "%ASA-6-430003: x".into(),
                Some(FTD),
            ),
            (
                "ios catch-all",
                vec![],
                "Jan 1 00:00:00 r: %SYS-5-CONFIG_I: Configured".into(),
                Some(IOS),
            ),
            ("ios needs a severity", vec![], "100% sure".into(), None),
            (
                "cef vendor and product",
                vec![],
                cef("Infoblox", "NIOS"),
                Some("filebeat.infoblox_nios_log.default"),
            ),
            (
                "cef case",
                vec![],
                cef("INFOBLOX", "nios"),
                Some("filebeat.infoblox_nios_log.default"),
            ),
            (
                "cef unknown pair",
                vec![],
                cef("Acme", "Widget"),
                Some("filebeat.cef_log.default"),
            ),
            (
                "cef citrix goes to adc",
                vec![],
                cef("Citrix", "NetScaler"),
                Some(ADC),
            ),
            (
                "cef citrix goes to waf without adc",
                vec![WAF],
                cef("Citrix", "NetScaler"),
                Some(WAF),
            ),
            (
                "a key set needs every key",
                vec![],
                "IN=eth0 OUT= SRC=1.2.3.4".into(),
                Some("filebeat.iptables_log.default"),
            ),
            (
                "a key set missing one",
                vec![],
                "IN=eth0 SRC=1.2.3.4".into(),
                None,
            ),
            ("empty", vec![], String::new(), None),
        ];
        for (name, candidates, line, want) in rows {
            let got = detector(&candidates)
                .detect(&body(&line))
                .map(|i| detectable()[i]);
            assert_eq!(got, want, "{name}");
        }
    }

    #[test]
    fn an_unknown_candidate_is_refused() {
        let err = Detector::new(&["filebeat.nope.default".to_owned()]).err();
        assert!(
            matches!(err, Some(DetectError::UnknownCandidate(name)) if name == "filebeat.nope.default")
        );
    }

    #[test]
    fn a_marker_is_found_in_any_haystack() {
        let d = detector(&[]);
        let h = Haystacks {
            body: b"x",
            head: b" web01 PulseSecure: - ",
        };
        assert_eq!(
            d.detect(&h).map(|i| detectable()[i]),
            Some("filebeat.pulse_connect_secure_log.default")
        );
    }

    #[test]
    fn every_detectable_source_is_registered_and_labelled() {
        for source in detectable() {
            assert!(
                registry::intake(source).is_some_and(|i| i.accepts(Envelope::Receiver)),
                "{source} does not accept a dfe-receiver delivery"
            );
            assert!(
                registry::lookup(source).is_some(),
                "{source} is not a registered transform"
            );
            assert!(label(source).is_some(), "{source} has no catalogue label");
        }
    }

    #[test]
    fn the_every_table_fits_its_scratch() {
        let every = SIGNATURES.iter().filter(|s| s.all).count();
        assert!(
            every <= MAX_EVERY,
            "{every} key-set entries against {MAX_EVERY}"
        );
        assert!(SIGNATURES.iter().all(|s| !s.all || s.markers.len() <= 8));
        assert!(SIGNATURES.len() < usize::from(u16::MAX));
    }
}
