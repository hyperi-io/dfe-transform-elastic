use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use anyhow::anyhow;
use tracing::instrument;
use vector::enrichment_tables;
use vrl::{
    compiler::{CompilationResult, TypeState},
    parser::ast::{Node, RootExpr},
};

use crate::utils::{TraceAnyhow, TraceError};

use super::{timezones::TimeZones, Program, VRL};

#[derive(Debug, Clone)]
pub struct AST(pub Vec<Node<RootExpr>>);

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> vrl::prelude::fmt::Result {
        self.0.iter().try_for_each(|expr| writeln!(f, "{}", expr))
    }
}

impl From<AST> for gtmpl::Value {
    fn from(value: AST) -> Self {
        gtmpl::Value::String(value.to_string())
    }
}

impl From<AST> for vrl::parser::Program {
    fn from(value: AST) -> Self {
        vrl::parser::Program(value.0)
    }
}

impl AST {
    pub fn inspect(self) -> Self {
        println!("{ast}", ast = self.clone());

        self
    }

    #[instrument(err, skip(self), name = "AST::compile")]
    pub fn compile(self) -> anyhow::Result<Program> {
        let ast = VRL::parse(self.to_string())?;

        let mut functions = vrl::stdlib::all();
        functions.append(&mut enrichment::vrl_functions());

        let mmdb_dbs = [
            ("geoip_city", "tests/data/enrichment_tables/GeoIP2-City-Test.mmdb"),
            ("geoip_asn", "tests/data/enrichment_tables/GeoLite2-ASN-Test.mmdb"),
        ]
        .into_iter()
        .map(|(key, value)| -> anyhow::Result<_> {
            Ok((
                key.to_owned(),
                Box::new(
                    enrichment_tables::geoip::Geoip::new(enrichment_tables::geoip::GeoipConfig {
                        path: value.to_string(),
                        locale: "en".to_string(),
                    })
                    .map_err(|err| anyhow!(err))
                    .trace_error()?,
                ) as Box<dyn enrichment::Table + Send + Sync>,
            ))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

        let csv_dbs = [(
            "timezones".into(),
            TimeZones::load_from_file("tests/data/enrichment_tables/timezones.csv")?.into_enrichment_table(),
        )]
        .into();

        let tables = enrichment::TableRegistry::default();
        tables.load(mmdb_dbs);
        tables.load(csv_dbs);

        let mut config = vrl::compiler::CompileConfig::default();
        config.set_custom(tables.clone());

        let CompilationResult { program, .. } = vrl::compiler::Compiler::compile(
            &functions,
            // Re-serialising and parsing the AST is expensive but it gives us accurate pretty-printed errors
            ast.clone().into(),
            &TypeState {
                local: vrl::compiler::state::LocalEnv::default(),
                external: vrl::compiler::state::ExternalEnv::default(),
            },
            config,
        )
        .map_err(|diagnostics| CompilerError {
            program: ast.clone(),
            diagnostics,
        })
        .trace_error()?;

        tables.finish_load();

        Ok(Program {
            compiled_program: program,
            ast,
        })
    }
}

#[derive(Clone)]
pub struct CompilerError {
    pub program: AST,
    pub diagnostics: vrl::diagnostic::DiagnosticList,
}

impl Debug for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> vrl::prelude::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for CompilerError {}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        vrl::diagnostic::Formatter::new(&self.program.to_string(), self.diagnostics.clone()).fmt(f)
    }
}
