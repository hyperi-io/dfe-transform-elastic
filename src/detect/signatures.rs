// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The marker table's entry types: which text on a line means which source.
//!
//! The table itself is `markers`, generated from each source's
//! `catalogue.detect` block in dfe-schemas. A marker is a literal, a set of
//! literals that must all appear (a key=value pair or a JSON key set), or a CEF
//! vendor and product.
//! The highest priority wins and the first entry wins a tie, so a broad
//! marker such as the IOS `%FAC-n-MNEM:` catch-all sits low and a specific one
//! sits high.
//!
//! Identical bytes go to the broader source: ASA over FTD, ADC over WAF, IOS
//! over the platforms that share its layout. A source that must win its own
//! lines carries a marker the broader one does not (`%FTD-`, a CEF header).
//! Excluding the broader source from `candidates` hands those lines to the
//! narrower one.

/// Checks the bytes around a literal hit, for the few markers no literal pins.
pub(super) type Verify = fn(&[u8], usize) -> bool;

/// One marker and the source it names.
pub(super) struct Signature {
    /// Registry name of the transform to run.
    pub source: &'static str,
    /// Higher wins. The first entry wins a tie.
    pub priority: u8,
    /// Literals searched for in the line.
    pub markers: &'static [&'static str],
    /// Every marker must appear, rather than any one.
    pub all: bool,
    /// Run at the start of each hit, when `all` is false.
    pub verify: Option<Verify>,
}

/// A CEF or LEEF header: vendor and product name the source.
pub(super) struct CefMarker {
    /// Registry name of the transform to run.
    pub source: &'static str,
    /// Higher wins.
    pub priority: u8,
    /// Header vendor, compared without regard to ASCII case.
    pub vendor: &'static str,
    /// Header product, compared without regard to ASCII case.
    pub product: &'static str,
}

pub(super) const fn one_of(
    source: &'static str,
    priority: u8,
    markers: &'static [&'static str],
) -> Signature {
    Signature {
        source,
        priority,
        markers,
        all: false,
        verify: None,
    }
}

pub(super) const fn every(
    source: &'static str,
    priority: u8,
    markers: &'static [&'static str],
) -> Signature {
    Signature {
        source,
        priority,
        markers,
        all: true,
        verify: None,
    }
}

pub(super) const fn checked(
    source: &'static str,
    priority: u8,
    markers: &'static [&'static str],
    verify: Verify,
) -> Signature {
    Signature {
        source,
        priority,
        markers,
        all: false,
        verify: Some(verify),
    }
}

pub(super) const fn cef(
    source: &'static str,
    priority: u8,
    vendor: &'static str,
    product: &'static str,
) -> CefMarker {
    CefMarker {
        source,
        priority,
        vendor,
        product,
    }
}
