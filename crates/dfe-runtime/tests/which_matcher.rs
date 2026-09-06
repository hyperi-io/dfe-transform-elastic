// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Which matcher claims a given script, pinned per script.
//!
//! `patterns.lock` locks the ladder's ORDER. This locks the ANSWER for scripts
//! we have reasoned about: a ladder edit that silently moves one of them to a
//! different matcher fails here with both names, rather than surfacing later as
//! a per-field corpus regression nobody can attribute.
//!
//! It also answers the question directly. `ran > 0` in the runtime catalogue
//! says a matcher claimed a script and nothing about which one, and reading the
//! ladder to work it out is slow and has been wrong: checkpoint_email's lookup
//! was attributed to `painless_field_tables` before this existed, and it is
//! `IndexedLookup`.
//!
//! Add a script here when a source's debt is traced to one. An empty binding is
//! a legitimate entry -- it records that NOTHING claims the script, which is a
//! different defect from claiming it and writing the wrong thing.

use dfe_runtime::painless_plan::PainlessPlan;

/// Verbatim from `pipelines/checkpoint_email/event/default.yml:434-438`.
const CHECKPOINT_EMAIL_SEVERITY: &str = "def severityValue = ctx.checkpoint_email.event.severity;\nif (severityValue > 0 && severityValue <= params.severity.length) {\n  ctx.checkpoint_email.event.put('severity_enum', params['severity'][(int)severityValue-1]);\n}";

/// Verbatim from `pipelines/first_epss/vulnerability/default.yml:86`.
const FIRST_EPSS_REFERENCE: &str = "ctx.vulnerability.reference = 'https://api.first.org/data/v1/epss?pretty=true&cve=' + ctx.vulnerability.id;";

/// Verbatim from `pipelines/system/auth/message.yml:573-583`.
const SYSTEM_SSH_CATEGORY: &str = "if (ctx.system.auth.ssh.event == \"Accepted\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\", \"session\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"success\";\n} else if (ctx.system.auth.ssh.event == \"Invalid\" || ctx.system.auth.ssh.event == \"Failed\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"failure\";\n}";

/// Verbatim from `pipelines/fortinet_fortimanager/log/default.yml:796-798`.
const FORTIMANAGER_DATE_CONCAT: &str = "if (ctx._temp?.time != null && ctx._temp?.date != null && ctx._temp?.tz != null) {\n  ctx._temp.date = ctx._temp.date + 'T' + ctx._temp.time + ctx._temp.tz;\n}";

/// Verbatim from `pipelines/zoom/webhook/phone.yml:77-81`.
const ZOOM_DURATION: &str = "ctx.event.start = ctx.zoom.phone.ringing_start_time; ctx.event.end = ctx.zoom.phone.call_end_time; ZonedDateTime start = ZonedDateTime.parse(ctx.event.start); ZonedDateTime end = ZonedDateTime.parse(ctx.event.end); ctx.event.duration = ChronoUnit.NANOS.between(start, end);";

/// Verbatim from `pipelines/beyondinsight_password_safe/asset/default.yml:27-30`.
const BEYONDINSIGHT_DROP: &str = "ctx.beyondinsight_password_safe.asset.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);";

/// Verbatim from `pipelines/beyondinsight_password_safe/asset/default.yml:63-73`.
const BEYONDINSIGHT_RENAME: &str = "Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.asset.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.asset = renamedFields;";

/// The matcher names a script binds to, most specific first.
fn binding(script: &str) -> Vec<String> {
    PainlessPlan::new(script).binding()
}

/// The variant name a binding starts with, which is what a ladder move changes.
fn heads(script: &str) -> Vec<String> {
    binding(script)
        .into_iter()
        .map(|pattern| {
            pattern
                .split(['(', ' ', '{'])
                .next()
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

#[test]
fn the_zoom_duration_reads_fields_the_script_writes_first() {
    // `NanosBetween` takes `event.start` and `event.end` as inputs, and the two
    // assignments that create them from `zoom.phone.*` are not in the plan.
    let held = binding(ZOOM_DURATION).join(" ");
    assert!(held.starts_with("NanosBetween"), "{held}");
    assert!(held.contains(r#"start: "event.start""#), "{held}");
    assert!(
        !held.contains("zoom.phone"),
        "the plan now carries the source assignments -- re-measure zoom: {held}"
    );
}

#[test]
fn the_beyondinsight_drop_is_claimed_and_its_rename_is_not() {
    // Two halves of one source, and only the second is a missing matcher.
    assert_eq!(
        heads(BEYONDINSIGHT_DROP),
        ["SentinelRemoval", "SentinelRemovalLiteral"]
    );
    assert!(
        binding(BEYONDINSIGHT_RENAME).is_empty(),
        "{:?}",
        binding(BEYONDINSIGHT_RENAME)
    );
}

#[test]
fn checkpoint_email_severity_binds_to_the_indexed_lookup() {
    assert_eq!(heads(CHECKPOINT_EMAIL_SEVERITY), ["IndexedLookup"]);
}

#[test]
fn first_epss_reference_keeps_the_literal_and_drops_the_field() {
    // `PlainAssignments` reads `'<literal>' + ctx.<path>` as the literal alone
    // and reports the plan whole, so the field never reaches the value.
    let held = binding(FIRST_EPSS_REFERENCE).join(" ");
    assert!(held.starts_with("PlainAssignments"), "{held}");
    assert!(held.contains("epss?pretty=true&cve="), "{held}");
    assert!(
        !held.contains("vulnerability.id"),
        "the concatenated field now reaches the plan -- re-measure first_epss: {held}"
    );
}

#[test]
fn the_system_ssh_ladder_flattens_its_list_literals() {
    // `EqualityLadder` unwraps a list literal to its first element as a scalar,
    // so the arm's two scalar writes are right and its two list writes are not.
    let held = binding(SYSTEM_SSH_CATEGORY).join(" ");
    assert!(held.starts_with("EqualityLadder"), "{held}");
    assert!(
        held.contains(r#"("event.category", String("authentication"))"#),
        "{held}"
    );
    assert!(
        !held.contains("session"),
        "the ladder now carries the second list element -- re-measure system: {held}"
    );
}

#[test]
fn fortimanager_date_concat_parses_to_an_empty_program() {
    // Static match and runtime apply are different questions: `matches()` is
    // true, the parsed `then` is empty, and the catalogue reads `ran 0`.
    let held = binding(FORTIMANAGER_DATE_CONCAT).join(" ");
    assert!(held.starts_with("PlainAssignments"), "{held}");
    assert!(
        held.contains("then: []"),
        "the concat now parses to something -- re-measure fortimanager: {held}"
    );
    assert!(PainlessPlan::new(FORTIMANAGER_DATE_CONCAT).matches());
}
