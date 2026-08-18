// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! CLI surface and the scalo service lifecycle binding.

use clap::{Parser, Subcommand};
use scalo::cli::{CliError, CommonArgs, ServiceApp, StandardCommand, VersionInfo};

use crate::config::Config;

/// Kafka-to-Kafka transform service for Beats and Elastic Agent data.
#[derive(Parser, Debug)]
#[command(name = "dfe-transform-elastic", version, about)]
pub struct App {
    #[command(flatten)]
    common: CommonArgs,

    #[command(subcommand)]
    command: Option<AppCommand>,
}

#[derive(Subcommand, Clone, Debug)]
enum AppCommand {
    /// Standard scalo commands: run, version, config-check, generate-artefacts.
    #[command(flatten)]
    Standard(StandardCommand),

    /// List the source names this build can transform.
    #[command(name = "sources")]
    Sources,
}

impl App {
    /// Handle subcommands that bypass the service lifecycle.
    ///
    /// Returns `Some(())` when handled.
    pub fn handle_local_command(&self) -> Option<()> {
        match self.command.as_ref()? {
            AppCommand::Sources => {
                for name in crate::registry::sources() {
                    println!("{name}");
                }
                Some(())
            }
            AppCommand::Standard(_) => None,
        }
    }
}

impl ServiceApp for App {
    type Config = Config;

    #[allow(clippy::unnecessary_literal_bound)]
    fn name(&self) -> &str {
        "dfe-transform-elastic"
    }

    #[allow(clippy::unnecessary_literal_bound)]
    fn env_prefix(&self) -> &str {
        "DFE_TRANSFORM_ELASTIC"
    }

    fn version_info(&self) -> VersionInfo {
        VersionInfo::new("dfe-transform-elastic", env!("CARGO_PKG_VERSION"))
    }

    fn common_args(&self) -> &CommonArgs {
        &self.common
    }

    fn command(&self) -> Option<&StandardCommand> {
        match &self.command {
            Some(AppCommand::Standard(cmd)) => Some(cmd),
            _ => None,
        }
    }

    fn load_config(&self, path: Option<&str>) -> Result<Config, CliError> {
        let config =
            Config::load(path).map_err(|e| CliError::Config(format!("failed to load: {e}")))?;
        config
            .validate()
            .map_err(|e| CliError::Config(format!("validation failed: {e}")))?;
        Ok(config)
    }

    #[cfg(feature = "kafka")]
    async fn run_service(
        &self,
        config: Config,
        runtime: scalo::cli::ServiceRuntime,
    ) -> Result<(), CliError> {
        crate::service::run(config, runtime)
            .await
            .map_err(|e| CliError::Service(e.to_string()))
    }

    #[cfg(not(feature = "kafka"))]
    async fn run_service(
        &self,
        _config: Config,
        _runtime: scalo::cli::ServiceRuntime,
    ) -> Result<(), CliError> {
        Err(CliError::Service(
            "built without the `kafka` feature; there is no transport to run".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> App {
        let mut command_line = vec!["dfe-transform-elastic"];
        command_line.extend_from_slice(args);
        App::parse_from(command_line)
    }

    #[test]
    fn standard_subcommands_reach_the_lifecycle() {
        assert!(parse(&["run"]).command().is_some());
        assert!(parse(&["version"]).command().is_some());
        assert!(parse(&["config-check"]).command().is_some());
    }

    #[test]
    fn sources_bypasses_the_lifecycle() {
        let app = parse(&["sources"]);
        assert!(app.command().is_none());
        assert!(app.handle_local_command().is_some());
    }

    #[test]
    fn run_is_not_a_local_command() {
        assert!(parse(&["run"]).handle_local_command().is_none());
    }

    #[test]
    fn env_prefix_matches_the_service_name() {
        let app = parse(&["run"]);
        assert_eq!(app.env_prefix(), "DFE_TRANSFORM_ELASTIC");
        assert_eq!(app.name(), "dfe-transform-elastic");
    }
}
