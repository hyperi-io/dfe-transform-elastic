// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The BUILT BINARY answers every subcommand that needs no broker.
//!
//! `tests/smoke.rs` covers the same startup path in process, which cannot catch
//! what only the executable has: a clap surface that stopped parsing, a
//! subcommand that panics before it prints, an exit code that says success on a
//! failure. A deploy finds those out; a test should.
//!
//! These subcommands are handled by `App::handle_local_command` BEFORE scalo's
//! lifecycle loads any config, so they need neither a broker nor a config file.

// A test asserts by panicking; the workspace lints ban that in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::process::{Command, Output};

/// Run the built binary with `args`, and return its output.
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dfe-transform-elastic"))
        .args(args)
        .output()
        .expect("the built binary runs")
}

/// Run it with one variable set on the CHILD's environment.
///
/// `std::env::set_var` is unsafe in edition 2024 and this crate forbids unsafe,
/// so the child's environment is the only one a test may set.
fn run_with_env(args: &[&str], key: &str, value: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dfe-transform-elastic"))
        .args(args)
        .env(key, value)
        .output()
        .expect("the built binary runs")
}

/// Run it and require a zero exit, reporting stderr when it is not.
fn run_ok(args: &[&str]) -> String {
    let out = run(args);
    assert!(
        out.status.success(),
        "`{}` exited {:?}: {}",
        args.join(" "),
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("stdout is utf-8")
}

#[test]
fn help_lists_every_local_subcommand() {
    let help = run_ok(&["--help"]);
    for command in [
        "sources",
        "emit-dockerfile",
        "emit-chart",
        "emit-compose",
        "emit-config",
        "emit-catalogue",
    ] {
        assert!(
            help.contains(command),
            "`--help` omits `{command}`:\n{help}"
        );
    }
}

#[test]
fn version_reports_this_crate() {
    let out = run_ok(&["version"]);
    assert!(
        out.contains(env!("CARGO_PKG_VERSION")),
        "`version` does not name the crate version: {out}"
    );
}

/// The listing an operator reads before writing `source.name`. A build that
/// carries no transforms would still exit 0, so the count is what matters.
#[test]
fn sources_lists_the_compiled_in_transforms() {
    let out = run_ok(&["sources"]);
    let names: Vec<&str> = out.lines().filter(|l| !l.trim().is_empty()).collect();
    assert!(
        names.len() >= 100,
        "only {} sources listed, expected the compiled-in catalogue",
        names.len()
    );
    assert!(
        names.contains(&"filebeat.okta.default"),
        "the shipped example's source is missing from `sources`"
    );
}

/// `emit-config` is what regenerates the committed `config.example.yaml`, so a
/// break here is a break in the drift test's own input.
#[test]
fn emit_config_prints_a_config_the_binary_accepts() {
    let emitted = run_ok(&["emit-config"]);
    assert!(emitted.contains("source:"), "no source section:\n{emitted}");
    assert!(emitted.contains("sink:"), "no sink section:\n{emitted}");

    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("config.yaml");
    std::fs::write(&path, &emitted).expect("write the emitted config");

    let out = run(&[
        "--config",
        path.to_str().expect("utf-8 path"),
        "config-check",
    ]);
    assert!(
        out.status.success(),
        "the binary rejects its own emitted config: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A deployment runs this in the pinned image and writes stdout to the file
/// dfe-engine reads, so it has to be the committed catalogue byte for byte.
#[test]
fn emit_catalogue_prints_the_committed_catalogue() {
    let emitted = run_ok(&["emit-catalogue"]);
    let committed = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/sources.yaml"))
        .expect("sources.yaml is committed");
    assert!(
        emitted == committed,
        "emit-catalogue differs from sources.yaml ({} bytes emitted, {} committed)",
        emitted.len(),
        committed.len()
    );
}

#[test]
fn emit_dockerfile_prints_a_dockerfile() {
    let out = run_ok(&["emit-dockerfile"]);
    assert!(out.contains("FROM "), "no FROM instruction:\n{out}");
}

#[test]
fn emit_compose_prints_a_service_fragment() {
    let out = run_ok(&["emit-compose"]);
    assert!(
        out.contains("dfe-transform-elastic"),
        "the fragment does not name the service:\n{out}"
    );
}

/// `emit-chart` is the one local command that can fail, and it reports the
/// failure through its EXIT CODE rather than a panic -- so a success here has
/// to mean the chart actually landed, not merely that the process ended.
#[test]
fn emit_chart_writes_a_chart_and_says_so_in_its_exit_code() {
    let dir = tempfile::tempdir().expect("temp dir");
    let target = dir.path().join("chart");
    let target_str = target.to_str().expect("utf-8 path");

    let out = run(&["emit-chart", target_str]);
    assert!(
        out.status.success(),
        "emit-chart exited {:?}: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        target.join("Chart.yaml").is_file(),
        "emit-chart exited 0 without writing Chart.yaml"
    );
}

/// An unknown subcommand must FAIL. Without this the suite would pass on a
/// binary whose clap surface had collapsed into accepting anything.
#[test]
fn an_unknown_subcommand_is_refused() {
    let out = run(&["definitely-not-a-subcommand"]);
    assert!(
        !out.status.success(),
        "an unknown subcommand exited 0, so the parser accepts anything"
    );
}

/// The flat, single-underscore form is the one override that reaches a
/// `--config` deployment, because the named file is not a scalo cascade layer
/// and the double-underscore form never sees it.
#[test]
fn a_flat_env_var_overrides_a_config_file() {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("config.yaml");
    std::fs::write(&path, run_ok(&["emit-config"])).expect("write the emitted config");
    let path = path.to_str().expect("utf-8 path");

    let clean = run(&["--config", path, "config-check"]);
    assert!(
        clean.status.success(),
        "the emitted config is refused before any override: {}",
        String::from_utf8_lossy(&clean.stderr)
    );

    // Validation refuses a source this build does not carry, so a refusal here
    // can only mean the variable reached the loaded config.
    let overridden = run_with_env(
        &["--config", path, "config-check"],
        "DFE_TRANSFORM_ELASTIC_SOURCE_NAME",
        "filebeat.nosuchthing",
    );
    assert!(
        !overridden.status.success(),
        "the flat env override did not reach a --config deployment"
    );
}
