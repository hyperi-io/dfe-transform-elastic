// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! CLI surface and the scalo service lifecycle binding.

use clap::{Parser, Subcommand};
use scalo::ScalingComponent;
use scalo::cli::{CliError, CommonArgs, ServiceApp, StandardCommand, VersionInfo};
use scalo::deployment::{generate_chart, generate_compose_fragment, generate_dockerfile};

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

    /// Print the deployment contract's Dockerfile to stdout.
    #[command(name = "emit-dockerfile")]
    EmitDockerfile,

    /// Write the deployment contract's Helm chart into `dir`.
    #[command(name = "emit-chart")]
    EmitChart {
        /// Directory to write the chart into.
        #[arg(default_value = "chart/dfe-transform-elastic")]
        dir: String,
    },

    /// Print the deployment contract's docker-compose fragment to stdout.
    #[command(name = "emit-compose")]
    EmitCompose,

    /// Print the shipped default configuration as YAML.
    #[command(name = "emit-config")]
    EmitConfig,
}

impl App {
    /// Handle subcommands that bypass the service lifecycle.
    ///
    /// `None` means the command belongs to the scalo lifecycle; otherwise the
    /// exit code this process should take, so the failure path stays a return
    /// rather than a `process::exit` that skips every destructor.
    pub fn handle_local_command(&self) -> Option<i32> {
        match self.command.as_ref()? {
            AppCommand::Sources => {
                for name in crate::registry::sources() {
                    println!("{name}");
                }
                Some(0)
            }
            AppCommand::EmitDockerfile => {
                println!(
                    "{}",
                    generate_dockerfile(&crate::deployment::contract(), None)
                );
                Some(0)
            }
            AppCommand::EmitChart { dir } => Some(Self::emit_chart(dir)),
            AppCommand::EmitCompose => {
                println!(
                    "{}",
                    generate_compose_fragment(&crate::deployment::contract())
                );
                Some(0)
            }
            AppCommand::EmitConfig => {
                print!("{}", crate::deployment::default_config_yaml());
                Some(0)
            }
            AppCommand::Standard(_) => None,
        }
    }

    /// Generate the chart into `dir`, reporting the exit code.
    fn emit_chart(dir: &str) -> i32 {
        if let Err(e) = generate_chart(&crate::deployment::contract(), dir, None) {
            eprintln!("error: failed to generate Helm chart: {e}");
            return 1;
        }
        if let Err(e) = crate::deployment::retarget_keda_trigger(dir) {
            eprintln!("error: {e}");
            return 1;
        }
        eprintln!("Helm chart generated in {dir}/");
        0
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
        // Fire-and-forget, and OFF unless `version_check.enabled` says
        // otherwise, so an operator who never configures it pays nothing and
        // reaches nothing. It must not sit on the batch loop's path.
        scalo::version_check::VersionCheck::new(
            scalo::version_check::VersionCheckConfig::from_cascade(
                self.name(),
                env!("CARGO_PKG_VERSION"),
            ),
        )
        .check_on_startup();

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

    fn register_metrics(&self, manager: &scalo::metrics::MetricsManager) {
        // Registration is the side effect; the handles are discarded because
        // this path never runs the service.
        let _ = crate::metrics::TransformMetrics::register(
            manager,
            env!("CARGO_PKG_VERSION"),
            crate::metrics::TransformMetrics::commit(),
        );
    }

    fn scaling_components(&self, _config: &Config) -> Vec<ScalingComponent> {
        // `set_component` is a no-op for an unregistered name, so every
        // component the service feeds must be declared here.
        vec![
            ScalingComponent::new("kafka_lag", 0.70, 200_000.0),
            ScalingComponent::new("batch_saturation", 0.30, 1.0),
        ]
    }

    fn deployment_contract(&self) -> Option<scalo::deployment::DeploymentContract> {
        Some(crate::deployment::contract())
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
        assert_eq!(app.handle_local_command(), Some(0));
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
