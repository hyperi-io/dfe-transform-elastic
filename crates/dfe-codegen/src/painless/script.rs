// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless script parsing entry point.
//!
//! Parses Painless source text into an ANTLR parse tree. The VRL transpilation
//! step has been removed and will be replaced with Rust codegen.

use super::parser::{
    self, painlesslexer::PainlessLexer, painlessparser::PainlessParser,
    painlessparser::PainlessParserContextType,
};
use antlr_rust::{common_token_stream::CommonTokenStream, DefaultErrorStrategy, InputStream};
use tracing::instrument;

/// Entry point for Painless script parsing.
pub struct Script;

impl Script {
    /// Parse Painless source text into a token stream (parse tree).
    #[instrument(name = "Script::parse", skip_all)]
    pub fn parse(text: &str) -> TokenStream<'_> {
        TokenStream(parser::parse(text))
    }
}

/// A parsed Painless token stream.
///
/// Previously this had a `.transpile()` method that converted to VRL AST.
/// That has been removed; codegen will be added in a future pass.
pub struct TokenStream<'input>(
    PainlessParser<
        'input,
        CommonTokenStream<'input, PainlessLexer<'input, InputStream<&'input str>>>,
        DefaultErrorStrategy<'input, PainlessParserContextType>,
    >,
);
