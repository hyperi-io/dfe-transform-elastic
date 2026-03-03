// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! CLI entry point for dfe-codegen.
//!
//! Converts Elastic ingest pipeline YAML into Rust transform modules.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use tracing::info;

use dfe_codegen::codegen::PipelineCodegen;
use dfe_codegen::pipeline::Pipeline;

#[derive(Parser)]
#[command(name = "dfe-codegen", about = "Elastic ingest pipeline to Rust code generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging.
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Rust transform modules from pipeline YAML.
    Generate {
        /// Path to a pipeline YAML file or directory of YAMLs.
        #[arg(short, long)]
        pipeline: PathBuf,

        /// Output directory for generated .rs files.
        #[arg(short, long)]
        output: PathBuf,

        /// Dry run — print generated code to stdout instead of writing files.
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up tracing
    let filter = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    match cli.command {
        Commands::Generate {
            pipeline,
            output,
            dry_run,
        } => cmd_generate(&pipeline, &output, dry_run),
    }
}

fn cmd_generate(pipeline_path: &Path, output_dir: &Path, dry_run: bool) -> Result<()> {
    let pipelines = collect_pipelines(pipeline_path)?;

    if pipelines.is_empty() {
        bail!("no pipeline YAML files found at {}", pipeline_path.display());
    }

    info!("found {} pipeline(s)", pipelines.len());

    if !dry_run {
        std::fs::create_dir_all(output_dir)
            .with_context(|| format!("failed to create output directory: {}", output_dir.display()))?;
    }

    for (name, yaml_path) in &pipelines {
        info!("generating: {}", name);

        let yaml = std::fs::read_to_string(yaml_path)
            .with_context(|| format!("failed to read {}", yaml_path.display()))?;

        let parsed = Pipeline::parse(&yaml)
            .with_context(|| format!("failed to parse pipeline: {}", name))?;

        let gen = PipelineCodegen::new(&parsed, name);
        let code = gen.generate()
            .with_context(|| format!("failed to generate code for: {}", name))?;

        if dry_run {
            println!("// === {} ===", name);
            println!("{}", code);
        } else {
            let out_file = output_dir.join(format!("{}.rs", name));
            std::fs::write(&out_file, &code)
                .with_context(|| format!("failed to write {}", out_file.display()))?;
            info!("  wrote: {}", out_file.display());
        }
    }

    info!("done — {} pipeline(s) generated", pipelines.len());
    Ok(())
}

/// Collect pipeline YAML files from a path (file or directory).
///
/// Returns a vec of (module_name, path) pairs.
fn collect_pipelines(path: &Path) -> Result<Vec<(String, PathBuf)>> {
    if path.is_file() {
        let name = module_name_from_path(path);
        return Ok(vec![(name, path.to_path_buf())]);
    }

    if path.is_dir() {
        let mut pipelines = Vec::new();
        for entry in std::fs::read_dir(path)
            .with_context(|| format!("failed to read directory: {}", path.display()))?
        {
            let entry = entry?;
            let p = entry.path();
            if p.is_file() && is_yaml(&p) {
                let name = module_name_from_path(&p);
                pipelines.push((name, p));
            }
        }
        pipelines.sort_by(|a, b| a.0.cmp(&b.0));
        return Ok(pipelines);
    }

    bail!("{} is not a file or directory", path.display());
}

/// Derive a Rust module name from a file path.
///
/// Strips the extension and converts hyphens/dots to underscores.
fn module_name_from_path(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .replace(['-', '.'], "_")
        .to_lowercase()
}

fn is_yaml(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("yaml" | "yml")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_name_basic() {
        assert_eq!(
            module_name_from_path(Path::new("cisco-asa.yml")),
            "cisco_asa"
        );
    }

    #[test]
    fn module_name_dots() {
        assert_eq!(
            module_name_from_path(Path::new("filebeat.module.system.yml")),
            "filebeat_module_system"
        );
    }

    #[test]
    fn module_name_nested() {
        assert_eq!(
            module_name_from_path(Path::new("/some/path/my-pipeline.yaml")),
            "my_pipeline"
        );
    }
}
