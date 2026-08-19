// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("ecs.version", json!("8.11.0"))?;

        event.set("observer.vendor", json!("Cisco"))?;

        event.set("observer.product", json!("Nexus"))?;

        event.set("observer.type", json!("switches"))?;

        event.set("event.kind", json!("event"))?;

        let _cond = { !event.has("event.original") };
        if _cond {
            if event.has("message") {
                event.rename("message", "event.original")?;
            }
        }

        let _cond = {
            event.has("event.original")
                && event
                    .get_str("event.original")
                    .is_some_and(|s| !s.is_empty())
        };
        if _cond {
            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}%{SYSLOGTIMESTAMP:temp.timestamp}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                if !cached_grok!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}%{SYSLOGTIMESTAMP:temp.timestamp}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$").extract_into(&input, event)? {
                    // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}%{SYSLOGTIMESTAMP:temp.syslog_timestamp}%{SPACE}%{WORD:cisco_nexus.log.timezone}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                    if !cached_grok!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}%{SYSLOGTIMESTAMP:temp.syslog_timestamp}%{SPACE}%{WORD:cisco_nexus.log.timezone}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$").extract_into(&input, event)? {
                        // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}%{SPACE}%{WORD:cisco_nexus.log.timezone})):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                        if !cached_grok_mapped!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}%{SPACE}%{WORD:cisco_nexus.log.timezone})):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$", [("temp_timestamp", "temp.timestamp")]).extract_into(&input, event)? {
                            // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:temp.syslog_timestamp}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}%{WORD:cisco_nexus.log.timezone}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                            if !cached_grok!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:temp.syslog_timestamp}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}%{WORD:cisco_nexus.log.timezone}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$").extract_into(&input, event)? {
                                // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}%{SPACE}%{WORD:cisco_nexus.log.timezone})):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                                if !cached_grok_mapped!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}%{SPACE}%{WORD:cisco_nexus.log.timezone})):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$", [("temp_timestamp", "temp.timestamp")]).extract_into(&input, event)? {
                                    // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>:%{SPACE}(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}%{SPACE}%{WORD:cisco_nexus.log.timezone})):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                                    if !cached_grok_mapped!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>:%{SPACE}(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}%{SPACE}%{WORD:cisco_nexus.log.timezone})):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$", [("temp_timestamp", "temp.timestamp")]).extract_into(&input, event)? {
                                        // Grok pattern: ^(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                                        if !cached_grok_mapped!("^(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$", [("temp_timestamp", "temp.timestamp")]).extract_into(&input, event)? {
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let _cond = { event.has("_conf.tz_offset") };
        if _cond {
            if event.has("_conf.tz_offset") {
                event.rename("_conf.tz_offset", "event.timezone")?;
            }
        }

        let _cond = {
            event.has("cisco_nexus.log.syslog_time")
                && event
                    .get_str("cisco_nexus.log.syslog_time")
                    .is_some_and(|s| !s.is_empty())
        };
        if _cond {
            if let Some(date_str) = event.get_as_string("cisco_nexus.log.syslog_time") {
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS\")")
            }
        }

        let _cond = { event.has("temp.syslog_timestamp") };
        if _cond {
            if let Some(date_str) = event.get_as_string("temp.syslog_timestamp") {
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS\")")
            }
        }

        let _cond = {
            !event.has("temp.timestamp")
                && event.has("temp.syslog_timestamp")
                && event.has("cisco_nexus.log.timezone")
        };
        if _cond {
            event.set(
                "temp.timestamp",
                json!(format!(
                    "{} {}",
                    event.get_str("temp.syslog_timestamp").unwrap_or(""),
                    event.get_str("cisco_nexus.log.timezone").unwrap_or("")
                )),
            )?;
        }

        let _cond = {
            event.has("temp.timestamp")
                && event
                    .get_str("temp.timestamp")
                    .is_some_and(|s| !s.is_empty())
                && ((!event.has("event.timezone"))
                    || (event.has("event.timezone")
                        && (event.has("cisco_nexus.log.timezone")
                            && event
                                .get_str("cisco_nexus.log.timezone")
                                .is_some_and(|s| !s.is_empty()))))
        };
        if _cond {
            if let Some(date_str) = event.get_as_string("temp.timestamp") {
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS\")")
            }
        }

        let _cond = {
            event.has("temp.timestamp")
                && event
                    .get_str("temp.timestamp")
                    .is_some_and(|s| !s.is_empty())
                && event.has("event.timezone")
                && (!event.has("cisco_nexus.log.timezone")
                    || event
                        .get_str("cisco_nexus.log.timezone")
                        .is_none_or(|s| s.is_empty()))
        };
        if _cond {
            if let Some(date_str) = event.get_as_string("temp.timestamp") {
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS\")")
            }
        }

        let _cond = {
            event.has("temp.timestamp")
                && event
                    .get_str("temp.timestamp")
                    .is_some_and(|s| !s.is_empty())
                && ((!event.has("event.timezone"))
                    || (event.has("event.timezone")
                        && (event.has("cisco_nexus.log.timezone")
                            && event
                                .get_str("cisco_nexus.log.timezone")
                                .is_some_and(|s| !s.is_empty()))))
        };
        if _cond {
            if let Some(date_str) = event.get_as_string("temp.timestamp") {
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS zzz\")")
            }
        }

        let _cond = {
            event.has("temp.timestamp")
                && event
                    .get_str("temp.timestamp")
                    .is_some_and(|s| !s.is_empty())
                && event.has("event.timezone")
                && (!event.has("cisco_nexus.log.timezone")
                    || event
                        .get_str("cisco_nexus.log.timezone")
                        .is_none_or(|s| s.is_empty()))
        };
        if _cond {
            if let Some(date_str) = event.get_as_string("temp.timestamp") {
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"yyyy MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss zzz\")")
                // Try Java datetime format: CustomTime(\"MMM  d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM  d HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"MMM dd HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM dd HH:mm:ss.SSS zzz\")")
                // Try Java datetime format: CustomTime(\"MMM d HH:mm:ss.SSS zzz\")
                // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"MMM d HH:mm:ss.SSS zzz\")")
            }
        }

        event.set(
            "log.syslog.priority",
            event
                .get("cisco_nexus.log.priority_number")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        event.set(
            "observer.name",
            event
                .get("cisco_nexus.log.switch_name")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        let _cond = { event.has("cisco_nexus.log.ip_address") };
        if _cond {
            event.append(
                "observer.ip",
                event
                    .get("cisco_nexus.log.ip_address")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let _cond = { event.has("cisco_nexus.log.ip_address") };
        if _cond {
            event.append(
                "related.ip",
                event
                    .get("cisco_nexus.log.ip_address")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        event.set(
            "event.code",
            event
                .get("cisco_nexus.log.type")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        event.set(
            "event.severity",
            event
                .get("cisco_nexus.log.severity")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        event.set(
            "log.syslog.severity.code",
            event
                .get("cisco_nexus.log.severity")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        event.set(
            "event.sequence",
            event
                .get("cisco_nexus.log.sequence_number")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        let _cond = { event.has("event.severity") };
        if _cond {
            // Painless script
            // Source: def LogLevelValue = (int) ctx.event.severity;\nif (LogLevelValue >= 0 && LogLevelValue < params.LogLevel.length) {\n  ctx.log.put('level', params['LogLevel'][LogLevelValue]);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def LogLevelValue = (int) ctx.event.severity;\nif (LogLevelValue >= 0 && LogLevelValue < params.LogLevel.length) {\n  ctx.log.put('level', params['LogLevel'][LogLevelValue]);\n}"#,
            )?;
        }

        let _cond = { event.has("cisco_nexus.log.priority_number") && event.has("event.severity") };
        if _cond {
            // Painless script
            // Source: ctx.log.syslog.facility = new HashMap();\nctx.log.syslog.facility.code = (ctx.cisco_nexus.log.priority_number - ctx.event.severity)/8;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"ctx.log.syslog.facility = new HashMap();\nctx.log.syslog.facility.code = (ctx.cisco_nexus.log.priority_number - ctx.event.severity)/8;\n"#,
            )?;
        }

        if event.has("cisco_nexus.log.description") {
            if let Some(s) = event.get_string("cisco_nexus.log.description") {
                let trimmed = s.trim().to_string();
                event.set("cisco_nexus.log.description", trimmed)?;
            }
        }

        event.set(
            "message",
            event
                .get("cisco_nexus.log.description")
                .cloned()
                .unwrap_or(Value::Null),
        )?;

        let _cond = {
            event.has("event.code")
                && event.get_str("event.code").is_some_and(|s| {
                    [
                        "IF_DOWN_ADMIN_DOWN",
                        "IF_ADMIN_UP",
                        "SPEED",
                        "IF_DUPLEX",
                        "IF_RX_FLOW_CONTROL",
                        "IF_TX_FLOW_CONTROL",
                        "IF_UP",
                        "IF_XCVR_WARNING",
                        "VSHD_SYSLOG_CONFIG_I",
                        "DETECT_MULTIPLE_PEERS",
                        "SYSTEM_MSG",
                        "UPDOWN",
                        "CFGWRITE_STARTED",
                        "CFGWRITE_DONE",
                        "INVAL_IP",
                        "L2FM_MAC_MOVE2",
                        "DUPLEX_MISMATCH",
                        "NATIVE_VLAN_MISMATCH",
                        "LOGIN_SUCCESS",
                        "LOGOUT",
                        "LOGOUT_C6K",
                        "L3_VPC_UNEQUAL_WEIGHT",
                        "AAA_ACCOUNTING_MESSAGE",
                        "TACACS_WARNING",
                        "DUP_HOSTS",
                        "NF_PARITY_ERROR",
                        "EXCESSIVE_PARITY_ERROR",
                        "LINEPROTO",
                        "THRESHOLD_VIOLATION",
                    ]
                    .contains(&s.to_uppercase().as_str())
                })
        };
        if _cond {
            // Begin nested pipeline: "pipeline_extract_message"
            let _cond = {
                event.get_str("event.code").is_some_and(|s| {
                    [
                        "IF_DOWN_ADMIN_DOWN",
                        "IF_ADMIN_UP",
                        "SPEED",
                        "IF_DUPLEX",
                        "IF_RX_FLOW_CONTROL",
                        "IF_TX_FLOW_CONTROL",
                        "IF_UP",
                        "IF_XCVR_WARNING",
                    ]
                    .contains(&s.to_uppercase().as_str())
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is up in mode %{DATA:cisco_nexus.log.interface.mode}$
                        if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is up in mode %{DATA:cisco_nexus.log.interface.mode}$").extract_into(&input, event)? {
            // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is %{GREEDYDATA}$
            if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is %{GREEDYDATA}$").extract_into(&input, event)? {
            // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational speed changed to %{DATA:cisco_nexus.log.operational.speed}$
            if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational speed changed to %{DATA:cisco_nexus.log.operational.speed}$").extract_into(&input, event)? {
            // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational duplex mode changed to %{DATA:cisco_nexus.log.operational.duplex_mode}$
            if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational duplex mode changed to %{DATA:cisco_nexus.log.operational.duplex_mode}$").extract_into(&input, event)? {
            // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Receive Flow Control state changed to %{DATA:cisco_nexus.log.operational.receive_flow_control_state}$
            if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Receive Flow Control state changed to %{DATA:cisco_nexus.log.operational.receive_flow_control_state}$").extract_into(&input, event)? {
            // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Transmit Flow Control state changed to %{DATA:cisco_nexus.log.operational.transmit_flow_control_state}$
            if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Transmit Flow Control state changed to %{DATA:cisco_nexus.log.operational.transmit_flow_control_state}$").extract_into(&input, event)? {
            // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, %{GREEDYDATA}$
            if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, %{GREEDYDATA}$").extract_into(&input, event)? {
            }
            }
            }
            }
            }
            }
            }
                    }
                    Ok(())
                })();
            }
            let _cond = {
                event.get_str("event.code").is_some_and(|s| {
                    [
                        "VSHD_SYSLOG_CONFIG_I",
                        "DETECT_MULTIPLE_PEERS",
                        "UPDOWN",
                        "CFGWRITE_STARTED",
                        "LINEPROTO",
                    ]
                    .contains(&s.to_uppercase().as_str())
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Configured from vty by %{USERNAME:user.name} on %{IP:source.ip}@%{DATA:cisco_nexus.log.terminal}$
                        if !cached_grok!("^Configured from vty by %{USERNAME:user.name} on %{IP:source.ip}@%{DATA:cisco_nexus.log.terminal}$").extract_into(&input, event)? {
            // Grok pattern: ^Multiple peers detected on %{DATA:cisco_nexus.log.interface.name}$
            if !cached_grok!("^Multiple peers detected on %{DATA:cisco_nexus.log.interface.name}$").extract_into(&input, event)? {
            // Grok pattern: ^Line (?i)protocol on Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.line_protocol_state}$
            if !cached_grok!("^Line (?i)protocol on Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.line_protocol_state}$").extract_into(&input, event)? {
            // Grok pattern: ^Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.state}$
            if !cached_grok!("^Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.state}$").extract_into(&input, event)? {
            // Grok pattern: ^%{DATA}(PID %{NUMBER:process.pid:long})%{GREEDYDATA}$
            if !cached_grok!("^%{DATA}(PID %{NUMBER:process.pid:long})%{GREEDYDATA}$").extract_into(&input, event)? {
            }
            }
            }
            }
            }
                    }
                    Ok(())
                })();
            }
            let _cond = {
                event
                    .get_str("event.code")
                    .is_some_and(|s| ["SYSTEM_MSG"].contains(&s.to_uppercase().as_str()))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA}authentication failure; %{GREEDYDATA:temp.message} - %{GREEDYDATA}$
                        if !cached_grok!("^%{DATA}authentication failure; %{GREEDYDATA:temp.message} - %{GREEDYDATA}$").extract_into(&input, event)? {
            // Grok pattern: ^%{DATA}Authentication failure for %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
            if !cached_grok!("^%{DATA}Authentication failure for %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$").extract_into(&input, event)? {
            // Grok pattern: ^%{DATA}Authentication failed for user %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
            if !cached_grok!("^%{DATA}Authentication failed for user %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$").extract_into(&input, event)? {
            // Grok pattern: ^Login failed for user %{USERNAME:user.name} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
            if !cached_grok!("^Login failed for user %{USERNAME:user.name} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$").extract_into(&input, event)? {
            // Grok pattern: ^%{DATA} : %{GREEDYDATA:temp.message2}$
            if !cached_grok!("^%{DATA} : %{GREEDYDATA:temp.message2}$").extract_into(&input, event)? {
            }
            }
            }
            }
            }
                    }
                    Ok(())
                })();
            }
            let _cond = {
                event.get_str("event.code").is_some_and(|s| {
                    [
                        "INVAL_IP",
                        "L2FM_MAC_MOVE2",
                        "DUPLEX_MISMATCH",
                        "NATIVE_VLAN_MISMATCH",
                        "THRESHOLD_VIOLATION",
                    ]
                    .contains(&s.to_uppercase().as_str())
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA:network.protocol} %{DATA}%{SPACE}Received packet with invalid destination IP address (%{DATA}) from %{CISCOMAC:source.mac} on %{DATA:cisco_nexus.log.interface.name}$
                        if !cached_grok!("^%{DATA:network.protocol} %{DATA}%{SPACE}Received packet with invalid destination IP address (%{DATA}) from %{CISCOMAC:source.mac} on %{DATA:cisco_nexus.log.interface.name}$").extract_into(&input, event)? {
            // Grok pattern: ^Mac %{CISCOMAC:source.mac} in %{DATA:cisco_nexus.log.interface.name} has moved from %{GREEDYDATA}$
            if !cached_grok!("^Mac %{CISCOMAC:source.mac} in %{DATA:cisco_nexus.log.interface.name} has moved from %{GREEDYDATA}$").extract_into(&input, event)? {
            // Grok pattern: ^%{DATA} mismatch discovered on %{DATA:cisco_nexus.log.network.ingress_interface}(?:\\(%{DATA}\\))?, with %{DATA:cisco_nexus.log.network.egress_interface}(?:\\(%{DATA}\\))?$
            if !cached_grok!("^%{DATA} mismatch discovered on %{DATA:cisco_nexus.log.network.ingress_interface}(?:\\(%{DATA}\\))?, with %{DATA:cisco_nexus.log.network.egress_interface}(?:\\(%{DATA}\\))?$").extract_into(&input, event)? {
            // Grok pattern: ^%{DATA:cisco_nexus.log.interface.name}: Rx power high warning; Operating value: %{DATA:cisco_nexus.log.operating_value}, Threshold value: %{DATA:cisco_nexus.log.threshold_value}.$
            if !cached_grok!("^%{DATA:cisco_nexus.log.interface.name}: Rx power high warning; Operating value: %{DATA:cisco_nexus.log.operating_value}, Threshold value: %{DATA:cisco_nexus.log.threshold_value}.$").extract_into(&input, event)? {
            }
            }
            }
            }
                    }
                    Ok(())
                })();
            }
            let _cond = {
                event.get_str("event.code").is_some_and(|s| {
                    ["LOGIN_SUCCESS", "LOGOUT", "LOGOUT_C6K"].contains(&s.to_uppercase().as_str())
                })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Login Success \\[user: %{USERNAME:user.name}\\] \\[Source: %{IP:source.ip}\\] \\[localport: %{NUMBER:source.port:long}\\] at %{GREEDYDATA}$
                        if !cached_grok!("^Login Success \\[user: %{USERNAME:user.name}\\] \\[Source: %{IP:source.ip}\\] \\[localport: %{NUMBER:source.port:long}\\] at %{GREEDYDATA}$").extract_into(&input, event)? {
            // Grok pattern: ^User %{USERNAME:user.name} %{GREEDYDATA}\\(%{IP:source.ip}\\)$
            if !cached_grok!("^User %{USERNAME:user.name} %{GREEDYDATA}\\(%{IP:source.ip}\\)$").extract_into(&input, event)? {
            }
            }
                    }
                    Ok(())
                })();
            }
            if event.has("source.mac") {
                if let Some(s) = event.get_string("source.mac") {
                    let re = cached_regex!("[.]");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("source.mac", replaced)?;
                }
            }
            // SKIPPED: pattern unsupported by the regex engine: (..)(?!$)
            if event.has("source.mac") {
                if let Some(s) = event.get_string("source.mac") {
                    let uppered = s.to_uppercase();
                    event.set("source.mac", uppered)?;
                }
            }
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("temp.message") {
                    if let Some(kv_str) = event.get_string("temp.message") {
                        for pair in cached_regex!("\\s+").split(&kv_str) {
                            if let Some((key, value)) = pair.split_once("=") {
                                if !key.is_empty() {
                                    event.set(&format!("temp.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })();
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("temp.message2") {
                    if let Some(kv_str) = event.get_string("temp.message2") {
                        for pair in kv_str.split(" ; ") {
                            if let Some((key, value)) = pair.split_once("=") {
                                if !key.is_empty() {
                                    event.set(&format!("temp.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })();
            if event.has("temp.logname") {
                event.rename("temp.logname", "cisco_nexus.log.logname")?;
            }
            if event.has("temp.uid") {
                event.rename("temp.uid", "cisco_nexus.log.uid")?;
            }
            if event.has("temp.euid") {
                event.rename("temp.euid", "cisco_nexus.log.euid")?;
            }
            if event.has("temp.tty") {
                event.rename("temp.tty", "cisco_nexus.log.tty")?;
            }
            if event.has("temp.ruser") {
                event.rename("temp.ruser", "cisco_nexus.log.ruser")?;
            }
            if event.has("temp.rhost") {
                event.rename("temp.rhost", "cisco_nexus.log.rhost")?;
            }
            if event.has("temp.user") {
                event.rename("temp.user", "user.name")?;
            }
            if event.has("temp.COMMAND") {
                event.rename("temp.COMMAND", "cisco_nexus.log.command")?;
            }
            if event.has("temp.PWD") {
                event.rename("temp.PWD", "cisco_nexus.log.pwd")?;
            }
            if event.has("temp.TTY") {
                event.rename("temp.TTY", "cisco_nexus.log.tty")?;
            }
            if event.has("temp.USER") {
                event.rename("temp.USER", "user.name")?;
            }
            if event.has("network.protocol") {
                if let Some(s) = event.get_string("network.protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }
            let _cond = {
                event.has("cisco_nexus.log.interface.name")
                    || event.has("cisco_nexus.log.network.ingress_interface")
                    || event.has("cisco_nexus.log.network.egress_interface")
                    || event.get_str("event.code").is_some_and(|s| {
                        [
                            "L2FM_MAC_MOVE2",
                            "L3_VPC_UNEQUAL_WEIGHT",
                            "AAA_ACCOUNTING_MESSAGE",
                            "DUP_HOSTS",
                            "NF_PARITY_ERROR",
                            "EXCESSIVE_PARITY_ERROR",
                        ]
                        .contains(&s.to_uppercase().as_str())
                    })
                    || event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("kex_exchange_identification"))
            };
            if _cond {
                event.set("event.category", json!(["network"]))?;
            }
            let _cond = {
                event.has("cisco_nexus.log.interface.name")
                    || event.has("cisco_nexus.log.network.ingress_interface")
                    || event.has("cisco_nexus.log.network.egress_interface")
                    || event.get_str("event.code").is_some_and(|s| {
                        [
                            "VSHD_SYSLOG_CONFIG_I",
                            "L2FM_MAC_MOVE2",
                            "L3_VPC_UNEQUAL_WEIGHT",
                            "AAA_ACCOUNTING_MESSAGE",
                            "DUP_HOSTS",
                            "NF_PARITY_ERROR",
                            "EXCESSIVE_PARITY_ERROR",
                        ]
                        .contains(&s.to_uppercase().as_str())
                    })
            };
            if _cond {
                event.set("event.type", json!(["info"]))?;
            }
            let _cond = { event.get_str("event.code") == Some("VSHD_SYSLOG_CONFIG_I") };
            if _cond {
                event.set("event.category", json!(["configuration"]))?;
            }
            let _cond = {
                event.get_str("event.code") == Some("LOGIN_SUCCESS")
                    || (event.get_str("event.code") == Some("SYSTEM_MSG")
                        && (event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("authentication"))
                            || event.get_str("message").is_some_and(|s| {
                                s.to_lowercase().contains("authentication failure")
                            })
                            || event
                                .get_str("message")
                                .is_some_and(|s| s.to_lowercase().contains("login"))))
            };
            if _cond {
                event.set("event.category", json!(["authentication"]))?;
            }
            let _cond =
                {
                    event.get_str("event.code") == Some("LOGIN_SUCCESS")
                        || (event.get_str("event.code") == Some("SYSTEM_MSG")
                            && (event.get_str("message").is_some_and(|s| {
                                s.to_lowercase().contains("authentication failed")
                            }) || event.get_str("message").is_some_and(|s| {
                                s.to_lowercase().contains("authentication failure")
                            }) || event
                                .get_str("message")
                                .is_some_and(|s| s.to_lowercase().contains("login failed"))))
                };
            if _cond {
                event.set("event.type", json!(["end"]))?;
            }
            let _cond = {
                event
                    .get_str("message")
                    .is_some_and(|s| s.to_lowercase().contains("kex_exchange_identification"))
            };
            if _cond {
                event.set("event.type", json!(["connection"]))?;
            }
            let _cond = {
                event
                    .get_str("message")
                    .is_some_and(|s| s.to_lowercase().contains("failed"))
                    || event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("failure"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }
            let _cond = {
                event
                    .get_str("message")
                    .is_some_and(|s| s.to_lowercase().contains("successful"))
                    || event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("success"))
                    || event.get_str("event.code") == Some("IF_ADMIN_UP")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }
            let _cond = { event.has("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("source.ip").cloned().unwrap_or(Value::Null),
                )?;
            }
            let _cond = { event.has("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    event.get("user.name").cloned().unwrap_or(Value::Null),
                )?;
            }
            // End nested pipeline: "pipeline_extract_message"
        }

        let _cond = {
            event.has("cisco_nexus.log.facility")
                && event
                    .get_str("cisco_nexus.log.facility")
                    .is_some_and(|s| s.to_lowercase().contains("arp"))
        };
        if _cond {
            event.set("network.protocol", json!("arp"))?;
        }

        event.remove("_conf");
        event.remove("temp");

        let _cond = {
            !event.has("tags")
                || !(event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                    serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"),
                    _ => false,
                }))
        };
        if _cond {
            event.remove("cisco_nexus.log.time");
            event.remove("cisco_nexus.log.description");
            event.remove("cisco_nexus.log.type");
            event.remove("cisco_nexus.log.severity");
            event.remove("cisco_nexus.log.priority_number");
            event.remove("cisco_nexus.log.ip_address");
            event.remove("cisco_nexus.log.switch_name");
            event.remove("cisco_nexus.log.sequence_number");
        }

        let _cond = {
            !event.has("tags")
                || !(event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_original_event")),
                    serde_json::Value::String(s) => s.contains("preserve_original_event"),
                    _ => false,
                }))
        };
        if _cond {
            event.remove("event.original");
        }

        // Painless script
        // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec(
            event,
            r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#,
        )?;

        let _cond = { event.has("error.message") };
        if _cond {
            event.set("event.kind", json!("pipeline_error"))?;
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
