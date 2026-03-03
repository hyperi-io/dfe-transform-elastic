use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use regex::{Regex, Replacer};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::{
    conditional::Conditional,
    on_failure::OnFailure,
    template_string::TemplateString,
    unsupported_fields, Validate,
};

// Cisco contains time patterns which do not contain the year
// This causes the chrono library underneath VRL to panic
// Elastic just assumes it means current year, meaning parsing historical logs
// will produce different results
#[derive(Debug, Clone, PartialEq)]
pub struct TimePattern {
    pattern: String,
    assume_current_year: bool,
    has_timezone: bool,
    has_timezone_name: bool,
    unix_ms: bool,
}

impl TimePattern {
    pub fn new(pattern: impl AsRef<str>) -> TimePattern {
        TimePattern {
            pattern: pattern.as_ref().to_string(),
            assume_current_year: false,
            has_timezone: false,
            has_timezone_name: false,
            unix_ms: false,
        }
    }

    pub fn assume_current_year(mut self, b: bool) -> Self {
        self.assume_current_year = b;
        self
    }

    pub fn has_timezone(mut self, b: bool) -> Self {
        self.has_timezone = b;
        self
    }

    pub fn has_timezone_name(mut self, b: bool) -> Self {
        self.has_timezone_name = b;
        self
    }

    pub fn unix_ms(mut self, b: bool) -> Self {
        self.unix_ms = b;
        self
    }
}

// https://docs.oracle.com/javase/8/docs/api/java/time/format/DateTimeFormatter.html
// https://docs.rs/chrono/latest/chrono/format/strftime/index.html#fn3
// Further Madness: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html
lazy_static! {
    static ref JAVA_DATETIME_REGEX: Regex = Regex::new(r#"([^/:. \n\-]+)"#).unwrap();
}

pub struct JavaDateTimeReplacer {
    pub error: anyhow::Result<()>,
    pub assume_current_year: bool,
    pub has_timezone: bool,
    pub has_timezone_name: bool,
}

impl JavaDateTimeReplacer {
    pub fn new() -> Self {
        JavaDateTimeReplacer {
            error: Ok(()),
            assume_current_year: true,
            has_timezone: false,
            has_timezone_name: false,
        }
    }
}

impl Default for JavaDateTimeReplacer {
    fn default() -> Self {
        Self::new()
    }
}

impl Replacer for &mut JavaDateTimeReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        if self.error.is_err() {
            return;
        }

        match &caps[1] {
            "a" => dst.push_str("%p"),
            "dd" | "d" => dst.push_str("%d"),
            "EEE" => dst.push_str("%a"),
            "h" => dst.push_str("%I"),
            "HH" => dst.push_str("%H"),
            "mm" => dst.push_str("%M"),
            "MM" | "M" => dst.push_str("%m"),
            "MMM" => dst.push_str("%b"),
            "ss" => dst.push_str("%S"),
            "SSS" => dst.push_str("%3f"),
            "XXX" | "Z" => {
                self.has_timezone = true;
                dst.push_str("%z")
            }
            "yyyy" => {
                self.assume_current_year = false;
                dst.push_str("%Y")
            }
            "zzz" | "z" => {
                self.has_timezone = true;
                self.has_timezone_name = true;
            }
            "strict_date_optional_time_nanos" => {
                self.has_timezone = true;
                self.assume_current_year = false;
                dst.push_str("%+")
            }
            other => self.error = Err(anyhow!("failed to transpile java date-time string \"{other}\"")),
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomTime(String);

impl Validate for CustomTime {
    #[instrument(name = "CustomTime::validate", err)]
    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeFormats {
    ISO8601,
    UNIX,
    #[allow(non_camel_case_types)]
    UNIX_MS,
    TAI64N,
    #[serde(untagged)]
    Custom(CustomTime),
}

impl Validate for TimeFormats {
    #[instrument(name = "TimeFormats::validate")]
    fn validate(&self) -> anyhow::Result<()> {
        match self {
            TimeFormats::ISO8601 | TimeFormats::UNIX | TimeFormats::UNIX_MS => Ok(()),
            TimeFormats::Custom(custom) => custom.validate(),
            format => bail!("{format:?} currently not supported as a time format"),
        }
    }
}

impl TimeFormats {
    /// Convert a TimeFormats value to a TimePattern for date parsing.
    #[allow(dead_code, clippy::wrong_self_convention)]
    #[instrument(name = "TimeFormats::to_pattern", err)]
    fn to_pattern(self) -> anyhow::Result<TimePattern> {
        Ok(match self {
            TimeFormats::ISO8601 => TimePattern::new("%+").has_timezone(true),
            TimeFormats::UNIX => TimePattern::new("%s%.f"),
            TimeFormats::UNIX_MS => TimePattern::new("%s%.3f").unix_ms(true),
            TimeFormats::Custom(custom) => {
                let mut replacer = JavaDateTimeReplacer::new();
                let pattern = JAVA_DATETIME_REGEX.replace_all(&custom.0, &mut replacer);

                replacer.error?;

                TimePattern::new(pattern)
                    .assume_current_year(replacer.assume_current_year)
                    .has_timezone(replacer.has_timezone)
                    .has_timezone_name(replacer.has_timezone_name)
            }
            other => bail!("{other:?} currently not supported by the transpiler"),
        })
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Date {
    pub field: String,
    pub formats: Vec<TimeFormats>,
    pub target_field: Option<String>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub on_failure: Option<OnFailure>,
    pub tag: Option<String>,
    pub timezone: Option<TemplateString>,
    pub output_format: Option<TimeFormats>,

    // Unsupported fields
    pub locale: Option<String>,
    pub ignore_failure: Option<bool>,
}

impl Validate for Date {
    #[instrument(name = "Date::validate", skip_all, err)]
    fn validate(&self) -> Result<()> {
        self.formats.iter().try_for_each(Validate::validate)?;

        unsupported_fields!("date", self, locale);

        Ok(())
    }
}
