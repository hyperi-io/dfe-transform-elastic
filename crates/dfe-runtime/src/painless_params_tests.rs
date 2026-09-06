// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//! Tests for [`super`], lifted out so the module reads at a human size.
//!
//! Attached with `#[path]` rather than a directory module: the parent is one
//! compilation unit either way, and a flat layout keeps `use super::*`
//! meaning exactly what it did before the split.

use super::*;
use serde_json::json;

/// Painless's `[local: value]` map literal evaluates the KEY as an expression,
/// so the map's key is the params row's value rather than the word "key".
///
/// Verbatim from `pipelines/ti_recordedfuture/threat/default.yml`, and the
/// captured output confirms the reading -- `{"sha256": "38e9..."}` after the
/// `rename` that follows.
#[test]
fn a_map_literal_takes_its_key_from_the_lookup() {
    let script = "def key = params[ctx.json.Algorithm];\n\
        if (key == null) {\n  throw new Exception(\"Unsupported hash algorithm '\" \
        + ctx.json.Algorithm + \"'\");\n}\n\
        def hashes = [key:ctx.json.Name];\nctx[\"_hashes\"] = hashes;";
    let pattern = parse_keyed_by_lookup(&crate::painless_common::normalise(script))
        .expect("the lookup is recognised");

    let table = json!({ "MD5": "md5", "SHA-256": "sha256" });
    let table = table.as_object().expect("a params table");

    let mut event = Event::new(json!({
        "json": { "Algorithm": "SHA-256", "Name": "38e992eb852ab0c4" }
    }));
    assert!(run_keyed_by_lookup(&mut event, &pattern, table));
    assert_eq!(event.get_str("_hashes.sha256"), Some("38e992eb852ab0c4"));

    // An algorithm with no row THROWS in Painless, and the processor's
    // on_failure appends to error.message rather than storing a fallback key.
    let mut unknown = Event::new(json!({
        "json": { "Algorithm": "SHA-3", "Name": "abc" }
    }));
    assert!(run_keyed_by_lookup(&mut unknown, &pattern, table));
    assert!(!unknown.has("_hashes"));
}

/// Hold the `ctx.` path readers to one answer, or to a stated reason.
///
/// Eighteen helpers read a dotted path out of Painless text and differ on
/// four axes: bracket segments, `?` handling, whether they validate, and
/// which end they search from. Nothing compared them, so a reader that
/// drifted was invisible until it produced a wrong field path.
///
/// Same move as `patterns.lock`: turn a property nobody can see into a diff
/// somebody has to approve.
#[test]
fn the_two_bracket_readers_agree() {
    use crate::painless_common::painless_path;

    // Spelling, then what both readers must make of it.
    let cases = [
        ("ctx.host.name", "host.name"),
        ("ctx.host?.name", "host.name"),
        ("ctx['host.name']", "host.name"),
        ("ctx[\"host\"].name", "host.name"),
        // A leading `@` is a real ECS field, not punctuation.
        ("ctx['@timestamp']", "@timestamp"),
    ];

    for (fragment, expected) in cases {
        assert_eq!(
            painless_path(fragment).as_deref(),
            Some(expected),
            "painless_path({fragment:?})"
        );
        assert_eq!(
            subject_path(fragment).strip_prefix("ctx.").map(clean_path),
            Some(expected.to_string()),
            "subject_path({fragment:?}) then a ctx. strip"
        );
    }
}

/// `clean_path` does not read map syntax, so a bracketed path reaching it
/// keeps its brackets. Pinned rather than fixed: `ctx_path_before` searches
/// for the literal `ctx.`, so a bracketed ROOT is not found at all.
#[test]
fn clean_path_leaves_map_syntax_alone() {
    assert_eq!(clean_path("host['name']"), "host['name']");
    assert_eq!(clean_path("host?.name"), "host.name");
}

/// The readers do NOT agree on which characters a path may hold, and the
/// difference is pinned rather than resolved.
///
/// `path_before` and `base_between` accept `alnum . _ ?` and so refuse
/// `@timestamp`; the `ctx_path_*` pair refuses only whitespace and a
/// terminator. Widening the strict pair would open matchers that have never
/// been exposed to a bad path, which is a parity change and not a tidy-up.
/// Narrowing the loose pair would refuse ECS fields that are real.
#[test]
fn the_readers_differ_on_the_at_sign_and_that_is_deliberate() {
    let script = "def t = ctx.@timestamp;";

    assert_eq!(
        ctx_path_before(script, ";"),
        Some("@timestamp".to_string()),
        "the loose pair takes a real ECS field"
    );
    assert_eq!(
        super::path_before(script, ";"),
        None,
        "the strict pair refuses it -- widen only with a corpus run"
    );
}

/// A backward search that spans two bindings declines rather than handing
/// back a field name no event can hold.
///
/// `falco_alerts` ships this pattern and the reader used to return
/// `proc.args;\n def items = args`, which bound a pattern that then wrote
/// nothing.
#[test]
fn a_backward_search_spanning_two_bindings_declines() {
    let script = "def path = ctx.proc.exepath; def args = ctx.proc.args; \
        def items = args.splitOnToken(' ');";

    assert_eq!(ctx_path_before(script, ".splitOnToken("), None);

    // A marker whose path IS the one before it still reads.
    assert_eq!(
        ctx_path_before(
            "def items = ctx.proc.args.splitOnToken(' ')",
            ".splitOnToken("
        ),
        Some("proc.args".to_string())
    );
}

/// `+=` accumulates onto its left side rather than assigning to it, so
/// splitting there yields a write target with the operator still attached.
/// `fortinet`'s tls version ships `ctx.tls.version += ".0"`.
#[test]
fn a_compound_assignment_is_not_a_write_target() {
    assert_eq!(split_assignment("ctx.tls.version += \".0\""), None);
    assert_eq!(split_assignment("ctx.a -= 1"), None);
    assert_eq!(split_assignment("ctx.a *= 2"), None);
}

/// The ordinary forms still split, and a comparison still does not.
#[test]
fn a_plain_assignment_still_splits() {
    assert_eq!(
        split_assignment("ctx.a = ctx.b"),
        Some(("ctx.a ", " ctx.b"))
    );
    assert_eq!(split_assignment("ctx.a == ctx.b"), None);
    assert_eq!(split_assignment("ctx.a != ctx.b"), None);
    assert_eq!(split_assignment("ctx.a >= 3"), None);
}

/// A field-to-field comparison used to read as unresolvable and answer
/// false forever, so the branch behind it was dead on every event.
#[test]
fn one_field_compares_against_another() {
    let same = Event::new(json!({ "a": "x", "b": "x", "c": "y" }));
    assert!(guard_holds(&same, "ctx.a == ctx.b"));
    assert!(!guard_holds(&same, "ctx.a == ctx.c"));
    assert!(guard_holds(&same, "ctx.a != ctx.c"));
    assert!(!guard_holds(&same, "ctx.a != ctx.b"));
}

/// Absent and explicitly null are ONE value to an ingest `if`.
#[test]
fn an_absent_field_equals_an_explicitly_null_one() {
    let event = Event::new(json!({ "a": null }));
    assert!(guard_holds(&event, "ctx.a == ctx.missing"));
}

/// An ingest conditional reads a view whose nested containers are fresh
/// wrappers with no `equals`, so two of them never compare equal however
/// identical their contents. `condition_eq` answers the same way.
#[test]
fn two_containers_never_compare_equal() {
    let event = Event::new(json!({
        "a": { "k": 1 }, "b": { "k": 1 },
        "list": [1, 2], "same": [1, 2]
    }));
    assert!(!guard_holds(&event, "ctx.a == ctx.b"));
    assert!(!guard_holds(&event, "ctx.list == ctx.same"));
    assert!(guard_holds(&event, "ctx.a != ctx.b"));
}

/// Verbatim from `pipelines/symantec_endpoint/log/default.yml`: the CSV
/// layout is identified by WHICH columns carried a `Key:` label, and the
/// matching row then names the holes by index.
#[test]
fn a_fingerprint_row_names_the_unlabelled_columns() {
    let script = "// Assume first column is always the host.hostname.\n\
        def hostname = ctx._csv_array.get(0);\n\
        if (/[\\.a-zA-Z0-9_-]+/.matcher(hostname).matches()) {\n  \
        if (ctx?.host == null) {\n    ctx['host'] = [:];\n  }\n  \
        ctx['host']['hostname'] = hostname;\n}\n\ndef provider = null;\n\
        for (def p: params.providers) {\n  \
        if (p.fingerprint == ctx._fingerprint || (p.fingerprint instanceof Collection \
        && p.fingerprint.contains(ctx._fingerprint))) {\n    provider = p;\n    \
        break;\n  }\n}\nif (provider == null) { return; }\n\n\
        ctx['event']['provider'] = provider.name;\n\
        if (provider?.event_category != null) {\n  \
        ctx['event']['category'] = new ArrayList(provider.event_category);\n}\n\
        if (provider?.event_type!= null) {\n  \
        ctx['event']['type'] = new ArrayList(provider.event_type);\n}\n\
        for (def c : provider.columns) {\n  \
        def v = ctx._csv_array.get(c.index).trim();\n  if (!v.isEmpty()) {\n    \
        ctx._csv_map[c.name] = v;\n  }\n}\n";

    let params = json!({
        "providers": [
            {
                "name": "System Log",
                "fingerprint": "site|server|NONE",
                "columns": [{ "index": 2, "name": "event_description" }]
            },
            {
                "name": "Agent Packet Log",
                // Two layouts under one name, so the row lists both.
                "fingerprint": ["NONE|application", "NONE|action"],
                "event_category": ["network"],
                "columns": [{ "index": 0, "name": "traffic_direction" }]
            }
        ]
    });

    let mut event = Event::new(json!({
        "_csv_array": ["srv01", "Server: srv01", "  Scan finished  "],
        "_csv_map": { "site": "SEPM" },
        "_fingerprint": "site|server|NONE"
    }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get_str("host.hostname"), Some("srv01"));
    assert_eq!(event.get_str("event.provider"), Some("System Log"));
    assert_eq!(
        event.get_str("_csv_map.event_description"),
        Some("Scan finished")
    );
    // The row names no categories, so the script writes none.
    assert!(!event.has("event.category"));

    // A row whose fingerprint is a LIST matches on membership.
    let mut listed = Event::new(json!({
        "_csv_array": ["inbound", "tcp"],
        "_csv_map": { "action": "allow" },
        "_fingerprint": "NONE|action"
    }));
    assert!(try_params_painless(&mut listed, script, &params));
    assert_eq!(listed.get_str("event.provider"), Some("Agent Packet Log"));
    assert_eq!(listed.get("event.category"), Some(&json!(["network"])));
    assert_eq!(
        listed.get_str("_csv_map.traffic_direction"),
        Some("inbound")
    );
}

/// Verbatim from `pipelines/checkpoint_email/event/default.yml`: a params
/// array subscripted 1-BASED, which the runner read as 0-based.
///
/// The off-by-one returned the NEXT row rather than missing, so severity 2
/// wrote `Medium` where the vendor writes `Low` -- a plausible value, in all
/// 18 events, with the matcher counted as handled throughout.
#[test]
fn a_one_based_subscript_counts_from_the_script_not_from_zero() {
    let script = "def severityValue = ctx.checkpoint_email.event.severity;\n\
        if (severityValue > 0 && severityValue <= params.severity.length) {\n  \
        ctx.checkpoint_email.event.put('severity_enum', \
        params['severity'][(int)severityValue-1]);\n}";
    let params = json!({ "severity": ["Lowest", "Low", "Medium", "High", "Critical"] });

    for (severity, expected) in [(2, "Low"), (3, "Medium"), (4, "High"), (5, "Critical")] {
        let mut event = Event::new(json!({
            "checkpoint_email": { "event": { "severity": severity } }
        }));
        assert!(try_params_painless(&mut event, script, &params));
        assert_eq!(
            event.get_str("checkpoint_email.event.severity_enum"),
            Some(expected),
            "severity {severity}"
        );
    }
}

/// The same subscript with no offset stays 0-based -- several pipelines spell
/// it that way, and a blanket subtraction would have moved every one of them.
#[test]
fn a_subscript_with_no_offset_still_counts_from_zero() {
    let script = "def n = ctx.a.n;\nctx.a.put('row', params['t'][n]);";
    let params = json!({ "t": ["zero", "one", "two"] });

    let mut event = Event::new(json!({ "a": { "n": 0 } }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get_str("a.row"), Some("zero"));
}

/// Both conventions, verbatim, from ONE pipeline --
/// `pipelines/cyberark_epm/aggregated_event/default.yml:222-259`.
///
/// `DeceptionType` is 1-based and guarded `value >= 1`; `DefenceAction` is
/// 0-based and guarded `value >= 0`. The second is correct in the corpus today
/// and a blanket subtraction would have broken it.
#[test]
fn two_conventions_in_one_pipeline_each_count_their_own_way() {
    let one_based = "def value = (int) ctx.cyberark_epm.aggregated_event.deception_type;\n\
        if (value >= 1 && value <= params.DeceptionType.length) {\n  \
        ctx.cyberark_epm.aggregated_event.put('deception_type_value', \
        params['DeceptionType'][value - 1]);\n}";
    let params =
        json!({ "DeceptionType": ["\"Local User LSASS\" honeypot", "\"Browsers\" honeypot"] });
    let mut event = Event::new(json!({
        "cyberark_epm": { "aggregated_event": { "deception_type": 1 } }
    }));
    assert!(try_params_painless(&mut event, one_based, &params));
    assert_eq!(
        event.get_str("cyberark_epm.aggregated_event.deception_type_value"),
        Some("\"Local User LSASS\" honeypot")
    );

    let zero_based = "def value = (int) ctx.cyberark_epm.aggregated_event.defence_action_id;\n\
        if (value >= 0 && value < params.DefenceAction.length) {\n  \
        ctx.cyberark_epm.aggregated_event.put('defence_action_value', \
        params['DefenceAction'][value]);\n}";
    let params = json!({ "DefenceAction": ["No action", "Detect", "Block"] });
    let mut event = Event::new(json!({
        "cyberark_epm": { "aggregated_event": { "defence_action_id": 0 } }
    }));
    assert!(try_params_painless(&mut event, zero_based, &params));
    assert_eq!(
        event.get_str("cyberark_epm.aggregated_event.defence_action_value"),
        Some("No action")
    );
}

/// An index below the offset writes nothing rather than wrapping.
///
/// `usize` subtraction would panic and a saturating one would return row 0,
/// which is a value the vendor never writes. The script's own guard keeps this
/// unreachable in the corpus; the runner does not get to rely on that.
#[test]
fn an_index_below_the_offset_writes_nothing() {
    let script = "def n = ctx.a.n;\nctx.a.put('row', params['t'][n - 1]);";
    let params = json!({ "t": ["one", "two"] });

    let mut event = Event::new(json!({ "a": { "n": 0 } }));
    assert!(try_params_painless(&mut event, script, &params));
    assert!(!event.has("a.row"));
}

/// Verbatim from `pipelines/symantec_endpoint_security/event/default.yml`:
/// a table lookup that writes NOTHING when the key misses, where
/// `LookupNormalise` would write the key back.
#[test]
fn a_guarded_lookup_writes_only_on_a_hit() {
    let script = "def obj = ctx.ses.file.type_id;\nif (params.containsKey(obj.toString())) {\n  \
        def type = params.get(obj.toString());\n  ctx.ses.file.type_value = type\n}";
    let params = json!({ "1": "File", "2": "Folder" });

    let mut hit = Event::new(json!({ "ses": { "file": { "type_id": 1 } } }));
    assert!(try_params_painless(&mut hit, script, &params));
    assert_eq!(hit.get_str("ses.file.type_value"), Some("File"));

    // A key the table misses leaves the target absent -- writing the key
    // back is the other pattern's behaviour, not this one's.
    let mut miss = Event::new(json!({ "ses": { "file": { "type_id": 99 } } }));
    assert!(try_params_painless(&mut miss, script, &params));
    assert!(!miss.has("ses.file.type_value"));
}

/// Verbatim from `pipelines/google_workspace/{chrome,meet}/default.yml`: the
/// row lands on a NAMED MEMBER of a container the script first guarantees, in
/// the two spellings the family ships, wrapped in the one-element list ECS's
/// `event.type` is.
///
/// The guarantee is not a write. Reading it as one made `event` itself the
/// target, so chrome stored the bare string `installation` there and every
/// later `event.<sub>` write failed on a string -- six events scored zero with
/// all 300 of their fields right.
#[test]
fn a_member_lookup_wraps_the_row_and_writes_past_the_container_guarantee() {
    let params = json!({ "browser_extension_install": "installation" });
    for script in [
        // `put`, keyed on the folded name.
        "if (ctx.event == null) {\\n  ctx.event = new HashMap();\\n}\\n\
         def type = params.get(ctx.google_workspace.chrome.name.toLowerCase());\\n\
         if (type == null) {\\n  ctx.event.remove('type');\\n} else {\\n  \
         ctx.event.put('type', [type]);\\n}",
        // The same thing assigned, which is how meet, keep, calendar, chat and
        // vault spell it.
        "ctx.event = ctx.event ?: [:];\\n\
         def type = params.get(ctx.google_workspace.chrome.name.toLowerCase());\\n\
         if (type == null) {\\n  ctx.event.remove('type');\\n} else {\\n  \
         ctx.event.type = [type];\\n}",
    ] {
        let mut hit = Event::new(json!({
            "event": { "kind": "event" },
            "google_workspace": { "chrome": { "name": "BROWSER_EXTENSION_INSTALL" } }
        }));
        assert!(try_params_painless(&mut hit, script, &params));
        assert_eq!(hit.get("event.type"), Some(&json!(["installation"])));
        assert_eq!(hit.get_str("event.kind"), Some("event"));

        // The miss branch REMOVES, where writing the key back would leave the
        // vendor's own name sitting in an ECS-vocabulary field.
        let mut miss = Event::new(json!({
            "event": { "kind": "event", "type": ["stale"] },
            "google_workspace": { "chrome": { "name": "UNLISTED" } }
        }));
        assert!(try_params_painless(&mut miss, script, &params));
        assert!(!miss.has("event.type"));
        assert_eq!(miss.get_str("event.kind"), Some("event"));
    }
}

/// Verbatim from `pipelines/macos/*/common-pipeline.yml`, 14 call sites: the
/// lookup is the `put` value itself, with no local in between.
///
/// A `put` carries no `=`, so the only assignment the script makes is the
/// container guarantee -- which is how the whole row landed at `log` and cost
/// macos all 23 of its events.
#[test]
fn a_member_lookup_reads_the_put_target_where_nothing_is_assigned() {
    let script = "ctx.log = ctx.log ?: [:];\\n\
         ctx.log.put(\"level\", params.get(ctx.json.messageType.toLowerCase()));";
    let params = json!({ "fault": "warning", "error": "error" });

    let mut event = Event::new(json!({ "json": { "messageType": "Fault" } }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get_str("log.level"), Some("warning"));
}

/// Verbatim from `pipelines/google_workspace/data_studio/default.yml`: the
/// wrap is spelled at the BINDING rather than at the write, so an unlisted key
/// stores the list holding the lookup's own null.
#[test]
fn a_member_lookup_carries_a_wrap_spelled_at_the_binding() {
    let script = "ctx.event = ctx.event ?: [:];\\ndef type = new ArrayList();\\n\
         type.add(params.get(ctx.google_workspace.data_studio.name));\\n\
         ctx.event.put('type', type);";
    let params = json!({ "VIEW": "access" });

    let mut hit = Event::new(json!({ "google_workspace": { "data_studio": { "name": "VIEW" } } }));
    assert!(try_params_painless(&mut hit, script, &params));
    assert_eq!(hit.get("event.type"), Some(&json!(["access"])));

    let mut miss = Event::new(json!({ "google_workspace": { "data_studio": { "name": "X" } } }));
    assert!(try_params_painless(&mut miss, script, &params));
    assert_eq!(miss.get("event.type"), Some(&json!([null])));
}

/// The miss behaviour is read off the guard the script puts round its write,
/// never assumed -- a `!= null` guard leaves the member as it was.
#[test]
fn a_member_lookup_reads_its_miss_behaviour_off_the_guard() {
    let script = "def t = params.get(ctx.winlog.task);\\nif (t != null) {\\n  \
         ctx.winlog.task = t;\\n}";
    let params = json!({ "13": "Registry value set" });

    let mut miss = Event::new(json!({ "winlog": { "task": "99" } }));
    assert!(try_params_painless(&mut miss, script, &params));
    assert_eq!(miss.get_str("winlog.task"), Some("99"));
}

/// What the pattern must NOT claim, because it cannot say the whole script.
///
/// Order is behaviour here: this matcher sits ahead of `LookupNormalise`, so
/// anything it takes wrongly is a source that silently changes output.
#[test]
fn a_member_lookup_declines_what_it_cannot_wholly_say() {
    for script in [
        // qualys_gav: a branch writes a LITERAL to the same member, which
        // claiming the lookup half would drop.
        "def os_type = ctx.qualys_gav.asset.operating_system.category1.toLowerCase();\\n\\n\
         ctx.host = ctx.host ?: [:];\\nctx.host.os = ctx.host.os ?: [:];\\n\\n\
         if (os_type.contains('centos') || os_type.contains('ubuntu')) {\\n  \
         ctx.host.os.put('type', 'linux');\\n} else {\\n  \
         ctx.host.os.put('type', params.get(os_type));\\n}\\n",
        // fortinet: the miss branch writes the folded KEY back, which is
        // `LookupNormalise`'s whole reason to exist.
        "def k = ctx.network.direction.toLowerCase(); def normalized = params.get(k); \
         if (normalized != null) {\\n    ctx.network.direction = normalized;\\n    return;\\n} \
         ctx.network.direction = k;",
        // A direct assignment names its own target correctly, so there is
        // nothing here to fix and taking it would change the miss behaviour of
        // every source that spells a lookup this way.
        "ctx.event = ctx.event ?: [:];\\n\
         ctx.event.severity = params.get(ctx.sysdig.cspm.control.severity.toLowerCase());",
        // bitwarden reads the table twice and fans the row over four fields.
        "if (ctx.bitwarden?.event?.type?.value == null || \
         params.get(ctx.bitwarden.event.type.value) == null) {\\n  return;\\n}\\n\
         def hm = new HashMap(params.get(ctx.bitwarden.event.type.value));\\n\
         ctx.event.category = hm.category;\\nctx.event.type = hm.type;",
    ] {
        assert!(
            parse_member_lookup(&crate::painless_common::normalise(script)).is_none(),
            "claimed a script it cannot wholly say: {script}"
        );
    }
}

/// Verbatim from `pipelines/stan/log/default.yml`: the abbreviation and
/// its expansion are BOTH params members, so the table is editable without
/// touching the script.
#[test]
fn a_ladder_rewrites_one_field_through_pairs_of_params_members() {
    let script = "if (ctx.log.level == params.inf) {\n          \
        ctx.log.level = params.info;\n        } else if (ctx.log.level == params.dbg) {\n          \
        ctx.log.level = params.debug;\n        } else if (ctx.log.level == params.wrn) {\n          \
        ctx.log.level = params.warning;\n        }";
    let params = json!({
        "inf": "INF", "info": "info",
        "dbg": "DBG", "debug": "debug",
        "wrn": "WRN", "warning": "warning",
    });

    let mut matched = Event::new(json!({ "log": { "level": "DBG" } }));
    assert!(try_params_painless(&mut matched, script, &params));
    assert_eq!(matched.get_str("log.level"), Some("debug"));

    // No arm matches, so the ladder falls through and the field stands.
    let mut unmatched = Event::new(json!({ "log": { "level": "TRC" } }));
    assert!(try_params_painless(&mut unmatched, script, &params));
    assert_eq!(unmatched.get_str("log.level"), Some("TRC"));

    // Absent is the script's own `== null` on every arm.
    let mut absent = Event::new(json!({}));
    assert!(try_params_painless(&mut absent, script, &params));
    assert_eq!(absent.get("log.level"), None);
}

/// Verbatim from `pipelines/carbonblack_edr/log/default.yml`: the ECS
/// categorisation table, keyed by `event.action`, merged onto a LOCAL
/// bound to `ctx.event`, with an `unknown` row for an action the table
/// does not list.
#[test]
fn a_categorisation_row_merges_onto_a_bound_local() {
    let script = "def clone(def ref) {\n  if (ref == null) return ref;\n  \
        if (ref instanceof Map) {\n    ref = ref.entrySet().stream().collect(\n      \
        Collectors.toMap(\n        e -> e.getKey(),\n        e -> clone(e.getValue())\n      \
        )\n    );\n  } else if (ref instanceof List) {\n    \
        ref = ref.stream().map(e -> clone(e)).collect(\n      Collectors.toList()\n    );\n  \
        }\n  return ref;\n}\ndef event = ctx.event;\nif (event == null) {\n  \
        event = new HashMap();\n  ctx[\"event\"] = event;\n}\n\
        def type = ctx.event.action;\n\
        def fields = params[type] != null? params[type] : params[\"unknown\"];\n\
        fields.forEach( (k, v) -> {\n  event[k] = clone(v);\n});\n";
    let params = json!({
        "binaryinfo.group.observed": { "kind": "event", "category": ["file"], "type": ["info"] },
        "unknown": { "kind": "event" },
    });

    let mut listed = Event::new(json!({
        "event": { "action": "binaryinfo.group.observed" },
    }));
    assert!(try_params_painless(&mut listed, script, &params));
    assert_eq!(listed.get_str("event.kind"), Some("event"));
    assert_eq!(listed.get("event.category"), Some(&json!(["file"])));
    assert_eq!(listed.get("event.type"), Some(&json!(["info"])));
    // The key itself is left where it was -- the merge adds, it does not
    // replace what it was keyed by.
    assert_eq!(
        listed.get_str("event.action"),
        Some("binaryinfo.group.observed")
    );

    // An action the table does not list takes the `unknown` row, which
    // carries a kind and nothing else.
    let mut unlisted = Event::new(json!({ "event": { "action": "unknown" } }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert_eq!(unlisted.get_str("event.kind"), Some("event"));
    assert_eq!(unlisted.get("event.category"), None);
}

/// Verbatim from `pipelines/cyberarkpas/audit/audit.yml`: the members
/// params names stay put, everything else moves down one level. The
/// vendor's reason is a mapping explosion, so a field the device invented
/// MUST end up under the flattened key and not beside the named ones.
#[test]
fn unlisted_members_move_under_the_scripts_own_rest_key() {
    let script = "Map audit = ctx.cyberarkpas.audit; \
        params.entrySet().stream().filter(e -> audit.containsKey(e.getKey())).forEach(lst -> {\n  \
        Map base = audit[lst.getKey()],\n      selected = new HashMap();\n  \
        lst.getValue().stream().filter(fld -> base.containsKey(fld)).forEach(fld -> {\n    \
        selected[fld] = base.remove(fld);\n  });\n  selected['other'] = base;\n  \
        audit[lst.getKey()] = selected;\n});\n";
    let params = json!({
        "ca_properties": ["address", "port"],
        "extra_details": ["command", "username"],
    });

    let mut event = Event::new(json!({ "cyberarkpas": { "audit": {
        "action": "Logon",
        "ca_properties": { "device_type": "database", "port": "1521", "address": "db1" },
        "extra_details": { "address": "10.0.0.1", "command": "ls", "psmid": "PSM01" },
    }}}));
    assert!(try_params_painless(&mut event, script, &params));

    // Listed: kept where it was, and in the PARAMS order rather than the
    // sub-map's -- Painless streams the list, not the map.
    assert_eq!(
        event.get("cyberarkpas.audit.ca_properties"),
        Some(&json!({
            "address": "db1",
            "port": "1521",
            "other": { "device_type": "database" },
        }))
    );

    // `address` is listed for ca_properties and NOT for extra_details, so
    // the same name lands on opposite sides of the split.
    assert_eq!(
        event.get("cyberarkpas.audit.extra_details"),
        Some(&json!({
            "command": "ls",
            "other": { "address": "10.0.0.1", "psmid": "PSM01" },
        }))
    );

    // A sibling params does not name is left exactly as it was.
    assert_eq!(event.get_str("cyberarkpas.audit.action"), Some("Logon"));
}

/// Verbatim from `pipelines/kolide/auth/categorize.yml`: an unconditional
/// write, then the row's columns or the fallback literals.
#[test]
fn a_table_row_fans_out_and_an_unlisted_key_takes_the_defaults() {
    let script = "def action = ctx.event.action;\nctx.event.kind = 'event';\n\n\
        def m = params.exact.get(action);\nif (m != null) {\n  \
        ctx.event.category = new ArrayList(m.category);\n  \
        ctx.event.type = new ArrayList(m.type);\n  \
        if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    \
        ctx.event.outcome = m.outcome;\n  }\n} else {\n  \
        ctx.event.category = ['authentication'];\n  ctx.event.type = ['info'];\n}";
    let params = json!({ "exact": {
        "sign_in_attempt": { "category": ["authentication", "session"], "type": ["start"] },
        "sign_in_denied": {
            "category": ["authentication"], "type": ["info"], "outcome": "failure",
        },
    }});

    let mut listed = Event::new(json!({ "event": { "action": "sign_in_attempt" } }));
    assert!(try_params_painless(&mut listed, script, &params));
    assert_eq!(listed.get_str("event.kind"), Some("event"));
    assert_eq!(
        listed.get("event.category"),
        Some(&json!(["authentication", "session"]))
    );
    assert_eq!(listed.get("event.type"), Some(&json!(["start"])));
    assert_eq!(listed.get("event.outcome"), None);

    let mut unlisted = Event::new(json!({ "event": { "action": "auth_log" } }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert_eq!(unlisted.get_str("event.kind"), Some("event"));
    assert_eq!(
        unlisted.get("event.category"),
        Some(&json!(["authentication"]))
    );
    assert_eq!(unlisted.get("event.type"), Some(&json!(["info"])));

    // The guarded column is the script's own `== null`, so an outcome the
    // pipeline already resolved stands.
    let mut resolved = Event::new(json!({
        "event": { "action": "sign_in_denied", "outcome": "success" },
    }));
    assert!(try_params_painless(&mut resolved, script, &params));
    assert_eq!(resolved.get_str("event.outcome"), Some("success"));

    let mut denied = Event::new(json!({ "event": { "action": "sign_in_denied" } }));
    assert!(try_params_painless(&mut denied, script, &params));
    assert_eq!(denied.get_str("event.outcome"), Some("failure"));
}

/// Verbatim from `pipelines/kolide/audit/categorize.yml`: the same table
/// read with `containsKey` and a subscript, and a fallback whose own
/// `event.type` comes from a local rather than a literal.
#[test]
fn a_contains_key_lookup_reads_the_same_row() {
    let script = "def action = ctx.event.action;\nctx.event.kind = 'event';\n\n\
        if (params.exact.containsKey(action)) {\n  def m = params.exact[action];\n  \
        ctx.event.category = new ArrayList(m.category);\n  \
        ctx.event.type = new ArrayList(m.type);\n  \
        if (m.containsKey('outcome')) {\n    ctx.event.outcome = m.outcome;\n  }\n\
        } else {\n  String type = 'change';\n  \
        if (action.endsWith('_created')) {\n    type = 'creation';\n  }\n  \
        ctx.event.category = ['configuration'];\n  ctx.event.type = [type];\n}";
    let params = json!({ "exact": {
        "api_key_secret_viewed": {
            "category": ["iam", "configuration"], "type": ["access"], "outcome": "success",
        },
    }});

    let mut listed = Event::new(json!({
        "event": { "action": "api_key_secret_viewed" },
    }));
    assert!(try_params_painless(&mut listed, script, &params));
    assert_eq!(listed.get_str("event.kind"), Some("event"));
    assert_eq!(
        listed.get("event.category"),
        Some(&json!(["iam", "configuration"]))
    );
    assert_eq!(listed.get("event.type"), Some(&json!(["access"])));
    assert_eq!(listed.get_str("event.outcome"), Some("success"));

    // The fallback's derived type is not a literal, so only the category
    // it writes plainly is reproduced.
    let mut unlisted = Event::new(json!({ "event": { "action": "audit_log" } }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert_eq!(
        unlisted.get("event.category"),
        Some(&json!(["configuration"]))
    );
    assert_eq!(unlisted.get("event.type"), None);
}

/// Verbatim from `pipelines/kolide/issues/categorize.yml`: statements
/// after the branch mean the pattern does not describe the whole script, so
/// it declines rather than dropping them.
#[test]
fn a_lookup_with_work_after_it_is_not_claimed() {
    let script = "def action = ctx.event.action;\ndef m = params.exact.get(action);\n\
        if (m != null) {\n  ctx.event.kind = m.kind;\n  \
        ctx.event.category = new ArrayList(m.category);\n} else {\n  \
        ctx.event.kind = 'event';\n  ctx.event.category = ['configuration'];\n}\n\n\
        if (ctx.rule?.id != null) {\n  \
        def domain = params.check_category.get(ctx.rule.id);\n}";
    assert!(parse_row_or_defaults(script).is_none());
}

/// Verbatim from `pipelines/cisco_meraki/events/default.yml`: a row whose
/// named columns are LISTS appended to array fields, alongside one copy
/// that runs whether or not the key has a row.
#[test]
fn a_rows_list_columns_append_and_the_copy_runs_regardless() {
    let script = "def alertTypeId = ctx.json.alertTypeId;\n\
        def eventMap = params.get('eventmap');\ndef eventData = eventMap.get(alertTypeId);\n\
        ctx.event.action = ctx.json.alertType;\nif (eventData == null) {\n  return;\n}\n\
        def eventCategory = eventData.get('category');\nif (eventCategory != null) {\n  \
        for (def c : eventCategory) {\n    ctx.event.category.add(c);\n  }\n}\n\
        def eventType = eventData.get('type');\nif (eventType != null) {\n  \
        for (def t : eventType) {\n    ctx.event.type.add(t);\n  }\n}";
    let params = json!({ "eventmap": {
        "cellular_up": { "type": ["start"] },
        "vrrp": { "category": ["configuration"], "type": ["change"] },
    }});

    let mut listed = Event::new(json!({
        "json": { "alertTypeId": "vrrp", "alertType": "Failover event detected" },
        "event": { "category": ["network"], "type": ["info"] }
    }));
    assert!(try_params_painless(&mut listed, script, &params));
    assert_eq!(
        listed.get_str("event.action"),
        Some("Failover event detected")
    );
    assert_eq!(
        listed.get("event.category"),
        Some(&json!(["network", "configuration"]))
    );
    assert_eq!(listed.get("event.type"), Some(&json!(["info", "change"])));

    // An unlisted key still gets the copy, which sits above the return.
    let mut unlisted = Event::new(json!({
        "json": { "alertTypeId": "mi_alert", "alertType": "Insight Alert" },
        "event": { "category": ["network"], "type": ["info"] }
    }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert_eq!(unlisted.get_str("event.action"), Some("Insight Alert"));
    assert_eq!(unlisted.get("event.category"), Some(&json!(["network"])));
    assert_eq!(unlisted.get("event.type"), Some(&json!(["info"])));
}

/// Verbatim from `pipelines/m365_defender/alert/default.yml`: the
/// categories come off the evidence list through the params table, and the
/// TYPE is picked from what the category set holds so far rather than from
/// the entry being read.
#[test]
fn evidence_categories_pick_their_type_from_the_set_so_far() {
    let script = "def eventCategory = new HashSet();\ndef eventType = new HashSet();\n\
         for (evidence in ctx.json.evidence) {\n  \
         String mapping = params[evidence[\"@odata.type\"]];\n}\n";
    let params = json!({
        "#microsoft.graph.security.deviceEvidence": "host",
        "#microsoft.graph.security.userEvidence": "iam",
        "#microsoft.graph.security.registryKeyEvidence": "registry",
        "apt": "threat",
    });

    let mut event = Event::new(json!({"json": {"evidence": [
        {"@odata.type": "#microsoft.graph.security.deviceEvidence"},
        {"@odata.type": "#microsoft.graph.security.userEvidence"},
        {"@odata.type": "#microsoft.graph.security.somethingUnmapped"},
    ]}}));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get("event.category"), Some(&json!(["host", "iam"])));
    assert_eq!(event.get("event.type"), Some(&json!(["info"])));

    // Once `registry` is in the set, everything AFTER it is `access` --
    // and the entry that put it there is too.
    let mut registry = Event::new(json!({"json": {"evidence": [
        {"@odata.type": "#microsoft.graph.security.registryKeyEvidence"},
        {"@odata.type": "#microsoft.graph.security.userEvidence"},
    ]}}));
    assert!(try_params_painless(&mut registry, script, &params));
    assert_eq!(registry.get("event.type"), Some(&json!(["access"])));

    // `determination` folds in afterwards, with no registry arm.
    let mut determined = Event::new(json!({
        "json": {"evidence": [], "determination": "APT"},
    }));
    assert!(try_params_painless(&mut determined, script, &params));
    assert_eq!(determined.get("event.category"), Some(&json!(["threat"])));
    assert_eq!(determined.get("event.type"), Some(&json!(["indicator"])));
}

/// Verbatim from `crowdstrike/data_stream/alert`, which offers two sources
/// for the platform name and takes whichever the event carries.
///
/// Only the first binding was read, so an event carrying the fallback got
/// no `host.os.type` at all.
#[test]
fn the_platform_name_falls_back_to_the_operating_system() {
    let script = "if (ctx.crowdstrike?.alert?.device?.platform_name != null) {\n  \
         String platform_name = ctx.crowdstrike.alert.device.platform_name.toLowerCase();\n  \
         for (String os: params.os_type) {\n    \
         if (platform_name.contains(os)) {\n      ctx.host.os.put('type', os);\n      \
         return;\n    }\n  }\n} else if (ctx.crowdstrike?.alert?.operating_system != null) {\n  \
         String operating_system = ctx.crowdstrike.alert.operating_system.toLowerCase();\n  \
         for (String os: params.os_type) {\n    \
         if (operating_system.contains(os)) {\n      ctx.host.os.put('type', os);\n      \
         return;\n    }\n  }\n}\n";
    let params = json!({
        "os_type": ["linux", "macos", "unix", "windows", "ios", "android"],
    });

    let mut primary = Event::new(json!({"crowdstrike": {"alert": {
        "device": {"platform_name": "Windows"}
    }}}));
    assert!(try_params_painless(&mut primary, script, &params));
    assert_eq!(primary.get_str("host.os.type"), Some("windows"));

    // The `else if` arm: no device block, so the alert's own field is read.
    let mut fallback = Event::new(json!({"crowdstrike": {"alert": {
        "operating_system": "Windows Server 2019"
    }}}));
    assert!(try_params_painless(&mut fallback, script, &params));
    assert_eq!(fallback.get_str("host.os.type"), Some("windows"));

    // Neither source present is not this script's business.
    let mut absent = Event::new(json!({"crowdstrike": {"alert": {}}}));
    try_params_painless(&mut absent, script, &params);
    assert!(!absent.has("host.os.type"));
}

/// Verbatim from `crowdstrike/data_stream/identity_protection_timeline`:
/// the family name is uppercased into the params table and the result put
/// on a field the script creates the parents for.
#[test]
fn an_uppercased_family_maps_through_params_onto_its_field() {
    let script = "def os = params[ctx.crowdstrike.idp.timeline.operating_system_info.family\
         .toUpperCase()];\nif (os != null) {\n  ctx.host = ctx.host ?: [:];\n  \
         ctx.host.os = ctx.host.os ?: [:];\n  ctx.host.os.type = os;\n}\n";
    let params = json!({
        "WINDOWS": "windows", "OSX": "macos", "UNIX": "unix",
        "LINUX": "linux", "IOS": "ios", "ANDROID": "android",
    });

    let mut event = Event::new(json!({"crowdstrike": {"idp": {"timeline": {
        "operating_system_info": {"family": "Windows"}
    }}}}));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get_str("host.os.type"), Some("windows"));

    // An unlisted family fails the script's own null check, so nothing lands.
    let mut unlisted = Event::new(json!({"crowdstrike": {"idp": {"timeline": {
        "operating_system_info": {"family": "Plan9"}
    }}}}));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert!(!unlisted.has("host.os.type"));
}

/// proofpoint's message parts: renamed through the key map, then fanned
/// out into the four ECS lists. The typed keys convert on the way through
/// -- `detected_size_bytes` from text to a number, `is_archive` from text
/// to a boolean -- because the vendor ships both as strings.
#[test]
fn message_parts_rename_and_fan_out() {
    let script = "def convertToLong(def value) { }\n\
         for (part in ctx.json.msgParts) {\n  \
         def msg_part = renameKeys(part, params);\n}\n";
    let params = json!({
        "detectedName": "detected_name",
        "detectedExt": "detected_ext",
        "detectedMime": "detected_mime",
        "detectedSizeBytes": "detected_size_bytes",
        "isArchive": "is_archive",
        "md5": "md5",
        "sha256": "sha256",
        "urls": "urls",
        "url": "url",
    });
    let mut event = Event::new(json!({"json": {"msgParts": [{
        "detectedName": "note.txt",
        "detectedExt": "txt",
        "detectedMime": "text/plain",
        "detectedSizeBytes": "1024",
        "isArchive": "false",
        "md5": "5d41402abc4b2a76b9719d911017c592",
        "sha256": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        "urls": [{"url": "https://example.com/a"}, {"url": "https://example.com/b"}],
        "somethingUnmapped": "carried as it stands",
    }]}}));

    assert!(try_params_painless(&mut event, script, &params));

    let base = "proofpoint_on_demand.message.msg_parts.0";
    assert_eq!(
        event.get_str(&format!("{base}.detected_name")),
        Some("note.txt")
    );
    assert_eq!(
        event.get(&format!("{base}.detected_size_bytes")),
        Some(&json!(1024))
    );
    assert_eq!(
        event.get(&format!("{base}.is_archive")),
        Some(&json!(false))
    );
    assert_eq!(
        event.get_str(&format!("{base}.somethingUnmapped")),
        Some("carried as it stands"),
        "an unmapped key is carried, not dropped"
    );

    assert_eq!(
        event.get("related.hash"),
        Some(&json!([
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            "5d41402abc4b2a76b9719d911017c592",
        ])),
        "sha256 before md5, the script's own order"
    );
    assert_eq!(
        event.get("url.full"),
        Some(&json!(["https://example.com/a", "https://example.com/b"]))
    );
    assert_eq!(
        event.get("email.attachments"),
        Some(&json!([{"file": {
            "name": "note.txt",
            "extension": "txt",
            "mime_type": "text/plain",
            "size": 1024,
            "hash": {
                "md5": "5d41402abc4b2a76b9719d911017c592",
                "sha256": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            },
        }}]))
    );
}

/// Verbatim from `pipelines/windows/forwarded/security-default.yml`: the
/// audit subcategory GUID keys a two-member row, braces stripped and the
/// key upper-cased first.
#[test]
fn a_keyed_row_puts_its_members_back() {
    let script = "if (ctx.winlog?.event_data?.SubcategoryGuid == null) {\n  return;\n}\n\
        def subCatGuid = ctx.winlog.event_data.SubcategoryGuid.replace(\"{\",\"\")\
        .replace(\"}\",\"\").toUpperCase();\n\
        if (!params.containsKey(subCatGuid)) {\n  return;\n}\n\
        ctx.winlog.event_data.put(\"Category\", params[subCatGuid][1]);\n\
        ctx.winlog.event_data.put(\"SubCategory\", params[subCatGuid][0]);";

    let params = json!({ "0CCE9243-69AE-11D9-BED3-505054503030":
        ["Network Policy Server", "Logon/Logoff"] });
    let mut event = crate::Event::new(json!({ "winlog": { "event_data": {
        "SubcategoryGuid": "{0cce9243-69ae-11d9-bed3-505054503030}"
    }}}));
    crate::codegen_api::painless_exec_params(&mut event, script, &params).expect("runs");

    assert_eq!(
        event.get_str("winlog.event_data.Category"),
        Some("Logon/Logoff")
    );
    assert_eq!(
        event.get_str("winlog.event_data.SubCategory"),
        Some("Network Policy Server")
    );
    // The guard above the assignment is not a field of its own.
    assert!(
        event
            .get("winlog.event_data.SubcategoryGuid == null) {")
            .is_none()
    );
}

/// Verbatim from `pipelines/crowdstrike/default.yml`, so a change upstream
/// shows up here as a miss.
const SENTINEL: &str = "ctx.crowdstrike.event.entrySet().removeIf(entry -> \
                        params.values.contains(entry.getValue()));";

/// Verbatim from `pipelines/aws/cloudtrail/default.yml`. The table read is
/// the LAST thing it does; three writes come first, and a matcher that
/// took only the merge dropped all three.
const CLOUDTRAIL_CATEGORY: &str = "ctx.event.kind = 'event';\n\
    ctx.event.type = ['info'];\n\
    if (ctx.aws?.cloudtrail?.error_code != null) {\n  \
    ctx.event.outcome = 'failure'\n} else {\n  \
    ctx.event.outcome = 'success'\n}\n\
    if (params.get(ctx.event.action) == null) {\n  return;\n}\n\
    def hm = new HashMap(params.get(ctx.event.action));\n\
    hm.forEach((k, v) -> ctx.event[k] = v);";

#[test]
fn a_lookup_merge_runs_what_comes_before_the_table_read() {
    let params = json!({"CreateUser": {"category": ["iam"], "type": ["creation"]}});
    let mut event = Event::new(json!({"event": {"action": "CreateUser"}}));

    assert!(try_params_painless(
        &mut event,
        CLOUDTRAIL_CATEGORY,
        &params
    ));
    assert_eq!(event.get_str("event.kind"), Some("event"));
    assert_eq!(event.get("event.type"), Some(&json!(["creation"])));
    assert_eq!(event.get_str("event.outcome"), Some("success"));
    assert_eq!(event.get("event.category"), Some(&json!(["iam"])));
}

/// The `else` arm is taken when the guard does not hold, and a list
/// literal reaches the field as a list.
#[test]
fn an_else_arm_and_a_list_literal_both_land() {
    let params = json!({});
    let mut event = Event::new(json!({
        "event": {"action": "Unlisted"},
        "aws": {"cloudtrail": {"error_code": "AccessDenied"}},
    }));

    assert!(try_params_painless(
        &mut event,
        CLOUDTRAIL_CATEGORY,
        &params
    ));
    assert_eq!(event.get_str("event.outcome"), Some("failure"));
    assert_eq!(
        event.get("event.type"),
        Some(&json!(["info"])),
        "the table has no row, so the preamble's own list stands"
    );
}

/// Verbatim from `pipelines/okta/ecs_category_type.yml`. Elastic's `gen`
/// emits this same script for every package that maps a vendor event name
/// onto ECS categorisation.
const ADD_UNIQUE: &str = "def addUnique(List dst, List src) {\n  src = src ?: [];\n  \
                          if (src.length == 0) {\n    return dst ?: [];\n  }\n  \
                          HashSet s = new HashSet(dst ?: []);\n  s.addAll(src);\n  \
                          return new ArrayList(s);\n}\n\
                          def p = params[ctx.okta.event_type];\n\
                          ctx.event.type = addUnique(ctx.event.type, p.type);\n\
                          ctx.event.category = addUnique(ctx.event.category, p.category);\n\
                          ctx.tags = addUnique(ctx.tags, p.tags);";

/// Verbatim from `pipelines/microsoft_defender_endpoint/vulnerability`,
/// tagged `script_map_host_os_type`.
const CONTAINED: &str = "String os_platform = \
    ctx.microsoft_defender_endpoint.vulnerability.os_platform.toLowerCase();\n\
    for (String os: params.os_type) {\n  if (os_platform.contains(os)) {\n    \
    ctx.host.os.put('type', os);\n    return;\n  }\n}\n\
    if (os_platform.contains('centos') || os_platform.contains('ubuntu')) {\n  \
    ctx.host.os.put('type', 'linux');\n}\n";

fn contained_params() -> Value {
    json!({ "os_type": ["linux", "macos", "windows"] })
}

/// The first member the subject contains wins, case-folded as the script
/// folds it.
#[test]
fn the_first_contained_member_is_the_answer() {
    let mut event = Event::new(json!({
        "microsoft_defender_endpoint": { "vulnerability": { "os_platform": "Windows10" } },
    }));

    assert!(try_params_painless(
        &mut event,
        CONTAINED,
        &contained_params()
    ));

    assert_eq!(event.get("host.os.type"), Some(&json!("windows")));
}

/// A subject no member matches falls to the script's own literal tail.
#[test]
fn a_subject_no_member_matches_falls_to_the_tail() {
    let mut event = Event::new(json!({
        "microsoft_defender_endpoint": { "vulnerability": { "os_platform": "CentOS7" } },
    }));

    assert!(try_params_painless(
        &mut event,
        CONTAINED,
        &contained_params()
    ));

    assert_eq!(event.get("host.os.type"), Some(&json!("linux")));
}

/// And one neither reaches leaves the field alone.
#[test]
fn a_subject_nothing_matches_writes_nothing() {
    let mut event = Event::new(json!({
        "microsoft_defender_endpoint": { "vulnerability": { "os_platform": "Plan9" } },
    }));

    assert!(try_params_painless(
        &mut event,
        CONTAINED,
        &contained_params()
    ));

    assert_eq!(event.get("host.os.type"), None);
}

/// Verbatim from `pipelines/microsoft_dnsserver/analytical/default.yml`,
/// cut to the branches that decide a key's fate.
const RENAME_KEYS: &str = "def renameKeys(Map src, Map keyMap) {\n  \
    def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    \
    def key = entry.getKey();\n    def value = entry.getValue();\n    \
    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        \
    dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        \
    dst[key] = renameKeys(value, keyMap);\n      }\n    } else {\n      \
    if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } \
    else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\n\
    ctx.microsoft_dnsserver.analytical = \
    renameKeys(ctx.microsoft_dnsserver.analytical, params)";

fn rename_params() -> Value {
    json!({ "QNAME": "question_name", "XID": "xid", "SID": "sid" })
}

/// A named key is renamed at any depth; one the map does not name keeps
/// its own, and the values are carried across untouched.
#[test]
fn named_keys_are_renamed_at_every_depth() {
    let mut event = Event::new(json!({
        "microsoft_dnsserver": { "analytical": {
            "QNAME": "google.es.",
            "XID": "7",
            "Untouched": "kept",
            "extended_data": { "SID": "S-1-5-21" },
            "listed": [{ "XID": "9" }, "plain"],
        } },
    }));

    assert!(try_params_painless(
        &mut event,
        RENAME_KEYS,
        &rename_params()
    ));

    let analytical = event.get("microsoft_dnsserver.analytical").unwrap();
    assert_eq!(analytical.get("question_name"), Some(&json!("google.es.")));
    assert_eq!(analytical.get("xid"), Some(&json!("7")));
    assert_eq!(analytical.get("Untouched"), Some(&json!("kept")));
    assert_eq!(analytical.get("QNAME"), None);
    assert_eq!(
        event.get("microsoft_dnsserver.analytical.extended_data.sid"),
        Some(&json!("S-1-5-21"))
    );
    assert_eq!(
        analytical.get("listed"),
        Some(&json!([{ "xid": "9" }, "plain"]))
    );
}

/// Verbatim from `pipelines/cisco_asa/default.yml`, tagged
/// `script_ecs_outcome_categorization`: a message id, then its outcome.
const TWO_LEVEL: &str =
    "params.get(ctx.event.code)?.get(ctx._temp_.outcome)?.forEach((k, v) -> ctx.event[k] = v);";

fn two_level_params() -> Value {
    json!({
        "106100": {
            "denied": { "type": ["connection", "denied"], "outcome": "failure", "action": "firewall-rule" },
            "permitted": { "type": ["connection", "allowed"], "outcome": "success", "action": "firewall-rule" },
        },
    })
}

/// Only the row the outcome selects is merged. Following one level fanned
/// the second level's keys out as fields of their own.
#[test]
fn a_two_level_table_merges_the_row_the_outcome_selects() {
    let mut event = Event::new(json!({
        "event": { "code": "106100" },
        "_temp_": { "outcome": "permitted" },
    }));

    assert!(try_params_painless(
        &mut event,
        TWO_LEVEL,
        &two_level_params()
    ));

    assert_eq!(event.get("event.outcome"), Some(&json!("success")));
    assert_eq!(event.get("event.action"), Some(&json!("firewall-rule")));
    assert_eq!(
        event.get("event.type"),
        Some(&json!(["connection", "allowed"]))
    );
    assert_eq!(event.get("event.denied"), None);
}

/// An outcome the table has no row for leaves the event alone.
#[test]
fn a_two_level_table_with_no_row_for_the_outcome_writes_nothing() {
    let mut event = Event::new(json!({
        "event": { "code": "106100" },
        "_temp_": { "outcome": "est-allowed" },
    }));

    assert!(try_params_painless(
        &mut event,
        TWO_LEVEL,
        &two_level_params()
    ));

    assert_eq!(event.get("event.action"), None);
}

/// Verbatim from `pipelines/cisco_ftd/default.yml`, tagged
/// `script_a14307b2`, cut to its loop.
const VALUE_MAPS: &str = "def getField(Map src, String[] path) {\n return null;\n}\n\
    def setField(Map dest, String[] path, def value) {\n return null;\n}\n\
    for (entry in params.entrySet()) {\n  def srcField = entry.getKey();\n  \
    def param = entry.getValue();\n  \
    def rawVal = getField(ctx, srcField.splitOnToken('.'));\n  \
    if (rawVal == null) continue;\n  String oldVal;\n  \
    if (rawVal instanceof AbstractList) {\n    if (rawVal.size() == 0) continue;\n    \
    oldVal = rawVal[0].toString();\n  } else {\n    oldVal = rawVal.toString();\n  }\n  \
    def newVal = param.map?.getOrDefault(oldVal.toLowerCase(), null);\n  \
    if (newVal != null) {\n    def dstField = param.getOrDefault('target', srcField);\n    \
    setField(ctx, dstField.splitOnToken('.'), newVal);\n  }\n}\n";

fn value_map_params() -> Value {
    json!({
        "dns.question.type": { "map": { "a host address": "A", "ip6 address": "AAAA" } },
        "dns.response_code": { "map": { "no error": "NOERROR" } },
        "ctx._temp_.cisco.message_id": {
            "target": "event.action",
            "map": { "430002": "connection-started" },
        },
    })
}

/// The device spells the record type out; ECS wants the mnemonic. The key
/// is folded to lower case, because the script folds it.
#[test]
fn each_field_is_normalised_through_its_own_map() {
    let mut event = Event::new(json!({
        "dns": { "question": { "type": "a host address" }, "response_code": "No error" },
    }));

    assert!(try_params_painless(
        &mut event,
        VALUE_MAPS,
        &value_map_params()
    ));

    assert_eq!(event.get("dns.question.type"), Some(&json!("A")));
    assert_eq!(event.get("dns.response_code"), Some(&json!("NOERROR")));
}

/// A params key written with a `ctx.` prefix names no field, so it matches
/// nothing -- which is what it does upstream too, and it is the reason
/// FTD's `message_id` never reaches `event.action` through this script.
#[test]
fn a_source_path_that_names_no_field_writes_nothing() {
    let mut event = Event::new(json!({ "_temp_": { "cisco": { "message_id": "430002" } } }));

    assert!(try_params_painless(
        &mut event,
        VALUE_MAPS,
        &value_map_params()
    ));

    assert_eq!(event.get("event.action"), None);
}

/// A value the map has no row for is left exactly as it was.
#[test]
fn a_value_absent_from_the_map_is_left_alone() {
    let mut event = Event::new(json!({ "dns": { "question": { "type": "mail exchange" } } }));

    assert!(try_params_painless(
        &mut event,
        VALUE_MAPS,
        &value_map_params()
    ));

    assert_eq!(
        event.get("dns.question.type"),
        Some(&json!("mail exchange"))
    );
}

/// Verbatim from `pipelines/cisco_ftd/default.yml`, tagged
/// `script_categorize_event`, cut to two of its outcome branches.
const CATEGORISE: &str = "if (ctx.event?.action == null || \
    !params.containsKey(ctx.event.action)) {\n  return;\n}\n\
    ctx.event.kind = params.get(ctx.event.action).get('kind');\n\
    ctx.event.category = params.get(ctx.event.action).get('category').clone();\n\
    ctx.event.type = params.get(ctx.event.action).get('type').clone();\n\
    if (ctx.event?.outcome == null) {\n  return;\n}\n\
    if (ctx.event.category.contains('network') || \
    ctx.event.category.contains('intrusion_detection')) {\n  \
    if (ctx.event.outcome == 'success') {\n    ctx.event.type.add('allowed');\n  }\n  \
    if (ctx.event.outcome == 'block') {\n    ctx.event.outcome = 'success';\n    \
    ctx.event.type.add('denied');\n  }\n}\n";

fn categorise_params() -> Value {
    json!({
        "flow-expiration": {
            "kind": "event",
            "category": ["network"],
            "type": ["connection", "end"],
        },
        "intrusion-detected": {
            "kind": "alert",
            "category": ["intrusion_detection"],
            "type": ["info"],
        },
    })
}

/// The row's three columns land as they are when there is no outcome to
/// refine them by.
#[test]
fn an_action_row_sets_the_ecs_categorisation() {
    let mut event = Event::new(json!({ "event": { "action": "flow-expiration" } }));

    assert!(try_params_painless(
        &mut event,
        CATEGORISE,
        &categorise_params()
    ));

    assert_eq!(event.get("event.kind"), Some(&json!("event")));
    assert_eq!(event.get("event.category"), Some(&json!(["network"])));
    assert_eq!(event.get("event.type"), Some(&json!(["connection", "end"])));
}

/// A vendor outcome is rewritten to the ECS one AND adds its own type.
#[test]
fn a_vendor_outcome_is_translated_and_adds_its_type() {
    let mut event = Event::new(json!({
        "event": { "action": "flow-expiration", "outcome": "block" },
    }));

    assert!(try_params_painless(
        &mut event,
        CATEGORISE,
        &categorise_params()
    ));

    assert_eq!(event.get("event.outcome"), Some(&json!("success")));
    assert_eq!(
        event.get("event.type"),
        Some(&json!(["connection", "end", "denied"]))
    );
}

/// An action with no row leaves the event exactly as it was.
#[test]
fn an_action_with_no_row_writes_nothing() {
    let mut event = Event::new(json!({ "event": { "action": "not-in-the-table" } }));

    assert!(try_params_painless(
        &mut event,
        CATEGORISE,
        &categorise_params()
    ));

    assert_eq!(event.get("event.kind"), None);
    assert_eq!(event.get("event.category"), None);
}

/// Verbatim from `pipelines/auth0/logs/default.yml`, tagged "Sets event
/// type, category and action based on type".
const AUTH0_ACTION: &str = "def eventType = ctx.auth0.logs.data.type;\n\
    def actions = params.get('actions');\n\
    def actionData = actions.get(eventType);\n\
    if (actionData == null) {\n    \
    ctx.event.action = 'unknown-' + eventType;\n    \
    ctx.event.type = ['info'];\n    \
    return;\n}\n\
    def eventTypeVal = actionData.get('value');\n\
    if (eventTypeVal != null) {\n    \
    ctx.auth0.logs.data.type = eventTypeVal;\n}\n\
    def actionType = actionData.get('type');\n\
    if (actionType != null) {\n  \
    ctx.event.type = new ArrayList(actionType);\n}\n\
    def actionCategory = actionData.get('category');\n\
    if (actionCategory != null) {\n  \
    for (def c : actionCategory) {\n    \
    ctx.event.category.add(c);\n  }\n}\n\
    def action = actionData.get('action');\n\
    if (action != null) {\n  \
    ctx.event.action = action;\n}\n\
    def classification = actionData.get('classification');\n\
    if (classification != null) {\n  \
    ctx.auth0.logs.data.classification = classification;\n}\n\
    if (classification.toLowerCase().contains(\"success\")) {\n  \
    ctx.event.outcome = \"success\";\n} else if \
    (classification.toLowerCase().contains(\"failure\")) {\n  \
    ctx.event.outcome = \"failure\";\n} else {\n  \
    ctx.event.outcome = \"unknown\";\n}";

/// Two real rows from the `actions` table, trimmed from the vendor's 105.
fn auth0_action_params() -> Value {
    json!({
        "actions": {
            "fu": {
                "classification": "Login - Failure",
                "value": "Invalid email or username",
                "type": ["info", "denied"],
                "category": ["intrusion_detection"],
                "action": "invalid-username-or-email",
            },
            "s": {
                "classification": "Login - Success",
                "value": "Successful login",
                "type": ["info", "start"],
                "category": ["session"],
                "action": "successful-login",
            },
        }
    })
}

/// A hit overwrites the field it was keyed by, fans four more columns
/// onto ctx -- appending `category` rather than replacing it -- and
/// derives `event.outcome` from the row's classification text. All six
/// writes, verbatim from `test-login-failure`'s "fu" event.
#[test]
fn auth0_hit_overwrites_its_key_and_fans_five_columns() {
    let mut event = Event::new(json!({
        "auth0": { "logs": { "data": { "type": "fu" } } },
        "event": { "category": ["authentication"] },
    }));

    assert!(try_params_painless(
        &mut event,
        AUTH0_ACTION,
        &auth0_action_params()
    ));

    assert_eq!(
        event.get("auth0.logs.data.type"),
        Some(&json!("Invalid email or username"))
    );
    assert_eq!(event.get("event.type"), Some(&json!(["info", "denied"])));
    assert_eq!(
        event.get("event.category"),
        Some(&json!(["authentication", "intrusion_detection"]))
    );
    assert_eq!(
        event.get("event.action"),
        Some(&json!("invalid-username-or-email"))
    );
    assert_eq!(
        event.get("auth0.logs.data.classification"),
        Some(&json!("Login - Failure"))
    );
    assert_eq!(event.get("event.outcome"), Some(&json!("failure")));
}

/// A key the table has no row for still writes `event.action` (prefixed
/// `unknown-`) and `event.type`, and touches nothing else -- the
/// script's own early return.
#[test]
fn auth0_miss_writes_the_unknown_prefix_and_stops() {
    let mut event = Event::new(json!({
        "auth0": { "logs": { "data": { "type": "zz" } } },
        "event": { "category": ["authentication"] },
    }));

    assert!(try_params_painless(
        &mut event,
        AUTH0_ACTION,
        &auth0_action_params()
    ));

    assert_eq!(event.get("event.action"), Some(&json!("unknown-zz")));
    assert_eq!(event.get("event.type"), Some(&json!(["info"])));
    assert_eq!(event.get("auth0.logs.data.type"), Some(&json!("zz")));
    assert_eq!(event.get("auth0.logs.data.classification"), None);
    assert_eq!(event.get("event.outcome"), None);
    assert_eq!(
        event.get("event.category"),
        Some(&json!(["authentication"]))
    );
}

/// Verbatim from `pipelines/cisco_ftd/default.yml`, trimmed to the two
/// helper definitions and the loop -- the 79-name list is replaced by two
/// of its members, since the matcher reads the list rather than knowing it.
const KEYED: &str = "boolean isEmpty(def value) {\n  return (value instanceof \
    AbstractList ? value.size() : value.length()) == 0;\n}\n\
    def appendOrCreate(Map dest, String[] path, def value) {\n return null;\n}\n\
    def msg = ctx._temp_.orig_security;\ndef counters = new HashMap();\n\
    def dest = new HashMap();\ndef dest_event = new HashMap();\n\
    def security_event_list = new ArrayList(['dst_ip', 'src_ip']);\n\
    ctx._temp_.cisco['security'] = dest;\n\
    ctx._temp_.cisco['security_event'] = dest_event;\n\
    for (entry in msg.entrySet()) {\n def param = params.get(entry.getKey());\n \
    if (param == null) {\n   continue;\n }\n \
    param.getOrDefault('id', []).forEach( id -> counters[id] = 1 + \
    counters.getOrDefault(id, 0) );\n if (!isEmpty(entry.getValue())) {\n  \
    param.getOrDefault('ecs', []).forEach( field -> appendOrCreate(ctx, \
    field.splitOnToken('.'), entry.getValue()) );\n  \
    if (security_event_list.contains(param.target)){\n    \
    dest_event[param.target] = entry.getValue();\n  }\n  else{\n    \
    dest[param.target] = entry.getValue();\n  }\n }\n}\n\
    if (ctx._temp_.cisco.message_id != \"\") return;\ndef best;\n\
    for (entry in counters.entrySet()) {\n if (best == null || \
    best.getValue() < entry.getValue()) best = entry;\n}\n\
    if (best != null) ctx._temp_.cisco.message_id = best.getKey();\n";

fn keyed_params() -> Value {
    json!({
        "DstIP": { "target": "dst_ip", "id": ["430002"], "ecs": ["destination.address"] },
        "SrcIP": { "target": "src_ip", "id": ["430002"], "ecs": ["source.address"] },
        "AC_RuleName": { "target": "access_control_rule_name", "id": ["430002"] },
        "Protocol": { "target": "protocol", "ecs": ["network.transport"] },
    })
}

/// falco's category ladder: a literal list, a local bound to the field,
/// and a second local holding its lower-cased form.
const FALCO_CATEGORY: &str = "def allowedValues = ['file', 'network', 'process'];\n\
    if (ctx?.falco?.output_fields?.evt != null && \
    ctx?.falco?.output_fields?.evt?.category != null) {\n\
    def inputCategory = ctx?.falco?.output_fields?.evt?.category;\n\
    def lowercaseCategory = inputCategory.toLowerCase();\n\
    if (allowedValues.contains(lowercaseCategory)) {\n\
    ctx.event.category = [inputCategory];\n\
    } else if (inputCategory == 'user') {\n\
    ctx.event.category = ['session'];\n\
    } else {\n\
    ctx.event.category = ['process'];\n\
    }\n} else {\n ctx.event.category = ['process'];\n}";

fn falco_category(category: &str) -> Value {
    let mut event = Event::new(json!({
        "falco": { "output_fields": { "evt": { "category": category } } },
        "event": {},
    }));
    Program::parse(FALCO_CATEGORY).run(&mut event);
    event.get("event.category").cloned().unwrap_or(Value::Null)
}

/// A local is a name for a field, and a guard asking about it is asking
/// about the field.
///
/// `allowedValues.contains(lowercaseCategory)` needs two hops --
/// `lowercaseCategory` to `inputCategory.toLowerCase()`, and that to the
/// `ctx.` path -- before the membership test can be read at all. Without
/// them the guard is `Never`, the first arm never runs, and every category
/// the else-ifs do not name falls to the final `['process']`. falco's
/// `file` and `network` events were categorised as `process`.
#[test]
fn a_local_naming_a_field_is_resolved_through_its_fold() {
    assert_eq!(falco_category("file"), json!(["file"]));
    assert_eq!(falco_category("network"), json!(["network"]));
    // The fold is what makes the membership test match.
    assert_eq!(falco_category("FILE"), json!(["FILE"]));
    // Not in the list, named by an else-if.
    assert_eq!(falco_category("user"), json!(["session"]));
    // Not in the list and not named: the vendor's own fallback.
    assert_eq!(falco_category("wat"), json!(["process"]));
}

/// cloudflare normalises an epoch to milliseconds by its MAGNITUDE, in
/// two data streams and 14 call sites.
///
/// Three things had to become readable together, which is why this is one
/// pattern rather than three patches: a numeric local behind a `(long)`
/// cast, a `>` comparison against a scientific literal, and a division.
#[test]
fn an_epoch_is_normalised_by_its_magnitude() {
    const SCRIPT: &str = "long t = (long)(ctx.json.Timestamp);\n\
        if (t > (long)(1e18)) {\n  ctx.json.Timestamp = t/(long)(1e6)\n\
        } else if (t < (long)(1e10))  {\n  ctx.json.Timestamp = t*(long)(1e3)\n}\n";

    fn normalised(stamp: i64) -> Value {
        let mut event = Event::new(json!({ "json": { "Timestamp": stamp } }));
        Program::parse(SCRIPT).run(&mut event);
        event.get("json.Timestamp").cloned().unwrap_or(Value::Null)
    }

    // Nanoseconds down to milliseconds.
    assert_eq!(
        normalised(1_771_459_200_000_000_000),
        json!(1_771_459_200_000_i64)
    );
    // Seconds up to milliseconds.
    assert_eq!(normalised(1_771_459_200), json!(1_771_459_200_000_i64));
    // Already milliseconds: neither arm holds.
    assert_eq!(normalised(1_771_459_200_000), json!(1_771_459_200_000_i64));
}

/// The gate has to accept every statement the handler parses.
///
/// `couchbase_cache` appends a Prometheus label to `tags` behind a
/// null-guard. `parse_literal_statement` read the `.add(` all along and
/// `statement_is_runnable` refused it, so the script was never whole and
/// nothing claimed it -- the F53 pair disagreeing, live.
#[test]
fn an_append_is_a_statement_the_gate_accepts() {
    let script = "if (ctx.tags == null) {\n    ctx.tags = new ArrayList();\n} \
        ctx.tags.add(ctx.prometheus.labels.job)";
    let program = Program::parse(script);
    assert!(program.is_whole(), "{program:?}");

    let mut event = Event::new(json!({ "prometheus": { "labels": { "job": "cache" } } }));
    assert!(program.run(&mut event));
    assert_eq!(event.get("tags"), Some(&json!(["cache"])));
}

/// A prune is the other half of many an `if`/`else` that sets on one arm,
/// and `tychon_browser` is the pattern: a sentinel timestamp when the vendor
/// said "installed", the key gone otherwise.
#[test]
fn a_remove_is_a_statement_the_walk_can_run() {
    const SCRIPT: &str = "if (['installed', 'true'].contains(ctx.tychon.package.installed)) {\n  \
        ctx.tychon.package.installed = '1970-01-01T00:00:01Z';\n} else {\n  \
        ctx.tychon.package.remove('installed');\n}\n";

    fn installed(value: &str) -> Value {
        let mut event = Event::new(json!({
            "tychon": { "package": { "installed": value, "name": "firefox" } },
        }));
        Program::parse(SCRIPT).run(&mut event);
        event.as_value().clone()
    }

    assert_eq!(
        installed("installed"),
        json!({ "tychon": { "package": {
            "installed": "1970-01-01T00:00:01Z", "name": "firefox" } } })
    );
    // The else arm prunes the key and leaves everything beside it.
    assert_eq!(
        installed("2026-01-01"),
        json!({ "tychon": { "package": { "name": "firefox" } } })
    );
}

/// tychon coerces a duration to a whole number in every one of its
/// streams, 38 call sites of one script.
///
/// Painless truncates toward zero on a `(long)` cast rather than rounding,
/// and the `.toString()` in the middle is why the vendor wrote it this
/// way: the field arrives as a string on some events and a number on
/// others.
#[test]
fn a_double_cast_to_long_truncates() {
    const SCRIPT: &str = "if (ctx.tychon?.script?.current_duration != null)\n{\n  \
        ctx.tychon.script.current_duration =\n    \
        (long) Double.parseDouble(ctx.tychon.script.current_duration.toString());\n}\n";

    fn coerced(value: Value) -> Value {
        let mut event = Event::new(json!({ "tychon": { "script": {} } }));
        event
            .set("tychon.script.current_duration", value)
            .expect("sets");
        Program::parse(SCRIPT).run(&mut event);
        event
            .get("tychon.script.current_duration")
            .cloned()
            .unwrap_or(Value::Null)
    }

    assert_eq!(coerced(json!("12.7")), json!(12));
    assert_eq!(coerced(json!(12.7)), json!(12));
    assert_eq!(coerced(json!("-3.9")), json!(-3));
    assert_eq!(coerced(json!(5)), json!(5));
    // Nothing to parse leaves the field as it was.
    assert_eq!(coerced(json!("nope")), json!("nope"));
}

/// A list's member count, written to the `_count` field beside it.
///
/// 37 sites over 31 files, and the `instanceof` guard around it has been
/// readable since this session -- only the WRITE was missing.
#[test]
fn a_list_writes_its_own_member_count() {
    const SCRIPT: &str = "if (ctx.process.args instanceof List) {\n  \
        ctx.process.args_count = ctx.process.args.size();\n}";

    fn counted(args: &Value) -> Value {
        let mut event = Event::new(json!({ "process": { "args": args } }));
        Program::parse(SCRIPT).run(&mut event);
        event
            .get("process.args_count")
            .cloned()
            .unwrap_or(Value::Null)
    }

    assert_eq!(counted(&json!(["-l", "-a", "/tmp"])), json!(3));
    assert_eq!(counted(&json!([])), json!(0));
    // Not a list: the script's own guard declines, so nothing is written.
    assert_eq!(counted(&json!("-l -a /tmp")), Value::Null);
}

/// `jamf_protect`'s telemetry lookup, verbatim, one of forty such sites.
///
/// A NAMED table read through a ternary with a LITERAL default -- the
/// sibling of `UppercaseLookupDefault`, which folds case, reads the whole
/// `params` map, spells it `getOrDefault` and defaults to the key.
#[test]
fn a_named_table_lookup_falls_back_to_its_literal() {
    fn address_type(script: &str, params: &Value, held: Value) -> Value {
        let mut event = Event::new(json!({
            "jamf_protect": { "telemetry": { "event": { "screensharing_attach": {} } } },
        }));
        if !held.is_null() {
            event
                .set(
                    "jamf_protect.telemetry.event.screensharing_attach.source_address_type",
                    held,
                )
                .expect("sets");
        }
        try_params_painless(&mut event, script, params);
        event
            .get("jamf_protect.telemetry.source_address_type")
            .cloned()
            .unwrap_or(Value::Null)
    }

    const SCRIPT: &str = "if (ctx.jamf_protect?.telemetry?.event?.screensharing_attach\
        ?.source_address_type != null) {\n    String itemType = ctx.jamf_protect.telemetry\
        .event.screensharing_attach.source_address_type.toString();\n    \
        def itemTypeString = params.itemTypeMap.containsKey(itemType) ? \
        params.itemTypeMap[itemType] : 'Unknown';\n    \
        ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    \
        ctx.jamf_protect.telemetry.source_address_type = itemTypeString;\n}\n";

    let params = json!({ "itemTypeMap": { "4": "IPv4", "6": "IPv6" } });

    assert_eq!(address_type(SCRIPT, &params, json!("4")), json!("IPv4"));
    // A number keys the table the same way: the script calls toString().
    assert_eq!(address_type(SCRIPT, &params, json!(6)), json!("IPv6"));
    // Not in the table: the script's own literal, not the key.
    assert_eq!(address_type(SCRIPT, &params, json!("9")), json!("Unknown"));
    // Absent: the `!= null` guard writes nothing, not the default.
    assert_eq!(address_type(SCRIPT, &params, Value::Null), Value::Null);
}

/// A local can be named after a field, and `carbonblack_edr` binds one
/// called `event`.
///
/// Substituting a segment that follows a dot rewrites the document's own
/// paths: `ctx.event.action` would become `ctx.<binding>.action`. The
/// corpus caught this as one field on `carbonblack_edr`, which is what the
/// per-source ratchet is for.
#[test]
fn a_local_named_after_a_field_leaves_paths_alone() {
    let script = "def event = ctx.winlog.event_data; ctx.event.action = 'x'; \
        ctx.kept = event;";
    let rewritten = inline_ctx_aliases(script);

    assert!(
        rewritten.contains("ctx.event.action = 'x'"),
        "a path segment was rewritten: {rewritten}"
    );
    assert!(
        rewritten.contains("ctx.kept = ctx.winlog.event_data"),
        "the bare variable was not resolved: {rewritten}"
    );
    assert!(
        !rewritten.contains("def event"),
        "the binding is dead text once every use carries the path: {rewritten}"
    );
}

/// `microsoft_dhcp_log`'s event-code lookup, the pattern behind six of the
/// dead-branch sites and the same one `system_security` writes.
const DHCP_LOOKUP: &str = "if (ctx.event?.code == null || \
    params.get(ctx.event.code) == null) {\n  return;\n}\n\
    def hm = new HashMap(params[ctx.event.code]);\n\
    hm.forEach((k, v) -> ctx.event[k] = v);";

fn dhcp_params() -> Value {
    json!({
        "10": { "action": "dhcp-new", "category": ["network"], "type": ["allowed"] },
    })
}

fn dhcp_event(code: Option<&str>) -> Event {
    let mut event = Event::new(json!({ "event": { "kind": "event" } }));
    if let Some(code) = code {
        event.set("event.code", json!(code)).expect("sets");
    }
    event
}

/// A guard the evaluator cannot read is only a defect when something
/// depends on the branch behind it.
///
/// `params.get(ctx.event.code) == null` reads as `Never`, so the `||` never
/// holds on that arm and the early `return` is dead. It changes nothing:
/// the arm's whole body is that `return`, and `try_lookup_merge`
/// independently writes nothing when the table has no row. Both roads end
/// at the same document.
///
/// Established before touching it, because the count is a SUSPECT count --
/// `jamf_protect_telemetry` looked the same and was writing full
/// executable paths as `process.name` on 99 sites.
#[test]
fn a_dead_early_return_over_a_missing_row_changes_nothing() {
    let params = dhcp_params();

    let mut listed = dhcp_event(Some("10"));
    assert!(try_params_painless(&mut listed, DHCP_LOOKUP, &params));
    assert_eq!(listed.get_str("event.action"), Some("dhcp-new"));

    // The code the vendor's dead `return` was meant to catch.
    let mut unlisted = dhcp_event(Some("99"));
    try_params_painless(&mut unlisted, DHCP_LOOKUP, &params);
    assert_eq!(
        unlisted.as_value(),
        &json!({ "event": { "kind": "event", "code": "99" } }),
        "an unlisted code must leave the document as it arrived"
    );

    let mut absent = dhcp_event(None);
    try_params_painless(&mut absent, DHCP_LOOKUP, &params);
    assert_eq!(
        absent.as_value(),
        &json!({ "event": { "kind": "event" } }),
        "an absent code must leave the document as it arrived"
    );
}

/// kafka's and elasticsearch's level ladders, whose list of error levels is
/// bound to a local before it is asked.
const ERROR_LEVELS: &str = "def errorLevels = [\"ERROR\", \"FATAL\"]; \
    if (ctx?.log?.level != null) {\n  if (errorLevels.contains(ctx.log.level)) {\n \
    ctx.event.type = [\"error\"];\n  } else {\n    ctx.event.type = [\"info\"];\n  }\n}";

fn level_type(level: &str) -> Value {
    let mut event = Event::new(json!({ "log": { "level": level } }));
    Program::parse(ERROR_LEVELS).run(&mut event);
    event.get("event.type").cloned().unwrap_or(Value::Null)
}

/// A list bound to a local is still a literal, and the guard asking it has
/// to be readable.
///
/// `["ERROR"].contains(ctx.log.level)` was read all along; the same list
/// behind a `def` was not, so the guard could never hold and EVERY event
/// took the else arm. `kafka_log` and `elasticsearch_server` stamped
/// `event.type: ["info"]` on their FATAL logs across 11 call sites.
#[test]
fn a_list_bound_to_a_local_is_still_read() {
    assert_eq!(level_type("FATAL"), json!(["error"]));
    assert_eq!(level_type("ERROR"), json!(["error"]));
    assert_eq!(level_type("INFO"), json!(["info"]));
}

/// The inlining is what makes the guard readable, so it has to reach the
/// gate as well as the runner -- one rewrite ahead of both, rather than two
/// walks that can drift (F53).
#[test]
fn inlining_a_local_list_reaches_the_gate_too() {
    let inlined = inline_local_lists(ERROR_LEVELS);
    assert!(
        inlined.contains("[\"ERROR\", \"FATAL\"].contains(ctx.log.level)"),
        "the local's use carries the literal: {inlined}"
    );
    assert!(
        !readable_term("errorLevels.contains(ctx.log.level)"),
        "the raw spelling is what the gate could never read"
    );
    assert!(
        readable_term("[\"ERROR\", \"FATAL\"].contains(ctx.log.level)"),
        "the inlined spelling is the one the gate is handed"
    );
}

/// A script with no local list is handed back untouched, so the common case
/// pays no allocation.
#[test]
fn a_script_with_no_local_list_is_not_rewritten() {
    let script = "ctx.event.kind = \"event\";";
    assert!(matches!(inline_local_lists(script), Cow::Borrowed(_)));
}

/// A local assigned twice is not a constant, so it is left alone rather
/// than inlined at the wrong value.
#[test]
fn a_local_list_reassigned_later_is_left_alone() {
    let script = "def levels = [\"A\"]; levels = [\"B\"]; \
        if (levels.contains(ctx.log.level)) { ctx.event.type = [\"x\"]; }";
    assert!(matches!(inline_local_lists(script), Cow::Borrowed(_)));
}

/// A script's `.add()` targets a List it created a line earlier, so one
/// member is a one-element ARRAY. cisco's own `appendOrCreate` helper is
/// the other rule and stores the first value bare, which is why the two
/// cannot share a writer.
#[test]
fn one_added_member_is_still_a_list() {
    let script = "if (ctx.related == null) {\n  ctx.put(\"related\", new HashMap());\n}\n\
        if (ctx.related.ip == null) {\n  ctx.related.put(\"ip\", new ArrayList());\n}\n\
        ctx.related.ip.add(ctx.source.ip);";
    let mut event = Event::new(json!({ "source": { "ip": "10.100.150.9" } }));

    assert!(Program::parse(script).run(&mut event));
    assert_eq!(event.get("related.ip"), Some(&json!(["10.100.150.9"])));
}

/// Each key lands in the map its target's membership decides, and feeds
/// every ECS field its row names.
#[test]
fn a_keyed_message_fans_out_through_its_table() {
    let mut event = Event::new(json!({
        "_temp_": {
            "cisco": { "message_id": "430002" },
            "orig_security": {
                "DstIP": "10.0.1.20",
                "SrcIP": "10.0.100.30",
                "AC_RuleName": "Rule-1",
                "Protocol": "icmp",
            },
        },
    }));

    assert!(try_params_painless(&mut event, KEYED, &keyed_params()));

    assert_eq!(event.get("destination.address"), Some(&json!("10.0.1.20")));
    assert_eq!(event.get("source.address"), Some(&json!("10.0.100.30")));
    assert_eq!(event.get("network.transport"), Some(&json!("icmp")));
    assert_eq!(
        event.get("_temp_.cisco.security_event.dst_ip"),
        Some(&json!("10.0.1.20"))
    );
    // `access_control_rule_name` is not in this cut-down list, so it goes
    // to the other map -- which is the whole point of reading the list.
    assert_eq!(
        event.get("_temp_.cisco.security.access_control_rule_name"),
        Some(&json!("Rule-1"))
    );
}

/// With no id in the header, the id the most keys vote for becomes it.
#[test]
fn an_absent_message_id_is_decided_by_the_keys_present() {
    let mut event = Event::new(json!({
        "_temp_": {
            "cisco": { "message_id": "" },
            "orig_security": { "DstIP": "10.0.1.20", "Protocol": "icmp" },
        },
    }));

    assert!(try_params_painless(&mut event, KEYED, &keyed_params()));

    assert_eq!(event.get("_temp_.cisco.message_id"), Some(&json!("430002")));
}

/// An id the header already carried is never overwritten by the vote.
#[test]
fn a_message_id_already_set_survives_the_vote() {
    let mut event = Event::new(json!({
        "_temp_": {
            "cisco": { "message_id": "430003" },
            "orig_security": { "DstIP": "10.0.1.20" },
        },
    }));

    assert!(try_params_painless(&mut event, KEYED, &keyed_params()));

    assert_eq!(event.get("_temp_.cisco.message_id"), Some(&json!("430003")));
}

/// Verbatim from `pipelines/cisco_asa/default.yml`, where it is tagged
/// `script_process_iana_number`. `cisco_ftd` ships the same script.
const IANA: &str = "def net = ctx.network; def iana = params[net.transport]; \
                    if (iana != null) {\n  net['iana_number'] = iana;\n  return;\n} \
                    def reverse = new HashMap(); def[] arr = new def[] { null }; \
                    for (entry in params.entrySet()) {\n  arr[0] = entry.getValue();\n  \
                    reverse.put(String.format(\"%d\", arr), entry.getKey());\n} \
                    def trans = reverse[net.transport]; if (trans != null) {\n  \
                    net['iana_number'] = net.transport;\n  net['transport'] = trans;\n}\n";

fn iana_params() -> Value {
    json!({ "icmp": 1, "tcp": 6, "udp": 17, "gre": 47 })
}

/// The ordinary direction: a transport NAME gets its protocol number.
#[test]
fn a_transport_name_gets_its_iana_number() {
    let mut event = Event::new(json!({ "network": { "transport": "tcp" } }));

    assert!(try_params_painless(&mut event, IANA, &iana_params()));

    assert_eq!(event.get("network.iana_number"), Some(&json!(6)));
    assert_eq!(event.get("network.transport"), Some(&json!("tcp")));
}

/// The device wrote the NUMBER into `transport`. Elastic moves it across
/// and puts the name back, rather than leaving a number in a name field.
#[test]
fn a_transport_number_is_moved_and_the_name_restored() {
    let mut event = Event::new(json!({ "network": { "transport": "17" } }));

    assert!(try_params_painless(&mut event, IANA, &iana_params()));

    assert_eq!(event.get("network.iana_number"), Some(&json!("17")));
    assert_eq!(event.get("network.transport"), Some(&json!("udp")));
}

/// A transport the table does not carry is left exactly as it was --
/// guessing a number for it would be worse than having none.
#[test]
fn an_unknown_transport_is_left_alone() {
    let mut event = Event::new(json!({ "network": { "transport": "sctp" } }));

    assert!(try_params_painless(&mut event, IANA, &iana_params()));

    assert_eq!(event.get("network.iana_number"), None);
    assert_eq!(event.get("network.transport"), Some(&json!("sctp")));
}

fn add_unique_params() -> Value {
    json!({
        "user.session.start": {
            "category": ["authentication"],
            "type": ["start"],
            "tags": ["identity"],
        },
    })
}

/// The row's columns must be UNIONED into what the pipeline already put
/// there -- the append processors ahead of this one have usually written
/// `event.type` already, and replacing it drops their work.
#[test]
fn add_unique_unions_the_row_into_the_existing_arrays() {
    let mut event = Event::new(json!({
        "okta": { "event_type": "user.session.start" },
        "event": { "type": ["info"], "category": ["session"] },
        "tags": ["forwarded"],
    }));

    assert!(try_params_painless(
        &mut event,
        ADD_UNIQUE,
        &add_unique_params()
    ));

    assert_eq!(event.get("event.type"), Some(&json!(["info", "start"])));
    assert_eq!(
        event.get("event.category"),
        Some(&json!(["session", "authentication"]))
    );
    assert_eq!(event.get("tags"), Some(&json!(["forwarded", "identity"])));
}

/// A member the row repeats must not appear twice -- the script is a set
/// union, not an append.
#[test]
fn add_unique_does_not_duplicate_what_is_already_there() {
    let mut event = Event::new(json!({
        "okta": { "event_type": "user.session.start" },
        "event": { "type": ["start"] },
    }));

    assert!(try_params_painless(
        &mut event,
        ADD_UNIQUE,
        &add_unique_params()
    ));
    assert_eq!(event.get("event.type"), Some(&json!(["start"])));
}

/// An arrays-absent event still gets the row, since `addUnique` treats a
/// null destination as empty.
#[test]
fn add_unique_creates_the_arrays_it_finds_missing() {
    let mut event = Event::new(json!({ "okta": { "event_type": "user.session.start" } }));

    assert!(try_params_painless(
        &mut event,
        ADD_UNIQUE,
        &add_unique_params()
    ));
    assert_eq!(
        event.get("event.category"),
        Some(&json!(["authentication"]))
    );
}

/// Verbatim from `pipelines/crowdstrike/default.yml`, trimmed to the loops
/// that decide the answer.
const FRAMEWORK: &str = "def tid = ctx.threat.tactic?.id;\n\
    def nid = ctx.threat.technique?.id;\n\
    def tname = ctx.threat.tactic?.name;\n\
    Set frameworks = new HashSet();\n\
    if (tid != null && !tid.isEmpty()) {\n  for (String t: tid) {\n    \
    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n    \
    else if (t.startsWith(\"TA\")) {\n      frameworks.add(params.framework_ma);\n    }\n  }\n}\n\
    if (nid != null && !nid.isEmpty()) {\n  for (String t: nid) {\n    \
    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\n\
    if (tname != null && !tname.isEmpty()) {\n  for (String t: tname) {\n    \
    if (params.falcon_tactic_names.contains(t.toLowerCase())) {\n      \
    frameworks.add(params.framework_cs);\n    }\n  }\n}\n\
    for (def preferred : params.framework_preference) {\n  \
    if (frameworks.contains(preferred)) {\n    ctx.threat.framework = preferred;\n    \
    return;\n  }\n}";

fn framework_params() -> Value {
    json!({
        "framework_preference": ["MITRE ATT&CK", "CrowdStrike Falcon Detections Framework"],
        "framework_cs": "CrowdStrike Falcon Detections Framework",
        "framework_ma": "MITRE ATT&CK",
        "falcon_tactic_names": ["malware", "exploit", "falcon overwatch"],
    })
}

#[test]
fn framework_reads_mitre_off_a_ta_prefixed_tactic() {
    let mut event = Event::new(json!({
        "threat": { "tactic": { "id": ["TA0002"] } },
    }));

    assert!(try_params_painless(
        &mut event,
        FRAMEWORK,
        &framework_params()
    ));
    assert_eq!(event.get_str("threat.framework"), Some("MITRE ATT&CK"));
}

/// A tactic NAME in the params list is Falcon's own framework, and the
/// comparison is case-insensitive because the script lower-cases first.
#[test]
fn framework_reads_falcon_off_a_named_tactic() {
    let mut event = Event::new(json!({
        "threat": { "tactic": { "name": ["Falcon OverWatch"] } },
    }));

    assert!(try_params_painless(
        &mut event,
        FRAMEWORK,
        &framework_params()
    ));
    assert_eq!(
        event.get_str("threat.framework"),
        Some("CrowdStrike Falcon Detections Framework")
    );
}

/// Both frameworks present is what the preference list exists for, and
/// MITRE is first in it.
#[test]
fn framework_follows_the_declared_preference_when_both_match() {
    let mut event = Event::new(json!({
        "threat": {
            "tactic": { "id": ["CS0001", "TA0002"] },
        },
    }));

    assert!(try_params_painless(
        &mut event,
        FRAMEWORK,
        &framework_params()
    ));
    assert_eq!(event.get_str("threat.framework"), Some("MITRE ATT&CK"));
}

/// Nothing matching writes nothing -- the script returns early rather than
/// picking the first preference.
#[test]
fn framework_writes_nothing_when_no_rule_matches() {
    let mut event = Event::new(json!({
        "threat": { "tactic": { "id": ["XX0001"] } },
    }));

    assert!(try_params_painless(
        &mut event,
        FRAMEWORK,
        &framework_params()
    ));
    assert!(!event.has("threat.framework"));
}

/// An event type the table does not carry throws in Painless and the
/// sub-pipeline's `on_failure` catches it. Nothing here models the throw,
/// so the event must at least come through untouched rather than mangled.
#[test]
fn add_unique_leaves_an_unknown_event_type_alone() {
    let mut event = Event::new(json!({
        "okta": { "event_type": "user.session.nosuchthing" },
        "event": { "type": ["info"] },
    }));

    assert!(try_params_painless(
        &mut event,
        ADD_UNIQUE,
        &add_unique_params()
    ));
    assert_eq!(event.get("event.type"), Some(&json!(["info"])));
}

fn sentinel_params() -> Value {
    json!({ "values": [null, "", "-", "N/A", "NA", 0] })
}

#[test]
fn strips_every_sentinel_the_params_name() {
    let mut event = Event::new(json!({
        "crowdstrike": { "event": {
            "keep": "value", "empty": "", "dash": "-", "na": "NA", "zero": 0, "nul": null,
        }},
    }));

    assert!(try_params_painless(
        &mut event,
        SENTINEL,
        &sentinel_params()
    ));

    let obj = event.get_object("crowdstrike.event").unwrap();
    assert_eq!(obj.len(), 1, "only `keep` survives: {obj:?}");
    assert_eq!(obj.get("keep"), Some(&json!("value")));
}

/// The metadata variant omits `0`, and a numeric zero must then survive.
#[test]
fn a_sentinel_absent_from_params_is_kept() {
    let script = SENTINEL.replace("crowdstrike.event", "crowdstrike.metadata");
    let mut event = Event::new(json!({
        "crowdstrike": { "metadata": { "zero": 0, "dash": "-" } },
    }));

    assert!(try_params_painless(
        &mut event,
        &script,
        &json!({ "values": [null, "", "-", "N/A", "NA"] }),
    ));

    let obj = event.get_object("crowdstrike.metadata").unwrap();
    assert_eq!(obj.get("zero"), Some(&json!(0)));
    assert!(!obj.contains_key("dash"));
}

/// Verbatim from `pipelines/windows/forwarded/security_standard.yml`.
const UAC_FLAGS: &str = "if (ctx.winlog?.event_data == null) {\n  return;\n}\n\
     Long newUacValue;\ntry {\n\
     newUacValue = Long.decode(ctx.winlog.event_data.NewUacValue.trim());\n\
     } catch (Exception e) {\n  return;\n}\nArrayList uacResult = new ArrayList();\n\
     for (entry in params.entrySet()) {\n  Long flag = Long.decode(entry.getKey());\n\
     if ((newUacValue.longValue() & flag.longValue()) == flag.longValue()) {\n\
     uacResult.add(entry.getValue());\n  }\n}\nif (uacResult.length == 0) {\n  return;\n}\n\
     ctx.winlog.event_data.put(\"NewUACList\", uacResult);\n";

/// The list comes out in the PARAMS' order, and the vendor's two tables
/// disagree about direction -- so neither sorting nor reversing is right.
#[test]
fn a_flag_list_keeps_the_params_order() {
    let mut event = Event::new(json!({
        "winlog": { "event_data": { "NewUacValue": " 0x210 " } },
    }));
    assert!(try_params_painless(
        &mut event,
        UAC_FLAGS,
        &json!({ "0x00000010": "USER_NORMAL_ACCOUNT", "0x00000200": "USER_DONT_EXPIRE_PASSWORD" }),
    ));
    assert_eq!(
        event.get("winlog.event_data.NewUACList"),
        Some(&json!(["USER_NORMAL_ACCOUNT", "USER_DONT_EXPIRE_PASSWORD"])),
        "the UAC table runs lowest bit first"
    );

    // The kerberos table runs the other way, and its list follows.
    let mut ticket = Event::new(json!({
        "winlog": { "event_data": { "NewUacValue": "0x40000001" } },
    }));
    assert!(try_params_painless(
        &mut ticket,
        UAC_FLAGS,
        &json!({ "0x40000000": "Forwardable", "0x00000001": "Validate" }),
    ));
    assert_eq!(
        ticket.get("winlog.event_data.NewUACList"),
        Some(&json!(["Forwardable", "Validate"]))
    );
}

/// Verbatim from `pipelines/microsoft_defender_endpoint/log/default.yml`,
/// which wraps the removal in a recursive drop and runs `drop(ctx)` after.
const SENTINEL_WRAPPED: &str = "boolean drop(Object o) {\n  if (o == null || o == \"\") {\n\
     return true;\n  } else if (o instanceof Map) {\n\
     ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n\
     } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n\
     return (((List) o).length == 0);\n  }\n  return false;\n}\n\
     if (!ctx.json.evidence.empty) {\n\
     ctx.json.evidence.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n\
     }\ndrop(ctx);\n";

#[test]
fn a_wrapped_sentinel_removal_still_sweeps_what_it_emptied() {
    let mut event = Event::new(json!({ "json": {
        "evidence": { "sha1": "abc", "url": null, "domain": "" }, "keep": "v"
    }}));

    assert!(try_params_painless(
        &mut event,
        SENTINEL_WRAPPED,
        &json!({ "values": [null, ""] }),
    ));
    assert_eq!(event.get_str("json.evidence.sha1"), Some("abc"));
    assert!(!event.has("json.evidence.url"));

    // An evidence list the vendor ships EMPTY goes with the sweep, and
    // leaving it behind put a stray `[]` in the output.
    let mut empty = Event::new(json!({ "json": { "evidence": [], "keep": "v" } }));
    assert!(try_params_painless(
        &mut empty,
        SENTINEL_WRAPPED,
        &json!({ "values": [null, ""] }),
    ));
    assert!(!empty.has("json.evidence"));
    assert_eq!(empty.get_str("json.keep"), Some("v"));
}

#[test]
fn a_missing_map_is_not_a_failure() {
    let mut event = Event::new(json!({ "other": 1 }));
    assert!(try_params_painless(
        &mut event,
        SENTINEL,
        &sentinel_params()
    ));
}

const FILETIME: &str = "def convertToUnix(def longValue) {\n\
                        if (longValue > 0x0100000000000000L) {\n\
                        return (longValue / 10000) - 11644473600000L;\n}\nreturn longValue;\n}\n\
                        for (def field : params.values) {\n\
                        def fieldValue = ctx.crowdstrike.event[field];\n\
                        ctx.crowdstrike.event[field] = convertToUnix(fieldValue);\n}";

#[test]
fn converts_every_filetime_field_the_params_name() {
    let mut event = Event::new(json!({
        "crowdstrike": { "event": {
            // 2020-01-01T00:00:00Z as a FILETIME, as a number and as a string.
            "StartTime": 132_223_104_000_000_000_i64,
            "EndTime": "132223104000000000",
            // Already UNIX seconds -- below the threshold, so untouched.
            "ContextTimeStamp": 1_577_836_800_i64,
            "Untouched": 132_223_104_000_000_000_i64,
        }},
    }));

    assert!(try_params_painless(
        &mut event,
        FILETIME,
        &json!({ "values": ["StartTime", "EndTime", "ContextTimeStamp"] }),
    ));

    assert_eq!(
        event.get_i64("crowdstrike.event.StartTime"),
        Some(1_577_836_800_000)
    );
    assert_eq!(
        event.get_i64("crowdstrike.event.EndTime"),
        Some(1_577_836_800_000)
    );
    assert_eq!(
        event.get_i64("crowdstrike.event.ContextTimeStamp"),
        Some(1_577_836_800)
    );
    assert_eq!(
        event.get_i64("crowdstrike.event.Untouched"),
        Some(132_223_104_000_000_000)
    );
}

/// Verbatim from `pipelines/azure/activitylogs/default.yml`.
const LOOKUP: &str = "if (ctx?.azure?.activitylogs?.category == null) { return; } \
                      def category = ctx.azure.activitylogs.category.toLowerCase(); \
                      if (params.get(category) == null) { return; } \
                      def hm = new HashMap(params.get(category)); \
                      hm.forEach((k, v) -> ctx.event[k] = v);";

fn lookup_params() -> Value {
    json!({
        "write": { "type": ["change"] },
        "read": { "type": ["access"] },
        "delete": { "type": ["deletion"] },
    })
}

#[test]
fn merges_the_row_the_keyed_field_selects() {
    let mut event = Event::new(json!({
        "azure": { "activitylogs": { "category": "Write" } },
        "event": {},
    }));

    assert!(try_params_painless(&mut event, LOOKUP, &lookup_params()));
    assert_eq!(event.get("event.type"), Some(&json!(["change"])));
}

#[test]
fn a_key_absent_from_the_table_changes_nothing() {
    let mut event = Event::new(json!({
        "azure": { "activitylogs": { "category": "Unmapped" } },
        "event": {},
    }));

    assert!(try_params_painless(&mut event, LOOKUP, &lookup_params()));
    assert_eq!(event.get_object("event").unwrap().len(), 0);
}

/// panw's decryption-log flags, as the generator emits them.
const BIT_FLAGS: &str = r"def labels = ctx.labels; if (labels == null) {
  labels = new HashMap();
  ctx['labels'] = labels;
} long value = ctx._temp_.labels; for (entry in params.entrySet()) {
  def flag = entry.getValue();
  if (flag instanceof String) {
  flag = Long.decode(flag);
  }
  if ((value & flag) != 0) {
  labels[entry.getKey()] = true;
  }
}
";

fn flag_params() -> Value {
    json!({
        "nat_translated": 0x0040_0000,
        "captive_portal": 0x0020_0000,
        "ssl_decrypted": 0x0100_0000,
    })
}

#[test]
fn sets_a_label_for_every_bit_the_field_has_set() {
    let mut event = Event::new(json!({
        "_temp_": { "labels": 0x0060_0000 },
    }));

    assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
    assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
    assert_eq!(event.get("labels.captive_portal"), Some(&json!(true)));
}

/// The script only ever writes `true`, so a clear bit must leave the label
/// ABSENT -- writing `false` would be an extra field Elastic never emits.
#[test]
fn a_clear_bit_leaves_its_label_absent() {
    let mut event = Event::new(json!({
        "_temp_": { "labels": 0x0040_0000 },
    }));

    assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
    assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
    assert_eq!(event.get("labels.captive_portal"), None);
    assert_eq!(event.get("labels.ssl_decrypted"), None);
}

/// `Long.decode` is in the vendor script because the flags have been known
/// to arrive stringified, and a hex string must decode as hex.
#[test]
fn a_stringified_hex_flag_decodes() {
    let mut event = Event::new(json!({ "_temp_": { "labels": 0x0040_0000 } }));
    let params = json!({ "nat_translated": "0x00400000" });

    assert!(try_params_painless(&mut event, BIT_FLAGS, &params));
    assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
}

#[test]
fn a_bitfield_carried_as_a_string_still_decodes() {
    let mut event = Event::new(json!({ "_temp_": { "labels": "4194304" } }));

    assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
    assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
}

#[test]
fn an_absent_bitfield_writes_no_labels() {
    let mut event = Event::new(json!({ "event": {} }));

    assert!(!try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
    assert_eq!(event.get("labels"), None);
}

/// A level whose key names no field stops the descent, and nothing of the
/// row above it is merged -- the second level's keys are not fields.
#[test]
fn a_two_level_table_stops_where_the_key_is_absent() {
    let script = "params.get(ctx.event.code).get(ctx._temp_.outcome)\
                  .forEach((k, v) -> ctx.event[k] = v);";
    let mut event = Event::new(json!({ "event": { "code": "750002" } }));

    assert!(try_params_painless(
        &mut event,
        script,
        &json!({ "750002": { "success": { "action": "started" } } }),
    ));
    assert_eq!(event.get("event.action"), None);
    assert_eq!(event.get("event.success"), None);
}

/// Verbatim from `pipelines/cisco/nexus/default.yml`.
const INDEXED: &str = "def LogLevelValue = (int) ctx.event.severity;\n\
                       if (LogLevelValue >= 0 && LogLevelValue < params.LogLevel.length) {\n  \
                       ctx.log.put('level', params['LogLevel'][LogLevelValue]);\n}";

#[test]
fn indexes_the_params_array_by_the_numeric_field() {
    let mut event = Event::new(json!({ "event": { "severity": 3 }, "log": {} }));

    assert!(try_params_painless(
        &mut event,
        INDEXED,
        &json!({ "LogLevel": ["emergency", "alert", "critical", "error", "warning"] }),
    ));
    assert_eq!(event.get_str("log.level"), Some("error"));
}

/// The vendor's bounds check is the array's own length.
#[test]
fn an_index_past_the_end_sets_nothing() {
    let mut event = Event::new(json!({ "event": { "severity": 9 }, "log": {} }));

    assert!(try_params_painless(
        &mut event,
        INDEXED,
        &json!({ "LogLevel": ["emergency", "alert"] }),
    ));
    assert!(!event.has("log.level"));
}

/// Verbatim from `pipelines/azure/auditlogs/default.yml`.
const SCALE: &str = "ctx.event.duration = ctx.event.duration * params.param_nano";

#[test]
fn scales_a_duration_by_the_params_constant() {
    let mut event = Event::new(json!({ "event": { "duration": 42 } }));

    assert!(try_params_painless(
        &mut event,
        SCALE,
        &json!({ "param_nano": 1_000_000_000_i64 }),
    ));
    assert_eq!(event.get_i64("event.duration"), Some(42_000_000_000));
}

/// Verbatim from `pipelines/aws/s3access/default.yml`.
const SCALE_COMPOUND: &str = "ctx.event.duration *= params.MS_TO_NS;";

#[test]
fn scales_a_duration_written_as_a_compound_multiply() {
    let mut event = Event::new(json!({ "event": { "duration": 17 } }));

    assert!(try_params_painless(
        &mut event,
        SCALE_COMPOUND,
        &json!({ "MS_TO_NS": 1_000_000_i64 }),
    ));
    assert_eq!(event.get_i64("event.duration"), Some(17_000_000));
}

/// Verbatim from `pipelines/azure/activitylogs/default.yml`.
const REPLACE: &str = "ctx.message = ctx.message.replace(params.empty_field_name, '')";

#[test]
fn strips_the_marker_the_params_name() {
    let mut event = Event::new(json!({ "message": "a<EMPTY>b<EMPTY>" }));

    assert!(try_params_painless(
        &mut event,
        REPLACE,
        &json!({ "empty_field_name": "<EMPTY>" }),
    ));
    assert_eq!(event.get_str("message"), Some("ab"));
}

#[test]
fn a_script_without_params_falls_through() {
    let mut event = Event::new(json!({}));
    assert!(!try_params_painless(&mut event, SENTINEL, &Value::Null));
}

/// Verbatim from `crowdstrike/identity_protection_timeline/default.yml`,
/// tagged `map_timeline_event_severity`: a TYPED local (`Integer severity`),
/// not `def`, bound to the params lookup.
#[test]
fn a_typed_local_still_carries_the_params_lookup() {
    let script = "Integer severity = params[ctx.crowdstrike.idp.timeline.event_severity.toUpperCase()];\n\
         if (severity != null) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.severity = severity;\n}\n";
    let params = json!({ "NEUTRAL": 21, "MODERATE": 47, "IMPORTANT": 73 });

    let mut event = Event::new(json!({
        "crowdstrike": { "idp": { "timeline": { "event_severity": "important" } } }
    }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get("event.severity"), Some(&json!(73)));
}

/// A key the table does not list leaves the field unset, matching the
/// script's own `if (severity != null)` guard.
#[test]
fn a_typed_local_lookup_miss_writes_nothing() {
    let script = "Integer severity = params[ctx.crowdstrike.idp.timeline.event_severity.toUpperCase()];\n\
         if (severity != null) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.severity = severity;\n}\n";
    let params = json!({ "NEUTRAL": 21, "MODERATE": 47, "IMPORTANT": 73 });

    let mut event = Event::new(json!({
        "crowdstrike": { "idp": { "timeline": { "event_severity": "unheard-of" } } }
    }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get("event.severity"), None);
}

/// Verbatim from `pipelines/watchguard_firebox/log/default.yml`: the merge
/// lambda routes `log_type` to the vendor namespace and every other column
/// to `ctx.event`.
#[test]
fn a_merge_lambda_routes_the_key_its_branch_names() {
    let script = "if (ctx.watchguard_firebox?.log?.msg_id == null || \
        params.get(ctx.watchguard_firebox.log.msg_id) == null) {\n  return;\n}\n\
        params.get(ctx.watchguard_firebox.log.msg_id).forEach((k, v) -> {\n  \
        if (k.equals(\"log_type\")) {\n    ctx.watchguard_firebox.log[k] = v;\n  \
        } else if (v instanceof List) {\n    ctx.event[k] = new ArrayList(v);\n  \
        } else {\n    ctx.event[k] = v;\n  }\n});";
    let params = json!({ "3000-0148": {
        "category": ["network"], "type": ["connection"],
        "outcome": "success", "log_type": "traffic",
    }});

    let mut event = Event::new(json!({
        "watchguard_firebox": { "log": { "msg_id": "3000-0148" } }
    }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get("event.category"), Some(&json!(["network"])));
    assert_eq!(event.get("event.type"), Some(&json!(["connection"])));
    assert_eq!(event.get_str("event.outcome"), Some("success"));
    assert_eq!(
        event.get_str("watchguard_firebox.log.log_type"),
        Some("traffic")
    );
    // The routed arm must not take the whole row with it.
    assert_eq!(event.get("watchguard_firebox.log.category"), None);
    assert_eq!(event.get("watchguard_firebox.log.type"), None);
    assert_eq!(event.get("watchguard_firebox.log.outcome"), None);
}

/// Verbatim from `pipelines/cyberarkpas/audit/audit.yml`: the row is a list
/// of instructions, each a literal value or a field to read.
#[test]
fn an_instruction_row_builds_the_list_its_foreach_writes() {
    let script = "String msgID = ctx.event?.code;\ndef actions = params.get(msgID);\n\
        if (actions == null) return;\nList values = new ArrayList();\n\
        for (def item : actions) {\n  def val = item.value;\n  \
        if (val == null && (val = read_field(ctx, item.from)) == null || val == \"\") continue;\n  \
        values.add([\n    \"to\": item.set,\n    \"value\": clone(val)\n  ]);\n}\n\
        if (!values.isEmpty()) ctx._tmp[\"values\"] = values;\n";
    let params = json!({ "180": [
        { "set": "user.target.name", "from": "cyberarkpas.audit.source_user" },
        { "set": "event.type", "value": ["user", "creation"] },
        { "set": "event.outcome", "value": "success" },
        { "set": "user.name", "from": "cyberarkpas.audit.absent" },
    ]});

    let mut event = Event::new(json!({
        "event": { "code": "180" },
        "cyberarkpas": { "audit": { "source_user": "PSMPApp_localhost" } },
    }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(
        event.get("_tmp.values"),
        Some(&json!([
            { "to": "user.target.name", "value": "PSMPApp_localhost" },
            { "to": "event.type", "value": ["user", "creation"] },
            { "to": "event.outcome", "value": "success" },
        ]))
    );

    // A code the table does not list writes nothing at all.
    let mut unlisted = Event::new(json!({ "event": { "code": "999" } }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert!(!unlisted.has("_tmp.values"));
}

/// Verbatim from `pipelines/box_events/events/default.yml`: params IS the
/// table, and the two columns sit under a `map` member.
#[test]
fn a_keyed_row_appends_each_column_onto_its_array() {
    let script = "def eventType = params.getOrDefault(ctx.box.event_type, null);\n\
        if (eventType != null) {\n  for (category in eventType.map.get('category')) {\n    \
        ctx.event.category.add(category);\n  }\n  \
        for ( type in eventType.map.get('type')) {\n    \
        ctx.event.type.add(type);\n  }\n}\n";
    let params = json!({
        "COPY": { "map": { "category": ["file"], "type": ["creation"] } },
    });

    let mut event = Event::new(json!({ "box": { "event_type": "COPY" } }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get("event.category"), Some(&json!(["file"])));
    assert_eq!(event.get("event.type"), Some(&json!(["creation"])));

    // A type the table does not list leaves both arrays alone.
    let mut unlisted = Event::new(json!({ "box": { "event_type": "UNHEARD_OF" } }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert!(!unlisted.has("event.category"));
}

/// Verbatim from `pipelines/suricata/eve/default.yml`: the other spelling
/// of the guard, routing to a path the arm names in full rather than to a
/// container keyed by `k`.
#[test]
fn a_merge_lambda_routes_to_the_path_its_arm_names() {
    let script = "ctx.event.kind = 'event';\nctx.event.category = ['network'];\n\
        def type_params = params.get(ctx?.suricata?.eve?.event_type);\n\
        if (type_params == null) {\n    return;\n}\n\
        type_params.forEach((k, v) -> {\n    if ('network_protocol' == k) {\n        \
        if (ctx.network == null) {\n            ctx.network = ['protocol': v];\n        \
        } else {\n            ctx.network.protocol = v;\n        }\n    \
        } else if (v instanceof List) {\n        ctx.event[k] = new ArrayList(v);\n    \
        } else {\n        ctx.event[k] = v;\n    }\n});";
    let params = json!({ "tls": { "type": ["protocol"], "network_protocol": "tls" }});

    let mut event = Event::new(json!({ "suricata": { "eve": { "event_type": "tls" } } }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get_str("network.protocol"), Some("tls"));
    assert_eq!(event.get("event.type"), Some(&json!(["protocol"])));
    assert_eq!(event.get("event.network_protocol"), None);
    // The literals the script writes before the lookup still land.
    assert_eq!(event.get_str("event.kind"), Some("event"));
    assert_eq!(event.get("event.category"), Some(&json!(["network"])));
}

/// The single-assignment lambda every other integration writes, which has
/// no routed arm and must still merge the row whole.
#[test]
fn a_merge_lambda_with_no_branch_still_merges_the_row() {
    let script = "params.get(ctx.event.code)?.forEach((k, v) -> ctx.event[k] = v);";
    let params = json!({ "302013": { "type": ["connection"], "outcome": "success" }});

    let mut event = Event::new(json!({ "event": { "code": "302013" } }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get("event.type"), Some(&json!(["connection"])));
    assert_eq!(event.get_str("event.outcome"), Some("success"));
}

/// Verbatim from `pipelines/opencanary/events/default.yml`: the log code
/// named through the table, and KEPT as its own name when the table has
/// no row for it. The target is a bracket subscript.
#[test]
fn an_unlisted_key_becomes_its_own_value() {
    let script = "String logType = ctx.opencanary.logtype.toString();\n        \
        if (ctx.log == null) {\n          ctx.log = new HashMap();\n        }\n        \
        if (params.get(logType) == null) {\n          ctx.log['logger'] = logType;\n        \
        } else {\n          ctx.log['logger'] = params.get(logType);\n        }";
    let params = json!({ "13001": "LOG_SNMP_CMD" });

    let mut known = Event::new(json!({ "opencanary": { "logtype": 13001 } }));
    assert!(try_params_painless(&mut known, script, &params));
    assert_eq!(known.get_str("log.logger"), Some("LOG_SNMP_CMD"));

    // A code the table does not carry stays as itself, which is a
    // FALLBACK rather than a default -- the value comes from the event.
    let mut unlisted = Event::new(json!({ "opencanary": { "logtype": 4242 } }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert_eq!(unlisted.get_str("log.logger"), Some("4242"));
}

/// Verbatim from `pipelines/bitdefender/push_notifications/default.yml`,
/// which ships four of these back to back differing only in the target.
/// The row is written WHOLE, so a list target takes the list.
#[test]
fn one_field_keys_a_table_and_the_row_lands_whole() {
    let script = "def schemaId = ctx.bitdefender?.event?.module.toString();\n      \
        def schema = params[schemaId];\n      if (schema != null) {\n        \
        if (ctx.event == null) {\n          ctx.event = new HashMap();\n        }\n        \
        ctx.event.type = schema;\n      }";
    let params = json!({ "aph": ["info", "access"], "av": ["info"] });

    let mut event = Event::new(json!({ "bitdefender": { "event": { "module": "aph" } } }));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get("event.type"), Some(&json!(["info", "access"])));

    // A module the table does not carry leaves the target alone.
    let mut unlisted = Event::new(json!({ "bitdefender": { "event": { "module": "other" } } }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert_eq!(unlisted.get("event.type"), None);
}

/// Verbatim from `pipelines/ti_eset/ip/default.yml`: the first label the
/// table has a row for, in the DOCUMENT's order rather than the table's.
#[test]
fn the_first_label_the_table_carries_wins() {
    let script = "for (def label : ctx.eset.labels) {\n  \
        if (params.containsKey(label)) {\n    \
        ctx.threat.indicator.confidence = params.get(label);\n    break;\n  }\n}";
    let params = json!({
        "malicious-activity": "High",
        "unwanted-activity": "Medium",
        "benign": "Low",
    });

    // `benign` is listed first in the DOCUMENT, so it wins over the
    // higher-confidence label that follows it.
    let mut event = Event::new(json!({ "eset": {
        "labels": ["unlisted", "benign", "malicious-activity"],
    }}));
    assert!(try_params_painless(&mut event, script, &params));
    assert_eq!(event.get_str("threat.indicator.confidence"), Some("Low"));

    // No label the table carries leaves the field unwritten.
    let mut unlisted = Event::new(json!({ "eset": { "labels": ["other"] } }));
    assert!(try_params_painless(&mut unlisted, script, &params));
    assert_eq!(unlisted.get("threat.indicator.confidence"), None);
}

/// Verbatim from `pipelines/sonicwall_firewall/log/default.yml`: every
/// mapped key deferred onto `_temp_.sets`, with the source key deferred
/// onto `_temp_.removes` for the `foreach` that applies them.
#[test]
fn a_params_table_defers_a_set_and_a_remove_per_mapped_key() {
    let script = "List sets = ctx._temp_.computeIfAbsent(\"sets\", k -> new ArrayList());\n\
        List removes = ctx._temp_.computeIfAbsent(\"removes\", k -> new ArrayList());\n\
        for (def src_field : ctx.sonicwall.firewall.entrySet()) {\n  \
        def key = src_field.getKey();\n  if (params[key] != null) {\n    \
        boolean mapped = false;\n    for (def action : params[key]) {\n      \
        def value = action.map == null? src_field.getValue() : action.map[src_field.getValue()];\n      \
        if (value != null) {\n        sets.add([\n          \"target\": action.to,\n          \
        \"value\": value\n        ]);\n      }\n    }\n    removes.add(key);\n  }\n}\n";
    let params = json!({
        "id": [{ "to": "observer.name" }],
        "pri": [
            { "to": "event.severity" },
            { "to": "log.level", "map": { "6": "info" } },
        ],
    });

    let mut event = Event::new(json!({ "sonicwall": { "firewall": {
        "id": "firewall", "pri": "6", "unmapped": "kept",
    }}}));
    assert!(try_params_painless(&mut event, script, &params));

    assert_eq!(
        event.get("_temp_.sets"),
        Some(&json!([
            { "target": "observer.name", "value": "firewall" },
            { "target": "event.severity", "value": "6" },
            { "target": "log.level", "value": "info" },
        ]))
    );
    // Only the MAPPED keys are deferred for removal.
    assert_eq!(event.get("_temp_.removes"), Some(&json!(["id", "pri"])));
}

/// The colon-joined spelling: one field split across the targets params
/// lists for it. sonicwall's `dst` is address, port and egress interface.
#[test]
fn a_colon_joined_field_defers_a_set_per_part() {
    let script = "List sets = ctx._temp_.computeIfAbsent(\"sets\", k -> new ArrayList());\n\
        List removes = ctx._temp_.computeIfAbsent(\"removes\", k -> new ArrayList());\n\
        for (def field : params.entrySet()) {\n  \
        String value = ctx.sonicwall.firewall[field.getKey()];\n  \
        if (value == null) continue;\n  String[] parts = value.splitOnToken(\":\");\n  \
        List mapping = field.getValue();\n  for ( int i = (int)Math.min(parts.length, mapping.size()) - 1\n      \
        ; i>=0\n      ; i--) {\n    sets.add([\n      \"target\": mapping[i],\n      \
        \"value\": parts[i]\n    ]);\n  }\n  removes.add(field.getKey());\n}\n";
    let params = json!({
        "dst": ["destination.address", "destination.port", "observer.egress.interface.name"],
    });

    let mut event = Event::new(json!({ "sonicwall": { "firewall": {
        "dst": "81.2.69.143:443:X1",
    }}}));
    assert!(try_params_painless(&mut event, script, &params));

    // Deferred in reverse, which is the order the vendor's loop walks.
    assert_eq!(
        event.get("_temp_.sets"),
        Some(&json!([
            { "target": "observer.egress.interface.name", "value": "X1" },
            { "target": "destination.port", "value": "443" },
            { "target": "destination.address", "value": "81.2.69.143" },
        ]))
    );
    assert_eq!(event.get("_temp_.removes"), Some(&json!(["dst"])));
}

/// Both directions totalled into one field, with the prefixes and the keys
/// named by params rather than spelled in the script.
#[test]
fn a_params_named_total_sums_every_direction() {
    let script = "for (def src : params.from) {\n  for (def key : params.keys) {\n    \
        def v = null;\n    if (ctx[src] != null && (v = ctx[src][key]) != null && v instanceof Long) {\n      \
        if (ctx[params.to] == null || !(ctx[params.to] instanceof Map)) {\n        \
        ctx[params.to] = new HashMap();\n      }\n      \
        if (ctx[params.to][key] == null || !(ctx[params.to][key] instanceof Long)) {\n        \
        ctx[params.to][key] = v;\n      } else {\n        ctx[params.to][key] += v;\n      }\n    }\n  }\n}\n";
    let params =
        json!({ "keys": ["bytes", "packets"], "from": ["source", "destination"], "to": "network" });

    let mut event = Event::new(json!({
        "source": { "bytes": 60, "packets": 1 },
        "destination": { "bytes": 40 },
    }));
    assert!(try_params_painless(&mut event, script, &params));

    assert_eq!(event.get("network.bytes"), Some(&json!(100)));
    // One side present is still a total; the other contributes nothing.
    assert_eq!(event.get("network.packets"), Some(&json!(1)));

    // A count still carrying its grok string is not a Long, so the
    // vendor's guard skips it and no total is written.
    let mut untyped = Event::new(json!({ "source": { "bytes": "60" } }));
    assert!(try_params_painless(&mut untyped, script, &params));
    assert_eq!(untyped.get("network.bytes"), None);
}

/// stormshield lifts its metadata keys into a child map, and REMOVES them.
///
/// Verbatim from `pipelines/stormshield/log/default.yml:613`. Leaving the
/// original behind would emit a field Elasticsearch does not.
#[test]
fn a_named_set_of_keys_moves_into_a_child_map() {
    let script = "if (!ctx.stormshield.containsKey(\"metadata\")) {\n    \
        ctx.stormshield.metadata = [:];\n}\nparams.names.forEach(k -> {\n    \
        if (ctx.stormshield.containsKey(k)) {\n        \
        ctx.stormshield.metadata[k] = ctx.stormshield[k];\n        \
        ctx.stormshield.remove(k);\n    }\n    return true;\n});";
    let params = json!({ "names": ["id", "pri", "absent"] });

    let mut event = Event::new(json!({
        "stormshield": { "id": "7", "pri": "5", "logtype": "alarm" }
    }));
    assert!(try_params_painless(&mut event, script, &params));

    assert_eq!(event.get_str("stormshield.metadata.id"), Some("7"));
    assert_eq!(event.get_str("stormshield.metadata.pri"), Some("5"));
    // MOVED, not copied.
    assert!(!event.has("stormshield.id"));
    assert!(!event.has("stormshield.pri"));
    // A key the script does not name is untouched.
    assert_eq!(event.get_str("stormshield.logtype"), Some("alarm"));
    // A named key the document lacks is not created empty.
    assert!(!event.has("stormshield.metadata.absent"));
}

/// stormshield keys its whole ECS event block off `logtype`.
///
/// Verbatim from `pipelines/stormshield/log/default.yml:699`, with the tables
/// cut to the entries the assertions read. All 44 of its events carry the
/// block and none of it was written.
#[test]
fn an_event_block_comes_from_the_table_its_subject_keys() {
    let script = "def logtype = ctx.stormshield?.logtype; \
        def entry = params.logtypes[logtype]; if (ctx.event == null) {\n    ctx.event = [:];\n} \
        if (entry == null) {\n    ctx.event.kind = 'event';\n} else {\n    \
        ctx.event.kind = entry.kind;\n    ctx.event.category = new ArrayList(entry.category);\n    \
        ctx.event.type = new ArrayList(entry.type);\n    \
        if (params.action_logtypes.contains(logtype) && ctx.event.action instanceof String) {\n      \
        def mapped = params.action_types[ctx.event.action.toLowerCase()];\n      \
        if (mapped != null && !ctx.event.type.contains(mapped)) {\n        \
        ctx.event.type.add(mapped);\n      }\n    }\n}";
    let params = json!({
        "logtypes": {
            "alarm": { "kind": "alert", "category": ["intrusion_detection", "network"],
                       "type": ["info"] },
            "authstat": { "kind": "metric", "category": ["authentication"], "type": ["info"] }
        },
        "action_types": { "pass": "allowed", "block": "denied" },
        "action_logtypes": ["alarm", "connection"]
    });

    // A listed logtype whose action maps: the extra type is APPENDED.
    let mut alarm = Event::new(json!({
        "stormshield": { "logtype": "alarm" }, "event": { "action": "Block" }
    }));
    assert!(try_params_painless(&mut alarm, script, &params));
    assert_eq!(alarm.get_str("event.kind"), Some("alert"));
    assert_eq!(
        alarm.get("event.category"),
        Some(&json!(["intrusion_detection", "network"]))
    );
    assert_eq!(alarm.get("event.type"), Some(&json!(["info", "denied"])));

    // Not in action_logtypes, so the action is never consulted.
    let mut stat = Event::new(json!({
        "stormshield": { "logtype": "authstat" }, "event": { "action": "pass" }
    }));
    assert!(try_params_painless(&mut stat, script, &params));
    assert_eq!(stat.get_str("event.kind"), Some("metric"));
    assert_eq!(stat.get("event.type"), Some(&json!(["info"])));

    // No entry writes the bare default and nothing else.
    let mut unknown = Event::new(json!({ "stormshield": { "logtype": "nosuch" } }));
    assert!(try_params_painless(&mut unknown, script, &params));
    assert_eq!(unknown.get_str("event.kind"), Some("event"));
    assert_eq!(unknown.get("event.category"), None);

    // The params table must not have grown a member from the append above.
    assert_eq!(
        params["logtypes"]["alarm"]["type"],
        json!(["info"]),
        "the entry was aliased rather than copied"
    );
}

/// `ti_recordedfuture` names its CSV columns by which layout it read.
///
/// Verbatim from `pipelines/ti_recordedfuture/threat/decode_csv.yml:19`. Four
/// columns for url, domain and IP; five for hash, with `Algorithm` inserted
/// second. The absence of the last column is the whole discriminator, and
/// without this the columns never become `json.Name` -- the next processor
/// raises `field not found` and the event goes down the error path.
#[test]
fn csv_columns_are_named_by_the_table_the_layout_picks() {
    let script = "def cols = params[ ctx._tmp_.col4 == null? \"default\" : \"hash\" ];\n\
        def src = ctx._tmp_;\ndef dst = new HashMap();\n\
        for (entry in cols.entrySet()) {\n  \
        dst[entry.getValue()] = src[entry.getKey()];\n}\nctx['json'] = dst;";
    let params = json!({
        "default": { "col0": "Name", "col1": "Risk", "col2": "RiskString",
                     "col3": "EvidenceDetails" },
        "hash": { "col0": "Name", "col1": "Algorithm", "col2": "Risk",
                  "col3": "RiskString", "col4": "EvidenceDetails" }
    });

    // Four columns: the default layout.
    let mut four = Event::new(json!({ "_tmp_": {
        "col0": "1.128.3.4", "col1": "99", "col2": "4/64", "col3": "{}"
    } }));
    assert!(try_params_painless(&mut four, script, &params));
    assert_eq!(four.get_str("json.Name"), Some("1.128.3.4"));
    assert_eq!(four.get_str("json.Risk"), Some("99"));
    assert_eq!(four.get_str("json.EvidenceDetails"), Some("{}"));
    assert!(!four.has("json.Algorithm"));

    // Five columns: the hash layout, which shifts everything after col0.
    let mut five = Event::new(json!({ "_tmp_": {
        "col0": "abc123", "col1": "SHA-256", "col2": "89", "col3": "3/50",
        "col4": "{}"
    } }));
    assert!(try_params_painless(&mut five, script, &params));
    assert_eq!(five.get_str("json.Name"), Some("abc123"));
    assert_eq!(five.get_str("json.Algorithm"), Some("SHA-256"));
    assert_eq!(five.get_str("json.Risk"), Some("89"));
    assert_eq!(five.get_str("json.EvidenceDetails"), Some("{}"));
}

/// The sentinel sweep reads a `ctx?.` path, not only the plain spelling.
///
/// Verbatim from `pipelines/juniper_srx/log`. `try_sentinel_removal` already
/// did this job; it found no map here because `ctx_path_before` read only
/// `ctx.`, so all 82 of the source's invocations were skipped.
#[test]
fn map_entries_are_dropped_by_the_value_a_params_list_names() {
    let script =
        "ctx?.juniper?.srx.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));";
    assert_eq!(
        ctx_path_before(script, ".entrySet().removeIf("),
        Some("juniper.srx".to_owned())
    );

    let params = json!({ "values": ["N/A", "unknown", ""] });
    let mut event = Event::new(json!({ "juniper": { "srx": {
        "source_address": "10.0.0.1",
        "nat_source_port": "N/A",
        "policy_name": "unknown",
        "reason": ""
    } } }));
    assert!(try_params_painless(&mut event, script, &params));

    assert_eq!(
        event.get_str("juniper.srx.source_address"),
        Some("10.0.0.1")
    );
    // Every placeholder goes, whichever key held it.
    assert!(!event.has("juniper.srx.nat_source_port"));
    assert!(!event.has("juniper.srx.policy_name"));
    assert!(!event.has("juniper.srx.reason"));
}

/// auditd picks its action by which candidate's fields the record HOLDS.
///
/// Verbatim from `pipelines/auditd/log/default.yml:1998`, with the tables cut
/// to the entries the assertions read. All 83 of its events go through it.
#[test]
fn a_record_action_comes_from_the_candidate_whose_fields_are_present() {
    let script = r#"boolean hasFields(HashMap base, def list) {
          if (list == null) return true;
          for (int i=0; i<list.length; i++)
            if (base[list[i]] == null) return false;
          return true;
        }
        if (ctx?.auditd?.log?.record_type == null) {
          return;
        }
        HashMap base = ctx.auditd.log;
        def acts = params.types.get(base.record_type);
        if (acts == null && base.syscall != null) {
          acts = params.syscalls.get(base?.syscall);
          if (acts == null) acts = params.syscalls.get('*');
        }
        if (acts == null) return;
        def act = null;
        for (int i=0; act == null && i<acts.length; i++) {
          if (hasFields(base, acts[i]["has_fields"])) act = acts[i];
        }
        if (act?.event != null) {
          def hm = new HashMap(act.event);
          hm.forEach((k, v) -> ctx.event[k] = v);
        }
        if (act?.copy != null) {
          List lst = new ArrayList();
          for(int i=0; i<act.copy.length; i++) {
            def value;
            def srcList = act.copy[i]["from"];
            for (int j=0; value == null && j<srcList.length; j++) {
              value = base[srcList[j]];
            }
            if (value != null && value instanceof String && value != 'unset' && value != '?') {
              String suffix = value ==~ /[0-9]+/? ".id" : ".name";
              lst.add(["target": act.copy[i]["to"] + suffix, "value": value]);
            }
          }
          if (lst.size() > 0) {
            ctx.auditd.log["copy"] = lst;
          }
        }"#;
    let params = json!({
        "types": {
            "AVC": [
                { "event": { "action": "violated-selinux-policy" },
                  "has_fields": ["seresult"] },
                { "event": { "action": "violated-apparmor-policy" },
                  "has_fields": ["apparmor"] }
            ],
            "ACCT_LOCK": [
                { "event": { "action": "locked-account", "category": ["iam"] },
                  "copy": [
                    { "from": ["auid", "AUID"], "to": "user" },
                    { "from": ["acct"], "to": "user.target" },
                    { "from": ["missing"], "to": "user.effective" }
                  ] }
            ]
        },
        "syscalls": {
            "execve": [{ "event": { "action": "executed" } }],
            "*": [{ "event": { "action": "used-syscall" } }]
        }
    });

    // Two candidates under one key: the one whose field is present wins.
    let mut apparmor = Event::new(json!({
        "auditd": { "log": { "record_type": "AVC", "apparmor": "DENIED" } }
    }));
    assert!(try_params_painless(&mut apparmor, script, &params));
    assert_eq!(
        apparmor.get_str("event.action"),
        Some("violated-apparmor-policy")
    );

    let mut selinux = Event::new(json!({
        "auditd": { "log": { "record_type": "AVC", "seresult": "denied" } }
    }));
    assert!(try_params_painless(&mut selinux, script, &params));
    assert_eq!(
        selinux.get_str("event.action"),
        Some("violated-selinux-policy")
    );

    // Neither candidate's fields are present, so nothing is written.
    let mut neither = Event::new(json!({
        "auditd": { "log": { "record_type": "AVC" } }
    }));
    assert!(try_params_painless(&mut neither, script, &params));
    assert!(!neither.has("event.action"));

    // No entry in the primary table falls through to the syscall...
    let mut syscall = Event::new(json!({
        "auditd": { "log": { "record_type": "SYSCALL", "syscall": "execve" } }
    }));
    assert!(try_params_painless(&mut syscall, script, &params));
    assert_eq!(syscall.get_str("event.action"), Some("executed"));

    // ... and an unlisted syscall to the wildcard.
    let mut wildcard = Event::new(json!({
        "auditd": { "log": { "record_type": "SYSCALL", "syscall": "nosuch" } }
    }));
    assert!(try_params_painless(&mut wildcard, script, &params));
    assert_eq!(wildcard.get_str("event.action"), Some("used-syscall"));

    // A record type in neither table, with no syscall, writes nothing.
    let mut unknown = Event::new(json!({
        "auditd": { "log": { "record_type": "NOSUCH" } }
    }));
    assert!(try_params_painless(&mut unknown, script, &params));
    assert!(!unknown.has("event.action"));

    // The copy list: `.id` for a numeric value, `.name` otherwise, the FIRST
    // source that holds a value, and nothing at all for a source list that
    // resolves to nothing.
    let mut copied = Event::new(json!({
        "auditd": { "log": { "record_type": "ACCT_LOCK", "AUID": "1000", "acct": "root" } }
    }));
    assert!(try_params_painless(&mut copied, script, &params));
    assert_eq!(copied.get_str("event.action"), Some("locked-account"));
    assert_eq!(
        copied.get("auditd.log.copy"),
        Some(&json!([
            { "target": "user.id", "value": "1000" },
            { "target": "user.target.name", "value": "root" }
        ]))
    );
}

/// auditd normalises every value of its record map in one pass.
///
/// Verbatim from `pipelines/auditd/log/default.yml:20`. Leaving it unbound
/// leaves a quote on `process.executable`, `process.name`, `user.terminal`
/// and `auditd.log.hostname` in every record that carries one.
#[test]
fn every_value_of_a_map_is_normalised_in_place() {
    let script = r#"String trimQuotes(def singleQuote, def doubleQuote, def v) {
            if (v.startsWith(singleQuote) || v.startsWith(doubleQuote)) {
                v = v.substring(1, v.length());
            }
            if (v.endsWith(singleQuote) || v.endsWith(doubleQuote)) {
                v = v.substring(0, v.length()-1);
            }
            return v;
        }
        def processFieldValue(String k, def v, def possibleHexKeys, def possibleBooleanKeys) {
            if (v == "?" || v == "(null)" || v == "") {
                return null;
            }
            if (possibleHexKeys.contains(k) && isHexAscii(v)) {
                v = convertHexToString(v);
            }
            if (possibleBooleanKeys.contains(k) && v instanceof String) {
                v = convertStringToBoolean(v);
            }
            if (v instanceof String) {
                v = trimQuotes("'", "\"", v);
            }
            if (k == "arch" && v == "c000003e") {
                v = "x86_64";
            }
            return v;
        }
        def audit = ctx.auditd.get("log");
        Iterator entries = audit.entrySet().iterator();
        while (entries.hasNext()) {
            def e = entries.next();
            def k = e.getKey();
            def v = e.getValue();
            if (v instanceof List) {
                int j = 0;
                for (int i = 0; i < v.length; i++) {
                    v[j] = processFieldValue(k, v[i], params.possibleHexKeys, params.possibleBooleanKeys);
                    if (v[j] != null) {
                        j++;
                    }
                }
                if (j < v.length) {
                    if (j == 0) {
                        entries.remove();
                        continue;
                    }
                    audit.put(k, v.subList(0, j));
                }
                continue;
            }
            v = processFieldValue(k, v, params.possibleHexKeys, params.possibleBooleanKeys);
            if (v == null) {
                entries.remove();
            } else {
                audit.put(k, v);
            }
        }"#;
    let params = json!({
        "possibleHexKeys": ["exe", "cmd", "cwd", "comm"],
        "possibleBooleanKeys": ["success"]
    });

    let mut event = Event::new(json!({ "auditd": { "log": {
        "exe": "\"/usr/sbin/sshd\"",
        "comm": "'sshd'",
        "cmd": "6C73202D6C",
        "cwd": "2F686F6D65",
        "success": "yes",
        "arch": "c000003e",
        "terminal": "?",
        "acct": "(null)",
        "empty": "",
        "pid": 1234,
        "a0": ["\"one\"", "?"],
        "a1": ["?"]
    } } }));
    assert!(try_params_painless(&mut event, script, &params));

    // Both quote spellings, one from each end.
    assert_eq!(event.get_str("auditd.log.exe"), Some("/usr/sbin/sshd"));
    assert_eq!(event.get_str("auditd.log.comm"), Some("sshd"));
    // Hex that DECODES, because the result carries a space.
    assert_eq!(event.get_str("auditd.log.cmd"), Some("ls -l"));
    // Hex that does not: every byte lands above `"`, so the original stands.
    assert_eq!(event.get_str("auditd.log.cwd"), Some("2F686F6D65"));
    assert_eq!(event.get("auditd.log.success"), Some(&json!(true)));
    assert_eq!(event.get_str("auditd.log.arch"), Some("x86_64"));
    // The three spellings of absent remove the key outright.
    assert!(!event.has("auditd.log.terminal"));
    assert!(!event.has("auditd.log.acct"));
    assert!(!event.has("auditd.log.empty"));
    // A non-string is not a candidate for any of it.
    assert_eq!(event.get("auditd.log.pid"), Some(&json!(1234)));
    // A list drops its absent elements, and goes entirely when none survive.
    assert_eq!(event.get("auditd.log.a0"), Some(&json!(["one"])));
    assert!(!event.has("auditd.log.a1"));
}

/// `cisco_asa`'s distinguished-name fold, verbatim from
/// `pipelines/cisco/asa/default.yml` (tag `script_b84935be`), with the params
/// block that call site carries.
///
/// The compat corpus cannot score this one: it holds no ASA
/// `Certificate was successfully validated` message, so `dn_parts` is never
/// built and the script's own null guard returns on all 512 events. The test
/// is what stands in for that.
const DN_PARTS: &str = "if (ctx._temp_?.cisco?.dn_parts == null) {\n  return;\n}\n\
    def parts = [:];\nctx._temp_.cisco.dn_parts.forEach((k,v) -> {\n  \
    if (params.containsKey(k)) {\n    \
    parts[params[k]] = (v instanceof List) ? v : [v];   \
    // `[v]` is a Painless list literal\n  } else {\n    return false;\n  }\n});\n\
    ctx._temp_.cisco.dn_parts = parts;\n";

fn dn_params() -> Value {
    json!({
        "ST": "state_or_province",
        "S": "state_or_province",
        "P": "state_or_province",
        "CN": "common_name",
        "C": "country",
        "L": "locality",
        "O": "organization",
        "OU": "organizational_unit"
    })
}

/// Every key the table names is renamed and wrapped, in the MAP's own order.
#[test]
fn a_distinguished_name_is_renamed_onto_its_ecs_members() {
    let mut event = Event::new(json!({ "_temp_": { "cisco": { "dn_parts": {
        "CN": "vpn.example.com",
        "OU": "IT",
        "O": "Example Pty Ltd",
        "C": "AU"
    } } } }));

    assert!(try_params_painless(&mut event, DN_PARTS, &dn_params()));
    assert_eq!(
        event.get("_temp_.cisco.dn_parts"),
        Some(&json!({
            "common_name": ["vpn.example.com"],
            "organizational_unit": ["IT"],
            "organization": ["Example Pty Ltd"],
            "country": ["AU"]
        }))
    );
}

/// A key the table does not name is DROPPED, and a value that is already a
/// list is not wrapped a second time.
///
/// Keeping the unnamed key is what the neighbouring `RenameKeys` does, and it
/// would carry the vendor's own abbreviation into `tls.server.x509.subject`
/// beside the ECS member.
#[test]
fn an_unnamed_distinguished_name_part_is_dropped() {
    let mut event = Event::new(json!({ "_temp_": { "cisco": { "dn_parts": {
        "CN": ["a.example.com", "b.example.com"],
        "SERIALNUMBER": "1234",
        "L": "Sydney"
    } } } }));

    assert!(try_params_painless(&mut event, DN_PARTS, &dn_params()));
    assert_eq!(
        event.get("_temp_.cisco.dn_parts"),
        Some(&json!({
            "common_name": ["a.example.com", "b.example.com"],
            "locality": ["Sydney"]
        }))
    );
}

/// The script's own null guard: no map, no write, and nothing counted as run.
#[test]
fn a_missing_distinguished_name_writes_nothing() {
    let mut event = Event::new(json!({ "_temp_": { "cisco": {} } }));
    assert!(!try_params_painless(&mut event, DN_PARTS, &dn_params()));
    assert_eq!(event.get("_temp_.cisco"), Some(&json!({})));
}

/// A stored expression the parse cannot say is declined WHOLE, rather than
/// claimed and half-run: `v.toString()` is a third behaviour, and writing the
/// keys right with the values wrong is the worse outcome.
#[test]
fn a_fold_storing_something_else_declines() {
    let script = DN_PARTS.replace("(v instanceof List) ? v : [v]", "v.toString()");
    assert!(!matches!(
        params_pattern(&crate::painless_common::normalise(&script)),
        Some(ParamsPattern::SelectRenameKeys(_))
    ));
}
