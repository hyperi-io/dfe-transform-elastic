// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless script parsing and Rust code transpilation.
//!
//! Parses Painless source text via the ANTLR4 parser, then transpiles
//! to Rust source code through an IR (intermediate representation).

use super::ir::PainlessScript;
use super::parser::{
    self, painlesslexer::PainlessLexer, painlessparser::PainlessParser,
    painlessparser::PainlessParserContextType,
};
use super::visitor::RustVisitor;
use antlr_rust::{common_token_stream::CommonTokenStream, DefaultErrorStrategy, InputStream};
use tracing::instrument;

/// Entry point for Painless script parsing and transpilation.
pub struct Script;

impl Script {
    /// Parse Painless source text into a token stream (parse tree).
    #[instrument(name = "Script::parse", skip_all)]
    pub fn parse(text: &str) -> TokenStream<'_> {
        TokenStream(parser::parse(text))
    }

    /// Transpile Painless source text directly to IR.
    ///
    /// This is the primary entry point for the Painless→Rust pipeline.
    /// Returns a `PainlessScript` IR that can be rendered to Rust source
    /// via `emitter::emit_script()`.
    #[instrument(name = "Script::transpile", skip_all)]
    pub fn transpile(source: &str) -> anyhow::Result<PainlessScript> {
        RustVisitor::transpile(source)
    }
}

/// A parsed Painless token stream.
///
/// Retained for backward compatibility with code that needs raw parse tree access.
pub struct TokenStream<'input>(
    PainlessParser<
        'input,
        CommonTokenStream<'input, PainlessLexer<'input, InputStream<&'input str>>>,
        DefaultErrorStrategy<'input, PainlessParserContextType>,
    >,
);
