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

use dfe_runtime::Event;
use dfe_runtime::painless_plan::{PainlessPlan, painless_exec_plan, painless_exec_plan_params};
use serde_json::json;

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

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/sysdig_event/default.rs`, which is
/// `pipelines/sysdig/event/default.yml`.
const SYSDIG_PARSE_DATE: &str = r"def parseDate(def rawtimestamp) {\n  long timestamp;\n  if (rawtimestamp instanceof String) {\n    timestamp = Long.parseLong(rawtimestamp);\n  } else if (rawtimestamp instanceof long) {\n    timestamp = (long) rawtimestamp;\n  }\n  if (String.valueOf(timestamp).length() == 19) {\n    long epoch = timestamp / 1000000000L;\n    long seconds = timestamp % 1000000000L;\n    return Instant.ofEpochSecond(epoch, seconds).atZone(ZoneOffset.UTC);\n  }\n  return '';\n} if (ctx.json?.timestamp != null) {\n  ctx.json.timestamp = parseDate(ctx.json.timestamp);\n} if (ctx.sysdig?.event?.content?.fields?.proc?.pid_ts != null) {\n  ctx.sysdig.event.content.fields.proc.pid_ts = parseDate(ctx.sysdig.event.content.fields.proc.pid_ts);\n} if (ctx.sysdig?.event?.content?.fields?.proc?.ppid_ts != null) {\n  ctx.sysdig.event.content.fields.proc.ppid_ts = parseDate(ctx.sysdig.event.content.fields.proc.ppid_ts);\n}\n";

/// The other two scripts in the tree that call `Instant.ofEpochSecond`, which
/// the reader for sysdig's helper must decline.
///
/// slack builds `@timestamp` from a microsecond field with no helper
/// declaration at all; jamf_protect calls the same constructor inline, once per
/// process, inside a hundred-line mapping that writes forty other fields. A
/// reader triggered on the constructor rather than on the helper would claim
/// both and write almost nothing.
const SLACK_ACTION_TIMESTAMP: &str = r#"def secs = (long)(ctx.slack.audit.details.action_timestamp/1e6);\ndef nanos = (long)(ctx.slack.audit.details.action_timestamp % 1e6) * 1000;\nctx[\"@timestamp\"] = Instant.ofEpochSecond(secs, nanos).atZone(ZoneId.of(\"UTC\"));\nctx.slack.audit.details.remove(\"action_timestamp\");\n"#;

const JAMF_PROTECT_PROCESS_START: &str = r"if (ctx.jamf_protect?.alerts?.input?.related?.processes != null && ctx.jamf_protect.alerts.input.related.processes.size() > 0) {\n    def process = ctx.jamf_protect.alerts.input.related.processes[0];\n    ctx.process = ctx.process ?: new HashMap();\n    ctx.process.name = process.name;\n    if (process.containsKey('startTimestamp')) {\n        ctx.process.start = Instant.ofEpochSecond(process.startTimestamp).toString();\n    }\n}\n";

/// Verbatim from the generated call sites in
/// `crates/dfe-transforms/src/filebeat/gitlab_api/default.rs` and
/// `.../gitlab_production/default.rs`, which are the whole of gitlab's
/// remaining parity debt once its date processor stopped aborting.
///
/// One vendor pattern, two answers: the api stream folds each `{key, value}`
/// entry into a single-key map and keeps the LIST, the production stream folds
/// the whole list into ONE map and dumps a `variables` member to JSON on the
/// way.
const GITLAB_API_PARAMS: &str = r"def keyValuePairs = [];\nfor (item in ctx.gitlab.api.params) {\n  def key = item.key;\n  def value = item.value;\n  def keyValueObject = [key: value];\n  keyValuePairs.add(keyValueObject)\n}\nctx.gitlab.api.params = keyValuePairs;\n";

const GITLAB_PRODUCTION_PARAMS: &str = r#"Map map = [:];\nfor (item in ctx.gitlab.production.params) {\n  def key = item.key;\n  def value = item.value;\n  if (key == \"variables\" && value instanceof Map) {\n    map[key] = Json.dump(value);\n  } else {\n    map[key] = value;\n  }\n}\nctx.gitlab.production.params = map;\n"#;

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
fn the_zeek_duration_binds_to_a_bare_scale_and_rounds_its_product() {
    // The script is `Math.round(<field> * params.scale)` and the plan is a bare
    // `Scale`, which is the claim. The defect was under it: `try_scale` stored
    // the raw product wherever it was not whole, so 8 of the connection
    // stream's 11 durations landed as a float ending in a fraction where
    // Elasticsearch writes the rounded integer.
    assert_eq!(heads(ZEEK_DURATION), ["Scale"]);

    // The WRITTEN value, over both halves of the stream's own data: a whole
    // product and a fractional one.
    let plan = PainlessPlan::new(ZEEK_DURATION);
    let params = json!({ "scale": 1_000_000_000_i64 });
    for (seconds, want) in [
        (0.076_967_f64, 76_967_000_i64),
        (0.104_128_837_585_449_22, 104_128_838),
        (0.000_908_851_623_535_156_2, 908_852),
    ] {
        let mut event = Event::new(json!({ "temp": { "duration": seconds } }));
        assert!(painless_exec_plan_params(&mut event, &plan, &params).is_ok());
        assert_eq!(
            event.get("event.duration"),
            Some(&json!(want)),
            "{seconds} scaled wrong"
        );
    }
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
        assert!(held.contains("Concat {"), "{name}: {held}");
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

/// Verbatim from the generated call sites in
/// `crates/dfe-transforms/src/filebeat/cyberark_epm_raw_event/default.rs`, and
/// the whole of that stream's parity debt beside the file mode below.
const CYBERARK_LOGON_STATUS: &str = r"def value = Long.toString(ctx.cyberark_epm.raw_event.logon_status_id);\nif (params.containsKey(value)) {\n  ctx.cyberark_epm.raw_event.put('logon_status_value', params[value]);\n}";

const CYBERARK_LOGON_ATTEMPT: &str = r"def value = Long.toString(ctx.cyberark_epm.raw_event.logon_attempt_type_id);\nif (params.containsKey(value)) {\n  ctx.cyberark_epm.raw_event.put('logon_attempt_value', params[value]);\n}";

const CYBERARK_FILE_MODE: &str = r"def getOctalValue(String permissions) {\n  def value = 0;\n  if (permissions.charAt(0) == (char) 'r') value += 4;\n  if (permissions.charAt(1) == (char) 'w') value += 2;\n  if (permissions.charAt(2) == (char) 'x') value += 1;\n  return value;\n}\nString permissionString = ctx.cyberark_epm.raw_event.file_access_permission;\nif (permissionString.length() != 10) {\n  return;\n}\nint owner = getOctalValue(permissionString.substring(1, 4));\nint group = getOctalValue(permissionString.substring(4, 7));\nint other = getOctalValue(permissionString.substring(7, 10));\nif (ctx.file == null) {\n  ctx.put('file', new HashMap());\n}\nctx.file.put('mode', Integer.toString(owner) + Integer.toString(group) + Integer.toString(other));";

#[test]
fn the_cyberark_logon_tables_bind_to_the_stringified_lookup() {
    // Both bound to `IndexedLookup`, the checkpoint_email arm above, which
    // indexes a params LIST by an offset. These stringify a long and read a
    // params MAP, so its runner declined and 19 fields over 10 events reached
    // nothing. The two arms sit next to each other and only the parse
    // separates them.
    for (name, script, source, target) in [
        (
            "status",
            CYBERARK_LOGON_STATUS,
            "cyberark_epm.raw_event.logon_status_id",
            "cyberark_epm.raw_event.logon_status_value",
        ),
        (
            "attempt",
            CYBERARK_LOGON_ATTEMPT,
            "cyberark_epm.raw_event.logon_attempt_type_id",
            "cyberark_epm.raw_event.logon_attempt_value",
        ),
    ] {
        let held = binding(script).join(" ");
        assert!(held.starts_with("StringifiedLookup"), "{name}: {held}");
        assert!(
            held.contains(&format!("source: {source:?}")),
            "{name}: {held}"
        );
        assert!(
            held.contains(&format!("target: {target:?}")),
            "{name}: {held}"
        );
    }
}

#[test]
fn the_cyberark_file_mode_binds_to_the_permission_scoring() {
    // The binding was EMPTY: `OctalString` triggers on
    // `Integer.toOctalString(`, and this script converts by scoring characters
    // and names no base. The width, the triplet bounds and the score table are
    // read off the script, so all three have to show here.
    let held = binding(CYBERARK_FILE_MODE).join(" ");
    assert!(held.starts_with("PermissionOctal"), "{held}");
    assert!(
        held.contains(r#"source: "cyberark_epm.raw_event.file_access_permission""#),
        "{held}"
    );
    assert!(held.contains(r#"target: "file.mode""#), "{held}");
    assert!(held.contains("length: 10"), "{held}");
    assert!(
        held.contains("triplets: [(1, 4), (4, 7), (7, 10)]"),
        "{held}"
    );
    assert!(
        held.contains("scores: [(0, 'r', 4), (1, 'w', 2), (2, 'x', 1)]"),
        "{held}"
    );
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
    assert!(
        held.contains(r#"field: "cef.extensions.message""#),
        "{held}"
    );
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
            r#"Concat { pieces: [Field("_temp.date"), Literal("T"), Field("_temp.time"), Field("_temp.tz")], fold: None }"#
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

/// azure's spelling of the same loop, and the one `KeyValuePairs` was written
/// for. It is the audit for the widening: the merge answer must not move.
const AZURE_AUTH_DETAILS: &str = r#"def tmp = [:];\nfor (item in ctx.azure.signinlogs.properties.authentication_processing_details) {\n    tmp[item.key] = item.value;\n}\nctx.azure.signinlogs.properties.authentication_processing_details = tmp;\n"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/tenable_io_audit/default.rs`. The
/// sharpest near-miss in the tree: one loop, the same `ctx.` path read and
/// written, and the key LOWERCASED on the way in.
const TENABLE_AUDIT_FIELDS: &str = r#"def fields = new HashMap();\nfor (f in ctx.tenable_io.audit.fields) {\n  fields.put(f.key.toLowerCase(), f.value);\n}\nctx.tenable_io.audit.fields = fields;"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/ti_opencti_indicator/default.rs`. The
/// fold is over a member of a LIST ITEM, so the path written is a local's.
const OPENCTI_STARTUP_INFO: &str = r#"if (ctx.observables?.edges instanceof List) {\n  for (def edge : ctx.observables.edges) {\n    if (edge.node?.startup_info instanceof List) {\n      def result = [:];\n      for (def kv : edge.node.startup_info) {\n        result[kv.key] = kv.value;\n      }\n      edge.node.startup_info = result;\n    }\n  }\n}\n"#;

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/google_secops_alert_v2/default.rs`, and
/// the whole of google_secops's parity debt. Three nested loops, a second
/// member read when the value is empty, and the key dropped when it is.
const GOOGLE_SECOPS_KV_FIELDS: &str = r#"String[] kvFields = new String[] {\"detection_fields\", \"outcomes\", \"rule_labels\"};\nfor (def detection : ctx.google_secops.alert_v2.detection) {\n  for (def fieldName : kvFields) {\n    if (!(detection[fieldName] instanceof List)) {\n      continue;\n    }\n    def flat = new HashMap();\n    for (def entry : detection[fieldName]) {\n      if (entry?.key == null || entry.key == '') {\n        continue;\n      }\n      if (entry.value != null && entry.value != '') {\n        flat[entry.key] = entry.value;\n      } else if (entry.source != null && entry.source != '') {\n        flat[entry.key] = entry.source;\n      }\n    }\n    if (flat.isEmpty()) {\n      detection.remove(fieldName);\n    } else {\n      detection[fieldName] = flat;\n    }\n  }\n}\n"#;

/// gitlab's two key/value folds, and azure's beside them.
///
/// Both gitlab folds bound to NOTHING, and both sat behind the date processor
/// that aborted the pipeline, so neither had ever been reached. They are 17 of
/// gitlab's failing events -- 9 on the api stream and 8 on production.
///
/// One loop with two answers, and only the ACCUMULATOR'S DECLARATION says
/// which: `[:]` folds the list into one map, `[]` keeps the list and folds
/// each record into a single-key map of its own. Read from the loop body
/// alone, all three of these are the same script.
#[test]
fn the_gitlab_key_value_folds_bind_to_the_accumulator_they_declare() {
    assert_eq!(
        binding(GITLAB_API_PARAMS),
        [concat!(
            r#"KeyValuePairs(KeyValueFold { path: "gitlab.api.params", "#,
            r#"into: MapPerRecord, dump_key: None })"#
        )]
    );
    assert_eq!(
        binding(GITLAB_PRODUCTION_PARAMS),
        [concat!(
            r#"KeyValuePairs(KeyValueFold { path: "gitlab.production.params", "#,
            r#"into: OneMap, dump_key: Some("variables") })"#
        )]
    );
    assert_eq!(
        binding(AZURE_AUTH_DETAILS),
        [concat!(
            r#"KeyValuePairs(KeyValueFold { path: "azure.signinlogs.properties"#,
            r#".authentication_processing_details", into: OneMap, dump_key: None })"#
        )]
    );
}

#[test]
fn the_key_value_fold_declines_the_three_loops_that_read_the_same_pair() {
    // Every one of these reads `<item>.key` and `<item>.value`, so the trigger
    // takes all three and only the parse turns them away. Claiming any would
    // write the fold and drop the rest of what the script does, with no error.
    //
    // tenable lowercases the key, so a claim would keep the vendor's own
    // casing where Elasticsearch writes lowercase; opencti folds a member of a
    // list ITEM, so the path written is a local's rather than the loop's; and
    // google_secops falls back to a second member and drops the key when both
    // are empty -- and it is that source's whole parity debt, sitting at 0/5.
    for (name, script) in [
        ("tenable_io", TENABLE_AUDIT_FIELDS),
        ("ti_opencti", OPENCTI_STARTUP_INFO),
        ("google_secops", GOOGLE_SECOPS_KV_FIELDS),
    ] {
        assert!(
            !binding(script)
                .iter()
                .any(|held| held.starts_with("KeyValuePairs")),
            "{name}: {:?}",
            binding(script)
        );
    }
}

#[test]
fn the_sysdig_helper_binds_its_three_fields_and_declines_the_other_callers() {
    // The binding was EMPTY, and every one of sysdig's date processors reads a
    // field this helper is what fills: the 19-digit integer reached an
    // `ISO8601` parser that cannot take it, and the throw aborted the pipeline.
    let held = binding(SYSDIG_PARSE_DATE).join(" ");
    assert!(held.starts_with("EpochNanosToDateTime"), "{held}");
    for field in [
        "json.timestamp",
        "sysdig.event.content.fields.proc.pid_ts",
        "sysdig.event.content.fields.proc.ppid_ts",
    ] {
        assert!(held.contains(&format!("{field:?}")), "lost {field}: {held}");
    }

    // The audit. Both call `Instant.ofEpochSecond` and neither declares the
    // helper, so a reader triggered on the constructor would take them.
    assert!(
        binding(SLACK_ACTION_TIMESTAMP).is_empty(),
        "{:?}",
        binding(SLACK_ACTION_TIMESTAMP)
    );
    assert!(
        !binding(JAMF_PROTECT_PROCESS_START)
            .first()
            .is_some_and(|held| held.starts_with("EpochNanosToDateTime")),
        "{:?}",
        binding(JAMF_PROTECT_PROCESS_START)
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

/// The three placements qualys writes its numeric coercion in, verbatim from
/// the call sites in `qualys_was_vulnerability/default.rs`.
const QUALYS_SCALAR: &str = r#"if (ctx.json.detection.detectionScore instanceof String) {\n  ctx.qualys_was.vulnerability.detection_score = Long.parseLong(ctx.json.detection.detectionScore);\n} else {\n  ctx.qualys_was.vulnerability.detection_score = (long)ctx.json.detection.detectionScore;\n}\n"#;

const QUALYS_COLLECT_MEMBERS: &str = r#"def wascList = new ArrayList(); for (wasc in ctx.json.detection.wasc.list) {\n  if (wasc.WASC?.code != null) {\n    if (wasc.WASC.code instanceof String) {\n      wasc.WASC.code = Long.parseLong(wasc.WASC.code);\n    } else {\n      wasc.WASC.code = (long)wasc.WASC.code;\n    }\n   }     \n   wascList.add(wasc.WASC);        \n} ctx.qualys_was.vulnerability.wasc_references = wascList;\n"#;

const QUALYS_COLLECT_STRINGS: &str = r#"ctx.vulnerability.id = new ArrayList(); for (cwe in ctx.json.detection.cwe.list) {\n  if (cwe instanceof String) {\n    ctx.vulnerability.id.add(cwe);\n  } else {\n    ctx.vulnerability.id.add(((long)cwe).toString());\n  } \n  \n}\n"#;

/// cloudflare reads the same two arms into a LOCAL and rescales it, so it must
/// keep its own matcher after `LongCoercion` joined the ladder.
#[test]
fn the_three_qualys_coercions_bind_and_cloudflare_keeps_its_rescale() {
    let held = binding(QUALYS_SCALAR).join(" ");
    assert!(held.starts_with("LongCoercion"), "{held}");
    assert!(
        held.contains(r#"source: "json.detection.detectionScore""#),
        "{held}"
    );

    let held = binding(QUALYS_COLLECT_MEMBERS).join(" ");
    assert!(held.contains(r#"member: "WASC""#), "{held}");
    assert!(held.contains(r#"key: "code""#), "{held}");

    let held = binding(QUALYS_COLLECT_STRINGS).join(" ");
    assert!(
        held.contains(r#"CollectStrings { list: "json.detection.cwe.list""#),
        "{held}"
    );

    assert_eq!(heads(CLOUDFLARE_WHEN_TO_MILLI), ["EpochToMillis"]);
    assert_eq!(heads(SYSDIG_PARSE_DATE), ["EpochNanosToDateTime"]);
}

/// github's five unbound scripts, verbatim from the generated call sites in
/// `crates/dfe-transforms/src/filebeat/github_*/default.rs`.
///
/// Every one of them measured an EMPTY binding, which is the whole of the
/// source's parity debt: 8 events on the dependabot pair, 5 on the issues
/// labels, and 7 across the two spans.
const GITHUB_DEPENDABOT_IDENTIFIERS: &str = r#"def enumeration = \"GHSA\";\ndef id = \"\";\ndef sa_ids = ctx.github.dependabot.security_advisory.identifiers;\nfor (def sa_id: sa_ids) {\n    id = sa_id.value;\n    if (!sa_id.type.equals(\"GHSA\")) {\n        enumeration = sa_id.type;\n        break;\n    }\n}\nctx.vulnerability.enumeration = enumeration;\nctx.vulnerability.id = id;\n"#;

const GITHUB_DEPENDABOT_REFERENCES: &str = r#"List references = new ArrayList();\ndef sa_references = ctx.github.dependabot.security_advisory.references;\nfor (def ref: sa_references) {\n    references.add(ref.url);\n}\nctx.vulnerability.reference = references;\n"#;

const GITHUB_ISSUES_LABELS: &str = r#"Map label;\nList labels = new ArrayList();\nList labels_raw = ctx._temp_.labels;\nString label_key, label_value;\nfor (Map label_raw: labels_raw) {\n    label = new HashMap();\n    label.put(\"name\", label_raw.name);\n    label.put(\"description\", label_raw.description);\n    labels.add(label);\n}\nctx.github.issues.labels = labels;\n"#;

const GITHUB_CODE_SCANNING_SPAN: &str = r#"def time_to_resolution = new HashMap();\ndef fixedAtDt = ctx.github.code_scanning.fixed_at;\ndef dismissedAtDt = ctx.github.code_scanning.dismissed_at;\ndef createdAtDt = ctx.github.code_scanning.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nif (fixedAtDt != null) {\n    zdt = ZonedDateTime.parse(fixedAtDt);\n    long fixedAtEpoch = zdt.toEpochSecond();\n    time_to_resolution.put(\"sec\", fixedAtEpoch - createdAtEpoch);\n    ctx.github.code_scanning.time_to_resolution = time_to_resolution;\n}\nelse {\n    zdt = ZonedDateTime.parse(dismissedAtDt);\n    long dismissedAtEpoch = zdt.toEpochSecond();\n    time_to_resolution.put(\"sec\", dismissedAtEpoch - createdAtEpoch);\n    ctx.github.code_scanning.time_to_resolution = time_to_resolution;\n}\n"#;

const GITHUB_SECRET_SCANNING_SPAN: &str = r#"def time_to_resolution = new HashMap();\ndef resolvedAtDt = ctx.github.secret_scanning.resolved_at;\ndef createdAtDt = ctx.github.secret_scanning.created_at;\nZonedDateTime zdt = ZonedDateTime.parse(createdAtDt);\nlong createdAtEpoch = zdt.toEpochSecond();\nzdt = ZonedDateTime.parse(resolvedAtDt);\nlong resolvedAtEpoch = zdt.toEpochSecond();\ntime_to_resolution.put(\"sec\", resolvedAtEpoch - createdAtEpoch);\nctx.github.secret_scanning.time_to_resolution = time_to_resolution;\n"#;

/// A `Map` allocated per entry and one allocated once are the same loop from
/// the body alone, and the difference is what the accumulator is handed.
///
/// Read as a member collection the labels script would write a list of names
/// and lose the descriptions, so the binding has to show both keys.
#[test]
fn the_github_list_walks_bind_to_the_job_their_accumulator_declares() {
    assert_eq!(
        binding(GITHUB_DEPENDABOT_REFERENCES),
        [concat!(
            r#"ListRebuild(ListRebuild { list: "github.dependabot.security_advisory"#,
            r#".references", target: "vulnerability.reference", take: Member("url") })"#
        )]
    );
    assert_eq!(
        binding(GITHUB_ISSUES_LABELS),
        [concat!(
            r#"ListRebuild(ListRebuild { list: "_temp_.labels", target: "#,
            r#""github.issues.labels", take: Record([("name", "name"), "#,
            r#"("description", "description")]) })"#
        )]
    );

    // The scan writes two fields off one walk, and the seed it excludes is
    // also the answer when no entry differs from it.
    let held = binding(GITHUB_DEPENDABOT_IDENTIFIERS).join(" ");
    assert!(held.starts_with("ScanTaggedList"), "{held}");
    assert!(held.contains(r#"value_member: "value""#), "{held}");
    assert!(held.contains(r#"tag_member: "type""#), "{held}");
    assert!(held.contains(r#"default_tag: "GHSA""#), "{held}");
    assert!(
        held.contains(r#"tag_target: "vulnerability.enumeration""#),
        "{held}"
    );
    assert!(
        held.contains(r#"value_target: "vulnerability.id""#),
        "{held}"
    );
}

/// One span, two spellings, and the branch reduces to an order of candidates.
///
/// The code scanning spelling carries an inline `!= null`, which is
/// `GuardedCopy`'s trigger, so the arm has to sit above it or the script is
/// claimed and nothing is written.
#[test]
fn both_github_span_spellings_bind_to_the_one_subtraction() {
    assert_eq!(
        binding(GITHUB_SECRET_SCANNING_SPAN),
        [concat!(
            r#"SecondsBetween(SecondsBetween { from: "github.secret_scanning"#,
            r#".created_at", to: ["github.secret_scanning.resolved_at"], "#,
            r#"key: "sec", target: "github.secret_scanning.time_to_resolution" })"#
        )]
    );
    assert_eq!(
        binding(GITHUB_CODE_SCANNING_SPAN),
        [concat!(
            r#"SecondsBetween(SecondsBetween { from: "github.code_scanning"#,
            r#".created_at", to: ["github.code_scanning.fixed_at", "#,
            r#""github.code_scanning.dismissed_at"], key: "sec", "#,
            r#"target: "github.code_scanning.time_to_resolution" })"#
        )]
    );
}

/// zeek's four unbound scripts, verbatim from the generated call sites in
/// `crates/dfe-transforms/src/filebeat/zeek_connection/default.rs`,
/// `.../zeek_files/default.rs` and `.../zeek_dns/default.rs`.
const ZEEK_DIRECTION: &str = r#"if (ctx.zeek?.connection?.local_orig == null ||\n    ctx.zeek?.connection?.local_resp == null) {\n  return;\n}\nif (ctx.zeek.connection.local_orig == true &&\n    ctx.zeek.connection.local_resp == true) {\n  ctx.network.direction = \"internal\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == true &&\n    ctx.zeek.connection.local_resp == false) {\n  ctx.network.direction = \"outbound\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == false &&\n    ctx.zeek.connection.local_resp == true) {\n  ctx.network.direction = \"inbound\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == false &&\n    ctx.zeek.connection.local_resp == false) {\n  ctx.network.direction = \"external\";\n  return;\n}"#;

const ZEEK_TX_HOST: &str =
    r"ctx.zeek.files.tx_host = ctx.zeek.files.tx_hosts[0]; ctx.zeek.files.remove('tx_hosts');";

const ZEEK_RX_HOST: &str =
    r"ctx.zeek.files.rx_host = ctx.zeek.files.rx_hosts[0]; ctx.zeek.files.remove('rx_hosts');";

const ZEEK_SESSION_ID: &str = r"ctx.zeek.session_id = ctx.zeek.files.session_ids[0];";

const ZEEK_DNS_ZIP: &str = r#"def answers = ctx.zeek.dns.answers; def ttls = ctx.zeek.dns.TTLs; if (answers.isEmpty() || ttls.isEmpty() || answers.length != ttls.length) {\n  return;\n} def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n  lst.add([\n    \"data\": answers[i],\n    \"ttl\": (int)ttls[i]\n  ])\n} if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;"#;

const ZEEK_DNS_HOIST: &str = r#"def answers = ctx.dns.answers; def iplist = new ArrayList(); for (def i = 0; i < ctx.dns.answers.length; i++) {\n  if (answers[i].containsKey(\"tmpip\")) {\n    iplist.add(answers[i].tmpip);\n    answers[i].remove(\"tmpip\");\n  }\n} ctx.dns.resolved_ip = iplist;"#;

/// The locality table was UNBOUND, and it is `network.direction` on all 18
/// events of zeek's connection stream.
#[test]
fn the_zeek_locality_binds_to_the_pair_table_with_all_four_rows() {
    let held = binding(ZEEK_DIRECTION).join(" ");
    assert!(held.starts_with("PairTable"), "{held}");
    assert!(
        held.contains(r#"left: "zeek.connection.local_orig""#),
        "{held}"
    );
    assert!(
        held.contains(r#"right: "zeek.connection.local_resp""#),
        "{held}"
    );
    assert!(held.contains(r#"target: "network.direction""#), "{held}");
    // Four rows, and the pair each names -- reading three would leave one
    // locality on whatever an earlier processor had written.
    for row in [
        r#"((Bool(true), Bool(true)), String("internal"))"#,
        r#"((Bool(true), Bool(false)), String("outbound"))"#,
        r#"((Bool(false), Bool(true)), String("inbound"))"#,
        r#"((Bool(false), Bool(false)), String("external"))"#,
    ] {
        assert!(held.contains(row), "lost {row}: {held}");
    }
}

/// The two host takes were CLAIMED and wrote their target, and the list each
/// consumed stayed behind as a field Elasticsearch does not emit -- 16 extras
/// over the 8 events of zeek's files stream.
#[test]
fn the_zeek_host_takes_drop_the_list_and_the_session_take_does_not() {
    for (name, script, array, target) in [
        (
            "tx",
            ZEEK_TX_HOST,
            "zeek.files.tx_hosts",
            "zeek.files.tx_host",
        ),
        (
            "rx",
            ZEEK_RX_HOST,
            "zeek.files.rx_hosts",
            "zeek.files.rx_host",
        ),
    ] {
        let held = binding(script).join(" ");
        assert!(held.starts_with("FirstElement"), "{name}: {held}");
        assert!(
            held.contains(&format!("array: {array:?}")),
            "{name}: {held}"
        );
        assert!(
            held.contains(&format!("target: {target:?}")),
            "{name}: {held}"
        );
        assert!(held.contains("consumes_array: true"), "{name}: {held}");
    }

    // The audit, from the same file: an identical take with NO removal beside
    // it keeps its list, and `zeek.files.session_ids` is a field Elasticsearch
    // does emit.
    let held = binding(ZEEK_SESSION_ID).join(" ");
    assert!(held.contains("consumes_array: false"), "{held}");
}

/// Both halves of zeek's DNS chain were UNBOUND, and the second reads only what
/// the first writes -- `dns.answers` and `dns.resolved_ip` on 6 of 7 events.
#[test]
fn the_zeek_dns_chain_binds_its_zip_and_the_hoist_that_follows_it() {
    // The zip declined on ONE word: it read `(long)` alone, so the `(int)` cast
    // on the TTL left a local named `(int)ttls` that is bound to nothing.
    assert_eq!(
        binding(ZEEK_DNS_ZIP),
        [concat!(
            r#"ZipLists(ZipLists { columns: [ZipColumn { key: "data", "#,
            r#"source: "zeek.dns.answers", to_long: false }, ZipColumn { "#,
            r#"key: "ttl", source: "zeek.dns.TTLs", to_long: true }], "#,
            r#"target: "dns.answers", mismatch: None })"#
        )]
    );
    assert_eq!(
        binding(ZEEK_DNS_HOIST),
        [concat!(
            r#"HoistMember(HoistMember { list: "dns.answers", "#,
            r#"member: "tmpip", target: "dns.resolved_ip" })"#
        )]
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

/// google_workspace's three email splits that also append a half to a list,
/// verbatim from the generated call sites in
/// `crates/dfe-transforms/src/filebeat/google_workspace_{login,groups,drive}/default.rs`.
const GOOGLE_WORKSPACE_LOGIN_AFFECTED: &str = r#"String[] splitmail = ctx.google_workspace.login.affected_email_address.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} if (ctx.user.target == null) {\n  ctx.user.target = new HashMap();\n} ctx.user.target.name = splitmail[0]; ctx.user.target.domain = splitmail[1]; ctx.related.user.add(splitmail[0]);\n"#;

const GOOGLE_WORKSPACE_GROUPS_MEMBER: &str = r#"String[] splitmail = ctx.google_workspace.groups.member.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} if (ctx.user.target == null) {\n  ctx.user.target = new HashMap();\n} if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} ctx.related.user.add(splitmail[0]); ctx.user.target.name = splitmail[0]; ctx.user.target.domain = splitmail[1]; ctx.user.target.email = ctx.google_workspace.groups.member.email;\n"#;

const GOOGLE_WORKSPACE_DRIVE_TARGET: &str = r#"String[] splitmail = ctx.google_workspace.drive.target.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} ctx.related.user.add(splitmail[0]);\n"#;

/// The two appends that must STAY with `AppendEach`, which is the audit for the
/// narrowing: crowdstrike is the script it was written for, and o365 appends a
/// loop variable over a list.
const CROWDSTRIKE_TAGS: &str = r#"if (ctx.crowdstrike.event.Tags instanceof List) {\n    for (tag in ctx.crowdstrike.event.Tags) {\n        if (tag instanceof Map) { ctx.tags.add(tag[\"Key\"] + \":\" + tag[\"ValueString\"]); }\n    }\n} else if (ctx.crowdstrike.event.Tags instanceof String) {\n    for (value in ctx.crowdstrike.event.Tags.splitOnToken(',')) { ctx.tags.add(value.trim()); }\n}"#;

#[test]
fn the_google_workspace_splits_bind_to_the_split_and_not_to_append_each() {
    // All three bound to `AppendEach`, which appends EVERY part of the split.
    // The address's domain therefore joined its name in `related.user`, and
    // because that arm is a hard stop `EmailSplit` never ran, so
    // `user.target.*` was never written at all.
    let held = binding(GOOGLE_WORKSPACE_LOGIN_AFFECTED).join(" ");
    assert!(held.starts_with("EmailSplit"), "{held}");
    assert!(held.contains(r#"names: ["user.target.name"]"#), "{held}");
    assert!(
        held.contains(r#"domains: ["user.target.domain"]"#),
        "{held}"
    );
    assert!(held.contains(r#"appends: [("related.user", 0)]"#), "{held}");

    let held = binding(GOOGLE_WORKSPACE_GROUPS_MEMBER).join(" ");
    assert!(held.starts_with("EmailSplit"), "{held}");
    assert!(
        held.contains(r#"emails: ["user.target.email"]"#),
        "{held}"
    );
    assert!(held.contains(r#"appends: [("related.user", 0)]"#), "{held}");

    // drive assigns NOTHING, so the truncated-form fallback would invent
    // `google_workspace.drive.name` and `.domain`. An append is a named target.
    assert_eq!(
        binding(GOOGLE_WORKSPACE_DRIVE_TARGET),
        [concat!(
            r#"EmailSplit(EmailSplit { source: "google_workspace.drive.target", "#,
            r#"names: [], domains: [], emails: [], appends: [("related.user", 0)] })"#
        )]
    );

    // The audit: both keep `AppendEach`.
    assert_eq!(heads(CROWDSTRIKE_TAGS), ["AppendEach"]);
    assert_eq!(heads(O365_ATTACHMENTS), ["AppendEach"]);
}

/// Verbatim from the generated call site in
/// `crates/dfe-transforms/src/filebeat/o365/default.rs`.
const O365_ATTACHMENTS: &str = r#"if (ctx.o365audit?.Item?.Attachments != null) {\n  for (attachmentObj in ctx.o365audit.Item.Attachments.splitOnToken(';')) {\n    if (attachmentObj != \"\") {\n      ctx.email.attachments.add(attachmentObj);\n    }\n  }\n}\n"#;

#[test]
fn the_google_workspace_splits_write_both_halves_where_the_script_puts_them() {
    // The WRITTEN values, against the corpus captures. `related.user` keeps its
    // duplicates -- login's expected list is ["foo", "foo"] -- so the append is
    // a plain push and never a set.
    let plan = PainlessPlan::new(GOOGLE_WORKSPACE_LOGIN_AFFECTED);
    let mut event = Event::new(json!({
        "google_workspace": { "login": { "affected_email_address": "foo@elastic.co" } },
        "related": { "user": ["foo"] },
    }));
    assert!(painless_exec_plan(&mut event, &plan).is_ok());
    assert_eq!(event.get("related.user"), Some(&json!(["foo", "foo"])));
    assert_eq!(event.get("user.target.name"), Some(&json!("foo")));
    assert_eq!(event.get("user.target.domain"), Some(&json!("elastic.co")));

    let plan = PainlessPlan::new(GOOGLE_WORKSPACE_GROUPS_MEMBER);
    let mut event = Event::new(json!({
        "google_workspace": { "groups": { "member": { "email": "user@example.com" } } },
        "related": { "user": ["foo"] },
    }));
    assert!(painless_exec_plan(&mut event, &plan).is_ok());
    assert_eq!(event.get("related.user"), Some(&json!(["foo", "user"])));
    assert_eq!(event.get("user.target.name"), Some(&json!("user")));
    assert_eq!(event.get("user.target.domain"), Some(&json!("example.com")));
    assert_eq!(
        event.get("user.target.email"),
        Some(&json!("user@example.com"))
    );

    // An address that does not cut in two writes NOTHING -- the script's own
    // `if (splitmail.length != 2) { return; }`. drive's capture carries a
    // markdown-wrapped address with two `@`, and Elasticsearch emits no
    // `related.user` for it.
    let plan = PainlessPlan::new(GOOGLE_WORKSPACE_DRIVE_TARGET);
    let mut event = Event::new(json!({
        "google_workspace": {
            "drive": { "target": "[jane.smith@example.org](mailto:jane.smith@example.org)" }
        },
    }));
    assert!(painless_exec_plan(&mut event, &plan).is_ok());
    assert_eq!(event.get("related.user"), None);
    assert_eq!(event.get("google_workspace.drive.name"), None);
    assert_eq!(event.get("google_workspace.drive.domain"), None);
}
