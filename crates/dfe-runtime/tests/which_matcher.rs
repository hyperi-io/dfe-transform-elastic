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

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/first_epss_vulnerability/default.rs`,
/// which is `pipelines/first_epss/vulnerability/default.yml:86`.
const FIRST_EPSS_REFERENCE: &str = r#"ctx.vulnerability.reference = 'https://api.first.org/data/v1/epss?pretty=true&cve=' + ctx.vulnerability.id;"#;

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

/// Verbatim from `pipelines/kolide/auth/categorize.yml:38-51`.
const KOLIDE_CATEGORIZE: &str = "def action = ctx.event.action;\nctx.event.kind = 'event';\n\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = ['authentication'];\n  ctx.event.type = ['info'];\n}";

/// Verbatim from zeek's duration scale.
const ZEEK_DURATION: &str = "ctx.event.duration = Math.round(ctx.temp.duration * params.scale)";

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/jamf_protect_telemetry/default.rs`,
/// which is `pipelines/jamf_protect/telemetry/pipeline_event_authentication.yml:30`.
///
/// This one and the three concat constants after it are the ESCAPED call-site
/// text, not the pipeline's own: the generator writes a folded YAML block out
/// as an escaped string, so wiz's double quotes reach the runtime as `\"` and
/// jamf's newline as two characters. `normalise` resolves both before any
/// matcher reads them, and a test written in the resolved form would pass over
/// a defect in that step.
const JAMF_PROTECT_REASON: &str = r#"ctx.event.reason = 'A user authentication happened using ' + ctx.jamf_protect.telemetry.authentication_method;\n"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/wiz_issue/default.rs`, which is
/// `pipelines/wiz/issue/default.yml:103`.
const WIZ_EVENT_URL: &str = r#"ctx.event.url = \"https://app.wiz.io/issues#~(filters~(status~())~issue~'\" + ctx.event.id + \")\";\n"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/wiz_cloud_configuration_finding/default.rs`,
/// which is `pipelines/wiz/cloud_configuration_finding/default.yml:134`.
const WIZ_CONFIGURATION_URL: &str = r#"ctx.event.url = \"https://app.wiz.io/findings/configuration-findings/cloud#~(filters~(status~()~rule~(equals~(~'\" + ctx.json.rule.id + \")))~groupBy~(~)~entity~(~'\" + ctx.event.id + \"*2cCONFIGURATION_FINDING))\";\n"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/tychon_softwareinventory/rest.rs`, which
/// is `pipelines/tychon/softwareinventory/rest.yml:72`.
const TYCHON_PACKAGE_CPE: &str = r#"ctx.tychon.package.cpe = \"cpe:/a:\" + ctx.tychon.package.name + \":\" + ctx.tychon.package.version"#;

/// Verbatim from `pipelines/endace/flow/endace.yml:64`.
const ENDACE_HALF_TIMEDELTA: &str =
    "ctx._conf.event.end = ctx._conf.event.end + ctx._conf.timedelta/2";

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
fn the_kolide_categorise_binds_to_a_structurally_correct_row_lookup() {
    // `RowOrDefaults` records the table, the three columns and both defaults,
    // so kolide's defect is not visible in the binding and needs the corpus.
    let held = binding(KOLIDE_CATEGORIZE).join(" ");
    assert!(held.starts_with("RowOrDefaults"), "{held}");
    assert!(held.contains(r#"subject: "event.action""#), "{held}");
    assert!(held.contains(r#"table: "exact""#), "{held}");
    assert!(
        held.contains(r#"defaults: [("event.category", Array [String("authentication")])"#),
        "{held}"
    );
}

#[test]
fn the_zeek_duration_binds_to_a_bare_scale() {
    // The script is `Math.round(<field> * params.scale)` and the plan is a bare
    // `Scale`. That is NOT the defect: `try_scale` stores an integer whenever
    // the product is whole, and zeek's durations are -- 0.103708982 * 1e9 is
    // exactly 103708982.0 in f64. Its eight wrong durations have another cause.
    assert_eq!(heads(ZEEK_DURATION), ["Scale"]);
}

#[test]
fn the_concat_keeps_its_fields_in_both_quote_styles() {
    // `Rhs::Concat` reads the whole expression. Before it, `literal_value`
    // answered the LEADING LITERAL alone and reported the write done, so each
    // of these wrote a bare prefix where the joined string belonged. Single
    // quotes and double quotes alike, and the double-quoted ones arrive
    // escaped.
    //
    // tychon is here because it is the form `ConcatAssignment` cannot take:
    // its statement carries no terminator and that arm opens with
    // `strip_suffix(';')?`. Two readers of one grammar, and this is the half
    // only the `Program` one reaches.
    for (name, script, literal, fields) in [
        (
            "jamf_protect",
            JAMF_PROTECT_REASON,
            "A user authentication happened using ",
            &["jamf_protect.telemetry.authentication_method"][..],
        ),
        (
            "wiz issue",
            WIZ_EVENT_URL,
            "https://app.wiz.io/issues",
            &["event.id"][..],
        ),
        (
            "tychon",
            TYCHON_PACKAGE_CPE,
            "cpe:/a:",
            &["tychon.package.name", "tychon.package.version"][..],
        ),
    ] {
        let held = binding(script).join(" ");
        assert!(held.starts_with("PlainAssignments"), "{name}: {held}");
        assert!(held.contains(literal), "{name}: {held}");
        assert!(held.contains("Concat("), "{name}: {held}");
        for field in fields {
            assert!(
                held.contains(&format!("Field({field:?})")),
                "{name} lost {field}: {held}"
            );
        }
    }
}

#[test]
fn the_wiz_configuration_url_stays_with_the_concat_assignment_arm() {
    // The one place the two readers of this grammar are told apart. wiz's
    // configuration URL carries no inline `!= null`, so `GuardedCopy` -- the
    // position above -- declines it whatever `Program` can parse, and the arm
    // written for the join keeps it. Contrast fortimanager below.
    let held = binding(WIZ_CONFIGURATION_URL).join(" ");
    assert!(held.starts_with("ConcatAssignment"), "{held}");
    assert!(held.contains(r#"Field("json.rule.id")"#), "{held}");
    assert!(held.contains(r#"Field("event.id")"#), "{held}");
}

#[test]
fn report_the_endace_binding() {
    println!(
        "endace half-timedelta: {:?}",
        binding(ENDACE_HALF_TIMEDELTA)
    );
}

#[test]
fn checkpoint_email_severity_binds_to_the_indexed_lookup() {
    assert_eq!(heads(CHECKPOINT_EMAIL_SEVERITY), ["IndexedLookup"]);
}

#[test]
fn first_epss_reference_keeps_both_halves_of_its_url() {
    // The whole source was this one expression, and reading the literal alone
    // put the same bare prefix on all nine of its events.
    let held = binding(FIRST_EPSS_REFERENCE).join(" ");
    assert!(held.starts_with("PlainAssignments"), "{held}");
    assert!(held.contains("epss?pretty=true&cve="), "{held}");
    assert!(held.contains(r#"Field("vulnerability.id")"#), "{held}");
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
fn fortimanager_date_concat_is_read_by_guarded_copy_and_still_joins() {
    // The one script both readers of this grammar can take, and the winner is
    // NOT the arm named after it. `GuardedCopy` is gated on an inline
    // `!= null`, which this script alone among the joins carries, and it sits
    // one position ABOVE `ConcatAssignment`; a `Program` that can write the
    // join is enough to make it claim the script.
    //
    // That is recorded rather than corrected because the two agree on the
    // answer -- all three fields, the separator, and nothing written when a
    // part is absent. The pin is here so the day they stop agreeing this
    // fails with both names, instead of surfacing as a corpus swing nobody can
    // attribute. Fortimanager measured 31/31 either way.
    let held = binding(FORTIMANAGER_DATE_CONCAT).join(" ");
    assert!(held.starts_with("GuardedCopy"), "{held}");
    assert!(
        held.contains(
            r#"Concat([Field("_temp.date"), Literal("T"), Field("_temp.time"), Field("_temp.tz")])"#
        ),
        "{held}"
    );
    assert!(PainlessPlan::new(FORTIMANAGER_DATE_CONCAT).matches());
}
