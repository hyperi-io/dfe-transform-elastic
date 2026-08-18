// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! User Agent string parsing and enrichment.
//!
//! Extracts browser name, version, OS, and device information from
//! User-Agent strings using pre-compiled regex patterns.

use std::sync::OnceLock;

use regex::Regex;
use serde_json::json;

use crate::error::{Result, TransformError};
use crate::event::Event;

// Pre-compiled regexes — compiled once on first use, reused for all events.
macro_rules! static_regex {
    ($name:ident, $pattern:expr) => {
        fn $name() -> &'static Regex {
            static RE: OnceLock<Regex> = OnceLock::new();
            RE.get_or_init(|| Regex::new($pattern).expect(concat!("invalid regex: ", $pattern)))
        }
    };
}

static_regex!(re_edge, r"(?i)Edg(?:e|A|iOS)?/(\d+[\d.]*)");
static_regex!(re_opera, r"(?i)OPR/(\d+[\d.]*)");
static_regex!(re_firefox, r"(?i)Firefox/(\d+[\d.]*)");
static_regex!(re_chrome, r"(?i)(?:Chrome|CriOS)/(\d+[\d.]*)");
static_regex!(re_safari, r"(?i)(?:Version/(\d+[\d.]*).*)?Safari/");
static_regex!(re_msie, r"MSIE (\d+[\d.]*)");
static_regex!(re_trident_rv, r"rv:(\d+[\d.]*)");
static_regex!(re_windows_nt, r"(?i)Windows NT (\d+\.\d+)");
static_regex!(re_mac_osx, r"(?i)Mac OS X (\d+[._\d]*)");
static_regex!(re_android, r"(?i)Android (\d+[\d.]*)");
static_regex!(re_ios_version, r"(?i)OS (\d+[_\d]*)");

/// Parsed User Agent result.
#[derive(Debug, Clone, Default)]
pub struct UserAgentResult {
    pub name: Option<String>,
    pub version: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub device: Option<String>,
}

/// Parse a User-Agent string into structured components.
pub fn parse(ua: &str) -> UserAgentResult {
    let mut result = UserAgentResult::default();

    // Browser detection (ordered by specificity)
    if let Some(caps) = re_edge().captures(ua) {
        result.name = Some("Edge".into());
        result.version = caps.get(1).map(|m| m.as_str().into());
    } else if let Some(caps) = re_opera().captures(ua) {
        result.name = Some("Opera".into());
        result.version = caps.get(1).map(|m| m.as_str().into());
    } else if let Some(caps) = re_firefox().captures(ua) {
        result.name = Some("Firefox".into());
        result.version = caps.get(1).map(|m| m.as_str().into());
    } else if let Some(caps) = re_chrome().captures(ua) {
        if !ua.contains("Edg") && !ua.contains("OPR") {
            result.name = Some("Chrome".into());
            result.version = caps.get(1).map(|m| m.as_str().into());
        }
    } else if ua.contains("Mobile")
        && ua.contains("Safari")
        && !ua.contains("Chrome")
        && !ua.contains("CriOS")
        && !ua.contains("Version/")
    {
        // Mobile Safari variants — no Version/ means embedded WebView, not full Safari
        if ua.contains("WKWebView") {
            result.name = Some("Mobile Safari UI/WKWebView".into());
        } else {
            result.name = Some("Mobile Safari".into());
        }
    } else if let Some(caps) = re_safari().captures(ua) {
        if !ua.contains("Chrome") && !ua.contains("CriOS") {
            result.name = Some("Safari".into());
            result.version = caps.get(1).map(|m| m.as_str().into());
        }
    } else if ua.contains("MSIE") || ua.contains("Trident") {
        result.name = Some("IE".into());
        if let Some(caps) = re_msie().captures(ua) {
            result.version = caps.get(1).map(|m| m.as_str().into());
        } else if let Some(caps) = re_trident_rv().captures(ua) {
            result.version = caps.get(1).map(|m| m.as_str().into());
        }
    }

    // Default: if no browser detected and UA string is non-empty, set "Other"
    if result.name.is_none() && !ua.is_empty() {
        result.name = Some("Other".into());
    }

    // OS detection
    if let Some(caps) = re_windows_nt().captures(ua) {
        result.os_name = Some("Windows".into());
        let nt_version = caps.get(1).map_or("", |m| m.as_str());
        result.os_version = Some(match nt_version {
            "10.0" => "10".into(),
            "6.3" => "8.1".into(),
            "6.2" => "8".into(),
            "6.1" => "7".into(),
            "6.0" => "Vista".into(),
            "5.1" => "XP".into(),
            other => other.into(),
        });
    } else if let Some(caps) = re_mac_osx().captures(ua) {
        result.os_name = Some("Mac OS X".into());
        result.os_version = caps.get(1).map(|m| m.as_str().replace('_', "."));
    } else if let Some(caps) = re_android().captures(ua) {
        result.os_name = Some("Android".into());
        result.os_version = caps.get(1).map(|m| m.as_str().into());
    } else if ua.contains("iPhone") || ua.contains("iPad") || ua.contains("iPod") {
        result.os_name = Some("iOS".into());
        if let Some(caps) = re_ios_version().captures(ua) {
            result.os_version = caps.get(1).map(|m| m.as_str().replace('_', "."));
        }
    } else if ua.contains("Linux") {
        result.os_name = Some("Linux".into());
    }

    // Device detection
    if ua.contains("Mobile") || ua.contains("Android") || ua.contains("iPhone") {
        result.device = Some("phone".into());
    } else if ua.contains("iPad") || ua.contains("Tablet") {
        result.device = Some("tablet".into());
    } else if result.os_name.is_some() {
        result.device = Some("pc".into());
    } else if ua.contains("bot")
        || ua.contains("Bot")
        || ua.contains("spider")
        || ua.contains("crawler")
    {
        result.device = Some("bot".into());
    }

    result
}

/// Enrich an event with parsed user agent data.
pub fn enrich(
    event: &mut Event,
    ua_field: &str,
    target_prefix: &str,
    ignore_missing: bool,
) -> Result<()> {
    let ua_str = match event.get_str(ua_field) {
        Some(v) => v.to_string(),
        None if ignore_missing => return Ok(()),
        None => {
            return Err(TransformError::FieldNotFound {
                path: ua_field.into(),
            });
        }
    };

    let ua = parse(&ua_str);

    event.set(&format!("{target_prefix}.original"), json!(ua_str))?;

    if let Some(name) = &ua.name {
        event.set(&format!("{target_prefix}.name"), json!(name))?;
    }
    if let Some(version) = &ua.version {
        event.set(&format!("{target_prefix}.version"), json!(version))?;
    }
    if let Some(os_name) = &ua.os_name {
        event.set(&format!("{target_prefix}.os.name"), json!(os_name))?;
        if let Some(os_version) = &ua.os_version {
            event.set(&format!("{target_prefix}.os.version"), json!(os_version))?;
            event.set(
                &format!("{target_prefix}.os.full"),
                json!(format!("{os_name} {os_version}")),
            )?;
        }
    }
    if let Some(device) = &ua.device {
        event.set(&format!("{target_prefix}.device.name"), json!(device))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_macos() {
        let ua = parse(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
        );
        assert_eq!(ua.name.as_deref(), Some("Chrome"));
        assert_eq!(ua.version.as_deref(), Some("51.0.2704.103"));
        assert_eq!(ua.os_name.as_deref(), Some("Mac OS X"));
        assert_eq!(ua.os_version.as_deref(), Some("10.10.5"));
        assert_eq!(ua.device.as_deref(), Some("pc"));
    }

    #[test]
    fn firefox_windows() {
        let ua =
            parse("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0");
        assert_eq!(ua.name.as_deref(), Some("Firefox"));
        assert_eq!(ua.version.as_deref(), Some("89.0"));
        assert_eq!(ua.os_name.as_deref(), Some("Windows"));
        assert_eq!(ua.os_version.as_deref(), Some("10"));
    }

    #[test]
    fn safari_iphone() {
        let ua = parse(
            "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Mobile/15E148 Safari/604.1",
        );
        assert_eq!(ua.name.as_deref(), Some("Safari"));
        assert_eq!(ua.version.as_deref(), Some("14.1.1"));
        assert_eq!(ua.os_name.as_deref(), Some("iOS"));
        assert_eq!(ua.device.as_deref(), Some("phone"));
    }

    #[test]
    fn edge_windows() {
        let ua = parse(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59",
        );
        assert_eq!(ua.name.as_deref(), Some("Edge"));
        assert_eq!(ua.version.as_deref(), Some("91.0.864.59"));
    }

    #[test]
    fn android_chrome() {
        let ua = parse(
            "Mozilla/5.0 (Linux; Android 11; SM-G991B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.120 Mobile Safari/537.36",
        );
        assert_eq!(ua.name.as_deref(), Some("Chrome"));
        assert_eq!(ua.os_name.as_deref(), Some("Android"));
        assert_eq!(ua.os_version.as_deref(), Some("11"));
        assert_eq!(ua.device.as_deref(), Some("phone"));
    }

    #[test]
    fn empty_string() {
        let ua = parse("");
        assert!(ua.name.is_none());
        assert!(ua.os_name.is_none());
        assert!(ua.device.is_none());
    }

    #[test]
    fn enrich_event() {
        let mut event = Event::new(serde_json::json!({
            "agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
        }));

        enrich(&mut event, "agent", "user_agent", false).unwrap();

        assert_eq!(event.get_str("user_agent.name").unwrap(), "Chrome");
        assert_eq!(
            event.get_str("user_agent.version").unwrap(),
            "51.0.2704.103"
        );
        assert_eq!(event.get_str("user_agent.os.name").unwrap(), "Mac OS X");
        assert_eq!(event.get_str("user_agent.device.name").unwrap(), "pc");
    }

    #[test]
    fn enrich_ignore_missing() {
        let mut event = Event::new(serde_json::json!({}));

        let result = enrich(&mut event, "agent", "user_agent", true);
        assert!(result.is_ok());
    }

    #[test]
    fn enrich_missing_error() {
        let mut event = Event::new(serde_json::json!({}));

        let result = enrich(&mut event, "agent", "user_agent", false);
        assert!(result.is_err());
    }
}
