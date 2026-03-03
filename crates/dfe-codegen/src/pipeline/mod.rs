// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Elastic ingest pipeline parsing and code generation.
//!
//! Parses Elastic ingest pipeline YAML into processor structs, validates
//! configuration, and generates Rust transform functions.

pub mod conditional;
pub mod dynamic_path;
pub mod field;
pub mod on_failure;
#[allow(clippy::module_inception)]
pub mod pipeline;
pub mod processors;
pub mod script_template;
pub mod template_string;

pub use pipeline::{Pipeline, Processor};

/// Validate exists so we can parse the full configuration options for each
/// processor and throw errors if we encounter options we don't have support
/// for yet. This allows us to incrementally evolve the compatibility of the
/// processors and get errors when we encounter an unsupported option.
pub trait Validate {
    fn validate(&self) -> anyhow::Result<()>;
}

/// Generate Rust source code from a parsed pipeline element.
///
/// This replaces the original VRL `Transpile` trait — instead of emitting
/// VRL AST nodes, we emit Rust source code strings that use dfe-runtime.
pub trait Codegen {
    /// Generate Rust source code for this pipeline element.
    fn codegen(&self) -> anyhow::Result<String>;
}

macro_rules! unsupported_fields {
    ($processor:literal, $self:ident, $field:ident) => {
        if $self.$field.is_some() {
            return Err(anyhow::anyhow!("codegen does not currently support '{}' in '{}' processor", stringify!($field), $processor).into())
        }
    };
    ($processor:literal, $self:ident, $field:ident, $($fields:ident),*) => (
        unsupported_fields!($processor, $self, $field);
        unsupported_fields!($processor, $self, $($fields),*);
    )
}

pub(crate) use unsupported_fields;

#[cfg(test)]
macro_rules! unsupported_fields_tests {
    ($name:literal, $configuration:literal, $key:ident => $value:literal) => {
        paste::paste! {
          #[test]
          pub fn [<unsupported_ $key>]() {
              let configuration = format!($configuration, stringify!($key), $value);

              assert_eq!(
                  crate::pipeline::Pipeline::parse(&configuration).expect_err(&format!("expected parser to return an error being unable to support '{}' in '{}'", stringify!($key), $name)).root_cause().to_string(),
                  format!(
                      "codegen does not currently support '{}' in '{}' processor",
                      stringify!($key),
                      $name
                  )
              );
          }
        }
    };
    ($name:literal, $configuration:literal, $key:ident => $value:literal, $($keys:ident => $values:literal),*) => {
      unsupported_fields_tests!($name, $configuration, $key => $value);
      unsupported_fields_tests!($name, $configuration, $($keys => $values),*);
    }
}

#[cfg(test)]
pub(crate) use unsupported_fields_tests;
