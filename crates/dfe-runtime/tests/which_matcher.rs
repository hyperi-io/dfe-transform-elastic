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

/// Verbatim from `pipelines/iptables/log/default.yml:264-280`.
const IPTABLES_MAPPINGS: &str = r"for (action in params.mappings) {\n  def src = ctx[action.source.object];\n  if (src != null) {\n    Map map = action.map;\n    String key = src[action.source.key];\n    String mapping = map[key];\n    if (mapping != null) {\n      Map dst = ctx[action.destination.object];\n      if (dst == null) {\n          dst = new HashMap();\n          ctx[action.destination.object] = dst;\n      }\n      dst[action.destination.key] = mapping;\n    }\n  }\n}";

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/infoblox_threat_defense_event/default.rs`,
/// which is `pipelines/infoblox_threat_defense/event/default.yml`.
const INFOBLOX_SEVERITY: &str = "ctx.event = ctx.event ?: [:];\\nif (ctx.infoblox_threat_defense.event.severity >= 0 && ctx.infoblox_threat_defense.event.severity <= 3 ) { // Severity level - 0,1,2,3 denotes Low severity\\n  ctx.event.severity = 21;\\n} else if (ctx.infoblox_threat_defense.event.severity >= 4 && ctx.infoblox_threat_defense.event.severity <= 6) { // Severity level - 4,5,6 denotes Medium severity\\n  ctx.event.severity = 47;\\n} else if (ctx.infoblox_threat_defense.event.severity == 7 || ctx.infoblox_threat_defense.event.severity == 8) { // Severity level - 7 and 8 denotes High severity\\n  ctx.event.severity = 73;\\n} else if (ctx.infoblox_threat_defense.event.severity == 9 || ctx.infoblox_threat_defense.event.severity == 10) { // Severity level - 9 and 10 denotes Critical severity\\n  ctx.event.severity = 99;\\n}";

/// Verbatim from the same file, the processor above the severity ladder.
const INFOBLOX_MESSAGE_QUOTES: &str = r#"if (ctx.cef.extensions.containsKey('message') && ctx.cef.extensions.message != null && ctx.cef.extensions.message instanceof String) {\n  if (ctx.cef.extensions.message.startsWith('\"') && ctx.cef.extensions.message.endsWith('\"') && ctx.cef.extensions.message.length() >= 2) {\n    ctx.cef.extensions.message = ctx.cef.extensions.message.substring(1, ctx.cef.extensions.message.length() - 1);\n  }\n}\n"#;

/// The four generated call sites in
/// `crates/dfe-transforms/src/filebeat/falco_alerts/default.rs`, which are the
/// whole of falco's parity debt. sysdig_alerts ships the technique script over
/// its own tag list.
const FALCO_MOUNTS: &str = r#"if (ctx.falco.output_fields?.container?.mounts != null) {\n    def mountsString = ctx.falco.output_fields.container.mounts;\n    def mountItems = mountsString.splitOnToken(' ');            \n    def mountsList = [];\n    for (int i = 0; i < mountItems.length; i++) {\n        def mountItem = mountItems[i];\n        def parts = mountItem.splitOnToken(':');\n        def mountRecord = [:];\n        mountRecord.source = parts.length > 0 ? parts[0] : null;\n        mountRecord.dest = parts.length > 1 ? parts[1] : null;\n        mountRecord.mode = parts.length > 2 ? parts[2] : null;\n        mountRecord.rdrw = parts.length > 3 ? parts[3] : null;\n        mountRecord.propagation = parts.length > 4 ? parts[4] : null;\n        mountsList.add(mountRecord);\n    }\n    ctx['falco.container.mounts'] = mountsList;\n} else {\n    ctx['falco.container.mounts'] = null;\n}\n"#;

const FALCO_TECHNIQUE: &str = r#"def mitreRegex = /T\\d{4}/;\nfor (int i = 0; i < ctx?.falco?.tags.length; i++) {\n    def tag = ctx?.falco?.tags[i];\n    def matcher = mitreRegex.matcher(tag);\n    if (matcher.find()) {\n        ctx['threat.technique.id'] = [matcher.group()];\n        break;\n    }\n}\n"#;

const FALCO_ISO8601: &str = r#"if (ctx.falco?.output_fields?.evt?.time != null) {\n    def timeField = ctx.falco.output_fields.evt.time;\n    def inputFormat = new SimpleDateFormat(\"yyyy-MM-dd'T'HH:mm:ss.SSSZ\");\n        if (timeField.iso8601 != null) {\n            if (timeField.iso8601 instanceof String) {\n                def formatted = inputFormat.parse(timeField.iso8601);\n                ctx['@timestamp'] = formatted;\n                ctx.falco.output_fields.evt.time.iso8601 = formatted;\n            } else if (timeField.iso8601 instanceof Long) {\n                long milliseconds = timeField.iso8601 / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n                ctx.falco.output_fields.evt.time.iso8601 = milliseconds;\n            }\n        } else if (timeField.rawtime != null) {\n            if (timeField.rawtime instanceof String) {\n                def formatted = inputFormat.parse(timeField.rawtime);\n                ctx['@timestamp'] = formatted;\n            } else if (timeField.rawtime instanceof Long) {\n                long milliseconds = timeField.rawtime / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n            }\n        } else {\n            if (timeField instanceof String) {\n                def formatted = inputFormat.parse(timeField);\n                ctx['@timestamp'] = formatted;\n            } else if (timeField instanceof Long) {\n                long milliseconds = timeField / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n            }\n        }\n} else {\n    def timeField = ctx.falco.output_fields.event.time;\n    def inputFormat = new SimpleDateFormat(\"yyyy-MM-dd'T'HH:mm:ss.SSSZ\");\n    if (ctx.falco?.output_fields?.event?.time != null) {\n      if (timeField instanceof String) {\n          def formatted = inputFormat.parse(timeField);\n          ctx['@timestamp'] = formatted;\n      } else if (timeField instanceof Long) {\n          long milliseconds = timeField / 1000000;\n          ctx['@timestamp'] = new Date(milliseconds);\n      }\n    }\n}\n"#;

const FALCO_ARGS: &str = r#"if (ctx.falco.output_fields?.proc?.exepath != null && ctx.falco.output_fields?.proc?.args != null) {\n    def path = ctx.falco.output_fields.proc.exepath;\n    def args = ctx.falco.output_fields.proc.args;\n    def argItems = args.splitOnToken(' ');\n    def finalList = [];\n    finalList.add(path);\n    for (int i = 0; i < argItems.length; i++) {\n        finalList.add(argItems[i]);\n    }\n    ctx['process']['args'] = finalList;\n}\n"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/jamf_pro_events/default.rs`, which is
/// `pipelines/jamf_pro/events/default.yml`.
const JAMF_PRO_CATEGORIES: &str = r#"def action = ctx.event?.action;\nif (action == null) {\n  return;\n}\ndef entry = params.actions.get(action);\nif (entry == null) {\n  return;\n}\ndef cats = new ArrayList();\ndef types = new ArrayList();\nif (entry.category != null) { cats.addAll(entry.category); }\nif (entry.type != null) { types.addAll(entry.type); }\nif (types.isEmpty()) { types.add('info'); }\nctx.event = ctx.event ?: [:];\nif (!cats.isEmpty()) { ctx.event.category = cats; }\nctx.event.type = types;"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/cloudflare_logpush_firewall_event/default.rs`,
/// which is `pipelines/cloudflare_logpush/firewall_event/default.yml`.
const CLOUDFLARE_QUERY_CUT: &str = r"ctx.url.query = ctx.url.query.substring(1);\n";

/// Verbatim from `pipelines/cloudflare_logpush/audit/default.yml:59-65`, the
/// single-field spelling that seventeen call sites share.
const CLOUDFLARE_WHEN_TO_MILLI: &str = r"long t = (long)(ctx.json.When);\nif (t > (long)(1e18)) {\n  ctx.json.When = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.When = t*(long)(1e3)\n}\n";

/// Verbatim from `pipelines/cloudflare_logpush/network_session/default.yml`,
/// the helper spelling.
const CLOUDFLARE_SESSION_TO_MILLI: &str = r"def convertToMillis(long timestamp) {\n  if (timestamp > (long)(1e18)) {\n    return timestamp/(long)(1e6)\n  } else if (timestamp < (long)(1e10))  {\n    return timestamp*(long)(1e3)\n  }\n  return timestamp\n}\nif (ctx.json?.SessionStartTime != null && ctx.json.SessionStartTime instanceof Number) {\n  ctx.json.SessionStartTime = convertToMillis(ctx.json.SessionStartTime);\n}\nif (ctx.json?.SessionEndTime != null && ctx.json.SessionEndTime instanceof Number) {\n  ctx.json.SessionEndTime = convertToMillis(ctx.json.SessionEndTime);\n}\n";

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/rapid7_insightvm_asset_vulnerability/default.rs`,
/// which is `pipelines/rapid7_insightvm/asset_vulnerability/default.yml`.
const RAPID7_SCANNER_NAME: &str = r"ctx.vulnerability = ctx.vulnerability ?: [:];\nctx.vulnerability.scanner = ctx.vulnerability.scanner ?: [:];\nfor (def o: ctx.rapid7_insightvm.asset_vulnerability.unique_identifiers) {\n  if (o.source == 'R7 Agent') {\n    ctx.vulnerability.scanner.put('name', o.id);\n    return;\n  }\n}\n";

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
fn the_jamf_pro_categories_bind_to_the_row_lookup_with_both_empty_rules() {
    // The binding was EMPTY, and the whole source read as needing polish: 22 of
    // its 24 captures were short by `event.category` and `event.type` alone.
    // Both writes are LISTS, and the two columns take different answers to an
    // empty one -- `category` is not written, `type` reads `['info']`.
    let held = binding(JAMF_PRO_CATEGORIES).join(" ");
    assert!(held.starts_with("RowOrDefaults"), "{held}");
    assert!(held.contains(r#"subject: "event.action""#), "{held}");
    assert!(held.contains(r#"table: "actions""#), "{held}");
    assert!(
        held.contains(
            r#"target: "event.category", member: "category", only_if_unset: false, empty: Skip"#
        ),
        "{held}"
    );
    assert!(
        held.contains(
            r#"target: "event.type", member: "type", only_if_unset: false, empty: Fill(Array [String("info")])"#
        ),
        "{held}"
    );
    // An action with no row writes nothing at all -- the script returns early.
    assert!(held.contains("defaults: []"), "{held}");
}

/// Verbatim from `pipelines/endace/flow/endace.yml:69`, the subtraction beside
/// the addition above.
const ENDACE_HALF_TIMEDELTA_START: &str =
    "ctx._conf.event.start = ctx._conf.event.start - ctx._conf.timedelta/2";

/// Verbatim from `pipelines/endace/flow/endace.yml:59`, the multiply that
/// builds the window the two above halve.
const ENDACE_TIMEDELTA: &str = "ctx._conf.timedelta = ctx._conf.endace_view_window * 60 * 1000";

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/citrix_adc_log/default.rs`, which is
/// `pipelines/citrix_adc/log/default.yml`.
const CITRIX_CUSTOM_DATE: &str = r#"def zone = ctx.event?.timezone != null ? ZoneId.of(ctx.event.timezone) : null;\ndef formatter = DateTimeFormatter.ofPattern(ctx._conf.custom_date_format);\ndef outFormatter = DateTimeFormatter.ofPattern(\"yyyy-MM-dd'T'HH:mm:ss.SSSXXX\");\n\nparams.fields.forEach(field -> {\n  if (!ctx._tmp?.containsKey(field)) {\n    return true;\n  }\n\n  try {\n    def localDateTime = LocalDateTime.parse(ctx._tmp[field], formatter);\n    ctx.citrix_adc.log[field] = outFormatter.format(ZonedDateTime.of(localDateTime, zone));\n  } catch (Exception e) {\n    /* Intentionally ignored */\n    return true;\n  }\n});"#;

/// endace's three arithmetic scripts, which between them build the URL in
/// `event.reference`.
///
/// The two halves were CLAIMED and wrote nothing: `SumOfFields` read
/// `_conf.timedelta/2` as a field name, and `GuardedDivide` read
/// `_conf.event.start - ctx._conf.timedelta` as one. Both then found no such
/// field and returned handled, so the epochs reached the URL unshifted and
/// nothing counted the miss.
#[test]
fn the_endace_arithmetic_binds_to_the_combination_with_its_divisor() {
    for (name, script, op) in [
        ("end", ENDACE_HALF_TIMEDELTA, "Add"),
        ("start", ENDACE_HALF_TIMEDELTA_START, "Subtract"),
    ] {
        let held = binding(script).join(" ");
        assert!(held.starts_with("CombineFields"), "{name}: {held}");
        assert!(held.contains(&format!("op: {op}")), "{name}: {held}");
        assert!(
            held.contains(r#"path: "_conf.timedelta", divisor: Some(2)"#),
            "{name}: {held}"
        );
    }

    // The window is MINUTES, so both literals belong to the factor.
    let held = binding(ENDACE_TIMEDELTA).join(" ");
    assert!(held.starts_with("ScaleField"), "{held}");
    assert!(held.contains("Long(60000)"), "{held}");
}

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/nagios_xi_service/default.rs`.
const NAGIOS_FREE_SPACE: &str = r#"if(ctx.nagios_xi?.service?.containsKey(\"root_partition\") == true) {\n    ctx.nagios_xi.service.root_partition.free_space = ctx.nagios_xi.service.root_partition.total_space - ctx.nagios_xi.service.root_partition.used_space\n}\n"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/rubrik_managed_volumes/default.rs`,
/// truncated at the second block, which reads a different subtree.
const RUBRIK_FREE_SIZE: &str = r"if (ctx.rubrik.managed_volumes?.volume_size?.bytes != null && ctx.rubrik.managed_volumes?.used_size?.bytes != null) {\n    ctx.rubrik.managed_volumes.free_size = [:];\n    ctx.rubrik.managed_volumes.free_size.bytes = ctx.rubrik.managed_volumes.volume_size.bytes - ctx.rubrik.managed_volumes.used_size.bytes;\n} else {\n    ctx.rubrik.managed_volumes.free_size = [:];\n    ctx.rubrik.managed_volumes.free_size.bytes = 0;\n}\n";

/// The other two subtractions in the tree, which the same widening reaches.
///
/// rubrik is the audit: it writes the SAME target in both arms of an `if`, so
/// a matcher that reads one arm would drop the zero the other writes.
#[test]
fn a_subtraction_binds_only_where_the_target_is_written_once() {
    let held = binding(NAGIOS_FREE_SPACE).join(" ");
    assert!(held.starts_with("CombineFields"), "{held}");
    assert!(held.contains("op: Subtract"), "{held}");
    assert!(
        held.contains(r#"path: "nagios_xi.service.root_partition.total_space", divisor: None"#),
        "{held}"
    );

    assert!(
        !binding(RUBRIK_FREE_SIZE)
            .first()
            .is_some_and(|held| held.starts_with("CombineFields")),
        "{:?}",
        binding(RUBRIK_FREE_SIZE)
    );
}

/// citrix_adc's five dates, reparsed with the pattern the document carries.
///
/// The binding was EMPTY, so nothing wrote `citrix_adc.log.*` and the date
/// processors behind it read `10/08/2024` with their own hard-coded
/// `MM/dd/yyyy`. Every such event landed in October.
#[test]
fn the_citrix_custom_date_binds_to_the_configured_format() {
    let held = binding(CITRIX_CUSTOM_DATE).join(" ");
    assert!(held.starts_with("ConfiguredDateFormat"), "{held}");
    assert!(held.contains(r#"names: "fields""#), "{held}");
    assert!(
        held.contains(r#"format: "_conf.custom_date_format""#),
        "{held}"
    );
    assert!(held.contains(r#"zone: Some("event.timezone")"#), "{held}");
    assert!(held.contains(r#"source: "_tmp""#), "{held}");
    assert!(held.contains(r#"target: "citrix_adc.log""#), "{held}");
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
fn the_iptables_mappings_bind_only_to_member_mappings() {
    // `MemberMappings` sits LAST in the params ladder, and the params ladder
    // runs before the text one. A single head is therefore two facts: no
    // earlier matcher claims the script, and no text matcher does either.
    assert_eq!(heads(IPTABLES_MAPPINGS), ["MemberMappings"]);
    assert_eq!(
        binding(IPTABLES_MAPPINGS),
        [r#"MemberMappings("mappings")"#]
    );
}

#[test]
fn the_infoblox_severity_ladder_binds_to_the_band_reader_with_numbers() {
    // The bands are written subject-first over a full `ctx` path with no local
    // and the values are numbers. `RangeLadder`, the position above, declines
    // each of those.
    let held = binding(INFOBLOX_SEVERITY).join(" ");
    assert!(held.starts_with("BandLadder"), "{held}");
    assert!(
        held.contains(r#"subject: "infoblox_threat_defense.event.severity""#),
        "{held}"
    );
    assert!(held.contains("Number(21)"), "{held}");
    assert!(held.contains("Number(99)"), "{held}");
    assert!(
        !held.contains("Text("),
        "a band naming a string writes the wrong type here: {held}"
    );
}

#[test]
fn the_infoblox_message_quotes_bind_to_the_strip_and_not_to_guarded_copy() {
    // The outer guard carries a `!= null`, which is `GuardedCopy`'s trigger,
    // and it sits BELOW this arm. Reverse that order and the field is copied
    // onto itself with its quotes intact, taking every later copy with it.
    let held = binding(INFOBLOX_MESSAGE_QUOTES).join(" ");
    assert!(held.starts_with("StripSurroundingPair"), "{held}");
    assert!(held.contains(r#"field: "cef.extensions.message""#), "{held}");
    assert!(held.contains(&format!("open: {:?}", '"')), "{held}");
    assert!(held.contains(&format!("close: {:?}", '"')), "{held}");
}

#[test]
fn falco_binds_all_four_of_its_scripts_to_their_own_matchers() {
    // The mounts and args arms sit ABOVE `AppendEach`, whose arm returns
    // whether or not its own parse succeeded. Below it both scripts bind to
    // nothing at all, which is where falco's 22 events went.
    let held = binding(FALCO_MOUNTS).join(" ");
    assert!(held.starts_with("SplitIntoRecords"), "{held}");
    assert!(held.contains("null_when_absent: true"), "{held}");
    assert!(held.contains(r#"("propagation", 4)"#), "{held}");

    let held = binding(FALCO_ARGS).join(" ");
    assert!(held.starts_with("PrependSplit"), "{held}");
    assert!(
        held.contains(r#"head: "falco.output_fields.proc.exepath""#),
        "{held}"
    );

    let held = binding(FALCO_TECHNIQUE).join(" ");
    assert!(held.starts_with("FirstMatchInList"), "{held}");
    assert!(held.contains(r#"list: "falco.tags""#), "{held}");
    assert!(held.contains("wrap_in_list: true"), "{held}");

    let held = binding(FALCO_ISO8601).join(" ");
    assert!(held.starts_with("LongDivide"), "{held}");
    assert!(held.contains("divisor: 1000000"), "{held}");
    assert!(
        held.contains(r#"target: "falco.output_fields.evt.time.iso8601""#),
        "{held}"
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

#[test]
fn the_rapid7_scanner_name_binds_to_the_list_member_select() {
    // The binding was EMPTY, and the whole of rapid7_insightvm's remaining debt
    // is this one field on twenty events. The matcher that reads the same loop
    // for crowdstrike's boolean flag now reads the member copy as well, so the
    // key it compares and the member it takes both have to show here.
    let held = binding(RAPID7_SCANNER_NAME).join(" ");
    assert!(held.starts_with("ListMemberSelect"), "{held}");
    assert!(
        held.contains(r#"list: "rapid7_insightvm.asset_vulnerability.unique_identifiers""#),
        "{held}"
    );
    assert!(held.contains(r#"key: "source""#), "{held}");
    assert!(held.contains(r#"values: ["R7 Agent"]"#), "{held}");
    assert!(
        held.contains(r#"Copy { member: "id", target: "vulnerability.scanner.name" }"#),
        "{held}"
    );
}

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/jamf_pro_inventory/default.rs`, which is
/// `pipelines/jamf_pro/inventory/default.yml`.
const JAMF_PRO_SNAKE_CASE: &str = r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\n\nif (ctx.jamf_pro.inventory != null) {\n  ctx.jamf_pro.inventory = keysToSnakeCase(ctx.jamf_pro.inventory);\n}\n"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/azure_signinlogs/default.rs`, the same
/// helper WITHOUT the regex's leading `_?`.
const AZURE_SIGNINLOGS_SNAKE_CASE: &str = r#"Map keysToSnakeCase(Map m) {\n  def regex = /([a-z])([A-Z]+)/;\n  def out = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    out.put(k, v);\n  }\n\n  return out;\n}\n\nctx.azure['signinlogs'] = keysToSnakeCase(ctx.azure.signinlogs);\n"#;

#[test]
fn the_snake_case_helper_binds_to_the_rule_its_own_body_spells() {
    // Both bound to `BeforeEveryUpper`, which is the character-walk copy of the
    // helper and not the one either of these ships. jamf_pro's inventory then
    // wrote `file_vault2_status` where Elasticsearch writes `file_vault2status`.
    let held = binding(JAMF_PRO_SNAKE_CASE).join(" ");
    assert_eq!(
        held,
        r#"KeysToSnakeCase(Some("jamf_pro.inventory"), CamelBreak)"#
    );

    // The same helper minus the `_?`, which is a different rewrite: it keeps
    // the underscore the other spelling's match eats.
    let held = binding(AZURE_SIGNINLOGS_SNAKE_CASE).join(" ");
    assert_eq!(
        held,
        r#"KeysToSnakeCase(Some("azure.signinlogs"), CamelBreakKeepingUnderscore)"#
    );
}

#[test]
fn the_cloudflare_query_cut_binds_to_the_leading_cut_and_carries_its_count() {
    // `PlainAssignments` sits above and declines: its right-hand-side grammar
    // has no method call, so `substring` is not a value it can read.
    assert_eq!(
        binding(CLOUDFLARE_QUERY_CUT),
        [r#"DropLeadingChars { field: "url.query", count: 1 }"#]
    );
}

#[test]
fn both_cloudflare_epoch_spellings_bind_to_the_one_rescale() {
    // Seventeen call sites spell the conversion inline over one field and
    // three declare a helper and call it per field. One matcher reads both,
    // and the fields it recovered are what the binding has to show.
    assert_eq!(
        binding(CLOUDFLARE_WHEN_TO_MILLI),
        [
            r#"EpochToMillis(EpochToMillis { fields: [("json.When", false)], parse_strings: false })"#
        ]
    );
    assert_eq!(
        binding(CLOUDFLARE_SESSION_TO_MILLI),
        [concat!(
            r#"EpochToMillis(EpochToMillis { fields: ["#,
            r#"("json.SessionStartTime", false), ("json.SessionEndTime", false)"#,
            r#"], parse_strings: false })"#
        )]
    );
}
