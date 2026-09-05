// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless script patterns whose behaviour lives in the processor's `params`.
//!
//! An Elastic `script` processor may carry a `params` block, and the recurring
//! patterns read their whole configuration from it -- the sentinel list to strip,
//! the field names to convert, the lookup table to merge. Matching on script
//! text alone cannot execute any of them, because the text says only *that* a
//! param is read, never what it holds.
//!
//! [`try_params_painless`] is tried before the text-only matchers in
//! [`crate::painless_common::try_known_painless`], so a pattern with a params
//! block runs against the pipeline's real table rather than a transcribed copy.

use std::borrow::Cow;

use serde_json::{Map, Value, json};

use crate::event::Event;
use crate::painless_helpers::{filetime_to_unix_ms, remove_sentinel_values};

/// A processor's `params` block, parsed once per CALL SITE.
///
/// The block is emitted as a JSON string rather than a `json!` literal: the
/// macro expands once per key, and the o365 operation table alone is deep
/// enough to blow rustc's recursion limit. Parsing it behind a per-site
/// `OnceLock` costs one parse per process and nothing per event -- the same
/// pattern [`crate::cached_grok`] uses, and for the same reason.
#[macro_export]
macro_rules! cached_params {
    ($json:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<::serde_json::Value> = ::std::sync::OnceLock::new();
        SITE.get_or_init(|| {
            ::serde_json::from_str($json).expect("codegen emits a valid params block")
        })
    }};
}

/// The FILETIME threshold, as it is written in the vendor scripts.
const FILETIME_LITERAL: &str = "0x0100000000000000L";

/// Run a Painless script whose behaviour is carried by its `params` block.
///
/// Returns false when nothing matched, so the caller falls through to the
/// text-only matchers.
pub fn try_params_painless(event: &mut Event, script: &str, params: &Value) -> bool {
    let Some(params) = params.as_object() else {
        return false;
    };
    let normalised = crate::painless_common::normalise(script);
    match params_pattern(&normalised) {
        Some(pattern) => run_params_pattern(event, &normalised, params, &pattern),
        None => false,
    }
}

/// The matcher a params script's text routes to.
///
/// Every branch of the params dispatch is terminal -- the first trigger that
/// holds names the matcher, whatever that matcher then returns -- so the whole
/// decision is a property of the script TEXT and is made once per call site by
/// [`crate::painless_plan::PainlessPlan`] rather than once per event. A pattern
/// whose trigger is itself a parse carries the parse's result.
/// The case a script folds a lookup key to before reading the table.
///
/// Painless folds the key, never the table, so a fold the parse misses does
/// not merely mis-case a lookup -- it misses every row and the pattern resolves
/// nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fold {
    None,
    Lower,
    Upper,
}

impl Fold {
    /// The fold an expression spells, and that expression without the call.
    fn strip(expr: &str) -> (String, Self) {
        for (call, fold) in [
            (".toLowerCase()", Self::Lower),
            (".toUpperCase()", Self::Upper),
        ] {
            if expr.contains(call) {
                return (expr.replace(call, ""), fold);
            }
        }
        (expr.to_string(), Self::None)
    }

    fn apply(self, value: &str) -> String {
        match self {
            Self::None => value.to_string(),
            Self::Lower => value.to_lowercase(),
            Self::Upper => value.to_uppercase(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamsPattern {
    AwsEntity(Box<crate::painless_entity::EntityScript>),
    DropEmptyMembers {
        parent: String,
        list: String,
    },
    IndexedRowColumns {
        subject: String,
        columns: Vec<(usize, String)>,
    },
    SysmonQueryResults,
    SysmonRegistry,
    MessageTable,
    FirstAsset,
    MatcherKv,
    LookupPut {
        source: String,
        target: String,
        removes: Vec<String>,
    },
    LookupWrapList {
        source: String,
        target: String,
    },
    ProtocolPrefix {
        list: String,
        fallback: String,
        subject: String,
        target: String,
    },
    EventBlockTable(Box<EventBlockTable>),
    RecordActionTable(Box<RecordActionTable>),
    NormaliseMapValues(Box<NormaliseMapValues>),
    MoveKeysIntoChild(Box<MoveKeysIntoChild>),
    MimecastLogType,
    InvocationDetails,
    ScheduledTask,
    ThreatIndicatorType(String),
    EvidenceCategories {
        source: String,
        key: String,
    },
    MsgParts,
    /// The first member of a ctx list the params table has a row for.
    FirstLabelInTable {
        list: String,
        target: String,
    },
    /// One field keying a params table, the row assigned WHOLE to a target.
    KeyedRowAssign {
        source: String,
        target: String,
    },
    /// The same lookup, falling back to the KEY where the table has no row.
    LookupOrKey {
        source: String,
        target: String,
    },
    /// One deferred `{target, value}` list built from a params table, in the
    /// three spellings the sonicwall family ships. Each defers its writes into
    /// `_temp_.sets` and its source keys into `_temp_.removes`, which a later
    /// `foreach` applies -- so a script can target arbitrary paths.
    DeferredFieldTable(String),
    DeferredSplitTable(String),
    DeferredAppendTable(String),
    /// `event.code` routed through one params table to name an event type,
    /// which a second table expands into the `event.*` block.
    MessageCodeEventType,
    /// Each key of `params.keys` totalled across the `params.from` prefixes
    /// into `params.to`. The literal-path spelling of the same sum is
    /// `KnownPattern::SumDirections`, which cannot read this one.
    SummedDirections,
    MappingRow(Box<MappingRow>),
    SecuritySddl,
    KeyedRowMembers(Box<KeyedRowMembers>),
    PutWrites {
        writes: Vec<PutWrite>,
        /// A `!["a","b"].contains(ctx.<p>)` return-guard the script opens with.
        require: Option<(String, Vec<String>)>,
    },
    PutFlagNames {
        container: String,
        member: String,
        source: String,
    },
    /// A NAMED params table read through a ternary, with a LITERAL default.
    ///
    /// `jamf_protect` writes one of these per telemetry field -- 40 sites over
    /// 13 files. Sibling to [`ParamsPattern::UppercaseLookupDefault`], which
    /// differs on four points: it folds case, reads the whole `params` map,
    /// spells the lookup `getOrDefault`, and defaults to the KEY rather than
    /// to a literal.
    TableLookupOrLiteral {
        source: String,
        table: String,
        target: String,
        default: String,
    },
    UppercaseLookupDefault {
        source: String,
        target: String,
    },
    NormalisedLookup {
        source: String,
        target: String,
        fold: Fold,
    },
    /// A flat table keyed by a field's STRING form, written to one sibling.
    ///
    /// The key expression carries its own parentheses and `.toString()`, so
    /// every bracket trigger testing for `params[ctx.` misses it. The
    /// `containsKey` guard means an unlisted key writes NOTHING, where the
    /// bracket catch-all stands the key in for its own value.
    StringifiedLookup {
        source: String,
        target: String,
    },
    SentinelRemoval,
    FiletimeFieldList,
    BitFlags,
    FirstContainedMember,
    RenameKeys,
    ValueMaps,
    RowColumns(Program),
    RowColumnAppends(Box<RowColumnAppends>),
    InstructionRows(Box<InstructionRows>),
    /// A row overwrites the field it was looked up by, fans several more
    /// columns onto ctx (one of them APPENDING rather than replacing), and a
    /// key with no row still writes two fields rather than nothing -- auth0's
    /// per-event-type action table.
    KeyedActionRow {
        /// The params key holding the lookup table.
        table: String,
        /// The `ctx.` path used as the lookup key.
        source: String,
    },
    KeyedRowAppends(Box<KeyedRowAppends>),
    KeyedMessageTable,
    ReversibleLookup,
    /// The literal writes the script makes on its own account travel with the
    /// pattern, so the four-hundred-line bodies are read once rather than per
    /// event.
    LookupMerge(Program),
    LookupColumns,
    LookupNormalise(LookupNormaliseScript, Program),
    /// A params lookup that writes nothing when the table misses.
    GuardedLookup(GuardedLookupScript),
    IndexedLookup,
    Scale,
    Replace,
    AddUniqueRow,
    FrameworkPreference,
    RowOrDefaults(Box<RowOrDefaults>),
    MergeRowOrFallback(Box<MergeRowOrFallback>),
    SelectMembers {
        subject: String,
        rest: String,
    },
    ParamRenameLadder {
        path: String,
        /// Each arm as (the params member holding the value to match, the
        /// params member holding what to write instead).
        arms: Vec<(String, String)>,
    },
    /// A CSV layout identified by the KEYS its labelled columns produced,
    /// then its unlabelled columns named by index from the matching row.
    CsvFingerprintProvider {
        /// The `ctx.` path holding the columns as a list.
        list: String,
        /// The `ctx.` map the named columns are added to.
        map: String,
        /// The `ctx.` path holding the joined key list.
        fingerprint: String,
    },
}

/// The one matcher this script's text triggers, or `None`.
///
/// The trigger order is load-bearing and each comment says why a branch sits
/// where it does; a script can spell several triggers and the FIRST wins,
/// exactly as the old inline dispatch behaved.
pub(crate) fn params_pattern(normalised: &str) -> Option<ParamsPattern> {
    // Pattern: aws cloudtrail's entity classifier, per-service enrichment
    // into TreeSets then classification through the params tables. First,
    // because its 950 lines spell half the other triggers somewhere. The
    // trigger IS the parse: a restructured script declines here and falls
    // through the dispatch unclaimed.
    if normalised.contains("enrichCtx")
        && normalised.contains("related.entity")
        && let Some(parsed) = crate::painless_entity::EntityScript::parse(normalised)
    {
        return Some(ParamsPattern::AwsEntity(Box::new(parsed)));
    }

    // Pattern: sysmon's semicolon-separated DNS QueryResults, where params is
    // the RR-number-to-name table. Checked first: the script also spells
    // `.put(` and `params`, which a later matcher reads as an indexed lookup.
    if normalised.contains("QueryResults") && normalised.contains("startsWith(\"type:\")") {
        return Some(ParamsPattern::SysmonQueryResults);
    }

    // Pattern: sysmon's registry fields, the hive abbreviated through the
    // params table and the Details value typed by its own text.
    if normalised.contains("ctx.registry = new HashMap()") && normalised.contains("TargetObject") {
        return Some(ParamsPattern::SysmonRegistry);
    }

    // Pattern: strip the vendor's sentinel values out of a map.
    if normalised.contains(".entrySet().removeIf(") && normalised.contains("entry.getValue()") {
        return Some(ParamsPattern::SentinelRemoval);
    }

    // Pattern: symantec_endpoint's CSV layout table, keyed on the fingerprint
    // the colon-pair reader built. Ahead of the generic table matchers, which
    // read `params.providers` as an ordinary per-field lookup and would claim
    // the script without ever naming a column.
    if normalised.contains("params.providers")
        && normalised.contains("p.fingerprint")
        && let Some(list) = base_between(normalised, "def hostname = ctx.", ".get(0)")
        && let Some(map) = base_between(normalised, "  ctx.", "[c.name] = v")
        && let Some(fingerprint) = base_between(normalised, "== ctx.", " ")
    {
        return Some(ParamsPattern::CsvFingerprintProvider {
            list,
            map,
            fingerprint,
        });
    }

    // Pattern: the params-driven table scripts, which name their targets in
    // `params` rather than spelling them. Three defer their writes onto
    // `_temp_.sets` and their source keys onto `_temp_.removes` for a later
    // `foreach` to apply, which is how one script targets arbitrary paths.
    // Ahead of the generic table matchers, which read the same `params[key]`
    // trigger as a plain lookup and would claim a script for a single field.
    if normalised.contains("computeIfAbsent(\"sets\"") {
        if normalised.contains("action.map == null")
            && let Some(base) = base_between(normalised, "for (def src_field : ctx.", ".entrySet()")
        {
            return Some(ParamsPattern::DeferredFieldTable(base));
        }
        if normalised.contains("splitOnToken(\":\")")
            && let Some(base) = base_between(normalised, "String value = ctx.", "[field.getKey()]")
        {
            return Some(ParamsPattern::DeferredSplitTable(base));
        }
        if normalised.contains("params.sources")
            && let Some(base) = base_between(normalised, "Map base = ctx.", ";")
        {
            return Some(ParamsPattern::DeferredAppendTable(base));
        }
    }

    // Pattern: the message code expanded into the whole event block through
    // two params tables, one naming the type and one holding its fields.
    if normalised.contains("params.message_codes[") && normalised.contains("params.event_types[") {
        return Some(ParamsPattern::MessageCodeEventType);
    }

    // Pattern: a per-direction total, with the directions and the keys named
    // by params rather than spelled in the script.
    if normalised.contains("params.from")
        && normalised.contains("params.keys")
        && normalised.contains("ctx[params.to]")
    {
        return Some(ParamsPattern::SummedDirections);
    }

    // Pattern: empty-string members dropped from each of the sub-maps params
    // names. Ahead of every table matcher, whose `params.<name>` trigger this
    // also spells and which reads the list as a lookup table.
    if normalised.contains("entry.getValue() != ''")
        && let Some(pattern) = parse_drop_empty_members(normalised)
    {
        return Some(pattern);
    }

    // Pattern: windows security's decoded scheduled-task XML, normalised
    // against the trigger and action tables params carries.
    if normalised.contains("ArrayList normalizeTriggers(") {
        return Some(ParamsPattern::ScheduledTask);
    }

    // Pattern: powershell's raw invocation details, one structured map per
    // line of the named event_data field.
    if normalised.contains("def parseRawDetail(String raw)") {
        return Some(ParamsPattern::InvocationDetails);
    }

    // Pattern: one field rewritten through an if/else-if ladder whose BOTH
    // sides are params members -- stan spells its log levels and its message
    // types this way. Early, because the parse is the trigger and it demands
    // the whole script be the ladder.
    if normalised.contains("== params.")
        && let Some(pattern) = parse_param_rename_ladder(normalised)
    {
        return Some(pattern);
    }

    // Pattern: keep the params-listed members of a sub-map where they are and
    // push the rest down one level -- cyberarkpas's mapping-explosion guard.
    // Early, because the parse is the trigger: only this pattern spells a params
    // VALUE streamed as the field list, so nothing else can be stolen by it.
    if normalised.contains(".getValue().stream().filter(")
        && let Some(pattern) = parse_select_members(normalised)
    {
        return Some(pattern);
    }

    // Pattern: every params label whose flag the field's bits carry, collected
    // into one list -- kerberos ticket options.
    if normalised.contains("Long.decode(")
        && normalised.contains("params.entrySet()")
        && let Some(pattern) = parse_put_flag_names(normalised)
    {
        return Some(pattern);
    }

    // Pattern: one params row put back under a name -- kerberos status and
    // encryption-type descriptions.
    if normalised.contains("params[ctx.")
        && let Some(pattern) = parse_put_lookup(normalised)
    {
        return Some(pattern);
    }

    // Pattern: a params ROW looked up by a normalised key, two of its members
    // put back -- windows security's audit subcategory GUID.
    if (normalised.contains("][0])") || normalised.contains("][1])"))
        && let Some(pattern) = KeyedRowMembers::parse(normalised)
    {
        return Some(ParamsPattern::KeyedRowMembers(Box::new(pattern)));
    }

    // Pattern: a field uppercased, then abbreviated through params -- m365's
    // registry hive, where an unlisted name stands for itself.
    if normalised.contains(".toUpperCase();")
        && let Some(pattern) = parse_uppercase_lookup_default(normalised)
    {
        return Some(pattern);
    }

    // Pattern: the same lookup through a NAMED table with a LITERAL default --
    // jamf_protect's telemetry, one per field. The trigger is the parse, with
    // only the ternary as a cheap reject.
    if normalised.contains(".containsKey(")
        && normalised.contains(".toString();")
        && let Some(pattern) = parse_table_lookup_or_literal(normalised)
    {
        return Some(pattern);
    }

    // Pattern: securityhub's threat-intel indicators, each vendor type mapped
    // through params to its STIX name and a hash indicator built when that
    // name is `file`.
    if normalised.contains("indicator.indicator.put(\"file\", file)")
        && let Some(source) = normalised
            .split_once("for (ti in ctx.")
            .and_then(|(_, rest)| rest.split_once(')'))
            .map(|(path, _)| clean_path(path.trim()))
    {
        return Some(ParamsPattern::ThreatIndicatorType(source));
    }

    params_pattern_tail(normalised)
}

/// The rest of the dispatch, split only because one function may not run past
/// 150 lines. Order still matters across the two halves: the first trigger
/// that fires wins, and these run after everything above.
// One branch per pattern, and the lock scanner needs every branch INLINE so it
// can resolve each dispatch site to one variant -- splitting further is what
// `patterns.lock` rejects, not just what would hide the order.
#[allow(clippy::too_many_lines)]
fn params_pattern_tail(normalised: &str) -> Option<ParamsPattern> {
    // Pattern: windows security descriptors expanded into readable ACL lines.
    if normalised.contains("void enrichSDDL(") {
        return Some(ParamsPattern::SecuritySddl);
    }

    // Pattern: one row of a NAMED params table, selected by a field, with a
    // literal fallback where the subject is not in the table.
    if normalised.contains("def at = ctx.")
        && normalised.contains(".get(at)")
        && let Some(pattern) = parse_mapping_row(normalised)
    {
        return Some(ParamsPattern::MappingRow(Box::new(pattern)));
    }

    // Pattern: proofpoint's message parts, renamed through the params key map
    // and fanned out into the ECS lists.
    if normalised.contains("for (part in ctx.json.msgParts)") {
        return Some(ParamsPattern::MsgParts);
    }

    // Pattern: m365's event categories and types off the evidence list, each
    // entry's `@odata.type` looked up in the params table.
    if normalised.contains("def eventCategory = new HashSet()")
        && let Some(source) = normalised
            .split_once("for (evidence in ctx.")
            .and_then(|(_, rest)| rest.split_once(')'))
            .map(|(path, _)| clean_path(path.trim()))
        && let Some(key) = normalised
            .split_once("params[evidence[")
            .and_then(|(_, rest)| rest.split_once(']'))
            .map(|(key, _)| key.trim().trim_matches(['"', '\'']).to_string())
    {
        return Some(ParamsPattern::EvidenceCategories { source, key });
    }

    // Pattern: mimecast's scored log-type classifier, keyed on its four
    // params tables.
    if normalised.contains("params.definite_positive") && normalised.contains("params.candidates") {
        return Some(ParamsPattern::MimecastLogType);
    }

    // Pattern: convert every named field from Windows FILETIME to UNIX ms.
    if normalised.contains(FILETIME_LITERAL) && normalised.contains("for (def field : params.") {
        return Some(ParamsPattern::FiletimeFieldList);
    }

    // Pattern: decode a bitfield into one boolean label per set bit.
    if normalised.contains("params.entrySet()") && normalised.contains("& flag") {
        return Some(ParamsPattern::BitFlags);
    }

    // Pattern: the first member of a params list the subject contains.
    if normalised.contains("for (String ") && normalised.contains(".put(") {
        return Some(ParamsPattern::FirstContainedMember);
    }

    // Pattern: the same lookup, but keeping the KEY where the table has no
    // row. Ahead of the plain assign, which would claim it and drop the
    // fallback -- opencanary keeps an unrecognised log code as its own name.
    if normalised.contains("params.get(")
        && normalised.contains("== null) {")
        && let Some(source) = base_between(normalised, " = ctx.", ".toString()")
        && let Some(target) = bracket_path_before(normalised, " = params.get(")
    {
        return Some(ParamsPattern::LookupOrKey { source, target });
    }

    // Pattern: one field keying a params table, the row assigned whole.
    // bitdefender ships four of these back to back, one per `event.*` field,
    // differing only in the target they name.
    if normalised.contains("params[schemaId]")
        && let Some(source) = base_between(normalised, "def schemaId = ctx.", ".toString()")
        && let Some(target) = path_before(normalised, " = schema;")
    {
        return Some(ParamsPattern::KeyedRowAssign { source, target });
    }

    // Pattern: the first member of a ctx LIST the params table has a row for,
    // assigned straight to a ctx path. The `.put(` spelling above is the same
    // idea through a map, and neither reads the other.
    if normalised.contains("params.containsKey(")
        && normalised.contains("params.get(")
        && normalised.contains("break;")
        && let Some(list) = base_between(normalised, " : ctx.", ")")
        && let Some(target) = path_before(normalised, " = params.get(")
    {
        return Some(ParamsPattern::FirstLabelInTable { list, target });
    }

    // Pattern: rename an object's keys, recursively, through a name map.
    if normalised.contains("keyMap.containsKey(key)") {
        return Some(ParamsPattern::RenameKeys);
    }

    // Pattern: several fields each normalised through their own value map.
    // Ahead of the reversible lookup, whose trigger this pattern also matches.
    if normalised.contains(".map?.getOrDefault(") || normalised.contains("param.map.") {
        return Some(ParamsPattern::ValueMaps);
    }

    // Pattern: the row is a LIST OF INSTRUCTIONS built into a list for a
    // `foreach` to write. Ahead of every `params.get(` matcher below, and of
    // the `LookupNormalise` catch-all that would otherwise claim it.
    if normalised.contains("values.add(")
        && let Some(pattern) = parse_instruction_rows(normalised)
    {
        return Some(ParamsPattern::InstructionRows(Box::new(pattern)));
    }

    // Pattern: params IS the table, keyed by a field, and each named column's
    // members are appended onto an ECS array. `getOrDefault` spells the lookup
    // so none of the `params.get(` triggers below ever sees it.
    if normalised.contains("params.getOrDefault(ctx.")
        && let Some(pattern) = parse_row_column_appends(normalised)
    {
        return Some(ParamsPattern::RowColumnAppends(Box::new(pattern)));
    }

    // Pattern: params IS the table, and one row's columns are written straight
    // onto ctx, then refined by the event's outcome. The key is spelled either
    // plainly or wrapped for `.toString()`, which a numeric or boolean key is.
    if (normalised.contains("params.get(ctx.") || normalised.contains("params.get((ctx."))
        && normalised.contains(").get('")
    {
        return Some(ParamsPattern::RowColumns(Program::parse(row_columns_tail(
            normalised,
        ))));
    }

    // Pattern: fan a parsed key/value message out through a params table.
    if normalised.contains("appendOrCreate(") && normalised.contains("params.get(entry.getKey())") {
        return Some(ParamsPattern::KeyedMessageTable);
    }

    // Pattern: the security pipeline's msobjs message-table decode, keyed on
    // its two auxiliary tables. Ahead of the indexed lookup, whose `.put(`
    // trigger its writes also spell.
    if normalised.contains("AccessMaskDescriptions") && normalised.contains("reversed_descriptions")
    {
        return Some(ParamsPattern::MessageTable);
    }

    // Pattern: sentinel_one's first-asset extraction, the os list in params.
    if normalised.contains("agent_uuid") && normalised.contains("params.os_type") {
        return Some(ParamsPattern::FirstAsset);
    }

    // Pattern: powershell's matcher-driven KV -- tab-prefixed keys, the
    // value everything up to the next key, multiline included.
    if normalised.contains("ctx.winlog?.event_data[params[") && normalised.contains("previousEnd") {
        return Some(ParamsPattern::MatcherKv);
    }

    // Pattern: a params row whose members are selected by INDEX, one field
    // each. Ahead of the lookup-put below, whose `= params.get(ctx.` trigger
    // this also spells and which writes the WHOLE row into the last field the
    // script names.
    if normalised.contains("= params.get(ctx.")
        && let Some(pattern) = parse_indexed_row_columns(normalised)
    {
        return Some(pattern);
    }

    // Pattern: look one field up in the table and `.put` the row somewhere
    // ELSE -- the security pipeline's logon type, dnsserver's QTYPE with its
    // trailing `.remove`. Ahead of the normalise pattern, which writes back to
    // the field it read.
    if normalised.contains("= params.get(ctx.")
        && let Some((source, target)) = parse_lookup_put(normalised)
    {
        return Some(ParamsPattern::LookupPut {
            source,
            target,
            removes: parse_removes(normalised),
        });
    }

    // Pattern: the row wrapped in a one-element list -- dnsserver's winlog
    // keywords.
    if normalised.contains("= params.get(ctx.")
        && normalised.contains("new ArrayList()")
        && let Some((source, target)) = parse_lookup_wrap_list(normalised)
    {
        return Some(ParamsPattern::LookupWrapList { source, target });
    }

    // Pattern: a named set of keys moved from a map into a child of it.
    if normalised.contains(".forEach(k ->")
        && normalised.contains("[k] = ctx.")
        && let Some(pattern) = parse_move_keys_into_child(normalised)
    {
        return Some(ParamsPattern::MoveKeysIntoChild(Box::new(pattern)));
    }

    // Pattern: every value of a map, normalised in place. Its own trigger is
    // the helper the script declares, which nothing else spells.
    if normalised.contains("trimQuotes(")
        && normalised.contains("processFieldValue(k, ")
        && let Some(pattern) = parse_normalise_map_values(normalised)
    {
        return Some(ParamsPattern::NormaliseMapValues(Box::new(pattern)));
    }

    // Pattern: the record's ECS action, chosen by which candidate's fields are
    // present. Ahead of the single-entry table matchers below, which read the
    // primary lookup as an ordinary one and would claim the script without
    // ever choosing between the candidates.
    if normalised.contains("has_fields")
        && let Some(pattern) = parse_record_action_table(normalised)
    {
        return Some(ParamsPattern::RecordActionTable(Box::new(pattern)));
    }

    // Pattern: the ECS event block a params table names, keyed on one field.
    // Ahead of the generic table matchers, which read `params.logtypes` as an
    // ordinary per-field lookup and would claim the script without writing
    // category or type.
    if normalised.contains(".kind = ")
        && normalised.contains("= params.")
        && let Some(pattern) = parse_event_block_table(normalised)
    {
        return Some(ParamsPattern::EventBlockTable(Box::new(pattern)));
    }

    // Pattern: prefix a field with a validated scheme -- zscaler web's
    // `<protocol>://<url>` build, the fallback scheme from params.
    if normalised.contains("+ '://' +")
        && let Some(pattern) = parse_protocol_prefix(normalised)
    {
        return Some(pattern);
    }

    // Pattern: map a field through a params table in whichever direction it
    // was written -- name to number, or a number already there back to a name.
    if normalised.contains("params.entrySet()") && normalised.contains("entry.getKey()") {
        return Some(ParamsPattern::ReversibleLookup);
    }

    // Pattern: look a field up in a static table and merge the row into ctx.
    //
    // The lambda's exact spelling is NOT the trigger. carbonblack_edr writes
    // `forEach( (k, v) ->` with a space, subscripts `params[type]` where the
    // others call `.get(`, and merges onto a local bound to `ctx.event`;
    // each of those three alone was enough to miss it, and missing it cost
    // `event.kind` on all 99 of its events. `[k] = ` is the merge itself and
    // is what the parse below reads the target from.
    if normalised.contains("forEach(")
        && normalised.contains("[k] = ")
        && (normalised.contains("params.get(") || normalised.contains("params["))
    {
        return Some(ParamsPattern::LookupMerge(Program::parse(normalised)));
    }

    // Pattern: auth0's per-event-type action table -- a row overwrites the
    // field it was keyed by and derives event.outcome from its
    // classification text. Ahead of the general column fan-out below, whose
    // >=3 `.get(` trigger this row also spells but which cannot fan an
    // ArrayList column or write the miss branch. `actions` and `eventType`
    // are this script's own names, unique across every pipeline.
    if normalised.contains("params.get('actions')")
        && normalised.contains("actions.get(eventType)")
        && let Some(pattern) = parse_keyed_action_row(normalised)
    {
        return Some(pattern);
    }

    // Pattern: a row whose named columns are LISTS appended to array fields,
    // with one copy above the early return. Ahead of the column fan-out below,
    // whose trigger this also spells and which declines on it, so the script
    // reached nothing at all.
    if normalised.contains("params.get(")
        && normalised.contains(".add(")
        && let Some(pattern) = parse_keyed_row_appends(normalised)
    {
        return Some(ParamsPattern::KeyedRowAppends(Box::new(pattern)));
    }

    // Pattern: look a row up in a nested table and fan its columns out,
    // appending the list-valued ones rather than replacing them.
    if normalised.contains("params.get(") && normalised.matches(".get(").count() >= 3 {
        return Some(ParamsPattern::LookupColumns);
    }

    // Pattern: normalise a field through a params table, keeping the input
    // when the table has no row for it.
    if normalised.contains("params.get(")
        && let Some(pattern) = parse_lookup_normalise(normalised)
    {
        return Some(ParamsPattern::LookupNormalise(
            pattern,
            Program::parse(normalised),
        ));
    }

    // Pattern: index a params array by a numeric field.
    if normalised.contains(".put(") && normalised.contains("params") {
        return Some(ParamsPattern::IndexedLookup);
    }

    // Pattern: scale a numeric field by a params constant.
    if scale_marker(normalised).is_some() {
        return Some(ParamsPattern::Scale);
    }

    // Pattern: strip a params-named marker out of a string field.
    if normalised.contains(".replace(params.") {
        return Some(ParamsPattern::Replace);
    }

    // Pattern: union a table row's list columns into the ECS arrays.
    if normalised.contains("addUnique(") && normalised.contains("params[ctx.") {
        return Some(ParamsPattern::AddUniqueRow);
    }

    // Pattern: pick one framework name from the prefixes of several ID lists.
    if normalised.contains("new HashSet()") && normalised.contains("params.framework_preference") {
        return Some(ParamsPattern::FrameworkPreference);
    }

    params_pattern_rest(normalised)
}

/// The last of the dispatch, split only because one function may not run past
/// 150 lines. Order still matters across all three parts: the first trigger
/// that fires wins, and these run after everything above.
fn params_pattern_rest(normalised: &str) -> Option<ParamsPattern> {
    // Pattern: a key normalised through the table onto ONE field, under the
    // script's own null check. Above the bracket catch-all, which cannot
    // resolve a key that still carries its normalising call and which stands an
    // unlisted key in for its own value where this drops it.
    if normalised.contains("params[ctx.")
        && let Some(pattern) = parse_normalised_lookup(normalised)
    {
        return Some(pattern);
    }

    // Pattern: the same lookup where the key carries its own parentheses and
    // `.toString()`, which every `params[ctx.` trigger above misses. Above the
    // catch-all because the script's `containsKey` guard DROPS an unlisted key
    // where the catch-all stands it in for its own value.
    if normalised.contains("!params.containsKey(")
        && normalised.contains("= params[")
        && let Some(pattern) = parse_stringified_lookup(normalised)
    {
        return Some(pattern);
    }

    // Pattern: the same normalise-through-a-table written with the bracket
    // form. LAST, so nothing that reads the brackets for its own pattern --
    // `addUnique` over a row, for one -- is claimed by the general case.
    if normalised.contains("params[ctx.")
        && let Some(pattern) = parse_lookup_normalise(normalised)
    {
        return Some(ParamsPattern::LookupNormalise(
            pattern,
            Program::parse(normalised),
        ));
    }

    // Pattern: the same lookup with no fallback, guarded on the table holding
    // the key. AFTER both `LookupNormalise` spellings, so it takes only what
    // they decline -- a miss must leave the target absent, not written back.
    if normalised.contains("params.containsKey(")
        && let Some(pattern) = parse_guarded_lookup(normalised)
    {
        return Some(ParamsPattern::GuardedLookup(pattern));
    }

    // Pattern: a NAMED params table's row merged WHOLE onto ctx, with further
    // rows of the same table standing in for a key that missed. The unnamed
    // table is `LookupMerge` above, which this cannot reach past.
    if normalised.contains("forEach((k, v) ->")
        && let Some(pattern) = parse_merge_row_or_fallback(normalised)
    {
        return Some(ParamsPattern::MergeRowOrFallback(Box::new(pattern)));
    }

    // Pattern: a NAMED params table's row fanned onto ctx, with literal
    // defaults where the key has no row. LAST, so every table matcher above
    // keeps the scripts it already claims.
    if let Some(pattern) = parse_row_or_defaults(normalised) {
        return Some(ParamsPattern::RowOrDefaults(Box::new(pattern)));
    }

    None
}

/// The ctx path before a literal, accepting a bracket subscript.
///
/// `ctx.log['logger']` names the field a dotted path would; the brackets are
/// how Painless spells a key it does not want read as an identifier, and
/// opencanary writes its target that way.
fn bracket_path_before(script: &str, close: &str) -> Option<String> {
    let head = &script[..script.find(close)?];
    let raw = head[head.rfind("ctx.")? + 4..].trim();

    let mut path = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(open) = rest.find('[') {
        path.push_str(&rest[..open]);
        let inner = &rest[open + 1..];
        let end = inner.find(']')?;
        path.push('.');
        path.push_str(inner[..end].trim().trim_matches(['\'', '"']));
        rest = &inner[end + 1..];
    }
    path.push_str(rest);

    let path = clean_path(&path);
    (!path.is_empty()
        && path
            .chars()
            .all(|c| c.is_alphanumeric() || "._@-".contains(c)))
    .then_some(path)
}

/// The row `source` keys, or the KEY ITSELF where the table has no row.
///
/// opencanary names its log type from a numeric code and keeps the code as
/// the name when it does not recognise it, which is a fallback rather than a
/// default -- the value written depends on the document.
fn run_lookup_or_key(
    event: &mut Event,
    source: &str,
    target: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some(key) = event.get(source).map(table_key) else {
        return true;
    };
    let value = params
        .get(&key)
        .cloned()
        .unwrap_or_else(|| Value::String(key));
    let _ = event.set(target, value);
    true
}

/// The ctx path immediately BEFORE a literal, read backwards from it.
///
/// The forward reader cannot answer this: a script that walks one ctx path and
/// assigns to another spells `ctx.` twice, and the first one is the loop's.
fn path_before(script: &str, close: &str) -> Option<String> {
    let head = &script[..script.find(close)?];
    let start = head.rfind("ctx.")? + 4;
    let raw = &head[start..];
    if raw.is_empty()
        || !raw
            .chars()
            .all(|c| c.is_alphanumeric() || "._?".contains(c))
    {
        return None;
    }
    let path = clean_path(raw);
    (!path.is_empty()).then_some(path)
}

/// The params row `source` keys, assigned WHOLE to `target`.
///
/// The row is a list for `event.type` and `event.category` and a string for
/// `event.kind` and `event.provider`, so it is written as it stands rather
/// than appended to. A key the table does not carry leaves the target alone.
fn run_keyed_row_assign(
    event: &mut Event,
    source: &str,
    target: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some(key) = event.get(source).map(table_key) else {
        return true;
    };
    if let Some(row) = params.get(&key) {
        let _ = event.set(target, row.clone());
    }
    true
}

/// The first member of `list` the table has a row for, written to `target`.
///
/// The vendor breaks on the first hit, so a document carrying several labels
/// takes the earliest one in ITS OWN order, not the table's.
fn run_first_label_in_table(
    event: &mut Event,
    list: &str,
    target: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some(Value::Array(members)) = event.get(list).cloned() else {
        return true;
    };
    for member in &members {
        if let Some(value) = params.get(&table_key(member)) {
            let _ = event.set(target, value.clone());
            break;
        }
    }
    true
}

/// The ctx path a script names between two literals, with its `?` stripped.
///
/// Returns None when the span is not a plain path, so a restructured script
/// declines rather than claiming a base it never touches.
fn base_between(script: &str, open: &str, close: &str) -> Option<String> {
    let start = script.find(open)? + open.len();
    let end = script[start..].find(close)? + start;
    let raw = &script[start..end];
    if raw.is_empty()
        || !raw
            .chars()
            .all(|c| c.is_alphanumeric() || "._?".contains(c))
    {
        return None;
    }
    let path = clean_path(raw);
    (!path.is_empty()).then_some(path)
}

/// Append to one of the deferred lists, creating it if this is the first push.
///
/// The vendor writes `ctx._temp_.computeIfAbsent("sets", k -> new ArrayList())`
/// and three scripts in a row push onto the same list, so the order of the
/// pushes is the order the `foreach` applies them -- a later set at the same
/// target wins.
fn push_deferred(event: &mut Event, path: &str, value: Value) {
    let mut items = match event.get(path) {
        Some(Value::Array(existing)) => existing.clone(),
        _ => Vec::new(),
    };
    items.push(value);
    let _ = event.set(path, Value::Array(items));
}

/// Defer a write of `value` to `target`.
fn defer_set(event: &mut Event, target: &str, value: Value) {
    let mut entry = Map::new();
    entry.insert("target".to_owned(), Value::String(target.to_owned()));
    entry.insert("value".to_owned(), value);
    push_deferred(event, "_temp_.sets", Value::Object(entry));
}

/// Defer the removal of one key of the source map.
fn defer_remove(event: &mut Event, key: &str) {
    push_deferred(event, "_temp_.removes", Value::String(key.to_owned()));
}

/// A value as the params tables key themselves: a string stays as it is, and
/// anything else takes its JSON rendering, so a number keys as its digits.
fn table_key(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

/// Every key of the source map routed through `params[key]`, a list of
/// actions each naming a `to` and optionally a `map` to translate the value.
///
/// A key the table does not name is left alone -- the vendor adds to `removes`
/// only inside the `params[key] != null` guard.
fn run_deferred_field_table(event: &mut Event, base: &str, params: &Map<String, Value>) -> bool {
    let Some(Value::Object(source)) = event.get(base).cloned() else {
        return true;
    };
    for (key, raw) in &source {
        let Some(Value::Array(actions)) = params.get(key) else {
            continue;
        };
        for action in actions {
            let Some(target) = action.get("to").and_then(Value::as_str) else {
                continue;
            };
            // A translated value that the table does not carry is DROPPED,
            // not passed through: the vendor guards on `value != null`.
            let value = match action.get("map") {
                Some(Value::Object(table)) => table.get(&table_key(raw)).cloned(),
                _ => Some(raw.clone()),
            };
            if let Some(value) = value {
                defer_set(event, target, value);
            }
        }
        defer_remove(event, key);
    }
    true
}

/// A colon-joined field split across the targets params lists for it.
///
/// sonicwall's `dst` is `81.2.69.143:443:X1`, which is the address, the port
/// and the egress interface. The vendor walks the pairs DOWNWARD, so the last
/// target is deferred first and the earliest one wins at the same target.
fn run_deferred_split_table(event: &mut Event, base: &str, params: &Map<String, Value>) -> bool {
    for (key, mapping) in params {
        let Some(targets) = mapping.as_array() else {
            continue;
        };
        // A key with no value is skipped BEFORE the remove, so it stays.
        let Some(text) = event.get_string(&format!("{base}.{key}")) else {
            continue;
        };
        let parts: Vec<&str> = text.split(':').collect();
        for index in (0..parts.len().min(targets.len())).rev() {
            if let Some(target) = targets[index].as_str() {
                defer_set(event, target, Value::String(parts[index].to_owned()));
            }
        }
        defer_remove(event, key);
    }
    true
}

/// One destination built by appending a fixed suffix to whichever source
/// field is present -- sonicwall's `dur` in seconds and `cdur` in
/// milliseconds, both becoming `event.duration` in nanoseconds.
///
/// The remove is OUTSIDE the presence check in the vendor script, so a named
/// field is dropped whether or not it was there.
fn run_deferred_append_table(event: &mut Event, base: &str, params: &Map<String, Value>) -> bool {
    let Some(Value::Object(source)) = event.get(base).cloned() else {
        return true;
    };
    let (Some(destination), Some(sources)) = (
        params.get("destination").and_then(Value::as_str),
        params.get("sources").and_then(Value::as_array),
    ) else {
        return true;
    };
    for entry in sources {
        let Some(field) = entry.get("field").and_then(Value::as_str) else {
            continue;
        };
        if let Some(raw) = source.get(field) {
            let append = entry.get("append").and_then(Value::as_str).unwrap_or("");
            defer_set(
                event,
                destination,
                Value::String(format!("{}{append}", table_key(raw))),
            );
        }
        defer_remove(event, field);
    }
    true
}

/// `event.code` through `params.message_codes` to name a type, then
/// `params.event_types` to expand that type into the `event.*` block.
///
/// A code the first table does not carry leaves the event alone. A type the
/// SECOND table does not carry makes the vendor throw, which its `on_failure`
/// catches and turns into a tagged error -- there is nothing to throw here, so
/// the event is left alone and the miss shows up as the missing fields.
fn run_message_code_event_type(event: &mut Event, params: &Map<String, Value>) -> bool {
    let Some(code) = event.get("event.code").map(table_key) else {
        return true;
    };
    let Some(evtype) = params
        .get("message_codes")
        .and_then(Value::as_object)
        .and_then(|codes| codes.get(&code))
        .and_then(Value::as_str)
    else {
        return true;
    };
    if let Some(actions) = params
        .get("event_types")
        .and_then(Value::as_object)
        .and_then(|types| types.get(evtype))
        .and_then(Value::as_object)
    {
        for (field, value) in actions {
            let _ = event.set(&format!("event.{field}"), value.clone());
        }
    }
    let _ = event.set("event.action", Value::String(evtype.to_owned()));
    true
}

/// Each key totalled across the named direction prefixes.
///
/// Only an integer counts on either side -- the vendor guards every operand on
/// `instanceof Long`, so a byte count still carrying its grok string is skipped
/// rather than coerced. An existing total is added to, not replaced, and the
/// addition saturates because both operands come off the wire.
fn run_summed_directions(event: &mut Event, params: &Map<String, Value>) -> bool {
    let (Some(from), Some(keys), Some(to)) = (
        params.get("from").and_then(Value::as_array),
        params.get("keys").and_then(Value::as_array),
        params.get("to").and_then(Value::as_str),
    ) else {
        return true;
    };

    for key in keys.iter().filter_map(Value::as_str) {
        let target = format!("{to}.{key}");
        let mut total = event.get(&target).and_then(Value::as_i64);
        let mut summed = false;
        for prefix in from.iter().filter_map(Value::as_str) {
            let Some(value) = event
                .get(&format!("{prefix}.{key}"))
                .and_then(Value::as_i64)
            else {
                continue;
            };
            total = Some(total.map_or(value, |running| running.saturating_add(value)));
            summed = true;
        }
        if summed && let Some(total) = total {
            let _ = event.set(&target, Value::from(total));
        }
    }
    true
}

/// Name a CSV line's unlabelled columns from the row its fingerprint matches.
///
/// The fingerprint is the joined key list the colon-pair reader built, so a
/// layout is identified by WHICH columns carried a `Key:` label and which did
/// not. Where a row matches, its `columns` name the holes by index.
fn run_csv_fingerprint_provider(
    event: &mut Event,
    list: &str,
    map: &str,
    fingerprint: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some(Value::Array(columns)) = event.get(list).cloned() else {
        return true;
    };

    // The first column is assumed to be the host, whenever it reads like one.
    if let Some(hostname) = columns.first().and_then(Value::as_str)
        && !hostname.is_empty()
        && hostname
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
    {
        let _ = event.set("host.hostname", hostname);
    }

    let Some(Value::Array(providers)) = params.get("providers") else {
        return true;
    };
    let Some(current) = event.get_str(fingerprint).map(str::to_owned) else {
        return true;
    };

    // A row names either one layout or several; against a list the `==` is
    // false and only the membership test can hit.
    let Some(provider) = providers.iter().find(|row| match row.get("fingerprint") {
        Some(Value::String(only)) => *only == current,
        Some(Value::Array(any)) => any.iter().any(|m| m.as_str() == Some(current.as_str())),
        _ => false,
    }) else {
        return true;
    };

    if let Some(name) = provider.get("name") {
        let _ = event.set("event.provider", name.clone());
    }
    for (member, target) in [
        ("event_category", "event.category"),
        ("event_type", "event.type"),
    ] {
        if let Some(value) = provider.get(member).filter(|value| !value.is_null()) {
            let _ = event.set(target, value.clone());
        }
    }

    let Some(Value::Array(named)) = provider.get("columns") else {
        return true;
    };
    let mut writes: Vec<(String, String)> = Vec::with_capacity(named.len());
    for column in named {
        let (Some(index), Some(name)) = (
            column.get("index").and_then(Value::as_u64),
            column.get("name").and_then(Value::as_str),
        ) else {
            continue;
        };
        let Some(value) = columns
            .get(usize::try_from(index).unwrap_or(usize::MAX))
            .and_then(Value::as_str)
        else {
            continue;
        };
        // Java's `trim` cuts at U+0020, not at Unicode whitespace.
        let trimmed = value.trim_matches(|c: char| c <= ' ');
        if !trimmed.is_empty() {
            writes.push((name.to_owned(), trimmed.to_owned()));
        }
    }
    for (name, value) in writes {
        let _ = event.set(&format!("{map}.{name}"), value);
    }
    true
}

/// Where a guarded params lookup reads its key and writes its answer.
///
/// `LookupNormalise` is the same idea with a fallback: it writes the KEY back
/// when the table misses. This one writes nothing, so they cannot share a
/// runner -- a miss here must leave the target absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardedLookupScript {
    key: String,
    target: String,
}

impl GuardedLookupScript {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(key: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            target: target.into(),
        }
    }
}

/// `if (params.containsKey(k)) { ctx.<target> = params.get(k); }`
fn parse_guarded_lookup(script: &str) -> Option<GuardedLookupScript> {
    // Balanced, because the key is `obj.toString()` and splitting on the first
    // `)` cuts inside that call rather than after the argument.
    let expr = last_call_argument(script, "params.containsKey(")?;
    let local = expr.trim().trim_end_matches(".toString()").trim();
    let bound = script.split_once(&format!(" {local} = ctx."))?.1;
    let key = clean_path(bound.split([';', '\n']).next()?.trim());
    let target = ctx_writes(script).last().map(|(path, _)| path.clone())?;
    (!key.is_empty() && !target.is_empty()).then(|| GuardedLookupScript::new(key, target))
}

/// Write the table's row for the key, and nothing at all when it misses.
pub fn guarded_lookup(
    event: &mut Event,
    pattern: &GuardedLookupScript,
    params: &Map<String, Value>,
) -> bool {
    let Some(key) = event.get_as_string(&pattern.key) else {
        return true;
    };
    if let Some(row) = params.get(&key) {
        let _ = event.set(&pattern.target, row.clone());
    }
    true
}

/// Run the matcher a pattern names, against one event.
#[allow(clippy::too_many_lines)] // One delegation arm per pattern; it grows with the pattern list.
pub(crate) fn run_params_pattern(
    event: &mut Event,
    normalised: &str,
    params: &Map<String, Value>,
    pattern: &ParamsPattern,
) -> bool {
    match pattern {
        ParamsPattern::AwsEntity(script) => {
            crate::painless_entity::run_entity_script(event, script, params)
        }
        ParamsPattern::DropEmptyMembers { parent, list } => {
            run_drop_empty_members(event, parent, list, params)
        }
        ParamsPattern::IndexedRowColumns { subject, columns } => {
            run_indexed_row_columns(event, subject, columns, params)
        }
        ParamsPattern::FirstLabelInTable { list, target } => {
            run_first_label_in_table(event, list, target, params)
        }
        ParamsPattern::KeyedRowAssign { source, target } => {
            run_keyed_row_assign(event, source, target, params)
        }
        ParamsPattern::LookupOrKey { source, target } => {
            run_lookup_or_key(event, source, target, params)
        }
        ParamsPattern::DeferredFieldTable(base) => run_deferred_field_table(event, base, params),
        ParamsPattern::DeferredSplitTable(base) => run_deferred_split_table(event, base, params),
        ParamsPattern::DeferredAppendTable(base) => run_deferred_append_table(event, base, params),
        ParamsPattern::CsvFingerprintProvider {
            list,
            map,
            fingerprint,
        } => run_csv_fingerprint_provider(event, list, map, fingerprint, params),
        ParamsPattern::MessageCodeEventType => run_message_code_event_type(event, params),
        ParamsPattern::SummedDirections => run_summed_directions(event, params),
        ParamsPattern::SysmonQueryResults => try_sysmon_query_results(event, normalised, params),
        ParamsPattern::SysmonRegistry => crate::painless_windows::run_registry(event, params),
        ParamsPattern::MessageTable => crate::painless_windows::run_message_table(event, params),
        ParamsPattern::FirstAsset => try_first_asset(event, params),
        ParamsPattern::MatcherKv => try_matcher_kv(event, params),
        ParamsPattern::LookupPut {
            source,
            target,
            removes,
        } => {
            if let Some(row) = event
                .get_as_string(source)
                .and_then(|key| params.get(&key))
                .cloned()
            {
                let _ = event.set(target, row);
            }
            // The script removes OUTSIDE its null guard, so a key with no
            // table row still goes.
            for path in removes {
                event.remove(path);
            }
            true
        }
        ParamsPattern::LookupWrapList { source, target } => {
            if let Some(row) = event
                .get_as_string(source)
                .and_then(|key| params.get(&key))
                .cloned()
            {
                let _ = event.set(target, Value::Array(vec![row]));
            }
            true
        }
        ParamsPattern::MimecastLogType => try_mimecast_log_type(event, params),
        ParamsPattern::InvocationDetails => try_invocation_details(event, params),
        ParamsPattern::ScheduledTask => crate::painless_scheduled_task::run(event, params),
        ParamsPattern::ThreatIndicatorType(source) => {
            try_threat_indicator_type(event, source, params)
        }
        ParamsPattern::EvidenceCategories { source, key } => {
            run_evidence_categories(event, source, key, params)
        }
        ParamsPattern::MsgParts => run_msg_parts(event, params),
        ParamsPattern::MappingRow(pattern) => run_mapping_row(event, pattern, params),
        ParamsPattern::SecuritySddl => crate::painless_sddl::run(event, params),
        ParamsPattern::KeyedRowMembers(pattern) => pattern.run(event, params),
        ParamsPattern::PutWrites { writes, require } => {
            if let Some((path, allowed)) = require
                && !event
                    .get_as_string(path)
                    .is_some_and(|held| allowed.contains(&held))
            {
                return true;
            }
            for write in writes {
                write.run(event, params);
            }
            true
        }
        ParamsPattern::PutFlagNames {
            container,
            member,
            source,
        } => {
            let Some(bits) = event.get(source).and_then(as_u64_flags) else {
                return true;
            };
            // Elasticsearch walks params in INSERTION order, and the vendor's
            // tables disagree about direction: kerberos ticket options run
            // highest bit first, the UAC attributes lowest.
            let names: Vec<Value> = params
                .iter()
                .filter_map(|(key, name)| {
                    let flag = as_u64_flags(&Value::String(key.clone()))?;
                    (flag != 0 && bits & flag == flag).then(|| name.clone())
                })
                .collect();
            if !names.is_empty() {
                let _ = event.set(&format!("{container}.{member}"), Value::Array(names));
            }
            true
        }
        ParamsPattern::TableLookupOrLiteral {
            source,
            table,
            target,
            default,
        } => {
            // The script's own `!= null` guard: an absent source writes
            // nothing at all, not the default.
            if let Some(key) = event.get_as_string(source) {
                let value = params
                    .get(table)
                    .and_then(Value::as_object)
                    .and_then(|rows| rows.get(&key))
                    .cloned()
                    .unwrap_or_else(|| json!(default));
                let _ = event.set(target, value);
            }
            true
        }
        ParamsPattern::UppercaseLookupDefault { source, target } => {
            if let Some(name) = event.get_str(source).map(str::to_uppercase) {
                let value = params.get(&name).cloned().unwrap_or_else(|| json!(name));
                let _ = event.set(target, value);
            }
            true
        }
        ParamsPattern::EventBlockTable(pattern) => run_event_block_table(event, pattern, params),
        ParamsPattern::RecordActionTable(pattern) => {
            run_record_action_table(event, pattern, params)
        }
        ParamsPattern::NormaliseMapValues(pattern) => {
            run_normalise_map_values(event, pattern, params)
        }
        ParamsPattern::MoveKeysIntoChild(pattern) => {
            run_move_keys_into_child(event, pattern, params)
        }
        ParamsPattern::ProtocolPrefix {
            list,
            fallback,
            subject,
            target,
        } => {
            let (Some(scheme), Some(tail)) = (
                event.get_str(subject).map(str::to_string),
                event.get_str(target).map(str::to_string),
            ) else {
                return true;
            };
            let valid = params
                .get(list)
                .and_then(Value::as_array)
                .is_some_and(|schemes| schemes.iter().any(|s| s.as_str() == Some(&scheme)));
            let scheme = if valid {
                scheme
            } else {
                match params.get(fallback).and_then(Value::as_str) {
                    Some(fallback) => fallback.to_string(),
                    None => return true,
                }
            };
            let _ = event.set(target, Value::String(format!("{scheme}://{tail}")));
            true
        }
        ParamsPattern::SentinelRemoval => try_sentinel_removal(event, normalised, params),
        ParamsPattern::FiletimeFieldList => try_filetime_field_list(event, normalised, params),
        ParamsPattern::BitFlags => try_bit_flags(event, normalised, params),
        ParamsPattern::FirstContainedMember => {
            try_first_contained_member(event, normalised, params)
        }
        ParamsPattern::RenameKeys => try_rename_keys(event, normalised, params),
        ParamsPattern::ValueMaps => try_value_maps(event, normalised, params),
        ParamsPattern::RowColumns(literals) => try_row_columns(event, normalised, params, literals),
        ParamsPattern::RowColumnAppends(pattern) => run_row_column_appends(event, pattern, params),
        ParamsPattern::InstructionRows(pattern) => run_instruction_rows(event, pattern, params),
        ParamsPattern::KeyedActionRow { table, source } => {
            run_keyed_action_row(event, table, source, params)
        }
        ParamsPattern::KeyedRowAppends(pattern) => run_keyed_row_appends(event, pattern, params),
        ParamsPattern::KeyedMessageTable => try_keyed_message_table(event, normalised, params),
        ParamsPattern::ReversibleLookup => try_reversible_lookup(event, normalised, params),
        ParamsPattern::LookupMerge(literals) => {
            try_lookup_merge(event, normalised, params, literals)
        }
        ParamsPattern::LookupColumns => try_lookup_columns(event, normalised, params),
        ParamsPattern::NormalisedLookup {
            source,
            target,
            fold,
        } => run_normalised_lookup(event, source, target, *fold, params),
        ParamsPattern::StringifiedLookup { source, target } => {
            run_stringified_lookup(event, source, target, params)
        }
        ParamsPattern::LookupNormalise(pattern, literals) => {
            lookup_normalise(event, pattern, literals, params)
        }
        ParamsPattern::GuardedLookup(pattern) => guarded_lookup(event, pattern, params),
        ParamsPattern::IndexedLookup => try_indexed_lookup(event, normalised, params),
        ParamsPattern::Scale => try_scale(event, normalised, params),
        ParamsPattern::Replace => try_replace(event, normalised, params),
        ParamsPattern::AddUniqueRow => try_add_unique_row(event, normalised, params),
        ParamsPattern::FrameworkPreference => try_framework_preference(event, normalised, params),
        ParamsPattern::RowOrDefaults(pattern) => run_row_or_defaults(event, pattern, params),
        ParamsPattern::MergeRowOrFallback(pattern) => {
            run_merge_row_or_fallback(event, pattern, params)
        }
        ParamsPattern::SelectMembers { subject, rest } => {
            run_select_members(event, subject, rest, params)
        }
        ParamsPattern::ParamRenameLadder { path, arms } => {
            run_param_rename_ladder(event, path, arms, params)
        }
    }
}

/// Read `def <t> = params.get(ctx.<source>);` and the `ctx.<base>.put("<leaf>",
/// <t>)` that stores it, as (source, target).
fn parse_lookup_put(script: &str) -> Option<(String, String)> {
    let at = script.find("= params.get(ctx.")?;
    let after = &script[at + "= params.get(ctx.".len()..];
    let (source, _) = after.split_once(')')?;
    let local = script[..at].split_whitespace().next_back()?;
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    let store = format!(", {local})");
    let store_at = script.find(&store)?;
    let before = &script[..store_at];
    let put_at = before.rfind(".put(\"")?;
    let leaf = before[put_at + ".put(\"".len()..].trim_end_matches('"');
    let base = clean_path(before[..put_at].rsplit("ctx.").next()?);
    Some((clean_path(source), format!("{base}.{leaf}")))
}

/// Every `ctx.<base>.remove("<key>")` in the script, as dotted paths.
fn parse_removes(script: &str) -> Vec<String> {
    let mut removes = Vec::new();
    // Painless quotes a string either way and pipelines use both.
    for quote in ['"', '\''] {
        let opens = format!(".remove({quote}");
        for (at, _) in script.match_indices(&opens) {
            let Some(key) = script[at + opens.len()..].split(quote).next() else {
                continue;
            };
            let before = &script[..at];
            let Some(ctx_at) = before.rfind("ctx.") else {
                continue;
            };
            let base = clean_path(&before[ctx_at + 4..]);
            if base
                .chars()
                .all(|c| c.is_alphanumeric() || c == '.' || c == '_')
            {
                removes.push(format!("{base}.{key}"));
            }
        }
    }
    removes
}

/// Read the wrap-in-list pattern: `def <t> = params.get(ctx.<source>); def
/// <list> = new ArrayList(); <list>.add(<t>); ctx.<target> = <list>;`.
fn parse_lookup_wrap_list(script: &str) -> Option<(String, String)> {
    let at = script.find("= params.get(ctx.")?;
    let after = &script[at + "= params.get(ctx.".len()..];
    let (source, _) = after.split_once(')')?;
    let local = script[..at].split_whitespace().next_back()?;

    let add = format!(".add({local})");
    let add_at = script.find(&add)?;
    let list = script[..add_at]
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()?;
    if list.is_empty() {
        return None;
    }

    let store = format!(" = {list};");
    let store_at = script.rfind(&store)?;
    let before = &script[..store_at];
    let target = clean_path(before[before.rfind("ctx.")? + 4..].trim());
    Some((clean_path(source), target))
}

/// powershell's matcher-driven KV over `winlog.event_data.<params.field>`:
/// a key is a TAB, a word run and an equals sign, and its value is
/// everything -- newlines, equals signs and all -- up to the next key,
/// trimmed. No key found writes nothing, which is where the script's own
/// `group` would have thrown.
fn try_matcher_kv(event: &mut Event, params: &Map<String, Value>) -> bool {
    let Some(field) = params.get("field").and_then(Value::as_str) else {
        return true;
    };
    let Some(text) = event
        .get_str(&format!("winlog.event_data.{field}"))
        .map(str::to_string)
    else {
        return true;
    };

    let bytes = text.as_bytes();
    let mut keys: Vec<(usize, usize, String)> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\t' {
            let mut j = i + 1;
            while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            if j > i + 1 && bytes.get(j) == Some(&b'=') {
                keys.push((i, j + 1, text[i + 1..j].to_string()));
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }

    for (index, (_, end, key)) in keys.iter().enumerate() {
        let until = keys.get(index + 1).map_or(text.len(), |(start, ..)| *start);
        let value = text[*end..until].trim().to_string();
        let _ = event.set(&format!("winlog.event_data.{key}"), json!(value));
    }
    true
}

/// `sentinel_one`'s first-asset extraction, transliterated: the container
/// defaults come first exactly as the script writes them -- WHICH is why the
/// provider-details instance, machine-type and region fallbacks are dead,
/// their `== null` guards seeing the empty defaults -- then the first asset,
/// the first detection-time asset, its cloud and kubernetes blocks, and the
/// params-listed os match that RETURNS from the whole script.
#[allow(clippy::too_many_lines)] // A transliteration; splitting it would hide the script's order.
fn try_first_asset(event: &mut Event, params: &Map<String, Value>) -> bool {
    for (path, empty) in [
        ("related", json!({})),
        ("related.hosts", json!([])),
        ("related.user", json!([])),
        ("related.ip", json!([])),
        ("host", json!({})),
        ("host.ip", json!([])),
        ("host.os", json!({})),
        ("cloud", json!({})),
        ("cloud.instance", json!({})),
        ("cloud.machine", json!({})),
        ("cloud.account", json!({})),
        ("cloud.machine.type", json!([])),
        ("cloud.region", json!([])),
        ("cloud.instance.id", json!([])),
        ("cloud.account.id", json!([])),
        ("cloud.account.name", json!([])),
        ("cloud.project", json!({})),
        ("cloud.project.id", json!([])),
        ("container", json!({})),
        ("container.id", json!([])),
        ("container.name", json!([])),
        ("container.image", json!({})),
        ("container.image.name", json!([])),
        ("container.labels", json!([])),
        ("orchestrator", json!({})),
        ("orchestrator.cluster", json!({})),
        ("orchestrator.namespace", json!([])),
        ("orchestrator.resource", json!({})),
        ("orchestrator.resource.parent", json!({})),
        ("orchestrator.resource.parent.type", json!([])),
        ("orchestrator.resource.label", json!([])),
        ("orchestrator.resource.name", json!([])),
        ("observer", json!({})),
    ] {
        if !event.has_value(path) {
            let _ = event.set(path, empty);
        }
    }

    if let Some(Value::Array(assets)) = event.get("sentinel_one.unified_alert.assets").cloned()
        && let Some(asset) = assets.first()
    {
        let field = |k: &str| asset.get(k).cloned();
        if let Some(v) = field("agent_uuid") {
            let _ = event.set("observer.serial_number", v);
        }
        if let Some(v) = field("agent_version") {
            let _ = event.set("observer.version", v);
        }
        if let Some(v) = field("id") {
            let _ = event.append("related.hosts", v);
        }
        if let Some(v) = field("name") {
            let _ = event.append("related.hosts", v.clone());
            let _ = event.set("host.name", v);
        }
        if let Some(v) = field("subcategory") {
            let _ = event.set("host.type", v);
        }
        if let Some(v) = field("last_logged_in_user") {
            let _ = event.append("related.user", v);
        }
    }

    let Some(Value::Array(dt_assets)) = event
        .get("sentinel_one.unified_alert.detection_time.assets")
        .cloned()
    else {
        return true;
    };
    let Some(first) = dt_assets.first() else {
        return true;
    };

    if let Some(a) = first.get("asset") {
        for key in ["console_ip_address", "ip_v4", "ip_v6"] {
            if let Some(v) = a.get(key).cloned() {
                let _ = event.append("host.ip", v.clone());
                let _ = event.append("related.ip", v);
            }
        }
        if let Some(v) = a.get("last_logged_in_user").cloned() {
            let _ = event.append("related.user", v);
        }
        if let Some(v) = a.get("os_name").cloned() {
            let _ = event.set("host.os.name", v);
        }
        if let Some(v) = a.get("os_revision").cloned() {
            let _ = event.set("host.os.version", v);
        }
        if let Some(os_type) = a.get("os_type").and_then(Value::as_str) {
            let lowered = os_type.to_lowercase();
            if let Some(Value::Array(names)) = params.get("os_type") {
                for os in names.iter().filter_map(Value::as_str) {
                    if lowered.contains(os) {
                        let _ = event.set("host.os.type", json!(os));
                        return true;
                    }
                }
            }
        }
    }

    let cloud = first.get("cloud");
    if let Some(c) = cloud {
        for (key, target) in [
            ("account_id", "cloud.account.name"),
            ("cloud_provider", "cloud.provider"),
            ("instance_id", "cloud.instance.id"),
            ("instance_size", "cloud.machine.type"),
            ("location", "cloud.region"),
        ] {
            if let Some(v) = c.get(key).cloned() {
                let _ = event.set(target, v);
            }
        }
        if let Some(pd) = c.get("provider_details") {
            for (key, target, only_if_absent) in [
                ("account_id", "cloud.account.id", false),
                ("instance_id", "cloud.instance.id", true),
                ("instance_type", "cloud.machine.type", true),
                ("project_id", "cloud.project.id", false),
                ("region", "cloud.region", true),
                ("service_account", "cloud.account.id", false),
                ("subscription_id", "cloud.account.id", false),
            ] {
                if let Some(v) = pd.get(key).cloned()
                    && (!only_if_absent || !event.has_value(target))
                {
                    let _ = event.set(target, v);
                }
            }
        }
    }

    if let Some(k) = first.get("kubernetes") {
        for (key, target) in [
            ("cluster_name", "orchestrator.cluster.name"),
            ("container_id", "container.id"),
            ("container_image_name", "container.image.name"),
            ("container_name", "container.name"),
            ("controller_type", "orchestrator.resource.parent.type"),
            ("namespace_name", "orchestrator.namespace"),
            ("pod_labels", "orchestrator.resource.label"),
            ("pod_name", "orchestrator.resource.name"),
        ] {
            if let Some(v) = k.get(key).cloned() {
                let _ = event.set(target, v);
            }
        }
    }
    true
}

/// mimecast's scored log-type classifier, tables from params.
///
/// Keys lowercase into a Java `HashSet`; a `definite_positive` hit wins
/// outright, then candidate ELIMINATION through the `negative` table (a lone
/// survivor wins), then the `positive` table scores what remains and every
/// co-equal winner is listed. Iteration orders are Java's hash orders, which
/// [`crate::painless_helpers::java_bucket`] reproduces -- the corpus's own
/// single-winner strings depend on them.
/// `def name = ctx.<source>.toUpperCase(); ... ctx.<target> = params.getOrDefault(name, name);`
///
/// The default being the KEY itself is what makes this its own pattern: a name
/// the table does not abbreviate stands for itself rather than going missing.
/// `params.<table>.containsKey(k) ? params.<table>[k] : '<default>'`, written
/// to a target and guarded on the source being present.
///
/// The whole script, as `jamf_protect` writes it forty times over:
///
/// ```painless
/// if (ctx.<source> != null) {
///     String itemType = ctx.<source>.toString();
///     def itemTypeString = params.<table>.containsKey(itemType)
///         ? params.<table>[itemType] : 'Unknown';
///     ctx.<root> = ctx.<root> != null ? ctx.<root> : new HashMap();
///     ctx.<target> = itemTypeString;
/// }
/// ```
///
/// The allocation line is noise: `Event::set` builds the parents anyway.
fn parse_table_lookup_or_literal(script: &str) -> Option<ParamsPattern> {
    let (head, rest) = script.split_once(".toString();")?;
    let source = clean_path(head.rsplit("ctx.").next()?);
    if source.is_empty() || source.contains(char::is_whitespace) {
        return None;
    }

    // The local the lookup is keyed on, so a script binding two is not read as
    // one -- the key and the ternary have to name the same thing.
    let bound = head.rsplit(['\n', ';']).next()?.split('=').next()?.trim();
    let key = bound.rsplit(char::is_whitespace).next()?;

    let (before, after) = rest.split_once(&format!(".containsKey({key})"))?;
    let table = before.rsplit("params.").next()?.trim().to_string();
    if table.is_empty() || table.contains(['.', ' ', '(', '[']) {
        return None;
    }

    // `? params.<table>[key] : '<default>'` -- the default is the quoted half.
    let (_, defaulted) = after.split_once('?')?;
    let (_, literal) = defaulted.split_once(':')?;
    let default = quoted_after(literal.split(';').next()?, "")?;

    // The LAST `ctx.` in the script is the write. The allocation line above it
    // names the root only, and `Event::set` builds that anyway.
    let assigned = script.rsplit_once("ctx.")?.1;
    let target = clean_path(assigned.split('=').next()?.trim());
    if target.is_empty() || target.contains(char::is_whitespace) {
        return None;
    }

    Some(ParamsPattern::TableLookupOrLiteral {
        source,
        table,
        target,
        default,
    })
}

fn parse_uppercase_lookup_default(script: &str) -> Option<ParamsPattern> {
    let (head, rest) = script.split_once(".toUpperCase();")?;
    let source = clean_path(head.rsplit("ctx.").next()?);
    if source.is_empty() || source.contains(char::is_whitespace) {
        return None;
    }

    let bound = head.rsplit(['\n', ';']).next()?.split('=').next()?.trim();
    let name = bound.rsplit(char::is_whitespace).next()?;
    let (assignment, _) = rest.split_once(&format!("= params.getOrDefault({name}, {name})"))?;
    let target = clean_path(assignment.rsplit("ctx.").next()?);
    if target.is_empty() || target.contains(char::is_whitespace) {
        return None;
    }

    Some(ParamsPattern::UppercaseLookupDefault { source, target })
}

/// The container and member name a `ctx.<c>.put("<m>", ...)` writes.
fn parse_put_target(script: &str) -> Option<(String, String)> {
    let at = script.find(".put(\"")?;
    let container = clean_path(script[..at].rsplit("ctx.").next()?);
    let member = script[at + ".put(\"".len()..]
        .split('"')
        .next()?
        .to_string();
    if member.is_empty()
        || !container
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
    {
        return None;
    }
    Some((container, member))
}

/// One `ctx.<container>.put("<member>", <value>)`, the value read from the
/// event either directly or through the params table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PutWrite {
    container: String,
    member: String,
    source: String,
    /// Whether the source's value is a params KEY rather than the value.
    through_params: bool,
    fold: Fold,
}

impl PutWrite {
    /// An absent source, or a key the table does not hold, writes nothing --
    /// which is what the script's own null and `containsKey` guards do.
    fn run(&self, event: &mut Event, params: &Map<String, Value>) {
        let Some(raw) = event.get(&self.source).cloned() else {
            return;
        };
        let value = if self.through_params {
            let Some(key) = raw.as_str() else { return };
            let key = self.fold.apply(key);
            let Some(value) = params.get(&key) else {
                return;
            };
            value.clone()
        } else {
            raw
        };
        let _ = event.set(&format!("{}.{}", self.container, self.member), value);
    }
}

/// Every `ctx.<c>.put("<m>", ...)` whose value the event supplies.
///
/// A put whose value is a fresh `HashMap` is the script building its own
/// container and is skipped -- taking it wrote a description into
/// `winlog.logon`, the map that put was creating.
/// `def <local> = params[ctx.<source>.toUpperCase()];` then `ctx.<target> = <local>`.
///
/// The lookup is bound to a local and copied out under a null check, so an
/// unlisted key leaves the field alone.
fn parse_normalised_lookup(script: &str) -> Option<ParamsPattern> {
    let key = ctx_path_between(script, "params[ctx.", "]")?;
    let (source, fold) = Fold::strip(&key);
    let source = source.trim().to_string();
    if source.is_empty()
        || !source
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
    {
        return None;
    }

    // The local the lookup binds, and the single field it is copied onto.
    // The write may be rooted at a bracket: ti_anomali's threatstream itype
    // lookup spells `ctx["threatintel_indicator_type"] = mapping` and nothing
    // else, so a `ctx.`-only search found no target and the lookup fell
    // through to the catch-all, which writes the KEY on a miss where this
    // script's own `!= null` writes nothing.
    let local = local_bound_to(script, "params[")?;
    let target = ctx_writes_rooted_either_way(script)
        .into_iter()
        .find(|(_, rhs)| rhs.trim() == local)
        .map(|(path, _)| path)?;

    Some(ParamsPattern::NormalisedLookup {
        source,
        target,
        fold,
    })
}

/// An unlisted key writes nothing -- the script's own `!= null` returns.
fn run_normalised_lookup(
    event: &mut Event,
    source: &str,
    target: &str,
    fold: Fold,
    params: &Map<String, Value>,
) -> bool {
    let Some(raw) = event.get_as_string(source) else {
        return true;
    };
    let key = fold.apply(&raw);
    if let Some(value) = params.get(&key) {
        let _ = event.set(target, value.clone());
    }
    true
}

/// The field a row lookup is keyed by, in either spelling.
///
/// `params.get(ctx.a.b)` and `params.get((ctx.a.b).toString())` name the same
/// field, and a matcher that reads only the first misses every numeric or
/// boolean key.
fn row_key_path(script: &str) -> Option<String> {
    const OPEN: &str = "params.get(";
    let at = script.find(OPEN)? + OPEN.len();
    key_path(&balanced_argument(&script[at..])?)
}

/// The ctx path a key expression names, with its parentheses and its
/// `.toString()` stripped.
///
/// Painless writes `(ctx.a.b).toString()` where the field is numeric and the
/// table's keys are strings; the path is the same one either way.
fn key_path(expr: &str) -> Option<String> {
    let expr = expr.trim();
    let expr = expr.strip_suffix(".toString()").unwrap_or(expr).trim();
    let expr = expr
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
        .unwrap_or(expr)
        .trim();
    let path = clean_path(expr.strip_prefix("ctx.")?);
    (!path.is_empty()
        && path
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@')))
    .then_some(path)
}

/// `if (<path> == null || !params.containsKey(<key>)) { return; }` then
/// `ctx.<target> = params[<key>];`, where both `<key>` spellings agree.
fn parse_stringified_lookup(script: &str) -> Option<ParamsPattern> {
    const GUARD: &str = "!params.containsKey(";
    let at = script.find(GUARD)?;
    let source = key_path(&balanced_argument(&script[at + GUARD.len()..])?)?;

    // The write must read the SAME key, or this is a different script that
    // happens to spell both halves.
    let target = ctx_writes(script).into_iter().find_map(|(path, rhs)| {
        let inner = rhs.trim().strip_prefix("params[")?.strip_suffix(']')?;
        (key_path(inner)? == source).then_some(path)
    })?;

    Some(ParamsPattern::StringifiedLookup { source, target })
}

fn run_stringified_lookup(
    event: &mut Event,
    source: &str,
    target: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some(key) = event.get_as_string(source) else {
        return true;
    };
    if let Some(value) = params.get(&key) {
        let _ = event.set(target, value.clone());
    }
    true
}

fn parse_put_lookup(script: &str) -> Option<ParamsPattern> {
    let mut writes = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = script[cursor..].find(".put(\"") {
        let at = cursor + rel;
        cursor = at + ".put(\"".len();

        let Some(member) = script[cursor..].split('"').next().map(str::to_string) else {
            continue;
        };
        let Some(value) = balanced_argument(&script[cursor + member.len() + 1..]) else {
            continue;
        };
        let container = clean_path(script[..at].rsplit("ctx.").next()?);
        if member.is_empty()
            || !container
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
        {
            continue;
        }

        let (through_params, key) = match value.strip_prefix("params[") {
            Some(inner) => (true, inner.trim_end_matches(']')),
            None => (false, value.as_str()),
        };
        let Some(key) = key.trim().strip_prefix("ctx.") else {
            continue;
        };
        let (source, fold) = Fold::strip(key);
        let source = clean_path(&source);
        if source.is_empty()
            || !source
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
        {
            continue;
        }

        writes.push(PutWrite {
            container,
            member,
            source,
            through_params,
            fold,
        });
    }

    (!writes.is_empty()).then(|| ParamsPattern::PutWrites {
        writes,
        require: parse_contains_guard(script),
    })
}

/// The text up to the paren that closes the call this argument sits in.
fn balanced_argument(rest: &str) -> Option<String> {
    let rest = rest.trim_start_matches(',').trim_start();
    let mut depth = 1usize;
    for (at, c) in rest.char_indices() {
        match c {
            '(' => depth += 1,
            ')' if depth == 1 => return Some(rest[..at].trim().to_string()),
            ')' => depth -= 1,
            _ => {}
        }
    }
    None
}

/// `!["a", "b"].contains(ctx.<path>)` inside a return-guard.
fn parse_contains_guard(script: &str) -> Option<(String, Vec<String>)> {
    let (head, tail) = script.split_once("].contains(ctx.")?;
    let literals: Vec<String> = head
        .rsplit_once("![")?
        .1
        .split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect();
    let path = clean_path(tail.split(')').next()?);
    (!literals.is_empty() && !path.is_empty()).then_some((path, literals))
}

/// `Long.decode(ctx.<path>)` against every params key, labels collected.
fn parse_put_flag_names(script: &str) -> Option<ParamsPattern> {
    let (container, member) = parse_put_target(script)?;
    let (_, tail) = script.split_once("Long.decode(ctx.")?;
    // The vendor wraps the read in `.trim()` where the field is text.
    let source = clean_path(tail.split(')').next()?.trim_end_matches(".trim("));
    if source.is_empty()
        || !source
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
    {
        return None;
    }

    Some(ParamsPattern::PutFlagNames {
        container,
        member,
        source,
    })
}

/// A params ROW looked up by a normalised key, some of its members put back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KeyedRowMembers {
    /// The field holding the raw key.
    source: String,
    /// Literal pairs the raw key has replaced out of it, in order.
    replacements: Vec<(String, String)>,
    /// Whether the key is upper-cased before the lookup.
    upper: bool,
    /// The map the members are put into.
    container: String,
    /// `(member name, index into the row)`.
    members: Vec<(String, usize)>,
}

impl KeyedRowMembers {
    fn parse(script: &str) -> Option<Self> {
        // `def <local> = ctx.<path>[.replace(..)]*[.toUpperCase()];`
        let (declaration, tail) = script.split_once(" = ctx.")?;
        let local = declaration.rsplit(char::is_whitespace).next()?.to_string();
        let binding = tail.split([';', '\n']).next()?;

        // The path is what comes before the normalising calls chained onto it.
        let path = binding
            .split_once(".replace(")
            .map_or(binding, |(head, _)| head);
        let path = path
            .split_once(".toUpperCase(")
            .map_or(path, |(head, _)| head);
        let source = clean_path(path);
        if source.is_empty()
            || !source
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
        {
            return None;
        }

        let replacements = binding
            .split(".replace(")
            .skip(1)
            .filter_map(|call| {
                let (arguments, _) = call.split_once(')')?;
                let mut literals = arguments.split('"').skip(1).step_by(2);
                Some((literals.next()?.to_string(), literals.next()?.to_string()))
            })
            .collect();

        // `ctx.<container>.put("<member>", params[<local>][<index>]);`
        let mut container = None;
        let mut members = Vec::new();
        for call in script.split(".put(\"").skip(1) {
            let Some((member, rest)) = call.split_once('"') else {
                continue;
            };
            let Some(index) = rest
                .split_once(&format!("params[{local}]["))
                .and_then(|(_, tail)| tail.split(']').next())
                .and_then(|n| n.parse::<usize>().ok())
            else {
                continue;
            };
            let head = &script[..script.find(&format!(".put(\"{member}\""))?];
            container = Some(clean_path(head.rsplit("ctx.").next()?));
            members.push((member.to_string(), index));
        }

        let container = container?;
        (!members.is_empty()).then_some(Self {
            source,
            replacements,
            upper: script.contains(".toUpperCase()"),
            container,
            members,
        })
    }

    /// An unlisted key writes nothing -- the script's own `containsKey` returns.
    fn run(&self, event: &mut Event, params: &Map<String, Value>) -> bool {
        let Some(raw) = event.get_str(&self.source).map(str::to_string) else {
            return true;
        };
        let mut key = raw;
        for (from, to) in &self.replacements {
            key = key.replace(from, to);
        }
        if self.upper {
            key = key.to_uppercase();
        }
        let Some(Value::Array(row)) = params.get(&key) else {
            return true;
        };

        for (member, index) in &self.members {
            if let Some(value) = row.get(*index) {
                let _ = event.set(&format!("{}.{member}", self.container), value.clone());
            }
        }
        true
    }
}

/// securityhub's threat-intel indicators mapped to their STIX type names.
///
/// The vendor type is the params key, so `HASH_MD5` becomes `file` and the
/// hash's own name is the tail of that key lowercased. Each indicator
/// OVERWRITES `threat.indicator.type`, so the last one in the list wins, while
/// the `file` enrichments accumulate.
/// One row of a NAMED params table, selected by a field, with a fallback.
///
/// `def m = params.<table>.get(ctx.<subject>)`, then each of the row's members
/// copied to the field it names -- and a whole set of literal defaults where
/// the subject is not in the table. zscaler's saas-security activity types
/// read this way, and without it every event took the fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingRow {
    subject: String,
    table: String,
    /// Target field and the row member it takes, in the script's order.
    writes: Vec<(String, String)>,
    /// What the else branch writes, as literal lists.
    defaults: Vec<(String, Vec<String>)>,
}

/// Read the subject, the table and both branches off the script.
fn parse_mapping_row(script: &str) -> Option<MappingRow> {
    let subject = script
        .split_once("def at = ctx.")
        .and_then(|(_, rest)| rest.split_once(';'))
        .map(|(path, _)| clean_path(path))?;
    let table = script
        .split_once("params.")
        .and_then(|(_, rest)| rest.split_once(".get("))
        .map(|(name, _)| name.trim().to_string())?;

    let mut writes = Vec::new();
    for site in script.split(" = new ArrayList(m.").skip(1) {
        let member = site.split(')').next()?.trim().to_string();
        let target = script
            .split(&format!(" = new ArrayList(m.{member})"))
            .next()
            .and_then(|head| head.rsplit_once("ctx."))
            .map(|(_, path)| clean_path(path))?;
        writes.push((target, member));
    }
    // The guarded scalar: `if (m.<member> != null) { ctx.<t> = m.<member>; }`.
    for site in script.split("if (m.").skip(1) {
        let Some((member, rest)) = site.split_once(" != null)") else {
            continue;
        };
        let Some(target) = rest
            .split_once(&format!("= m.{member}"))
            .and_then(|(head, _)| head.rsplit_once("ctx."))
            .map(|(_, path)| clean_path(path))
        else {
            continue;
        };
        writes.push((target, member.trim().to_string()));
    }
    if writes.is_empty() {
        return None;
    }

    // `ctx.<target> = ['a', 'b'];` in the else branch.
    let mut defaults = Vec::new();
    if let Some((_, tail)) = script.split_once("} else {") {
        for line in tail.lines().map(str::trim) {
            let Some(rest) = line.strip_prefix("ctx.") else {
                continue;
            };
            let Some((target, literal)) = rest.split_once(" = [") else {
                continue;
            };
            let members: Vec<String> = literal
                .trim_end_matches([';', '}', ' '])
                .trim_end_matches(']')
                .split(',')
                .map(|item| item.trim().trim_matches(['\'', '"']).to_string())
                .filter(|item| !item.is_empty())
                .collect();
            if !members.is_empty() {
                defaults.push((clean_path(target), members));
            }
        }
    }

    Some(MappingRow {
        subject,
        table,
        writes,
        defaults,
    })
}

/// Copy the selected row's members out, or write the defaults.
fn run_mapping_row(event: &mut Event, pattern: &MappingRow, params: &Map<String, Value>) -> bool {
    let row = event
        .get_as_string(&pattern.subject)
        .and_then(|key| params.get(&pattern.table)?.get(&key).cloned());

    let Some(row) = row else {
        for (target, members) in &pattern.defaults {
            let _ = event.set(target, json!(members));
        }
        return true;
    };

    for (target, member) in &pattern.writes {
        if let Some(held) = row.get(member).filter(|v| !v.is_null()) {
            let _ = event.set(target, held.clone());
        }
    }
    true
}

/// One column a row fans out, and where it lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowColumn {
    target: String,
    member: String,
    /// The script guards the write on the target still being unset --
    /// `if (m.containsKey('outcome') && ctx.event.outcome == null)`.
    only_if_unset: bool,
}

/// A row of a NAMED params table fanned onto ctx, with a whole set of literal
/// defaults where the key has no row.
///
/// ```painless
/// def action = ctx.event.action;
/// ctx.event.kind = 'event';
/// def m = params.exact.get(action);
/// if (m != null) {
///   ctx.event.category = new ArrayList(m.category);
///   ctx.event.type = new ArrayList(m.type);
///   if (m.containsKey('outcome') && ctx.event.outcome == null) {
///     ctx.event.outcome = m.outcome;
///   }
/// } else {
///   ctx.event.category = ['authentication'];
///   ctx.event.type = ['info'];
/// }
/// ```
///
/// The whole kolide package categorises this way. Without the pattern the
/// `!= null` catch-all claims the script and runs only the ELSE branch, so
/// every event with a row comes out carrying the fallback's `kind`,
/// `category` and `type`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowOrDefaults {
    /// The `ctx.` path the lookup key is read from.
    subject: String,
    /// The params member holding the table.
    table: String,
    /// Literal writes above the branch, which run whether or not the key has
    /// a row.
    prelude: Vec<(String, Value)>,
    columns: Vec<RowColumn>,
    defaults: Vec<(String, Value)>,
}

/// Whitespace and `//` comments, skipped.
pub(crate) fn skip_trivia(mut text: &str) -> &str {
    loop {
        text = text.trim_start();
        let Some(after) = text.strip_prefix("//") else {
            return text;
        };
        text = after.find('\n').map_or("", |at| &after[at + 1..]);
    }
}

/// The test and block of the `if (...) { ... }` at the head of `text`, plus
/// whatever follows it.
fn guard_and_block(text: &str) -> Option<(&str, &str, &str)> {
    let (guard, after) = balanced(skip_trivia(text), '(', ')')?;
    let (block, after) = balanced(skip_trivia(after), '{', '}')?;
    Some((guard, block, after))
}

/// Where the script branches on the row being absent, in either polarity.
fn null_branch_at(script: &str, row: &str) -> Option<usize> {
    script
        .find(&format!("if ({row} != null)"))
        .or_else(|| script.find(&format!("if ({row} == null)")))
}

/// A params row whose members are selected by INDEX, one field each.
///
/// ```painless
/// def settings = params.get(ctx.event.code);
/// if (settings != null) {
///   if (settings[0] != null) { ctx.event.type = settings[0]; }
///   if (settings[1] != null) { ctx.event.kind = settings[1]; }
///   if (settings[2] != null) { ctx.event.outcome = settings[2]; }
/// }
/// ```
///
/// `hpe_aruba_cx` classifies 1,871 events this way, and read as a plain
/// lookup-put the WHOLE row landed in the last field the script names --
/// `event.outcome` holding the type and the kind as well.
fn parse_indexed_row_columns(script: &str) -> Option<ParamsPattern> {
    let (head, rest) = script.split_once("= params.get(ctx.")?;
    let row = head
        .trim_end()
        .rsplit([' ', '\n'])
        .next()?
        .trim()
        .to_string();
    let subject = clean_path(rest.split_once(')')?.0.trim());
    if row.is_empty() || subject.is_empty() {
        return None;
    }

    // Each write names its index twice -- once in the guard, once in the value
    // -- and only a pair that agrees is this pattern.
    let mut columns = Vec::new();
    let marker = format!("{row}[");
    for (at, _) in rest.match_indices(&marker) {
        let after = &rest[at + marker.len()..];
        let Some((index, after)) = after.split_once(']') else {
            continue;
        };
        let Ok(index) = index.trim().parse::<usize>() else {
            continue;
        };
        let Some((assignment, _)) = after.split_once(&format!("= {row}[{index}]")) else {
            continue;
        };
        let Some(target) = assignment.rsplit_once("ctx.").map(|(_, path)| path.trim()) else {
            continue;
        };
        if target.is_empty() || target.contains(['(', ' ', '[']) {
            continue;
        }
        columns.push((index, clean_path(target)));
    }

    (!columns.is_empty()).then_some(ParamsPattern::IndexedRowColumns { subject, columns })
}

fn run_indexed_row_columns(
    event: &mut Event,
    subject: &str,
    columns: &[(usize, String)],
    params: &Map<String, Value>,
) -> bool {
    let Some(row) = event
        .get_as_string(subject)
        .and_then(|key| params.get(&key))
        .and_then(Value::as_array)
        .cloned()
    else {
        return true;
    };
    for (index, target) in columns {
        if let Some(held) = row.get(*index).filter(|v| !v.is_null()) {
            let _ = event.set(target, held.clone());
        }
    }
    true
}

/// Empty-string members dropped from each sub-map params names.
///
/// ```painless
/// for (key in params.keys) {
///   if (ctx['osquery']['result'][key] == null) { continue; }
///   def dict = new HashMap();
///   for (entry in ctx['osquery']['result'][key].entrySet()) {
///     if (entry.getValue() != '') { dict[entry.getKey()] = entry.getValue(); }
///   }
///   ctx['osquery']['result'][key] = dict;
/// }
/// ```
///
/// osquery ships one column per table field whether the row filled it or not,
/// so the empty ones are the difference between 666 matching events and 2,213.
fn parse_drop_empty_members(script: &str) -> Option<ParamsPattern> {
    // `for (key in params.keys)` names the loop variable and the params member.
    let (head, rest) = script.split_once(" in params.")?;
    let variable = head.rsplit(['(', ' ']).next()?.trim().to_string();
    let list = rest.split_once(')')?.0.trim().to_string();
    if variable.is_empty() || !list.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // The map the loop subscripts, with the loop variable cut off the end.
    let (head, _) = rest.split_once(&format!("[{variable}]"))?;
    let parent = clean_path(
        subject_path(head.rsplit_once("ctx")?.1)
            .trim_start_matches('.')
            .trim(),
    );
    (!parent.is_empty()).then_some(ParamsPattern::DropEmptyMembers { parent, list })
}

fn run_drop_empty_members(
    event: &mut Event,
    parent: &str,
    list: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some(keys) = params.get(list).and_then(Value::as_array) else {
        return false;
    };
    for key in keys.iter().filter_map(Value::as_str) {
        let path = format!("{parent}.{key}");
        let Some(Value::Object(members)) = event.get(&path) else {
            continue;
        };
        let kept: Map<String, Value> = members
            .iter()
            .filter(|(_, value)| value.as_str() != Some(""))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        if kept.len() != members.len() {
            let _ = event.set(&path, Value::Object(kept));
        }
    }
    true
}

/// The test and both blocks of the `if (...) { ... } else { ... }` at the head
/// of `text`, plus whatever follows it.
fn if_else_blocks(text: &str) -> Option<(&str, &str, &str, &str)> {
    let (guard, after) = balanced(skip_trivia(text), '(', ')')?;
    let (then, after) = balanced(skip_trivia(after), '{', '}')?;
    let tail = skip_trivia(after).strip_prefix("else")?;
    let (otherwise, after) = balanced(skip_trivia(tail), '{', '}')?;
    Some((guard, then, otherwise, after))
}

/// Every `ctx.<path> = <literal>;` statement in a block, in source order.
///
/// Statement-wise rather than a text scan, so an `if` nested in the block
/// contributes nothing -- audit's fallback derives its `event.type` from a
/// local the ladder above it sets, and reading that as a literal would write
/// the word `type`.
pub(crate) fn literal_writes(block: &str) -> Vec<(String, Value)> {
    let mut out = Vec::new();
    for statement in block.split(';') {
        let Some((subject, value)) = split_assignment(statement) else {
            continue;
        };
        // The target is the last line of the subject, so a `}` closing the
        // statement before it is not read as part of the path.
        let Some(path) = subject
            .rsplit(['\n', '{', '}'])
            .next()
            .map(str::trim)
            .and_then(|line| line.strip_prefix("ctx."))
        else {
            continue;
        };
        if let Some(literal) = literal_value(value.trim()) {
            out.push((clean_path(path), literal));
        }
    }
    out
}

/// The `if (<row>.containsKey('<member>')...) { ctx.<t> = <row>.<member>; }`
/// writes, and the block with those `if`s cut out.
///
/// Cutting them out is what stops the plain scan below reading the same
/// assignment a second time WITHOUT its guard.
fn guarded_columns(block: &str, row: &str) -> (Vec<RowColumn>, String) {
    let opens = format!("if ({row}.containsKey(");
    let mut columns = Vec::new();
    let mut plain = String::with_capacity(block.len());
    let mut rest = block;

    while let Some(at) = rest.find(&opens) {
        plain.push_str(&rest[..at]);
        let head = &rest[at + "if".len()..];
        let Some((test, after)) = balanced(skip_trivia(head), '(', ')') else {
            break;
        };
        let Some((body, after)) = balanced(skip_trivia(after), '{', '}') else {
            break;
        };
        if let Some(member) = quoted_after(test, "")
            && let Some(target) = body
                .split_once(&format!("= {row}.{member}"))
                .and_then(|(head, _)| head.rsplit_once("ctx."))
                .map(|(_, path)| clean_path(path.trim()))
        {
            columns.push(RowColumn {
                target,
                member,
                only_if_unset: test.contains("== null"),
            });
        }
        rest = after;
    }

    plain.push_str(rest);
    (columns, plain)
}

/// Read the key, the table and both branches off the script.
fn parse_row_or_defaults(script: &str) -> Option<RowOrDefaults> {
    // Three spellings of the same lookup: bound to a local and null-tested,
    // bound by BRACKET subscript and null-tested, or tested with `containsKey`
    // and subscripted inside the branch.
    let (table, key, branch_at, row) = match script.split_once(".get(") {
        Some((head, tail)) if head.contains("params.") => {
            let table = head.rsplit_once("params.")?.1.trim().to_string();
            let key = tail.split_once(')')?.0.trim().to_string();
            let row = head.rsplit_once(" = ")?.0.rsplit(' ').next()?.to_string();
            let at = null_branch_at(script, &row)?;
            (table, key, at, row)
        }
        _ if script.contains("params.") && !script.contains("if (params.") => {
            let (head, tail) = script.split_once("params.")?;
            let (table, tail) = tail.split_once('[')?;
            let (key, _) = tail.split_once(']')?;
            let row = head.rsplit_once(" = ")?.0.rsplit(' ').next()?.to_string();
            let at = null_branch_at(script, &row)?;
            (table.trim().to_string(), key.trim().to_string(), at, row)
        }
        _ => {
            let at = script.find("if (params.")?;
            let head = &script[at..];
            let table = head.split_once("params.")?.1.split_once(".containsKey(")?.0;
            let key = head
                .split_once(".containsKey(")?
                .1
                .split_once(')')?
                .0
                .trim();
            let row = head
                .split_once(&format!("params.{table}[{key}]"))?
                .0
                .rsplit_once(" = ")?
                .0
                .rsplit(' ')
                .next()?
                .to_string();
            (table.to_string(), key.to_string(), at, row)
        }
    };

    // A dotted table is a nested lookup this pattern cannot resolve, and taking
    // it would write the defaults over a key that HAS a row.
    if !table.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // The key is a local bound to a `ctx.` path, or the path written inline.
    let subject = match key.strip_prefix("ctx.") {
        Some(path) => clean_path(path),
        None => crate::painless_common::ctx_path_bound_to(script, &key)?,
    };
    let branch = &script[branch_at + "if".len()..];
    let (then, otherwise, after) = if let Some((_, then, otherwise, after)) = if_else_blocks(branch)
    {
        (then, otherwise, after)
    } else {
        // `if (<row> == null) { return; }` and then the body: the same pattern
        // spelled as an early return, with nothing written for a miss.
        let (_, block, rest) = guard_and_block(branch)?;
        if !block.contains("return") {
            return None;
        }
        (rest, "", "")
    };
    // The branch has to be the END of the script: kolide's issues stream adds
    // a second lookup and a ternary after it, and claiming that text would
    // silently drop both.
    if !skip_trivia(after).is_empty() {
        return None;
    }

    let (mut columns, plain) = guarded_columns(then, &row);
    // Every conditional in the branch has to have been one of those columns,
    // or the scan below reads a guarded write without its guard.
    if plain.contains("if (") {
        return None;
    }

    let member_of = format!("{row}.");
    for (at, _) in plain.match_indices(&member_of) {
        let head = plain[..at].trim_end();
        let head = head
            .strip_suffix("new ArrayList(")
            .unwrap_or(head)
            .trim_end();
        let Some(head) = head.strip_suffix('=').filter(|h| !h.ends_with(['=', '!'])) else {
            continue;
        };
        let Some((_, target)) = head.rsplit_once("ctx.") else {
            continue;
        };
        let member: String = plain[at + member_of.len()..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if member.is_empty() {
            continue;
        }
        columns.push(RowColumn {
            target: clean_path(target.trim()),
            member,
            only_if_unset: false,
        });
    }
    if columns.is_empty() {
        return None;
    }

    Some(RowOrDefaults {
        subject,
        table,
        prelude: literal_writes(&script[..branch_at]),
        columns,
        defaults: literal_writes(otherwise),
    })
}

/// Fan the selected row's columns out, or write the defaults.
fn run_row_or_defaults(
    event: &mut Event,
    pattern: &RowOrDefaults,
    params: &Map<String, Value>,
) -> bool {
    for (target, value) in &pattern.prelude {
        let _ = event.set(target, value.clone());
    }

    let row = event
        .get_as_string(&pattern.subject)
        .and_then(|key| params.get(&pattern.table)?.get(&key).cloned());
    let Some(row) = row else {
        for (target, value) in &pattern.defaults {
            let _ = event.set(target, value.clone());
        }
        return true;
    };

    for column in &pattern.columns {
        if column.only_if_unset && event.has_value(&column.target) {
            continue;
        }
        if let Some(held) = row.get(&column.member).filter(|v| !v.is_null()) {
            let _ = event.set(&column.target, held.clone());
        }
    }
    true
}

/// A row of a NAMED params table merged WHOLE onto ctx, with further params
/// rows standing in where the key has none.
///
/// ```painless
/// def action = ctx.json.event_type;
/// ctx.event.action = action;
/// ctx.event.kind = 'event';
/// def mapping = params.mappings.get(action);
/// if (mapping == null && action.startsWith('bugbot_')) {
///   mapping = params.bugbot;
/// }
/// mapping = mapping ?: params.defaults;
/// def hm = new HashMap(mapping);
/// hm.forEach((k, v) -> ctx.event[k] = v);
/// ```
///
/// [`RowOrDefaults`] is the same lookup fanned COLUMN BY COLUMN with literal
/// fallbacks; this one names its columns nowhere and its fallbacks are rows of
/// the same table. cursor's audit stream is the whole of it, and the five
/// `event.*` fields it writes gate almost every processor after it -- the
/// per-action renames, the lowercase, `user.target.email` and through that
/// `related.user` -- so an unclaimed script here cost the source all 30 events
/// rather than five fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeRowOrFallback {
    /// The `ctx.` path the lookup key is read from.
    subject: String,
    /// The params member holding the table.
    table: String,
    /// Where the row's columns land.
    target: String,
    /// Literal writes the script makes on its own account.
    prelude: Vec<(String, Value)>,
    /// Fields written the KEY's own value, which a literal scan cannot see
    /// because the right-hand side is the local the key was bound to.
    key_writes: Vec<String>,
    /// A params row for a key that missed but carries this prefix.
    prefix_row: Option<(String, String)>,
    /// A params row for a key that missed outright.
    default_row: Option<String>,
}

fn parse_merge_row_or_fallback(script: &str) -> Option<MergeRowOrFallback> {
    // The merge names the target, and its lambda parameter is fixed by the
    // trigger below.
    let (head, body) = script.split_once("forEach(")?;
    let target = ctx_path_between(body, "ctx.", "[k] = ")?;
    if target.is_empty() {
        return None;
    }

    // A NAMED table only. The unnamed `params.get(` form is `LookupMerge`,
    // which claims it above this.
    let (before, after) = head.split_once(".get(")?;
    let table = before.rsplit_once("params.")?.1.trim().to_string();
    if table.is_empty() || !table.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let key = after.split_once(')')?.0.trim();
    let subject = crate::painless_common::ctx_path_bound_to(head, key)?;
    let row = before.rsplit_once(" = ")?.0.rsplit(' ').next()?;
    if row.is_empty() {
        return None;
    }

    // `if (<row> == null && <key>.startsWith('<prefix>')) { <row> = params.<name>; }`
    let prefix_row = head
        .split_once(&format!("{key}.startsWith("))
        .and_then(|(_, tail)| {
            let prefix = quoted_after(tail, "")?;
            let assigned = tail.split_once(&format!("{row} = params."))?.1;
            let name: String = assigned
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            (!name.is_empty()).then_some((prefix, name))
        });

    // `<row> = <row> ?: params.<name>;` -- the last resort.
    let default_row = head.split_once("?: params.").and_then(|(_, tail)| {
        let name: String = tail
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        (!name.is_empty()).then_some(name)
    });

    Some(MergeRowOrFallback {
        subject,
        table,
        target,
        prelude: literal_writes(head),
        key_writes: key_written_paths(head, key),
        prefix_row,
        default_row,
    })
}

/// The `ctx.<path> = <key>;` targets, which [`literal_writes`] cannot see.
fn key_written_paths(head: &str, key: &str) -> Vec<String> {
    let mut out = Vec::new();
    for statement in head.split(';') {
        let Some((subject, value)) = split_assignment(statement) else {
            continue;
        };
        if value.trim() != key {
            continue;
        }
        if let Some(path) = subject
            .rsplit(['\n', '{', '}'])
            .next()
            .map(str::trim)
            .and_then(|line| line.strip_prefix("ctx."))
        {
            out.push(clean_path(path));
        }
    }
    out
}

/// Merge the row the key selects, or the first fallback row that applies.
fn run_merge_row_or_fallback(
    event: &mut Event,
    pattern: &MergeRowOrFallback,
    params: &Map<String, Value>,
) -> bool {
    // Absent is the processor's own `if` guard, and every one of these scripts
    // opens by reading the key.
    let Some(key) = event.get_as_string(&pattern.subject) else {
        return true;
    };
    for path in &pattern.key_writes {
        let _ = event.set(path, Value::String(key.clone()));
    }
    for (target, value) in &pattern.prelude {
        let _ = event.set(target, value.clone());
    }

    let row = params
        .get(&pattern.table)
        .and_then(|table| table.get(&key))
        .or_else(|| {
            let (prefix, name) = pattern.prefix_row.as_ref()?;
            key.starts_with(prefix.as_str()).then(|| params.get(name))?
        })
        .or_else(|| params.get(pattern.default_row.as_ref()?));
    let Some(Value::Object(row)) = row else {
        return true;
    };

    for (member, value) in row.clone() {
        let _ = event.set(&format!("{}.{member}", pattern.target), value);
    }
    true
}

/// Read an if/else-if ladder that rewrites ONE field, both sides named by
/// params members:
///
/// ```text
/// if (ctx.<p> == params.<a1>) { ctx.<p> = params.<b1>; }
/// else if (ctx.<p> == params.<a2>) { ctx.<p> = params.<b2>; } ...
/// ```
///
/// The vendor keeps the abbreviations and their expansions in params so the
/// table is editable without touching the script. Nothing about it is
/// source-specific, but every arm must name the SAME field -- a ladder that
/// switches fields half way is a different pattern and is declined here rather
/// than half-run.
fn parse_param_rename_ladder(script: &str) -> Option<ParamsPattern> {
    // Nothing may precede the ladder. Running the arms of a script that also
    // does something else writes the vendor's value while skipping its work.
    let mut chunks = script.split("if (");
    if !chunks.next()?.trim().is_empty() {
        return None;
    }

    let mut path: Option<String> = None;
    let mut arms: Vec<(String, String)> = Vec::new();
    for chunk in chunks {
        let (condition, body) = chunk.split_once(')')?;
        let (subject, compare) = condition.split_once("== params.")?;
        let subject = clean_path(subject.trim().strip_prefix("ctx.")?);
        let from = leading_name(compare);

        let (assigned, to) = body.split_once("= params.")?;
        let written = clean_path(assigned.rsplit_once("ctx.")?.1.trim());
        let to = leading_name(to);

        if subject != written
            || from.is_empty()
            || to.is_empty()
            || *path.get_or_insert_with(|| subject.clone()) != subject
        {
            return None;
        }
        arms.push((from, to));
    }

    // One arm is an `if`, not a ladder, and is claimed by the patterns that read
    // a single guarded write.
    let path = path.filter(|_| arms.len() > 1)?;
    Some(ParamsPattern::ParamRenameLadder { path, arms })
}

/// The identifier a params reference starts with.
fn leading_name(text: &str) -> String {
    text.chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

fn run_param_rename_ladder(
    event: &mut Event,
    path: &str,
    arms: &[(String, String)],
    params: &Map<String, Value>,
) -> bool {
    let Some(held) = event.get(path).cloned() else {
        return true;
    };
    for (from, to) in arms {
        // Painless compares the VALUES, so a params member the block does not
        // carry compares equal to nothing and the arm is simply skipped.
        if params.get(from) != Some(&held) {
            continue;
        }
        if let Some(replacement) = params.get(to) {
            let _ = event.set(path, replacement.clone());
        }
        return true;
    }
    true
}

/// Read the sub-map prune: which map is reshuffled, and under which member the
/// unlisted fields land.
///
/// Every name in the script is a LOCAL, so all six are read off the text rather
/// than assumed. The pattern is:
///
/// ```text
/// Map <map> = ctx.<subject>;
/// params.entrySet().stream().filter(e -> <map>.containsKey(e.getKey())).forEach(<lst> -> {
///   Map <base> = <map>[<lst>.getKey()], <sel> = new HashMap();
///   <lst>.getValue().stream().filter(f -> <base>.containsKey(f)).forEach(f -> {
///     <sel>[f] = <base>.remove(f);
///   });
///   <sel>['<rest>'] = <base>;
///   <map>[<lst>.getKey()] = <sel>;
/// });
/// ```
fn parse_select_members(script: &str) -> Option<ParamsPattern> {
    let (head, body) = script.split_once("params.entrySet()")?;
    let subject = ctx_path_before(head, ";")?;
    let map = head
        .rsplit_once(" = ")?
        .0
        .rsplit(char::is_whitespace)
        .next()?;
    if subject.is_empty() || !is_local_name(map) {
        return None;
    }

    // The filter must gate on the SAME local, or the script is walking some
    // other map and the pattern below means nothing.
    if !body.contains(&format!("{map}.containsKey(")) {
        return None;
    }
    let lst = body
        .split_once(".forEach(")?
        .1
        .split_once("->")?
        .0
        .trim()
        .trim_start_matches("def ")
        .trim();
    if !is_local_name(lst) {
        return None;
    }

    let row = format!("{map}[{lst}.getKey()]");
    let base = body
        .split_once(&format!("{row},"))?
        .0
        .rsplit_once(" = ")?
        .0
        .rsplit(char::is_whitespace)
        .next()?;
    let sel = body
        .split_once(" = new HashMap()")?
        .0
        .rsplit(char::is_whitespace)
        .next()?;
    if !is_local_name(base) || !is_local_name(sel) {
        return None;
    }

    // The move itself, then the write-back. Without both, the script is
    // reading the sub-map rather than replacing it.
    if !body.contains(&format!("{sel}[")) || !body.contains(&format!("{base}.remove(")) {
        return None;
    }
    if !body.contains(&format!("{row} = {sel};")) {
        return None;
    }

    // `<sel>['<rest>'] = <base>;` -- the semicolon is what separates it from
    // the `<sel>[f] = <base>.remove(f);` above.
    let assign = format!("] = {base};");
    let at = body.find(&assign)?;
    let subscript = &body[..at];
    let open = subscript.rfind('[')?;
    let rest = unquote(&subscript[open + 1..]);
    if rest.is_empty() || !subscript[..open].trim_end().ends_with(sel) {
        return None;
    }

    Some(ParamsPattern::SelectMembers { subject, rest })
}

/// A bare Painless identifier -- what every local in these scripts is.
fn is_local_name(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn run_select_members(
    event: &mut Event,
    subject: &str,
    rest: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some(held) = event.get_object(subject) else {
        return true;
    };

    // Built first, because `get_object` borrows the event for as long as the
    // sub-maps are read. Only the keys params names are touched, so the clone
    // is a handful of small maps rather than the document.
    let mut rebuilt: Vec<(String, Value)> = Vec::new();
    for (key, listed) in params {
        let (Some(Value::Object(base)), Some(listed)) = (held.get(key), listed.as_array()) else {
            continue;
        };
        // Painless streams the LIST, so the kept members come out in the
        // params order and not the sub-map's.
        let mut left = base.clone();
        let mut selected = Map::new();
        for name in listed.iter().filter_map(Value::as_str) {
            // shift_remove, never remove: under `preserve_order` the swapping
            // variant drops the LAST key into the freed slot.
            if let Some(value) = left.shift_remove(name) {
                selected.insert(name.to_string(), value);
            }
        }
        // Unconditional, exactly as the script writes it -- a sub-map with
        // nothing left over still gets an empty `rest`.
        selected.insert(rest.to_string(), Value::Object(left));
        rebuilt.push((key.clone(), Value::Object(selected)));
    }

    for (key, value) in rebuilt {
        let _ = event.set(&format!("{subject}.{key}"), value);
    }
    true
}

/// Keys the vendor ships as text that the rename typed on the way through.
const MSG_PART_LONGS: [&str; 2] = ["detected_size_bytes", "size_decoded_bytes"];
const MSG_PART_BOOLS: [&str; 7] = [
    "is_archive",
    "is_corrupted",
    "is_deleted",
    "is_protected",
    "is_timed_out",
    "is_virtual",
    "is_rewritten",
];

/// One message part with its keys renamed through the params map.
///
/// Recursive, because a part carries nested maps and lists of maps. A key the
/// map does not name is carried as it stands, and an EMPTY key becomes
/// `MISSING_KEY` -- the vendor's own placeholder, not ours.
fn rename_msg_part(value: &Value, key_map: &Map<String, Value>) -> Value {
    match value {
        Value::Object(entries) => {
            let mut renamed = Map::new();
            for (key, held) in entries {
                let mapped = key_map.get(key).and_then(Value::as_str);
                let name = match (mapped, key.is_empty()) {
                    (Some(mapped), _) => mapped.to_string(),
                    (None, true) => "MISSING_KEY".to_string(),
                    (None, false) => key.clone(),
                };
                let converted = match held {
                    Value::Object(_) | Value::Array(_) => rename_msg_part(held, key_map),
                    scalar => match mapped {
                        Some(name) if MSG_PART_LONGS.contains(&name) => scalar
                            .as_i64()
                            .or_else(|| scalar.as_str().and_then(|t| t.trim().parse().ok()))
                            .map_or_else(|| scalar.clone(), |number| json!(number)),
                        // `Boolean.parseBoolean` is true only for the literal
                        // "true", case-insensitively, and false for everything
                        // else including nonsense.
                        Some(name) if MSG_PART_BOOLS.contains(&name) => match scalar {
                            Value::String(text) => json!(text.eq_ignore_ascii_case("true")),
                            already => already.clone(),
                        },
                        _ => scalar.clone(),
                    },
                };
                renamed.insert(name, converted);
            }
            Value::Object(renamed)
        }
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| match item {
                    Value::Object(_) => rename_msg_part(item, key_map),
                    other => other.clone(),
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

/// proofpoint's message parts, and the four lists they feed.
///
/// Each part is renamed through the params key map, then fans out: the part
/// itself, both its hashes into `related.hash`, every URL it carries into
/// `url.full`, and an attachment record into `email.attachments`. The lists
/// are created empty even when the loop adds nothing, which is what the
/// script's own `put(..., new ArrayList())` does before it.
fn run_msg_parts(event: &mut Event, params: &Map<String, Value>) -> bool {
    let Some(Value::Array(parts)) = event.get("json.msgParts").cloned() else {
        return true;
    };

    let mut renamed_parts = Vec::with_capacity(parts.len());
    let mut hashes = Vec::new();
    let mut urls = Vec::new();
    let mut attachments = Vec::new();

    for part in &parts {
        let renamed = rename_msg_part(part, params);

        for name in ["sha256", "md5"] {
            if let Some(hash) = renamed.get(name).filter(|v| !v.is_null()) {
                hashes.push(hash.clone());
            }
        }
        if let Some(Value::Array(part_urls)) = renamed.get("urls") {
            for url in part_urls {
                if let Some(full) = url.get("url").filter(|v| !v.is_null()) {
                    urls.push(full.clone());
                }
            }
        }

        let mut file = Map::new();
        for (target, source) in [
            ("name", "detected_name"),
            ("extension", "detected_ext"),
            ("mime_type", "detected_mime"),
            ("size", "detected_size_bytes"),
        ] {
            if let Some(held) = renamed.get(source).filter(|v| !v.is_null()) {
                file.insert(target.to_string(), held.clone());
            }
        }
        let mut hash = Map::new();
        for name in ["md5", "sha256"] {
            if let Some(held) = renamed.get(name).filter(|v| !v.is_null()) {
                hash.insert(name.to_string(), held.clone());
            }
        }
        if !hash.is_empty() {
            file.insert("hash".to_string(), Value::Object(hash));
        }
        attachments.push(json!({ "file": Value::Object(file) }));

        renamed_parts.push(renamed);
    }

    let _ = event.set(
        "proofpoint_on_demand.message.msg_parts",
        Value::Array(renamed_parts),
    );
    let _ = event.set("related.hash", Value::Array(hashes));
    let _ = event.set("url.full", Value::Array(urls));
    let _ = event.set("email.attachments", Value::Array(attachments));
    true
}

/// m365's `event.category` and `event.type` off the alert evidence list.
///
/// Each entry's `@odata.type` is looked up in the params table; a hit adds its
/// category and then picks a type from what the set holds SO FAR -- registry
/// wins over threat, threat over everything else -- which is the script's own
/// order and not a per-entry decision. `determination` is folded in the same
/// way afterwards. Both come out deduplicated and sorted.
fn run_evidence_categories(
    event: &mut Event,
    source: &str,
    key: &str,
    params: &Map<String, Value>,
) -> bool {
    let mut categories: Vec<String> = Vec::new();
    let mut types: Vec<String> = Vec::new();

    let classify = |categories: &mut Vec<String>, types: &mut Vec<String>, mapping: &str| {
        if !categories.iter().any(|held| held == mapping) {
            categories.push(mapping.to_string());
        }
        let kind = if categories.iter().any(|held| held == "registry") {
            "access"
        } else if categories.iter().any(|held| held == "threat") {
            "indicator"
        } else {
            "info"
        };
        if !types.iter().any(|held| held == kind) {
            types.push(kind.to_string());
        }
    };

    if let Some(Value::Array(evidence)) = event.get(source).cloned() {
        for entry in &evidence {
            let Some(mapping) = entry
                .get(key)
                .and_then(Value::as_str)
                .and_then(|kind| params.get(kind))
                .and_then(Value::as_str)
            else {
                continue;
            };
            classify(&mut categories, &mut types, mapping);
        }
    }

    // `determination` has no registry arm of its own; the script only asks
    // whether the set already holds `threat`.
    if let Some(mapping) = event
        .get_str("json.determination")
        .map(str::to_lowercase)
        .and_then(|held| params.get(&held).cloned())
        .as_ref()
        .and_then(Value::as_str)
    {
        if !categories.iter().any(|held| held == mapping) {
            categories.push(mapping.to_string());
        }
        let kind = if categories.iter().any(|held| held == "threat") {
            "indicator"
        } else {
            "info"
        };
        if !types.iter().any(|held| held == kind) {
            types.push(kind.to_string());
        }
    }

    for (path, mut values) in [("event.type", types), ("event.category", categories)] {
        if values.is_empty() {
            continue;
        }
        values.sort_unstable();
        let _ = event.set(path, json!(values));
    }
    true
}

fn try_threat_indicator_type(event: &mut Event, source: &str, params: &Map<String, Value>) -> bool {
    let Some(Value::Array(indicators)) = event.get(source).cloned() else {
        return true;
    };

    for indicator in &indicators {
        let Some(kind) = indicator.get("type").and_then(Value::as_str) else {
            continue;
        };
        let Some(mapped) = params.get(kind).and_then(Value::as_str) else {
            continue;
        };
        let _ = event.set("threat.indicator.type", json!(mapped));
        if mapped != "file" {
            continue;
        }
        // `HASH_MD5` -> `md5`. A type with no second token would throw in
        // Painless, and the processor ignores its own failures, so skip it.
        let Some(name) = kind.split('_').nth(1) else {
            continue;
        };
        let Some(value) = indicator.get("value") else {
            continue;
        };
        let mut hash = Map::new();
        hash.insert(name.to_lowercase(), value.clone());
        let _ = event.append(
            "threat.enrichments",
            json!({ "indicator": { "file": { "hash": hash } } }),
        );
    }
    true
}

/// powershell's `parseRawDetail`: one structured map per raw detail line.
///
/// `<type>(<related command>): <value>`, and a `ParameterBinding` type splits
/// its value again into `name=<n>; value=<v>`. A line that fits neither pattern
/// keeps its whole text under `value`, which is the script's own fallback.
fn try_invocation_details(event: &mut Event, params: &Map<String, Value>) -> bool {
    // Site-local cells: the two patterns are literals, so they compile once
    // per process and never touch a shared map on the hot path.
    #[allow(
        clippy::expect_used,
        reason = "a literal pattern either compiles for every input or for none, so a failure here is a build-time bug the suite catches, not a runtime one"
    )]
    static DETAIL: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"^([^(]+)\(([^)]+)\):\s*(.+)$").expect("a literal pattern compiles")
    });
    #[allow(
        clippy::expect_used,
        reason = "a literal pattern either compiles for every input or for none, so a failure here is a build-time bug the suite catches, not a runtime one"
    )]
    static BINDING: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"^name=(.+);\s*value=(.+)$").expect("a literal pattern compiles")
    });

    let Some(field) = params.get("field").and_then(Value::as_str) else {
        return true;
    };
    let Some(Value::Array(values)) = event.get(&format!("winlog.event_data.{field}")).cloned()
    else {
        return true;
    };

    let mut details: Vec<Value> = match event.get("_temp.details") {
        Some(Value::Array(existing)) => existing.clone(),
        _ => Vec::new(),
    };
    for raw in values.iter().filter_map(Value::as_str) {
        let Some(caps) = DETAIL.captures(raw) else {
            details.push(json!({ "value": raw }));
            continue;
        };
        let (kind, command, value) = (&caps[1], &caps[2], &caps[3]);
        if kind != "ParameterBinding" {
            details.push(json!({
                "type": kind,
                "related_command": command,
                "value": value,
            }));
            continue;
        }
        match BINDING.captures(value) {
            Some(pair) => details.push(json!({
                "type": kind,
                "related_command": command,
                "name": &pair[1],
                "value": &pair[2],
            })),
            None => details.push(json!({ "value": value })),
        }
    }

    if !details.is_empty() {
        let _ = event.set("_temp.details", Value::Array(details));
    }
    true
}

fn try_mimecast_log_type(event: &mut Event, params: &Map<String, Value>) -> bool {
    use crate::painless_helpers::{java_bucket, java_table_size};

    let Some(Value::Object(mimecast)) = event.get("mimecast") else {
        return true;
    };
    let table = |name: &str| params.get(name).and_then(Value::as_object);
    let (Some(definite), Some(negative), Some(positive), Some(candidates)) = (
        table("definite_positive"),
        table("negative"),
        table("positive"),
        table("candidates"),
    ) else {
        return true;
    };

    // The event's keys, lowercased into a HashSet and walked in ITS hash
    // order -- insertion order is the mimecast map's own bucket walk.
    let mut keys: Vec<String> = {
        let map_table = java_table_size(mimecast.len());
        let mut walked: Vec<(usize, usize, String)> = mimecast
            .keys()
            .enumerate()
            .map(|(position, key)| (java_bucket(key, map_table), position, key.to_lowercase()))
            .collect();
        walked.sort_by_key(|(bucket, position, _)| (*bucket, *position));
        let mut seen = Vec::new();
        for (_, _, key) in walked {
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        seen
    };
    let set_table = java_table_size(keys.len());
    let mut ordered: Vec<(usize, usize, String)> = keys
        .drain(..)
        .enumerate()
        .map(|(position, key)| (java_bucket(&key, set_table), position, key))
        .collect();
    ordered.sort_by_key(|(bucket, position, _)| (*bucket, *position));
    let keys: Vec<String> = ordered.into_iter().map(|(_, _, key)| key).collect();

    for key in &keys {
        if let Some(kind) = definite.get(key).and_then(Value::as_str) {
            let _ = event.set("mimecast.log_type", json!(kind));
            return true;
        }
    }

    // Elimination: candidates in the params table's own hash order.
    let candidate_table = java_table_size(candidates.len());
    let mut score: Vec<(usize, usize, String, i64)> = candidates
        .keys()
        .enumerate()
        .map(|(position, name)| {
            (
                java_bucket(name, candidate_table),
                position,
                name.clone(),
                0,
            )
        })
        .collect();
    score.sort_by_key(|(bucket, position, ..)| (*bucket, *position));

    for key in &keys {
        if let Some(Value::Array(kinds)) = negative.get(key) {
            for kind in kinds.iter().filter_map(Value::as_str) {
                score.retain(|(_, _, name, _)| name != kind);
            }
        }
    }
    if score.len() == 1 {
        let _ = event.set("mimecast.log_type", json!(score[0].2));
        return true;
    }

    let mut max = 0i64;
    for key in &keys {
        if let Some(Value::Array(kinds)) = positive.get(key) {
            for kind in kinds.iter().filter_map(Value::as_str) {
                if let Some(entry) = score.iter_mut().find(|(_, _, name, _)| name == kind) {
                    entry.3 += 1;
                    max = max.max(entry.3);
                }
            }
        }
    }
    score.retain(|(_, _, _, points)| *points >= max);

    let winners: Vec<Value> = score
        .into_iter()
        .map(|(_, _, name, _)| json!(name))
        .collect();
    let _ = event.set("mimecast.log_type", Value::Array(winners));
    true
}

/// The ECS event block a params table names, keyed on one field.
///
/// stormshield keys `event.kind`, `event.category` and `event.type` off its
/// `logtype`, falls back to a bare `kind` when the table has no entry, and
/// then appends one more type for a subset of log types whose `event.action`
/// names it. All 44 of its events carry the block, and none of it was written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EventBlockTable {
    subject: String,
    target: String,
    table: String,
    default_kind: String,
    /// The subjects that take the second lookup, and the table it reads. Both
    /// or neither -- a script without the append writes only the block.
    action_subjects: Option<String>,
    action_types: Option<String>,
    action_field: String,
}

/// `def entry = params.<table>[<local>]; ... ctx.<target>.kind = entry.kind;`
fn parse_event_block_table(script: &str) -> Option<EventBlockTable> {
    // The lookup names the table and the local the subject was read into.
    let (table, rest) = script.split_once("= params.")?.1.split_once('[')?;
    let table = table.trim();
    let (local, _) = rest.split_once(']')?;
    let local = local.trim();
    if table.is_empty() || local.is_empty() || local.starts_with("ctx.") {
        return None;
    }
    let subject = crate::painless_common::ctx_path_bound_to(script, local)?;

    // The miss arm names the target and the kind written without an entry.
    let (head, tail) = script.split_once("== null)")?.1.split_once(".kind = ")?;
    let target = clean_path(
        head.rsplit("ctx.")
            .next()?
            .trim()
            .trim_start_matches('{')
            .trim(),
    );
    let default_kind = tail
        .trim()
        .trim_start_matches(['\'', '"'])
        .split(['\'', '"'])
        .next()?
        .to_owned();
    if target.is_empty() || default_kind.is_empty() {
        return None;
    }
    // Only claim the script that writes the WHOLE block; a kind on its own is
    // an ordinary lookup and has its own arms.
    if !script.contains(".category = ") || !script.contains(".type = ") {
        return None;
    }

    // The optional second lookup: both halves or neither.
    let action_subjects = script
        .split_once("params.")
        .and_then(|(_, rest)| rest.split_once(&format!(".contains({local})")))
        .and_then(|(head, _)| head.rsplit("params.").next())
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty() && !name.contains(char::is_whitespace));
    let action = script
        .split_once(".toLowerCase()]")
        .and_then(|(head, _)| head.rsplit_once("params.")?.1.split_once('['))
        .map(|(table, field)| {
            let field = field
                .trim()
                .trim_start_matches("ctx?.")
                .trim_start_matches("ctx.");
            (table.trim().to_owned(), clean_path(field))
        });

    let (action_types, action_field) = match action {
        Some((table, field)) if !table.is_empty() && !field.is_empty() => (Some(table), field),
        _ => (None, String::new()),
    };
    // A subject list with no table to read is not this pattern.
    let action_subjects = action_types.as_ref().and(action_subjects);

    Some(EventBlockTable {
        subject,
        target,
        table: table.to_owned(),
        default_kind,
        action_subjects,
        action_types,
        action_field,
    })
}

/// The record's ECS action, chosen by which candidate's fields are PRESENT.
///
/// auditd keys a params table on the record type, falls back to a second table
/// keyed on the syscall and then to a `'*'` catch-all, and every entry is a
/// LIST of candidates rather than one. The winner is the first whose
/// `has_fields` are all present on the record -- `AVC` is `violated-selinux-policy`
/// when the record carries `seresult` and `violated-apparmor-policy` when it
/// carries `apparmor`. The chosen entry writes the ECS event block, and stages
/// a list of `{target, value}` pairs that a later `foreach` renders into
/// fields. All 83 of auditd's events go through it, and none of it was written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecordActionTable {
    /// The container the candidates' fields are read from.
    base: String,
    /// The field keying the primary table.
    subject: String,
    primary: String,
    /// The second table, and the field keying it. Both or neither.
    fallback: Option<String>,
    fallback_subject: String,
    /// Where the chosen entry's event block is merged.
    target: String,
    /// Where the rendered pairs are staged for the `foreach` that follows.
    copy_field: String,
}

/// `def acts = params.<table>.get(<local>.<field>); ... hasFields(<local>, acts[i]["has_fields"])`
fn parse_record_action_table(script: &str) -> Option<RecordActionTable> {
    // The CALL site names the caller's local; the signature names its own
    // parameter, so read the last occurrence rather than the first.
    let local = script
        .rsplit_once("hasFields(")?
        .1
        .split_once(',')?
        .0
        .trim()
        .to_owned();
    if local.is_empty() || local.contains(char::is_whitespace) {
        return None;
    }
    let base = crate::painless_common::ctx_path_bound_to(script, &local)?;

    // The primary lookup names its table and the field keying it.
    let (primary, rest) = script.split_once("= params.")?.1.split_once(".get(")?;
    let primary = primary.trim();
    let subject = subject_under(rest.split_once(')')?.0, &local, &base)?;
    if primary.is_empty() {
        return None;
    }

    // The fallback: a second table, keyed on another field, with a wildcard
    // entry when that misses. All three or none -- a single-table script is an
    // ordinary lookup and has its own arms.
    let fallback = rest
        .split_once("= params.")
        .and_then(|(_, tail)| tail.split_once(".get("))
        .filter(|_| script.contains(".get('*')") || script.contains(".get(\"*\")"))
        .and_then(|(table, tail)| {
            let subject = subject_under(tail.split_once(')')?.0, &local, &base)?;
            Some((table.trim().to_owned(), subject))
        });
    let (fallback, fallback_subject) = match fallback {
        Some((table, subject)) if !table.is_empty() => (Some(table), subject),
        _ => (None, String::new()),
    };

    // The merge target, off the lambda that writes the block.
    let target = clean_path(script.split_once("-> ctx.")?.1.split_once('[')?.0.trim());

    // The staged list -- its container and the key it is written under.
    let (head, _) = script.split_once("] = lst")?;
    let (path, key) = head.rsplit("ctx.").next()?.trim().split_once('[')?;
    let key = key.trim().trim_matches(['\'', '"']);
    if target.is_empty() || path.is_empty() || key.is_empty() {
        return None;
    }
    let copy_field = format!("{}.{key}", clean_path(path.trim()));

    Some(RecordActionTable {
        base,
        subject,
        primary: primary.to_owned(),
        fallback,
        fallback_subject,
        target,
        copy_field,
    })
}

/// `base.record_type` / `base?.syscall` -> the full path of that field.
fn subject_under(argument: &str, local: &str, base: &str) -> Option<String> {
    let field = argument
        .trim()
        .strip_prefix(local)?
        .trim_start_matches('?')
        .strip_prefix('.')?
        .trim();
    if field.is_empty() || field.contains(['(', ' ']) {
        return None;
    }
    Some(format!("{base}.{field}"))
}

/// Every value of a map, normalised in place.
///
/// auditd's records arrive as the daemon wrote them: values quoted, some
/// hex-encoded, some spelling a boolean as `yes`, and absent ones written as
/// `?` or `(null)`. One script walks the whole map and fixes all of it, so
/// leaving it unbound leaves a quote on `process.executable`, `process.name`,
/// `user.terminal` and `auditd.log.hostname` in every record that carries one.
///
/// The hex decode has a guard worth stating, because it reads as a bug and is
/// not one: a decoded value is kept only if it needed the encoding -- if every
/// byte lands above `"` and below DEL, the ORIGINAL hex is returned. A path
/// like `/home` survives as `2F686F6D65`, and only a value with a space, a
/// quote or a control character is actually decoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormaliseMapValues {
    /// The map whose values are rewritten.
    container: String,
    /// The values that mean absent, and remove the key.
    placeholders: Vec<String>,
    /// The params lists naming the keys that take each conversion.
    hex_keys: String,
    boolean_keys: String,
    /// The one key rewritten wholesale: its name, the value, the replacement.
    rewrite: Option<(String, String, String)>,
}

/// `trimQuotes(...)` over `audit.entrySet().iterator()`, driven by params.
fn parse_normalise_map_values(script: &str) -> Option<NormaliseMapValues> {
    // `def audit = ctx.auditd.get("log");` -- the map being walked.
    let (head, tail) = script.split_once(".get(")?;
    let path = clean_path(head.rsplit("ctx.").next()?.trim());
    let key = tail.split_once(')')?.0.trim().trim_matches(['\'', '"']);
    if path.is_empty() || key.is_empty() || path.contains(' ') {
        return None;
    }
    let container = format!("{path}.{key}");

    // The placeholder arm, which removes the key rather than rewriting it.
    let placeholders = crate::painless_common::quoted_members(
        script.split_once("if (v == ")?.1.split_once(") {")?.0,
    );
    if placeholders.is_empty() {
        return None;
    }

    // Both key lists come off the call, where they are still `params.<name>`.
    let arguments = script.split_once("processFieldValue(k, ")?.1.split_once(')')?.0;
    let mut lists = arguments
        .split(',')
        .filter_map(|argument| argument.trim().strip_prefix("params."))
        .map(|name| name.trim().to_owned());
    let hex_keys = lists.next()?;
    let boolean_keys = lists.next()?;
    if hex_keys.is_empty() || boolean_keys.is_empty() {
        return None;
    }

    // The optional wholesale rewrite of one key's value.
    let rewrite = script
        .split_once("if (k == ")
        .and_then(|(_, rest)| {
            let (condition, body) = rest.split_once(") {")?;
            let mut literals = crate::painless_common::quoted_members(condition).into_iter();
            let key = literals.next()?;
            let from = literals.next()?;
            let to = crate::painless_common::quoted_members(body.split_once(';')?.0)
                .into_iter()
                .next()?;
            Some((key, from, to))
        })
        .filter(|(key, from, to)| !key.is_empty() && !from.is_empty() && !to.is_empty());

    Some(NormaliseMapValues {
        container,
        placeholders,
        hex_keys,
        boolean_keys,
        rewrite,
    })
}

/// Is this the even-length, all-hex-digit string the decode accepts?
fn is_hex_ascii(value: &str) -> bool {
    !value.is_empty()
        && value.len().is_multiple_of(2)
        && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// Decode the pairs, and keep the result ONLY if it needed the encoding.
fn decode_hex_ascii(hex: &str) -> Option<String> {
    let bytes = hex.as_bytes();
    let mut decoded = String::with_capacity(bytes.len() / 2);
    let mut needed = false;
    for pair in bytes.as_chunks::<2>().0 {
        let pair = std::str::from_utf8(pair).ok()?;
        let mut code = u32::from_str_radix(pair, 16).ok()?;
        if code < 33 || code == 34 || code == 127 {
            needed = true;
        }
        if code < 32 || code == 127 {
            decoded.push('^');
            code ^= 64;
        }
        decoded.push(char::from_u32(code)?);
    }
    needed.then_some(decoded)
}

/// The script's `processFieldValue`: `None` removes the key.
fn normalise_one_value(
    key: &str,
    value: &Value,
    pattern: &NormaliseMapValues,
    hex_keys: &[&str],
    boolean_keys: &[&str],
) -> Option<Value> {
    let mut value = value.clone();

    if let Value::String(text) = &value
        && pattern.placeholders.iter().any(|spelling| spelling == text)
    {
        return None;
    }

    if let Value::String(text) = &value
        && hex_keys.contains(&key)
        && is_hex_ascii(text)
        && let Some(decoded) = decode_hex_ascii(text)
    {
        value = Value::String(decoded);
    }

    if let Value::String(text) = &value
        && boolean_keys.contains(&key)
    {
        let text = text.to_lowercase();
        return Some(Value::Bool(text == "yes" || text == "true" || text == "1"));
    }

    if let Value::String(text) = &value {
        // ONE quote each end, and the two ends are independent -- the script
        // strips a leading quote whether or not a trailing one follows.
        let trimmed = text.strip_prefix(['\'', '"']).unwrap_or(text);
        let trimmed = trimmed.strip_suffix(['\'', '"']).unwrap_or(trimmed);
        if trimmed.len() != text.len() {
            value = Value::String(trimmed.to_owned());
        }
    }

    if let Some((rewrite_key, from, to)) = &pattern.rewrite
        && key == rewrite_key
        && value.as_str() == Some(from.as_str())
    {
        value = Value::String(to.clone());
    }

    Some(value)
}

/// Walk the map, rewriting each value and dropping the ones that mean absent.
fn run_normalise_map_values(
    event: &mut Event,
    pattern: &NormaliseMapValues,
    params: &Map<String, Value>,
) -> bool {
    let Some(container) = event.get(&pattern.container).and_then(Value::as_object) else {
        return true;
    };

    let list = |name: &str| -> Vec<&str> {
        params
            .get(name)
            .and_then(Value::as_array)
            .map(|names| names.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default()
    };
    let hex_keys = list(&pattern.hex_keys);
    let boolean_keys = list(&pattern.boolean_keys);

    // REBUILT in order rather than edited in place: a removal must not move
    // another key, and the surviving keys keep the positions they had.
    let mut rebuilt = Map::with_capacity(container.len());
    for (key, value) in container {
        let kept = match value {
            Value::Array(values) => {
                let kept: Vec<Value> = values
                    .iter()
                    .filter_map(|value| {
                        normalise_one_value(key, value, pattern, &hex_keys, &boolean_keys)
                    })
                    .collect();
                // Every element dropped removes the key, the same as a scalar.
                (!kept.is_empty()).then(|| Value::Array(kept))
            }
            other => normalise_one_value(key, other, pattern, &hex_keys, &boolean_keys),
        };
        if let Some(kept) = kept {
            rebuilt.insert(key.clone(), kept);
        }
    }

    let _ = event.set(&pattern.container, Value::Object(rebuilt));
    true
}

/// A named set of keys MOVED from a map into a child of it.
///
/// stormshield lifts its metadata this way -- `params.names` lists the keys,
/// and each one found is copied under `stormshield.metadata` and removed from
/// where it was. The remove is half the point: leaving the original behind
/// would emit a field Elasticsearch does not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MoveKeysIntoChild {
    parent: String,
    child: String,
    names: String,
}

/// `params.<names>.forEach(k -> { ctx.<parent>.<child>[k] = ctx.<parent>[k];
/// ctx.<parent>.remove(k); })`
fn parse_move_keys_into_child(script: &str) -> Option<MoveKeysIntoChild> {
    let names = script
        .split_once("params.")?
        .1
        .split_once(".forEach(")?
        .0
        .trim();
    if names.is_empty() || names.contains(char::is_whitespace) {
        return None;
    }

    // The move itself names both halves: the destination on the left, the
    // parent it is read from on the right.
    let (head, tail) = script.split_once("[k] = ctx.")?;
    let destination = clean_path(head.rsplit("ctx.").next()?.trim());
    let parent = clean_path(tail.split_once('[')?.0.trim());
    let child = destination.strip_prefix(&format!("{parent}."))?.to_owned();

    // Without the remove this is a COPY, which leaves the source behind and is
    // a different pattern.
    if !script.contains(&format!("ctx.{parent}.remove(k)")) {
        return None;
    }
    (!parent.is_empty() && !child.is_empty()).then(|| MoveKeysIntoChild {
        parent,
        child,
        names: names.to_owned(),
    })
}

fn run_move_keys_into_child(
    event: &mut Event,
    pattern: &MoveKeysIntoChild,
    params: &Map<String, Value>,
) -> bool {
    let Some(names) = params.get(&pattern.names).and_then(Value::as_array) else {
        return true;
    };
    for name in names.iter().filter_map(Value::as_str) {
        let source = format!("{}.{name}", pattern.parent);
        // A key the document does not carry is skipped, not created empty.
        let Some(value) = event.get(&source).cloned() else {
            continue;
        };
        let _ = event.set(
            &format!("{}.{}.{name}", pattern.parent, pattern.child),
            value,
        );
        event.remove(&source);
    }
    true
}

/// Choose the candidate whose fields are present, then write its block.
fn run_record_action_table(
    event: &mut Event,
    pattern: &RecordActionTable,
    params: &Map<String, Value>,
) -> bool {
    // No record type at all is the script's own early return.
    let Some(key) = event.get_as_string(&pattern.subject) else {
        return true;
    };
    // CLONED: the candidates' fields are read off this container while the
    // block below writes to the document.
    let Some(base) = event.get(&pattern.base).and_then(Value::as_object).cloned() else {
        return true;
    };

    let lookup = |table: &str, key: &str| {
        params
            .get(table)
            .and_then(Value::as_object)
            .and_then(|table| table.get(key))
            .and_then(Value::as_array)
    };

    let candidates = lookup(&pattern.primary, &key).or_else(|| {
        // The fallback runs only when the record names a syscall, and the
        // wildcard only when that syscall is not itself listed.
        let fallback = pattern.fallback.as_deref()?;
        let syscall = event.get_as_string(&pattern.fallback_subject)?;
        lookup(fallback, &syscall).or_else(|| lookup(fallback, "*"))
    });
    let Some(candidates) = candidates else {
        return true;
    };

    let chosen = candidates.iter().find(|candidate| {
        // No `has_fields` is an unconditional candidate, which is why the
        // FIRST match wins rather than the best one.
        candidate.get("has_fields").is_none_or(|required| {
            required.as_array().is_some_and(|required| {
                required.iter().all(|field| {
                    field
                        .as_str()
                        .and_then(|field| base.get(field))
                        .is_some_and(|value| !value.is_null())
                })
            })
        })
    });
    let Some(chosen) = chosen else {
        return true;
    };

    if let Some(block) = chosen.get("event").and_then(Value::as_object) {
        for (field, value) in block {
            let _ = event.set(&format!("{}.{field}", pattern.target), value.clone());
        }
    }

    let Some(copies) = chosen.get("copy").and_then(Value::as_array) else {
        return true;
    };
    let mut staged: Vec<Value> = Vec::with_capacity(copies.len());
    for copy in copies {
        // The first source that HOLDS a value, not the first that is named.
        let value = copy
            .get("from")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|name| base.get(name.as_str()?))
            .find(|value| !value.is_null());
        // `instanceof String` in the script, and auditd writes these two for a
        // field it could not resolve.
        let (Some(Value::String(value)), Some(target)) =
            (value, copy.get("to").and_then(Value::as_str))
        else {
            continue;
        };
        if value == "unset" || value == "?" {
            continue;
        }
        // `==~ /[0-9]+/` is a WHOLE-string match in Painless.
        let numeric = !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit());
        let suffix = if numeric { "id" } else { "name" };
        staged.push(json!({ "target": format!("{target}.{suffix}"), "value": value }));
    }
    if !staged.is_empty() {
        let _ = event.set(&pattern.copy_field, Value::Array(staged));
    }
    true
}

/// Write the block the table names, then the action's extra type.
fn run_event_block_table(
    event: &mut Event,
    pattern: &EventBlockTable,
    params: &Map<String, Value>,
) -> bool {
    let Some(key) = event.get_as_string(&pattern.subject) else {
        return true;
    };
    let entry = params
        .get(&pattern.table)
        .and_then(Value::as_object)
        .and_then(|table| table.get(&key));

    let Some(entry) = entry else {
        // No entry is not a failure: the script writes the bare kind.
        let _ = event.set(
            &format!("{}.kind", pattern.target),
            json!(pattern.default_kind),
        );
        return true;
    };

    for field in ["kind", "category", "type"] {
        if let Some(value) = entry.get(field) {
            // COPIED, not aliased: the script says `new ArrayList(..)`, and
            // the append below would otherwise grow the params table itself.
            let _ = event.set(&format!("{}.{field}", pattern.target), value.clone());
        }
    }

    let (Some(subjects), Some(types)) = (&pattern.action_subjects, &pattern.action_types) else {
        return true;
    };
    let listed = params
        .get(subjects)
        .and_then(Value::as_array)
        .is_some_and(|list| list.iter().any(|item| item.as_str() == Some(key.as_str())));
    if !listed {
        return true;
    }
    // `instanceof String` in the script, so a non-string action adds nothing.
    let Some(action) = event.get(&pattern.action_field).and_then(Value::as_str) else {
        return true;
    };
    let Some(mapped) = params
        .get(types)
        .and_then(Value::as_object)
        .and_then(|table| table.get(&action.to_lowercase()))
        .cloned()
    else {
        return true;
    };

    let target = format!("{}.type", pattern.target);
    let mut types: Vec<Value> = match event.get(&target) {
        Some(Value::Array(existing)) => existing.clone(),
        Some(other) => vec![other.clone()],
        None => Vec::new(),
    };
    if !types.contains(&mapped) {
        types.push(mapped);
        let _ = event.set(&target, Value::Array(types));
    }
    true
}

/// Read the scheme-prefix pattern: `if (params.<list>.contains(ctx.<subject>))
/// { ctx.<target> = ctx.<subject> + '://' + ctx.<target>; } else {
/// ctx.<target> = params.<fallback> + '://' + ... }`.
fn parse_protocol_prefix(script: &str) -> Option<ParamsPattern> {
    let at = script.find(".contains(ctx.")?;
    let before = &script[..at];
    let list = before[before.rfind("params.")? + 7..].to_string();
    let after = &script[at + ".contains(ctx.".len()..];
    let (subject, _) = after.split_once(')')?;

    // The concatenation's tail names the target; the else-arm's params read
    // names the fallback scheme.
    let concat_at = script.find("+ '://' + ctx.")?;
    let tail = &script[concat_at + "+ '://' + ctx.".len()..];
    let target = tail
        .split([';', '\n'])
        .next()?
        .trim()
        .trim_end_matches(';')
        .trim();
    let fallback_at = script.rfind("= params.")?;
    let fallback = script[fallback_at + "= params.".len()..]
        .split_whitespace()
        .next()?
        .to_string();

    if list.is_empty() || fallback.is_empty() {
        return None;
    }
    Some(ParamsPattern::ProtocolPrefix {
        list,
        fallback,
        subject: clean_path(subject),
        target: clean_path(target),
    })
}

/// Split sysmon's `QueryResults` into `dns.answers`, `dns.resolved_ip` and
/// `related.hosts`.
///
/// Event 22 packs the whole DNS response into one semicolon-separated string:
///
/// ```text
/// type:  5 f2.taboola.map.fastly.net;::ffff:151.101.66.2;::ffff:151.101.2.2;
/// ```
///
/// An entry opening `type:` is a record whose middle token is the IANA RR
/// NUMBER -- `params` is the number-to-name table and is read rather than
/// transcribed, so the vendor adding a type is picked up by regenerating.
/// Anything else is a resolved address. A `type:` entry with a third token
/// names a host, which is where the CNAME chain in `related.hosts` comes
/// from; without one only the type is recorded.
///
/// The `::ffff:` unwrapping and the `convert` to an ip are separate processors
/// downstream, so this leaves the addresses exactly as the vendor wrote them.
fn try_sysmon_query_results(event: &mut Event, _script: &str, params: &Map<String, Value>) -> bool {
    let Some(results) = event.get_str("winlog.event_data.QueryResults") else {
        return false;
    };

    let mut answers: Vec<Value> = Vec::new();
    let mut ips: Vec<Value> = Vec::new();
    let mut hosts: Vec<Value> = Vec::new();

    for answer in results.split(';') {
        if answer.is_empty() {
            continue;
        }
        if !answer.starts_with("type:") {
            ips.push(Value::String(answer.to_string()));
            continue;
        }

        let parts: Vec<&str> = answer.split_whitespace().collect();
        if parts.len() < 2 {
            // The script throws here, and a throw runs the processor's
            // on_failure rather than writing a half-parsed answer.
            return false;
        }
        // An unknown number reads as null in Painless, and the null reaches
        // the document, so it does here too.
        let kind = params.get(parts[1]).cloned().unwrap_or(Value::Null);
        if parts.len() == 3 {
            answers.push(serde_json::json!({"type": kind, "data": parts[2]}));
            hosts.push(Value::String(parts[2].to_string()));
        } else {
            answers.push(serde_json::json!({"type": kind}));
        }
    }

    if !answers.is_empty() {
        let _ = event.set("dns.answers", Value::Array(answers));
    }
    if !ips.is_empty() {
        let _ = event.set("dns.resolved_ip", Value::Array(ips));
    }
    if !hosts.is_empty() {
        let _ = event.set("related.hosts", Value::Array(hosts));
    }
    true
}

/// Collect a set of framework names from several ID lists, then pick one by a
/// declared preference order.
///
/// `CrowdStrike`'s `threat.framework`: a tactic id starting `TA` means MITRE, one
/// starting `CS` means Falcon, and a tactic NAME in the params list means Falcon
/// too. Every name, prefix and list member is read out of the script and its
/// params rather than transcribed, so the vendor extending any of them is picked
/// up by regenerating.
fn try_framework_preference(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(Value::Array(preference)) = params.get("framework_preference") else {
        return false;
    };

    let mut found: Vec<Value> = Vec::new();
    for (var, block) in loop_blocks(script) {
        let Some(path) = crate::painless_common::ctx_path_bound_to(script, &var) else {
            continue;
        };
        for member in string_members(event, &path) {
            for (prefix, key) in prefix_rules(block) {
                if member.starts_with(&prefix)
                    && let Some(name) = params.get(&key)
                    && !found.contains(name)
                {
                    found.push(name.clone());
                }
            }
            for (list_key, key) in membership_rules(block) {
                let Some(Value::Array(list)) = params.get(&list_key) else {
                    continue;
                };
                let lowered = Value::String(member.to_lowercase());
                if list.contains(&lowered)
                    && let Some(name) = params.get(&key)
                    && !found.contains(name)
                {
                    found.push(name.clone());
                }
            }
        }
    }

    if found.is_empty() {
        return true;
    }
    let chosen = if found.len() == 1 {
        found[0].clone()
    } else {
        // The preference list decides; a framework absent from it falls back to
        // whichever was collected first, which is what the script's final line
        // does with an unordered set.
        preference
            .iter()
            .find(|wanted| found.contains(wanted))
            .unwrap_or(&found[0])
            .clone()
    };
    let _ = event.set("threat.framework", chosen);
    true
}

/// Each `for (String t: <var>) { ... }` in the script, as its variable and the
/// text of its body.
///
/// The body runs to the next loop header, which is enough: the rules inside one
/// are all that is read from it.
fn loop_blocks(script: &str) -> Vec<(String, &str)> {
    const HEADER: &str = "for (String t: ";

    let mut blocks = Vec::new();
    let parts: Vec<&str> = script.split(HEADER).collect();
    for part in parts.iter().skip(1) {
        let Some((var, body)) = part.split_once(')') else {
            continue;
        };
        blocks.push((var.trim().to_string(), body));
    }
    blocks
}

/// Every `t.startsWith('<prefix>')` paired with the `params.<key>` its arm adds.
fn prefix_rules(block: &str) -> Vec<(String, String)> {
    rules_in(block, ".startsWith(")
}

/// Every `params.<list>.contains(...)` paired with the `params.<key>` its arm
/// adds.
fn membership_rules(block: &str) -> Vec<(String, String)> {
    const MARKER: &str = ".contains(";

    let mut rules = Vec::new();
    let mut at = 0;
    while let Some(found) = block[at..].find(MARKER) {
        let call = at + found;
        at = call + MARKER.len();

        // The list is whatever `params.` name sits immediately before the call.
        let head = &block[..call];
        let Some(start) = head.rfind("params.") else {
            continue;
        };
        let list = &head[start + "params.".len()..];
        if list.is_empty() || !list.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let Some(key) = added_params_key(&block[at..]) else {
            continue;
        };
        rules.push((list.to_string(), key));
    }
    rules
}

/// The `(<literal>, params.<key>)` pairs an arm opened by `marker` names.
fn rules_in(block: &str, marker: &str) -> Vec<(String, String)> {
    let mut rules = Vec::new();
    for segment in block.split(marker).skip(1) {
        let Some(literal) = quoted_after(segment, "") else {
            continue;
        };
        let Some(key) = added_params_key(segment) else {
            continue;
        };
        rules.push((literal, key));
    }
    rules
}

/// The params key of the FIRST `frameworks.add(params.<key>)` in `text`.
fn added_params_key(text: &str) -> Option<String> {
    let start = text.find(".add(params.")? + ".add(params.".len();
    let tail = &text[start..];
    let end = tail
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(tail.len());
    Some(tail[..end].to_string())
}

/// The string members of a field, whether it holds one or a list of them.
fn string_members(event: &Event, path: &str) -> Vec<String> {
    match event.get(path) {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect(),
        Some(Value::String(one)) => vec![one.clone()],
        _ => Vec::new(),
    }
}

/// `def p = params[ctx.<subject>];` then, per column,
/// `ctx.<target> = addUnique(ctx.<target>, p.<column>);`
///
/// Elastic's `gen` emits this as an `ecs_category_type` sub-pipeline for every
/// package that maps a vendor event name onto ECS categorisation, so the table
/// is the params block and a vendor adding an event type is picked up by
/// regenerating rather than by editing Rust.
///
/// `addUnique` is a set union, and the ECS arrays it writes are compared as
/// sets -- `tests/compare-policy.yaml` names both -- so the Java `HashSet`
/// iteration order it comes out in is not reproduced.
///
/// A subject with no row throws in Painless, and the sub-pipeline's `on_failure`
/// turns that into `event.kind: pipeline_error`. Nothing here models the throw,
/// so an unknown event type leaves the event as it was.
fn try_add_unique_row(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(subject) = ctx_path_between(script, "params[ctx.", "]") else {
        return false;
    };
    let bindings = add_unique_bindings(script);
    if bindings.is_empty() {
        return false;
    }

    let Some(key) = event.get_as_string(&subject) else {
        return true;
    };
    let Some(Value::Object(row)) = params.get(&key) else {
        return true;
    };
    let row = row.clone();

    for (target, column) in bindings {
        let Some(Value::Array(additions)) = row.get(&column) else {
            continue;
        };
        let mut merged = match event.get(&target) {
            Some(Value::Array(existing)) => existing.clone(),
            Some(other) => vec![other.clone()],
            None => Vec::new(),
        };
        for addition in additions {
            if !merged.contains(addition) {
                merged.push(addition.clone());
            }
        }
        let _ = event.set(&target, Value::Array(merged));
    }
    true
}

/// Every `ctx.<target> = addUnique(ctx.<target>, p.<column>)` in the script, as
/// the pair it names.
fn add_unique_bindings(script: &str) -> Vec<(String, String)> {
    const CALL: &str = "= addUnique(";

    let mut pairs = Vec::new();
    let mut at = 0;
    while let Some(found) = script[at..].find(CALL) {
        let call = at + found;
        let head = &script[..call];
        at = call + CALL.len();

        // The last `ctx.` before the call is the assignment's left-hand side;
        // the one inside the call is the same path read back.
        let Some(start) = head.rfind("ctx.") else {
            continue;
        };
        let target = clean_path(&head[start + "ctx.".len()..]);

        let Some((_, column)) = script[at..].split_once(',') else {
            continue;
        };
        let Some((_, column)) = column.trim_start().split_once('.') else {
            continue;
        };
        let end = column
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(column.len());
        pairs.push((target, column[..end].to_string()));
    }
    pairs
}

/// `long value = ctx.<source>;` then, per params entry,
/// `if ((value & flag) != 0) { <target>[entry.getKey()] = true; }`
///
/// The params block IS the flag table, so a vendor adding a bit is picked up by
/// regenerating rather than by editing Rust. Only set bits are written; a clear
/// bit leaves the label absent rather than false, which is what the script does.
///
/// panw's decryption-log flags are the pattern, and they gate two of its largest
/// remaining blockers -- `labels.nat_translated` and `labels.captive_portal`.
fn try_bit_flags(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(source) = ctx_path_between(script, "long value = ctx.", ";") else {
        return false;
    };
    // The map the labels land in, from `ctx['labels'] = labels;`.
    let target = quoted_after(script, "ctx[").unwrap_or_else(|| "labels".to_owned());

    let Some(bits) = event.get(&source).and_then(as_u64_flags) else {
        return false;
    };

    for (name, flag) in params {
        // Elastic's own script decodes a string flag, because Kibana has been
        // known to hand the params block back with the numbers stringified.
        let Some(flag) = as_u64_flags(flag) else {
            continue;
        };
        if flag != 0 && bits & flag != 0 {
            let path = format!("{target}.{name}");
            if event.set(&path, Value::Bool(true)).is_err() {
                return false;
            }
        }
    }
    true
}

/// A bitfield, however the pipeline happens to carry it.
///
/// A `0x`-prefixed string is what `Long.decode` in the vendor script exists to
/// handle, and a float is what a large flag becomes if it round-trips as JSON.
fn as_u64_flags(value: &Value) -> Option<u64> {
    match value {
        Value::Number(n) => n
            .as_u64()
            .or_else(|| n.as_i64().and_then(|i| u64::try_from(i).ok()))
            .or_else(|| n.as_f64().map(|f| f as u64)),
        Value::String(s) => {
            let s = s.trim();
            s.strip_prefix("0x")
                .or_else(|| s.strip_prefix("0X"))
                .map_or_else(|| s.parse().ok(), |hex| u64::from_str_radix(hex, 16).ok())
        }
        _ => None,
    }
}

/// `ctx.<path>.entrySet().removeIf(entry -> params.<name>.contains(entry.getValue()))`
///
/// The sentinel list is the params entry, so a vendor adding `"-"` to it is
/// picked up by regenerating rather than by editing Rust.
fn try_sentinel_removal(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(path) = ctx_path_before(script, ".entrySet().removeIf(") else {
        return false;
    };
    let Some(Value::Array(sentinels)) = params_ref(script, params, "params.") else {
        return false;
    };
    let sentinels = sentinels.clone();

    // A missing map is not a failure -- the `if` on the processor already
    // guards it, and Elastic's own script returns without touching ctx.
    if let Some(Value::Object(map)) = pointer_mut(event, &path) {
        remove_sentinel_values(map, &sentinels);
    }

    // The defender pipelines wrap this removal in a recursive drop-empty and
    // run `drop(ctx)` after it. The removal claims the script, so the sweep
    // that clears what the removal just emptied has to run here too.
    if script.contains("instanceof Map")
        && script.contains("instanceof List")
        && script.contains("(ctx)")
    {
        crate::painless_common::drop_empty_recursive(
            event,
            &crate::painless_common::DropPolicy::read(script),
        );
    }
    true
}

/// `for (def field : params.<name>) { ctx.<path>[field] = convertToUnix(...) }`
///
/// Only the numeric conversion is modelled: a value that is neither a number
/// nor a digit string is left alone, which is what the vendor script's
/// `instanceof` ladder does.
fn try_filetime_field_list(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(container) = ctx_path_before(script, "[field]") else {
        return false;
    };
    let Some(Value::Array(fields)) = params_ref(script, params, "for (def field : params.") else {
        return false;
    };

    for field in fields.clone() {
        let Some(name) = field.as_str() else { continue };
        let path = format!("{container}.{name}");
        let converted = match event.get(&path) {
            Some(Value::Number(n)) => n.as_i64().map(filetime_to_unix_ms),
            // The script skips a string holding a fractional value.
            Some(Value::String(s)) if !s.contains('.') => {
                s.parse::<i64>().ok().map(filetime_to_unix_ms)
            }
            _ => None,
        };
        if let Some(v) = converted {
            let _ = event.set(&path, v);
        }
    }
    true
}

/// `params.get(<key>)[.get(<key>)...]` then `forEach((k, v) -> ctx.<t>[k] = v)`
///
/// The table is the params block itself, keyed by one field's value per level.
/// `cisco_asa` uses one level for a message id and two for a message id and its
/// outcome, and both end the same way -- the row that survives the chain is
/// merged into `ctx.event`.
///
/// Following only the first level fanned the SECOND level's keys out as if
/// they were fields, which is how `event.denied.action` and its two siblings
/// appeared in place of one `event.action`.
fn try_lookup_merge(
    event: &mut Event,
    script: &str,
    params: &Map<String, Value>,
    literals: &Program,
) -> bool {
    // The lambda is written both inline and as a braced block that branches on
    // the value's type, so the target is read from the `ctx.<path>[k] =`
    // assignment in the lambda's LAST arm rather than from its head.
    let Some(body) = script.split_once("forEach(").map(|(_, tail)| tail) else {
        return false;
    };
    let Some(target) = merge_default_target(script, body) else {
        return false;
    };
    let routes = merge_routes(body);
    // `.get(` first, then the bracket spelling -- carbonblack_edr subscripts
    // the table, and reading only the call form found no key at all.
    let keys = match get_chain(script) {
        chain if chain.is_empty() => bracket_key(script).into_iter().collect(),
        chain => chain,
    };
    if keys.is_empty() {
        return false;
    }

    // The literal writes the script makes on its own account. A script that
    // sets `event.kind`, `event.type` and an outcome before looking the row up
    // -- aws's cloudtrail categorisation is the pattern -- was claimed here and
    // only its merge ran, so those three came out missing. The walk skips
    // anything it cannot read, so the lookup and the `forEach` pass it by.
    literals.run(event);

    // `params[k] != null ? params[k] : params['<name>']` -- a key the table does
    // not list still gets a row.
    let fallback = bracket_fallback(script).and_then(|name| params.get(&name));

    let mut node = Some(&Value::Null);
    for (level, expr) in keys.iter().enumerate() {
        let Some(key) = resolve_key(event, script, expr) else {
            // The keyed field is absent, and every one of these scripts opens
            // by returning when that is so.
            return true;
        };
        node = if level == 0 {
            params.get(&key)
        } else {
            node.and_then(Value::as_object)
                .and_then(|map| map.get(&key))
        };
        if node.is_none() {
            node = fallback;
            break;
        }
    }

    let Some(Value::Object(row)) = node else {
        return true;
    };
    for (k, v) in row.clone() {
        let path = match routes.iter().find(|(key, _)| *key == k) {
            Some((_, MergeRoute::At(path))) => path.clone(),
            Some((_, MergeRoute::Under(path))) => format!("{path}.{k}"),
            None => format!("{target}.{k}"),
        };
        let _ = event.set(&path, v);
    }
    true
}

/// The target of the LAST `ctx.<path>[k] = ` in a merge lambda.
///
/// That assignment is the `else` arm, where every key the branches above it do
/// not name lands. Reading the FIRST one instead took `watchguard_firebox`'s
/// routed arm as the target for the whole row, so `event.category`,
/// `event.type` and `event.outcome` were written under
/// `watchguard_firebox.log` on all 637 events.
fn merge_default_target(script: &str, body: &str) -> Option<String> {
    let end = body.rfind("[k] = ")?;
    let reference = body[..end]
        .rsplit(|c: char| c.is_whitespace() || matches!(c, '{' | '}' | ';' | '(' | ')'))
        .next()?;
    if let Some(path) = reference
        .strip_prefix("ctx.")
        .or_else(|| reference.strip_prefix("ctx?."))
    {
        return Some(clean_path(path));
    }
    // A LOCAL bound to a ctx path. carbonblack_edr binds `event` at the top and
    // merges onto that, so reading only the spelled-out `ctx.` form found no
    // target and the whole pattern declined.
    crate::painless_common::ctx_path_bound_to(script, reference)
}

/// The key expression of a `params[<expr>]` lookup.
///
/// The bracket twin of [`get_chain`]. A QUOTED subscript names a table rather
/// than a key, which is a different pattern, so it is declined here.
fn bracket_key(script: &str) -> Option<String> {
    let head = script.split("forEach").next().unwrap_or(script);
    let at = head.find("params[")?;
    let inner = head[at + "params[".len()..].split_once(']')?.0.trim();
    (!inner.is_empty() && !inner.starts_with(['"', '\''])).then(|| inner.to_string())
}

/// The params row a key that missed falls back to: `... : params['<name>']`.
fn bracket_fallback(script: &str) -> Option<String> {
    let head = script.split("forEach").next().unwrap_or(script);
    let inner = head.rsplit_once(": params[")?.1.split_once(']')?.0.trim();
    inner.starts_with(['"', '\'']).then(|| unquote(inner))
}

/// Where one routed key's value goes.
enum MergeRoute {
    /// `ctx.<path>[k] = v` -- the key itself is the last segment.
    Under(String),
    /// `ctx.<path> = v` -- a fixed path the arm names outright.
    At(String),
}

/// The key-routed arms of a merge lambda, each with the path it writes.
///
/// A row normally merges whole into one container, but `watchguard_firebox`
/// sends `log_type` to its own namespace and suricata sends `network_protocol`
/// to `network.protocol`. Both spellings of the guard appear across the
/// integrations, and so do both spellings of the write.
///
/// An arm that writes somewhere else entirely -- `beyondtrust_pra` collects
/// `category` into a local list -- names no path and is left to the default,
/// which is what the merge did with that key before routing existed.
fn merge_routes(body: &str) -> Vec<(String, MergeRoute)> {
    let mut routes = Vec::new();
    let mut rest = body;
    while let Some((key, after)) = next_routed_key(rest) {
        // Balanced, because the arm can hold an `if`/`else` of its own and a
        // split on the next `else` would cut it in half.
        let Some(at) = after.find('{') else {
            rest = after;
            continue;
        };
        let Some((arm, tail)) = balanced(&after[at..], '{', '}') else {
            rest = after;
            continue;
        };
        if let Some(route) = route_target(arm) {
            routes.push((key, route));
        }
        rest = tail;
    }
    routes
}

/// The next `k.equals('<key>')` or `'<key>' == k` guard, and the text after it.
fn next_routed_key(body: &str) -> Option<(String, &str)> {
    let call = body.find("k.equals(").and_then(|at| {
        let (key, after) = body[at + "k.equals(".len()..].split_once(')')?;
        Some((at, unquote(key), after))
    });
    let compare = body.find("== k").and_then(|at| {
        let literal = body[..at].trim_end().rsplit(['(', ' ']).next()?;
        // Only a quoted literal is a key; `ctx.network == null` is not one.
        (literal.starts_with(['"', '\'']))
            .then(|| (at, unquote(literal), &body[at + "== k".len()..]))
    });
    // Whichever guard the script spells first, so the arms are read in order.
    let (_, key, after) = match (call, compare) {
        (Some(by_call), Some(by_compare)) => Some(if by_call.0 <= by_compare.0 {
            by_call
        } else {
            by_compare
        }),
        (found @ Some(_), None) | (None, found @ Some(_)) => found,
        (None, None) => None,
    }?;
    Some((key, after))
}

/// The path an arm writes the routed value to.
fn route_target(arm: &str) -> Option<MergeRoute> {
    if let Some(end) = arm.rfind("[k] = ") {
        let head = &arm[..end];
        let start = head.rfind("ctx.")? + "ctx.".len();
        return Some(MergeRoute::Under(clean_path(&head[start..])));
    }
    // The LAST `ctx.<path> = v`, because an arm that has to build its container
    // first writes the same value twice -- suricata's `ctx.network` map literal
    // then `ctx.network.protocol` -- and the second names the path in full.
    let end = arm.rfind(" = v")?;
    let head = &arm[..end];
    let start = head.rfind("ctx.")? + "ctx.".len();
    Some(MergeRoute::At(clean_path(&head[start..])))
}

/// A Painless string literal's contents.
fn unquote(literal: &str) -> String {
    literal.trim().trim_matches(['"', '\'']).to_string()
}

/// The arguments of a `params.get(a)[?].get(b)...` chain, outermost first.
///
/// Read from the LAST `params.get(`, because these scripts commonly look the
/// row up once to null-check it and again to use it, and only calls chained
/// directly onto the previous one are levels of the same table.
fn get_chain(script: &str) -> Vec<String> {
    let head = script.split("forEach").next().unwrap_or(script);
    let Some(start) = head.rfind("params.get(") else {
        return Vec::new();
    };
    let mut rest = &head[start + "params".len()..];
    let mut keys = Vec::new();
    loop {
        let Some(after) = rest
            .strip_prefix(".get(")
            .or_else(|| rest.strip_prefix("?.get("))
        else {
            return keys;
        };
        let mut depth = 1usize;
        let mut end = None;
        for (index, c) in after.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(index);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { return keys };
        keys.push(after[..end].trim().to_string());
        rest = &after[end + 1..];
    }
}

/// `def row = params.get('<table>').get(ctx.<subject>);` then a column each:
/// `def c = row.get('<key>'); for (def x : c) { ctx.<path>.add(x) }` or
/// `ctx.<path> = c;`, with `ctx.<path> = ctx.<subject>` when the row is absent.
///
/// Cisco Meraki's event map is the pattern: one vendor subtype expands into an
/// ECS action plus additions to `event.type` and `event.category`. Appending
/// matters -- the pipeline has already put `info` in `event.type`, and a
/// replacing write drops it.
fn try_lookup_columns(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(Value::Object(table)) = params_ref(script, params, "params.get('") else {
        return false;
    };
    // The table is bound to a local first, and the row lookup goes through it.
    let Some(table_local) = script
        .split_once("def ")
        .and_then(|(_, rest)| rest.split_once(" = params.get("))
        .map(|(name, _)| name.trim().to_string())
    else {
        return false;
    };
    let Some(key_expr) = last_call_argument(script, &format!("{table_local}.get(")) else {
        return false;
    };
    let Some(key) = resolve_key(event, script, &key_expr) else {
        return true;
    };

    let Some(Value::Object(row)) = table.get(&key) else {
        // Every one of these scripts falls back to writing the raw subtype.
        if let Some(target) = fallback_target(script) {
            let _ = event.set(&target, Value::String(key));
        }
        return true;
    };
    let row = row.clone();

    for (local, column) in column_bindings(script) {
        let Some(value) = row.get(&column) else {
            continue;
        };
        match appended_target(script, &local) {
            Some(path) => {
                let mut existing = match event.get(&path) {
                    Some(Value::Array(a)) => a.clone(),
                    _ => Vec::new(),
                };
                match value {
                    Value::Array(items) => existing.extend(items.iter().cloned()),
                    other => existing.push(other.clone()),
                }
                let _ = event.set(&path, Value::Array(existing));
            }
            None => {
                if let Some(path) = assigned_target(script, &local) {
                    let _ = event.set(&path, value.clone());
                }
            }
        }
    }
    true
}

/// `def <local> = <row>.get('<column>')` pairs, minus the row lookup itself.
fn column_bindings(script: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for segment in script.split("def ").skip(1) {
        let Some((local, rest)) = segment.split_once(" = ") else {
            continue;
        };
        if rest.contains("params") {
            continue;
        }
        let Some(column) = rest
            .split_once(".get('")
            .and_then(|(_, s)| s.split_once('\''))
        else {
            continue;
        };
        found.push((local.trim().to_string(), column.0.to_string()));
    }
    found
}

/// The `ctx.` path a `for (def x : <local>) { ctx.<path>.add(x) }` appends to.
fn appended_target(script: &str, local: &str) -> Option<String> {
    let needle = format!(" : {local})");
    let (_, tail) = script.split_once(&needle)?;
    let end = tail.find('}')?;
    ctx_path_before(&tail[..end], ".add(")
}

/// The `ctx.` path a bare `ctx.<path> = <local>;` assignment writes.
fn assigned_target(script: &str, local: &str) -> Option<String> {
    let needle = format!("= {local};");
    ctx_path_before(script, &needle)
}

/// The `ctx.` path the no-row branch writes the raw subject into.
fn fallback_target(script: &str) -> Option<String> {
    let (head, _) = script.split_once("== null")?;
    let (_, tail) = script[head.len()..].split_once('{')?;
    ctx_path_before(tail, " = ")
}

/// `def k = ctx.<path>.toLowerCase(); def v = params.get(k);`
/// `if (v != null) { ctx.<path> = v; return; } ctx.<path> = k;`
///
/// A vendor-vocabulary-to-ECS map: fortinet's `outgoing` is ECS `outbound`.
/// The fallback writes the LOOKUP KEY back, not the original, so a value the
/// table misses still comes out lower-cased -- and the pipeline's own
/// allow-list check downstream then sees the same string Elastic would.
pub(crate) fn lookup_normalise(
    event: &mut Event,
    pattern: &LookupNormaliseScript,
    literals: &Program,
    params: &Map<String, Value>,
) -> bool {
    let Some(raw) = event.get_as_string(&pattern.key) else {
        return true;
    };
    let key = pattern.fold.apply(&raw);

    let value = params.get(&key).cloned().unwrap_or(Value::String(key));
    let _ = event.set(&pattern.target, value);

    // Whatever else the script writes on its own account, AFTER the lookup so
    // a `ctx.x = null` that clears the field the key came from is not read
    // before it is used. mimecast's siem_logs is that pattern exactly.
    literals.run(event);
    true
}

/// Where a normalise-through-a-table reads its key and writes its answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupNormaliseScript {
    /// The `ctx.` path the lookup key comes from, already unwrapped from
    /// whatever local the script bound it to.
    key: String,
    fold: Fold,
    target: String,
}

impl LookupNormaliseScript {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(key: impl Into<String>, fold: Fold, target: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            fold,
            target: target.into(),
        }
    }
}

/// Read the key path, its fold and the target out of the script once.
///
/// `None` where the script is not this pattern after all, which declines the
/// trigger and leaves the text to the matchers below it -- the same
/// fall-through the runner used to do per event.
fn parse_lookup_normalise(script: &str) -> Option<LookupNormaliseScript> {
    // `params[key]` and `params.get(key)` are the same operation in Painless
    // and the integrations use both -- sysmon's DNS status table is written
    // with the brackets.
    let key_expr = last_call_argument(script, "params.get(")
        .or_else(|| last_bracket_subscript(script, "params["))?;
    // The field assigned from the lookup, falling back to the last assignment
    // where the lookup is bound to a local first.
    let writes = ctx_writes(script);
    let target = writes
        .iter()
        .find(|(_, rhs)| rhs.contains("params.get(") || rhs.contains("params["))
        .map(|(path, _)| path.clone())
        .or_else(|| writes.last().map(|(path, _)| path.clone()))?;
    let (key, fold) = lookup_key_path(script, &key_expr)?;
    Some(LookupNormaliseScript { key, fold, target })
}

/// The first member of a params list the subject contains, written to a field.
///
/// Microsoft's Defender pipelines classify by substring: an OS platform that
/// contains `linux` is Linux, a vulnerability id that contains `CVE` is a CVE.
/// The list is the params, and a trailing block of literal `contains` tests
/// catches what the list misses.
fn try_first_contained_member(
    event: &mut Event,
    script: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some((text, folded)) = subjects_of_bindings(script)
        .into_iter()
        .find_map(|(subject, folded)| Some((event.get_as_string(&subject)?, folded)))
    else {
        return false;
    };
    let text = if folded {
        text.to_lowercase()
    } else {
        text.to_uppercase()
    };

    let Some(members) = params_ref(script, params, "params.").and_then(Value::as_array) else {
        return false;
    };
    let Some(target) = put_target(script) else {
        return false;
    };

    for member in members {
        let Some(member) = member.as_str() else {
            continue;
        };
        if text.contains(member) {
            let _ = event.set(&target, member);
            return true;
        }
    }

    // The tail: `if (x.contains('centos') || ...) { ctx.a.put('b', 'linux'); }`
    // Its subject is the same local, already folded, so the literals are
    // tested against the text in hand rather than re-read from the event.
    for block in script.split("if (").skip(1) {
        // `) {` ends the header; the first `)` alone sits inside `contains(`.
        let Some((guard, body)) = block.split_once(") {") else {
            continue;
        };
        if !guard.contains(".contains(") {
            continue;
        }
        let matched = guard
            .split("||")
            .filter_map(|term| quoted_after(term, ".contains("))
            .any(|literal| text.contains(&literal));
        if matched && let Some(literal) = quoted_after(body, ", ") {
            let _ = event.set(&target, literal);
            break;
        }
    }
    true
}

/// The `ctx.` path a `String x = ctx.<path>[.toLowerCase()];` binds, and
/// whether it was folded DOWN rather than up.
/// Every `String <local> = ctx.<path>[.toLowerCase()];` binding, in order.
///
/// The script offers its sources as an `if / else if` chain, so all of them are
/// read and the caller takes the first the event actually carries. A
/// `for (String x: ...)` loop header binds nothing and is skipped.
fn subjects_of_bindings(script: &str) -> Vec<(String, bool)> {
    let mut subjects = Vec::new();
    for chunk in script.split("String ").skip(1) {
        let Some((_, bound)) = chunk.split(';').next().and_then(|s| s.split_once('=')) else {
            continue;
        };
        let bound = bound.trim();
        let folded = bound.contains(".toLowerCase()");
        let path = bound
            .replace(".toLowerCase()", "")
            .replace(".toUpperCase()", "");
        let Some(path) = path.trim().strip_prefix("ctx.") else {
            continue;
        };
        subjects.push((clean_path(path), folded));
    }
    subjects
}

/// The field a `ctx.<path>.put('<key>', ...)` writes.
fn put_target(script: &str) -> Option<String> {
    let path = ctx_path_before(script, ".put(")?;
    let key = quoted_after(script, ".put(")?;
    Some(format!("{path}.{key}"))
}

/// Rename an object's keys, recursively, through a name map.
///
/// Windows hands its event data over in the vendor's own casing --
/// `AdditionalInfo`, `QNAME`, `XID` -- and the pipeline renames the lot in one
/// pass before anything else reads them. Everything downstream is keyed on the
/// renamed form, so a miss here costs far more than the keys themselves.
///
/// A key the map does not name keeps its own, and a value that is not an
/// object or a list is carried across untouched.
fn try_rename_keys(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some((path, _)) = ctx_writes(script)
        .into_iter()
        .find(|(_, rhs)| rhs.contains(", params)"))
    else {
        return false;
    };
    let Some(subject) = event.get(&path).cloned() else {
        // The processor's own `if` guards the object being absent.
        return false;
    };
    let _ = event.set(&path, renamed_keys(&subject, params));
    true
}

/// `value` with every key the map names replaced, at any depth.
fn renamed_keys(value: &Value, names: &Map<String, Value>) -> Value {
    match value {
        Value::Object(members) => Value::Object(
            members
                .iter()
                .map(|(key, member)| {
                    let key = names
                        .get(key)
                        .and_then(Value::as_str)
                        .unwrap_or(key)
                        .to_string();
                    (key, renamed_keys(member, names))
                })
                .collect(),
        ),
        Value::Array(items) => {
            Value::Array(items.iter().map(|item| renamed_keys(item, names)).collect())
        }
        other => other.clone(),
    }
}

/// Normalise several fields, each through a value map of its own.
///
/// The params are keyed by SOURCE PATH, and each entry carries the map and,
/// optionally, a different destination. Cisco's FTD uses it to turn the DNS
/// record type the device spells out -- "a host address" -- into the mnemonic
/// ECS wants, and the response code likewise.
///
/// A key is looked up folded to lower case, because that is what the script
/// does; a path that names no field is skipped, which is how the entry keyed
/// `ctx._temp_.cisco.message_id` behaves upstream as well. Its `ctx.` prefix
/// makes the script look for a field of that name, and there is none.
fn try_value_maps(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let folded = script.contains(".toLowerCase()");
    for (source, entry) in params {
        let Some(entry) = entry.as_object() else {
            continue;
        };
        let Some(table) = entry.get("map").and_then(Value::as_object) else {
            continue;
        };
        // A list contributes its first member, as the script reads it.
        let current = match event.get(source) {
            Some(Value::Array(items)) => items.first().and_then(scalar_text),
            Some(value) => scalar_text(value),
            None => None,
        };
        let Some(current) = current else {
            continue;
        };
        let key = if folded {
            current.to_lowercase()
        } else {
            current
        };
        if let Some(replacement) = table.get(&key) {
            let target = entry
                .get("target")
                .and_then(Value::as_str)
                .unwrap_or(source.as_str());
            let _ = event.set(target, replacement.clone());
        }
    }
    true
}

/// Write one params row's columns onto ctx, then refine them by outcome.
///
/// This is Elastic's ECS categorisation pattern: the vendor action selects a row
/// giving `event.kind`, `event.category` and `event.type`, and a tail of
/// guarded statements then adds `allowed` or `denied` and rewrites the outcome
/// into an ECS one. It is `cisco_ftd`'s remaining 398 corpus events, and the
/// same pattern appears wherever a package maps an action onto categorisation.
fn try_row_columns(
    event: &mut Event,
    script: &str,
    params: &Map<String, Value>,
    literals: &Program,
) -> bool {
    let Some(key_path) = row_key_path(script) else {
        return false;
    };
    let Some(key) = event.get_as_string(&key_path) else {
        return true;
    };
    let Some(row) = params.get(&key).and_then(Value::as_object).cloned() else {
        // Every one of these scripts returns early on a key it has no row for.
        return true;
    };

    let mut wrote = false;
    for (path, rhs) in ctx_writes(script) {
        if !rhs.contains("params.get(") {
            continue;
        }
        let Some(column) = rhs
            .rsplit_once(".get('")
            .and_then(|(_, s)| s.split_once('\''))
        else {
            continue;
        };
        if let Some(value) = row.get(column.0) {
            let _ = event.set(&path, value.clone());
            wrote = true;
        }
    }
    if !wrote {
        return false;
    }

    literals.run(event);
    // A script that drops the field it keyed by leaves it behind otherwise.
    for removed in parse_removes(script) {
        event.remove(&removed);
    }
    true
}

/// Everything after the last table read, which is the refinement tail
/// [`try_row_columns`] walks. Empty where the script has no such tail.
fn row_columns_tail(script: &str) -> &str {
    script
        .rfind("params.get(")
        .and_then(|cut| script[cut..].split_once(';'))
        .map_or("", |(_, rest)| rest)
}

/// A table row whose named columns are LISTS appended to array fields, with
/// one copy that runs whether or not the key has a row.
///
/// ```painless
/// def alertTypeId = ctx.json.alertTypeId;
/// def eventData = params.get('eventmap').get(alertTypeId);
/// ctx.event.action = ctx.json.alertType;
/// if (eventData == null) { return; }
/// def eventCategory = eventData.get('category');
/// if (eventCategory != null) { for (def c : eventCategory) { ctx.event.category.add(c); } }
/// ```
///
/// The copy sits ABOVE the early return, so the eleven unclassified alert
/// types `cisco_meraki` ships still get an `event.action`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KeyedRowAppends {
    table: String,
    key: String,
    copy: Option<(String, String)>,
    /// Row column paired with the array field its members are appended to.
    appends: Vec<(String, String)>,
}

/// Read the table, the key it is looked up by, the unconditional copy, and
/// every column-to-field append.
fn parse_keyed_row_appends(script: &str) -> Option<KeyedRowAppends> {
    let table = script
        .split_once("params.get('")
        .and_then(|(_, rest)| rest.split_once('\''))
        .map(|(name, _)| name.to_string())?;
    let table_local = local_bound_to(script, "params.get(")?;
    let key_local = last_call_argument(script, &format!("{table_local}.get("))?;
    let key = ctx_path_between(script, &format!("def {key_local} = ctx."), ";")?;
    let row_local = local_bound_to(script, &format!("{table_local}.get("))?;

    // `ctx.<target> = ctx.<source>;` on its own line. An `.add(` line opens
    // the same way and has no ` = ctx.`, so it cannot be read as one.
    let copy = script.lines().find_map(|line| {
        let statement = line.trim().strip_prefix("ctx.")?.split_once(';')?.0;
        let (target, source) = statement.split_once(" = ctx.")?;
        Some((clean_path(target), clean_path(source)))
    });

    let needle = format!("= {row_local}.get('");
    let mut appends = Vec::new();
    for (at, opener) in script.match_indices(needle.as_str()) {
        let rest = &script[at + opener.len()..];
        let Some((column, tail)) = rest.split_once('\'') else {
            continue;
        };
        let Some(add_at) = tail.find(".add(") else {
            continue;
        };
        let Some((_, path)) = tail[..add_at].rsplit_once("ctx.") else {
            continue;
        };
        appends.push((column.to_string(), clean_path(path)));
    }

    (!appends.is_empty()).then_some(KeyedRowAppends {
        table,
        key,
        copy,
        appends,
    })
}

/// The copy first, then each column's members appended in the row's order.
fn run_keyed_row_appends(
    event: &mut Event,
    pattern: &KeyedRowAppends,
    params: &Map<String, Value>,
) -> bool {
    if let Some((target, source)) = &pattern.copy
        && let Some(value) = event.get(source).cloned()
    {
        let _ = event.set(target, value);
    }

    let Some(rows) = params.get(&pattern.table).and_then(Value::as_object) else {
        return true;
    };
    let Some(key) = event.get_as_string(&pattern.key) else {
        return true;
    };
    let Some(row) = rows.get(&key).and_then(Value::as_object) else {
        return true;
    };
    for (column, field) in &pattern.appends {
        if let Some(Value::Array(items)) = row.get(column) {
            for item in items {
                add_to_list(event, field, item.clone());
            }
        }
    }
    true
}

/// Read `def <t> = params.get('<table>'); def <row> = <t>.get(<key>);`, and
/// the `ctx.` path the key local is itself bound from.
fn parse_keyed_action_row(script: &str) -> Option<ParamsPattern> {
    let table = script
        .split_once("params.get('")
        .and_then(|(_, rest)| rest.split_once('\''))
        .map(|(name, _)| name.to_string())?;
    let table_local = local_bound_to(script, "params.get(")?;
    let key_local = last_call_argument(script, &format!("{table_local}.get("))?;
    let source = ctx_path_between(script, &format!("def {key_local} = ctx."), ";")?;

    Some(ParamsPattern::KeyedActionRow { table, source })
}

/// A key with no row still writes `event.action` and `event.type`, matching
/// the script's own early return rather than leaving the event untouched.
fn run_keyed_action_row(
    event: &mut Event,
    table: &str,
    source: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some(rows) = params.get(table).and_then(Value::as_object) else {
        return true;
    };
    let Some(key) = event.get_as_string(source) else {
        return true;
    };
    let Some(row) = rows.get(&key).and_then(Value::as_object) else {
        let _ = event.set("event.action", Value::String(format!("unknown-{key}")));
        let _ = event.set("event.type", json!(["info"]));
        return true;
    };

    if let Some(value) = row.get("value") {
        let _ = event.set(source, value.clone());
    }
    if let Some(kind) = row.get("type") {
        let _ = event.set("event.type", kind.clone());
    }
    if let Some(Value::Array(added)) = row.get("category") {
        for item in added {
            add_to_list(event, "event.category", item.clone());
        }
    }
    if let Some(action) = row.get("action") {
        let _ = event.set("event.action", action.clone());
    }
    // Every row in the vendor's table carries a classification, so this
    // never meets the null the upstream script's own unguarded
    // `.toLowerCase()` would throw on.
    if let Some(classification) = row.get("classification").and_then(Value::as_str) {
        let parent = source.rsplit_once('.').map_or("", |(head, _)| head);
        let _ = event.set(
            &format!("{parent}.classification"),
            Value::String(classification.to_string()),
        );
        let lower = classification.to_lowercase();
        let outcome = if lower.contains("success") {
            "success"
        } else if lower.contains("failure") {
            "failure"
        } else {
            "unknown"
        };
        let _ = event.set("event.outcome", Value::String(outcome.to_string()));
    }

    true
}

/// A tail of `if (<test>) { ... }` blocks over literal appends and writes,
/// parsed once per CALL SITE.
///
/// The grammar is deliberately tiny, because that is all these tails do once
/// the row is on the event: compare a field to a literal, and append or assign
/// another literal. Anything outside it is left alone rather than guessed at.
///
/// The text is tokenised ONCE, here, never per event. Windows'
/// `security_standard` is four hundred lines of guarded copies, and at the
/// shipped 20,000 events a batch re-reading that text was the whole cost. Every
/// decision the text alone settles -- which pattern a statement is, which path it
/// writes, which literal it compares against -- is resolved into this tree, so
/// the per-event walk only ever reads the event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Program {
    statements: Vec<Stmt>,
    whole: bool,
}

/// One statement of that grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Stmt {
    /// Ends the SCRIPT, not the block. The flag has to travel out of the
    /// recursion: letting the enclosing walk carry on ran the whole body of
    /// every script that opens by returning on the wrong event code.
    Return,
    If {
        test: Guard,
        then: Vec<Stmt>,
        /// Empty where the script wrote no `else`, which then runs nothing.
        alt: Vec<Stmt>,
    },
    Literal(Literal),
}

/// One write the script makes from a value it already has to hand.
///
/// `ctx.a.put('k', v)` and `ctx.a.k = v` name the same write once the key is
/// folded into the path, so they share a variant.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Literal {
    Append {
        path: String,
        value: Rhs,
    },
    Set {
        path: String,
        value: Rhs,
    },
    /// `ctx.<base>.remove('<leaf>')` -- a prune the script makes on its own
    /// account, and the other half of many an `if`/`else` that sets on one arm.
    Remove {
        path: String,
    },
}

/// Where a write's value comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Rhs {
    /// `String.valueOf(ctx.a.b)` is the vendors' spelling of "write this as
    /// text", and aws stamps `management_event` with it.
    Text(String),
    Field(String),
    /// `ctx.event.category = [ctx.<path>]` -- a field wrapped in a one-element
    /// list, which is how a script promotes a scalar into an ECS array field.
    FieldInList(String),
    /// `ctx.<path>.size()` -- how many members a list holds, which a script
    /// writes to a `_count` field beside it. 37 sites over 31 files.
    SizeOf(String),
    /// `ctx.<path> / 1000000` -- rescale a number by a whole factor, which is
    /// how a script moves an epoch between units.
    Scaled {
        path: String,
        factor: i64,
        divide: bool,
    },
    /// `(long) Double.parseDouble(ctx.<path>.toString())` -- coerce to a whole
    /// number, truncating toward zero as the Painless cast does.
    ///
    /// The `.toString()` is the point of the pattern: the field arrives as a
    /// string on some events and a number on others.
    LongOf(String),
    Literal(Value),
}

/// Whether `name` is assigned exactly once in `body` -- its own declaration.
///
/// A local written twice is not a constant, so inlining it would pick one of
/// its values and run that on every event.
fn assigned_once(body: &str, name: &str) -> bool {
    let bytes = body.as_bytes();
    let mut assignments = 0usize;
    let mut at = 0;
    while let Some(found) = body[at..].find(name) {
        let start = at + found;
        at = start + name.len();
        let before = start.checked_sub(1).map(|index| bytes[index]);
        if before.is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_') {
            continue;
        }
        let after = body[at..].trim_start();
        if after.starts_with('=') && !after.starts_with("==") {
            // A SELF-FOLD is not a second value: `level = level.toLowerCase()`
            // narrows what the local already holds, so it stays resolvable.
            if after
                .trim_start_matches('=')
                .trim_start()
                .strip_prefix(name)
                .is_some_and(|tail| tail.trim_start().starts_with(".toLowerCase()"))
            {
                continue;
            }
            assignments += 1;
        }
    }
    assignments == 1
}

/// Whether `name` narrows itself with `<name> = <name>.toLowerCase();`.
fn folds_itself(body: &str, name: &str) -> bool {
    body.contains(&format!("{name} = {name}.toLowerCase()"))
}

/// Replace `needle` with `with`, skipping a match that is the tail of a longer
/// identifier -- `levels.contains(` also sits inside `errorLevels.contains(`.
fn replace_uses(body: &str, needle: &str, with: &str) -> String {
    let bytes = body.as_bytes();
    let mut out = String::with_capacity(body.len());
    let mut at = 0;
    while let Some(found) = body[at..].find(needle) {
        let start = at + found;
        out.push_str(&body[at..start]);
        let before = start.checked_sub(1).map(|index| bytes[index]);
        if before.is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_') {
            out.push_str(needle);
        } else {
            out.push_str(with);
        }
        at = start + needle.len();
    }
    out.push_str(&body[at..]);
    out
}

/// Substitute a local that is just another name for a `ctx.` field.
///
/// `def inputCategory = ctx.falco.output_fields.evt.category;` then
/// `def lowercaseCategory = inputCategory.toLowerCase();` then a guard asking
/// about `lowercaseCategory`. Every hop is a rename, and until they are
/// followed the guard names something no event carries, so it reads as
/// [`Term::Never`].
///
/// Only a local ASSIGNED ONCE and bound to a `ctx.` path, optionally through
/// one case fold, is followed -- anything else is a value this cannot know.
/// Substitution runs forward from each declaration, so a chain resolves in one
/// pass and a local never rewrites its own binding.
fn inline_ctx_aliases(body: &str) -> Cow<'_, str> {
    if !body.contains("ctx") || next_declaration(body, 0).is_none() {
        return Cow::Borrowed(body);
    }

    let mut text = Cow::Borrowed(body);
    // Two passes: the second resolves a local whose binding the first rewrote.
    for _ in 0..2 {
        let Some(next) = one_alias_pass(&text) else {
            break;
        };
        text = Cow::Owned(next);
    }
    text
}

/// How a local is introduced. `def` is the common spelling and a TYPED one is
/// not unusual -- cloudflare opens its epoch ladder with `long t = ...`.
const DECLARATIONS: &[&str] = &["def ", "long ", "int ", "double ", "float ", "String "];

/// The next local declaration at or after `from`, as the offset just past its
/// keyword.
fn next_declaration(body: &str, from: usize) -> Option<usize> {
    DECLARATIONS
        .iter()
        .filter_map(|keyword| {
            body[from..]
                .find(keyword)
                .map(|at| from + at + keyword.len())
        })
        .min()
}

/// One left-to-right sweep, substituting each resolvable local after its own
/// declaration. `None` when nothing was substituted.
fn one_alias_pass(body: &str) -> Option<String> {
    let mut out = body.to_string();
    let mut changed = false;
    let mut at = 0;

    while let Some(declaration) = next_declaration(&out, at) {
        // Offsets are taken against `out` directly. Deriving them from a
        // trimmed name instead loses the whitespace either side of the `=`.
        at = declaration;
        let Some(equals) = out[declaration..].find('=') else {
            break;
        };
        let name = out[declaration..declaration + equals].trim().to_string();
        if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let value_start = declaration + equals + 1;
        let Some(terminator) = out[value_start..].find(';') else {
            continue;
        };
        let value = out[value_start..value_start + terminator]
            .trim()
            .to_string();
        let Some(resolved) = ctx_alias_value(&value) else {
            continue;
        };
        if !assigned_once(&out, &name) {
            continue;
        }
        // The fold travels with the path, and its own statement goes: left in
        // place it would rewrite to `ctx.<p>.toLowerCase() = ...`. It sits
        // AFTER the declaration, so the offsets taken above stay valid.
        let mut resolved = resolved;
        if folds_itself(&out, &name) {
            resolved = format!("{resolved}.toLowerCase()");
            out = out.replace(&format!("{name} = {name}.toLowerCase();"), "");
            changed = true;
        }

        // Forward of the declaration only, so the binding itself is untouched.
        let after = value_start + terminator + 1;
        let rewritten = replace_word(&out[after..], &name, &resolved);
        if rewritten != out[after..] {
            changed = true;
        }
        out = out[..after].to_string() + &rewritten;
        at = after;

        // The binding is dead text once every use of it carries the path, and
        // a declaration the walk cannot run keeps the script from being whole.
        if let Some(without) = drop_inlined_declaration(&out, &name) {
            at = at.min(without.len());
            out = without;
            changed = true;
        }
    }

    changed.then_some(out)
}

/// Close a `.contains(` argument: one terminator, one closing parenthesis.
///
/// Trimming every trailing `)` instead swallows the argument's OWN call, so
/// `ctx.a.toLowerCase()` arrived as `ctx.a.toLowerCase(` and read as a field
/// path with a bracket in it.
fn contains_tail(text: &str) -> &str {
    let text = text.trim();
    let text = text.strip_suffix(';').unwrap_or(text).trim_end();
    text.strip_suffix(')').unwrap_or(text).trim()
}

/// A JSON number as a `long`, the way a Painless cast reads it.
///
/// A float truncates toward zero rather than rounding, and a value past `i64`
/// is not one.
fn whole_of(value: &Value) -> Option<i64> {
    value.as_i64().or_else(|| {
        value
            .as_f64()
            .map(f64::trunc)
            .and_then(|n| (n.abs() < 9.2e18).then_some(n as i64))
    })
}

/// A whole number, however the script spells it: `1000`, `1e3`, `(long)(1e3)`.
///
/// `None` for anything with a fraction or beyond `i64`, so a threshold this
/// cannot represent exactly stays unreadable rather than comparing wrongly.
fn whole_number(text: &str) -> Option<i64> {
    let text = text.trim().trim_end_matches([')', ';']).trim();
    let text = text
        .strip_prefix("(long)")
        .map_or(text, |rest| rest.trim().trim_start_matches('(').trim());
    let text = text.trim_end_matches('L');
    if let Ok(whole) = text.parse::<i64>() {
        return Some(whole);
    }
    let number = text.parse::<f64>().ok()?;
    (number.fract() == 0.0 && number.abs() < 9.2e18).then_some(number as i64)
}

/// Split a trailing `.toLowerCase()` off an expression.
///
/// The fold applies to the EVENT's value at run time, not to this text, so it
/// travels as a flag. `.toUpperCase()` is deliberately NOT read: the members it
/// would be tested against are upper case too, and lowering the value there
/// would stop every one of them matching. It stays unreadable, as it is today.
///
fn strip_case_fold(text: &str) -> (&str, bool) {
    text.strip_suffix(".toLowerCase()")
        .map_or((text, false), |bare| (bare, true))
}

/// The `ctx.` expression a local stands for, if it stands for one.
///
/// A bare path, or a path through one case fold. `.toLowerCase()` is kept
/// rather than applied, because [`Term::parse`] reads it and the fold has to
/// happen against the EVENT's value, not this text.
fn ctx_alias_value(value: &str) -> Option<String> {
    // The path travels UNCAST: the comparisons and arithmetic downstream read
    // the event's own number, so the `(long)` says nothing they need.
    let value = value.strip_prefix("(long)").map_or(value, |rest| {
        rest.trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim()
    });
    let bare = value
        .strip_suffix(".toLowerCase()")
        .or_else(|| value.strip_suffix(".toUpperCase()"))
        .unwrap_or(value);
    if !bare.starts_with("ctx.") && !bare.starts_with("ctx?.") {
        return None;
    }
    // A call or a subscript is a value this cannot follow.
    (!bare.contains(['(', ')', '[', ']', ' ', '\n'])).then(|| value.to_string())
}

/// Drop `def <name> = ...;` once every use of it has been inlined.
///
/// The declaration is not a statement the walk can run, so a script keeping
/// one is never WHOLE and `PlainAssignments` declines it -- which left
/// activemq's level ladder unbound even after its list was read.
///
/// Only when the name appears exactly once in the whole body, which is the
/// declaration itself. Anything else means a use the substitution did not
/// reach, and the binding still has to be there for it.
fn drop_inlined_declaration(body: &str, name: &str) -> Option<String> {
    if word_uses(body, name) != 1 {
        return None;
    }
    let start = DECLARATIONS
        .iter()
        .find_map(|keyword| body.find(&format!("{keyword}{name}")))?;
    let end = body[start..].find(';')? + start + 1;
    let mut out = body[..start].to_string();
    out.push_str(body[end..].trim_start());
    Some(out)
}

/// Whole-word occurrences of `name`, counting a segment after a dot as part of
/// a path rather than a use of the variable.
fn word_uses(body: &str, name: &str) -> usize {
    fn word(c: u8) -> bool {
        c.is_ascii_alphanumeric() || c == b'_'
    }

    let bytes = body.as_bytes();
    let mut seen = 0;
    let mut at = 0;
    while let Some(found) = body[at..].find(name) {
        let start = at + found;
        let end = start + name.len();
        at = end;
        let before = start.checked_sub(1).map(|index| bytes[index]);
        if before.is_some_and(|c| c == b'.' || word(c)) || bytes.get(end).copied().is_some_and(word)
        {
            continue;
        }
        seen += 1;
    }
    seen
}

/// Replace occurrences of `name` used as a VARIABLE.
///
/// `nameSuffix` and `prefixName` are other identifiers. So is `a.name`: a
/// segment after a dot is a member access, and substituting there rewrites the
/// document's own field paths -- a local called `event` would turn every
/// `ctx.event.action` into `ctx.<whatever it was bound to>.action`.
fn replace_word(body: &str, name: &str, with: &str) -> String {
    fn word(c: u8) -> bool {
        c.is_ascii_alphanumeric() || c == b'_'
    }

    let bytes = body.as_bytes();
    let mut out = String::with_capacity(body.len());
    let mut at = 0;
    while let Some(found) = body[at..].find(name) {
        let start = at + found;
        let end = start + name.len();
        out.push_str(&body[at..start]);
        let before = start.checked_sub(1).map(|index| bytes[index]);
        let after = bytes.get(end).copied();
        let member = before.is_some_and(|c| c == b'.');
        if member || before.is_some_and(word) || after.is_some_and(word) {
            out.push_str(name);
        } else {
            out.push_str(with);
        }
        at = end;
    }
    out.push_str(&body[at..]);
    out
}

/// Inline a list literal bound to a local, so the guard that asks it is
/// readable.
///
/// `def errorLevels = ["ERROR", "FATAL"]; ... if (errorLevels.contains(x))` is
/// one literal spelled across two statements. The inline form was read all
/// along and this one was not, so the guard parsed as [`Term::Never`], never
/// held, and every event took the else arm -- `kafka_log` and
/// `elasticsearch_server` stamped `event.type: ["info"]` on their FATAL logs.
///
/// It rewrites ONCE, ahead of the statement walk, so [`readable_term`] and
/// [`Term::parse`] are both handed the inlined form. Teaching one of them
/// alone is exactly how those two walks drift apart (F53).
fn inline_local_lists(body: &str) -> Cow<'_, str> {
    if !body.contains("def ") || !body.contains(".contains(") {
        return Cow::Borrowed(body);
    }

    let mut swaps: Vec<(String, String, String)> = Vec::new();
    let mut at = 0;
    while let Some(found) = body[at..].find("def ") {
        at += found + "def ".len();
        // A `def` with no `=` (`def best;`) runs its name into the statements
        // after it, which the identifier check below refuses.
        let Some((name, tail)) = body[at..].split_once('=') else {
            break;
        };
        let name = name.trim();
        if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let Some((inside, _)) = balanced(tail.trim_start(), '[', ']') else {
            continue;
        };
        let literal = format!("[{inside}]");
        if !matches!(literal_value(&literal), Some(Value::Array(_))) {
            continue;
        }
        let use_site = format!("{name}.contains(");
        if !body.contains(&use_site) || !assigned_once(body, name) {
            continue;
        }
        swaps.push((name.to_string(), use_site, format!("{literal}.contains(")));
    }

    if swaps.is_empty() {
        return Cow::Borrowed(body);
    }
    let mut text = body.to_string();
    for (name, needle, with) in &swaps {
        text = replace_uses(&text, needle, with);
        if let Some(without) = drop_inlined_declaration(&text, name) {
            text = without;
        }
    }
    Cow::Owned(text)
}

impl Program {
    /// Read a body into the tree the per-event walk runs.
    pub(crate) fn parse(body: &str) -> Self {
        // Aliases first: a local list's members are literals, but a local
        // NAMING a field has to be followed before the list can be asked about
        // it.
        let body = inline_ctx_aliases(body);
        let body = inline_local_lists(&body);
        let mut whole = true;
        let statements = parse_statements(&body, &mut whole);
        Self { statements, whole }
    }

    /// Run the tree, reporting whether anything was written -- which is how a
    /// caller tells a script it read from one it walked past.
    pub(crate) fn run(&self, event: &mut Event) -> bool {
        walk(event, &self.statements).0
    }

    /// Whether EVERY statement in the body sits inside the strict subset.
    ///
    /// Running the readable statements of ANY script was tried as a last-resort
    /// catch-all and rejected: a partial read writes a value where the vendor's
    /// whole script would have written a different one, and the corpus said
    /// that is worse than writing nothing. So a script qualifies only when
    /// nothing in it would be silently skipped.
    ///
    /// The subset is NARROWER than what [`Program::run`] executes: it takes no
    /// `return`, no braceless `else if`, no `.add(` or `.put(` form, and no `@`
    /// in a copied path. That is deliberate -- this answer decides whether to
    /// claim a script at all, so it declines wherever it is not certain.
    pub(crate) fn is_whole(&self) -> bool {
        self.whole
    }

    /// Whether any branch holds a write at all.
    ///
    /// A tree of pure guards and returns cannot write whatever the event says,
    /// so a pattern gating on this declines at match time rather than claiming a
    /// script and answering false on every event.
    pub(crate) fn can_write(&self) -> bool {
        fn any_write(statements: &[Stmt]) -> bool {
            statements.iter().any(|statement| match statement {
                Stmt::Literal(_) => true,
                Stmt::If { then, alt, .. } => any_write(then) || any_write(alt),
                Stmt::Return => false,
            })
        }
        any_write(&self.statements)
    }
}

/// Whether one comparison is a form [`Term::holds`] resolves rather than
/// answering `false` by default.
fn readable_term(term: &str) -> bool {
    let term = term.trim().trim_start_matches('!').trim();
    if term.starts_with('[') {
        // A literal list's `.contains`, which reads its argument itself.
        return term.contains(".contains(");
    }
    let Some((subject, wanted)) = term
        .split_once("==")
        .or_else(|| term.split_once("!="))
        .or_else(|| term.split_once(".contains("))
    else {
        // A bare field, whose boolean value is the test.
        let path = subject_path(term);
        return path.starts_with("ctx.")
            && path[4..]
                .chars()
                .all(|c| c.is_alphanumeric() || "._?".contains(c));
    };
    let wanted = wanted.trim().trim_end_matches(')').trim();
    subject_path(subject).trim().starts_with("ctx.")
        && (wanted.starts_with("ctx.")
            || wanted == "null"
            || crate::painless_common::painless_literal(wanted).is_some())
}

/// Whether one statement sits inside the strict subset [`Program::is_whole`]
/// answers for.
///
/// Container allocation (`ctx.a = new HashMap()`) counts as runnable and does
/// nothing: `Event::set` builds the parents a later write needs.
///
/// A statement the HANDLER parses is runnable by definition, so it is asked
/// first and the rest of this only has to cover what parses to nothing.
/// Answering that question twice is what F53 named: `.add(` was handled and
/// refused here, so `couchbase_cache`'s `ctx.tags.add(...)` made its script
/// un-whole and nothing claimed it.
fn statement_is_runnable(statement: &str) -> bool {
    if parse_literal_statement(statement).is_some() {
        return true;
    }
    let Some((subject, value)) = split_assignment(statement) else {
        return false;
    };
    if subject.trim().strip_prefix("ctx.").is_none() {
        return false;
    }
    let value = value.trim().trim_end_matches(';').trim();
    if value.starts_with("new HashMap(") || value.starts_with("new ArrayList(") {
        return true;
    }
    if let Some(inner) = value
        .strip_prefix("String.valueOf(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        return inner.trim().starts_with("ctx.");
    }
    // Anything else has to be a value the walk can actually resolve.
    literal_value(value).is_some()
        || value.strip_prefix("ctx.").is_some_and(|path| {
            path.chars()
                .all(|c| c.is_alphanumeric() || "._?['\"]".contains(c))
        })
}

/// Read a body's statements, and decide [`Program::is_whole`] on the way.
///
/// ONE walk over the text answers both. The tree and the strict gate used to be
/// separate functions over the same grammar, which meant a pattern taught to one
/// could silently miss the other.
fn parse_statements(body: &str, whole: &mut bool) -> Vec<Stmt> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(offset) = rest.find(|c: char| !c.is_whitespace()) {
        rest = &rest[offset..];

        // A comment is not a statement. Reading one as a statement swallowed
        // everything up to the next `;` -- which is INSIDE the block after it,
        // so the walk resumed mid-block and the vendor's commented scripts ran
        // nothing.
        if let Some(after) = rest.strip_prefix("//") {
            rest = after.find('\n').map_or("", |at| &after[at + 1..]);
            continue;
        }
        if let Some(after) = rest.strip_prefix("/*") {
            rest = after.find("*/").map_or("", |at| &after[at + 2..]);
            continue;
        }

        if rest.starts_with("return") {
            // Nothing past it is reachable, so nothing past it is parsed.
            *whole = false;
            out.push(Stmt::Return);
            return out;
        }
        if let Some(after) = rest.strip_prefix("if") {
            let Some((test, after)) = balanced(after.trim_start(), '(', ')') else {
                *whole = false;
                return out;
            };
            let Some((block, after)) = balanced(after.trim_start(), '{', '}') else {
                *whole = false;
                return out;
            };
            // Every comparison has to be one the evaluator can decide, or the
            // walk takes an arm on a coin toss.
            if !test
                .split("||")
                .flat_map(|clause| clause.split("&&"))
                .all(readable_term)
            {
                *whole = false;
            }
            // An `else` arm when there is one. `else if` has no braces of its
            // own, so the whole tail becomes the alternative and the recursion
            // reads it as another `if`.
            let (alternative, after) = match after.trim_start().strip_prefix("else") {
                Some(tail) => {
                    if let Some((body, rest)) = balanced(tail.trim_start(), '{', '}') {
                        (body, rest)
                    } else {
                        *whole = false;
                        (tail, "")
                    }
                }
                None => ("", after),
            };
            out.push(Stmt::If {
                test: Guard::parse(test),
                then: parse_statements(block, whole),
                alt: parse_statements(alternative, whole),
            });
            rest = after;
            continue;
        }

        // A plain statement, up to its terminator.
        let end = rest.find(';').unwrap_or(rest.len());
        let statement = &rest[..end];
        rest = &rest[(end + 1).min(rest.len())..];
        if statement.is_empty() {
            continue;
        }
        if !statement_is_runnable(statement.trim()) {
            *whole = false;
        }
        // A statement no reader resolves could never write, so the tree simply
        // does not carry it.
        if let Some(literal) = parse_literal_statement(statement) {
            out.push(Stmt::Literal(literal));
        }
    }
    out
}

/// Run parsed statements, reporting whether anything was written and whether a
/// `return` was reached.
///
/// The second flag has to travel out of the recursion: a `return` inside a
/// block ends the SCRIPT, and letting the enclosing walk carry on ran the whole
/// body of every script that opens by returning on the wrong event code.
fn walk(event: &mut Event, statements: &[Stmt]) -> (bool, bool) {
    let mut wrote = false;
    for statement in statements {
        match statement {
            Stmt::Return => return (wrote, true),
            Stmt::If { test, then, alt } => {
                let branch = if test.holds(event) { then } else { alt };
                let (branch_wrote, returned) = walk(event, branch);
                wrote |= branch_wrote;
                if returned {
                    return (wrote, true);
                }
            }
            Stmt::Literal(literal) => wrote |= run_literal(event, literal),
        }
    }
    (wrote, false)
}

/// Split `text` at the region opened by `open` and closed by its match.
pub(crate) fn balanced(text: &str, open: char, close: char) -> Option<(&str, &str)> {
    let mut chars = text.char_indices();
    let (_, first) = chars.next()?;
    if first != open {
        return None;
    }
    let mut depth = 1usize;
    for (index, c) in chars {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some((
                    &text[open.len_utf8()..index],
                    &text[index + close.len_utf8()..],
                ));
            }
        }
    }
    None
}

/// Rewrite Painless's map syntax into the dotted form paths are read in.
///
/// Only where it is subscripting with a literal: `ctx['@timestamp']` is a path
/// and `params[net.transport]` is a lookup, and the difference is the quotes.
pub(crate) fn subject_path(term: &str) -> String {
    // Null-safe navigation goes too: `ctx?.event` is `ctx.event`, and leaving
    // the `?` on defeats the `ctx.` prefix every reader below strips.
    term.replace("?.", ".")
        .replace("['", ".")
        .replace("[\"", ".")
        .replace("']", "")
        .replace("\"]", "")
}

/// An `if` test: `||` of `&&` of comparisons, resolved as far as the text goes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard(Vec<Vec<Term>>);

/// One comparison, with everything the text settles already settled.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Term {
    /// A leading `!`, inverting what it wraps.
    Not(Box<Term>),
    /// `["4778", "4779"].contains(ctx.event.code)`, the early-return gate.
    ListContains {
        members: Vec<String>,
        argument: Argument,
    },
    /// `ctx.related.user.contains(<argument>)`, the append-once guard.
    FieldContains { path: String, argument: Argument },
    /// `ctx.<path> == <wanted>`, with the `!=` spelling as `negated`.
    Compare {
        path: String,
        wanted: Wanted,
        negated: bool,
        /// `ctx.<p>.toLowerCase() == "red"` compares the FOLDED value. The
        /// script's own literal is already lower case, so only the event's
        /// side folds.
        lowered: bool,
    },
    /// A bare field IS the test: Painless reads its boolean value, and arista
    /// gates its whole outcome ladder on `if (ctx.arista.blocked)`.
    Truthy(String),
    /// `ctx.<path> > 1e18` -- a magnitude test, which is how a script tells
    /// nanoseconds from seconds without being told the unit.
    ///
    /// Read BEFORE the bare-field fallback, or the whole comparison becomes a
    /// `Truthy` on a path with an operator in it, which no event carries and
    /// which the dead-branch census cannot see.
    /// The threshold is an `i64` because `Term` is `Eq` and because every
    /// spelling these scripts use -- `1e18`, `1e10`, `1e3` -- is a whole
    /// number. One that is not stays unreadable.
    Magnitude {
        path: String,
        than: i64,
        greater: bool,
    },
    /// `ctx.<path> instanceof List` -- a type test the document answers.
    ///
    /// Only the three container kinds, because JSON settles those exactly.
    /// `instanceof long` is NOT read: a JSON number carries no width, so a
    /// `long` test cannot be told from a `double` one here.
    InstanceOf { path: String, kind: JsonKind },
    /// Nothing the text resolves, so no event can make it hold.
    Never,
}

/// The JSON kinds an `instanceof` can be answered from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JsonKind {
    List,
    Map,
    Text,
}

impl JsonKind {
    /// The Painless type names that settle to one JSON kind.
    fn parse(name: &str) -> Option<Self> {
        match name.trim() {
            "List" | "Collection" | "ArrayList" => Some(Self::List),
            "Map" | "HashMap" => Some(Self::Map),
            "String" => Some(Self::Text),
            _ => None,
        }
    }

    fn matches(self, value: &Value) -> bool {
        match self {
            Self::List => value.is_array(),
            Self::Map => value.is_object(),
            Self::Text => value.is_string(),
        }
    }
}

/// What a `.contains(` is asked about.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Argument {
    Literal(String),
    Field(String),
    /// A field asked about in lower case, which is how a script tests
    /// membership of a lower-cased list against a vendor's mixed-case value.
    LoweredField(String),
}

/// The right-hand side of an `==` or `!=`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Wanted {
    Null,
    Text(String),
    /// A bare `true` / `false` / number is as common a right-hand side as a
    /// quoted string, and reading only the quoted form made every one of them
    /// compare FALSE -- `carbon_black`'s netconn direction took the wrong arm of
    /// its `== true` and wrote source and destination the wrong way round.
    Value(Value),
    /// Another `ctx.` field, compared by VALUE and only where both sides are
    /// scalars.
    ///
    /// An ingest `if` reads a read-only view of the document and every access
    /// to a nested container builds a fresh wrapper with no `equals`, so two
    /// containers never compare equal there however identical their contents.
    /// `codegen_api::condition_eq` answers the same way for generated
    /// conditionals.
    Field(String),
    /// A side no literal reader resolves. It equals nothing the event holds,
    /// which makes the `!=` spelling of it TRUE.
    Unreadable,
}

/// Evaluate one `if` test straight from its text.
///
/// The branch resolver in [`crate::painless_common`] rewrites a script per
/// event, so its tests have no call site that could hold a parsed [`Guard`].
pub(crate) fn guard_holds(event: &Event, test: &str) -> bool {
    Guard::parse(test).holds(event)
}

impl Guard {
    fn parse(test: &str) -> Self {
        Self(
            test.split("||")
                .map(|conjunction| {
                    conjunction
                        .split("&&")
                        .map(|term| Term::parse(term.trim()))
                        .collect()
                })
                .collect(),
        )
    }

    fn holds(&self, event: &Event) -> bool {
        self.0
            .iter()
            .any(|conjunction| conjunction.iter().all(|term| term.holds(event)))
    }
}

impl Term {
    fn parse(term: &str) -> Self {
        if let Some(inner) = term.strip_prefix('!') {
            // `!x.contains(y)` -- a bare `!ctx.field` is not a pattern these use.
            return Self::Not(Box::new(Self::parse(inner.trim())));
        }
        // A LITERAL list opens every early-return gate these scripts write, and
        // is read FIRST because the rewrite below turns its `["` into a
        // separator -- which matched nothing, so the gate always held.
        if term.starts_with('[')
            && let Some((list, argument)) = term.split_once(".contains(")
            && let Some(Value::Array(members)) = literal_value(list)
        {
            let argument = contains_tail(argument);
            let argument = if let Some(text) = quoted_after(argument, "") {
                Argument::Literal(text)
            } else {
                // Normalised on the ARGUMENT alone, before the `ctx.` strip:
                // the term's own leading `["` is a list, not a path segment.
                let argument = subject_path(argument);
                let (argument, lowered) = strip_case_fold(&argument);
                match argument.strip_prefix("ctx.") {
                    Some(path) if lowered => Argument::LoweredField(clean_path(path)),
                    Some(path) => Argument::Field(clean_path(path)),
                    None => return Self::Never,
                }
            };
            return Self::ListContains {
                members: members
                    .iter()
                    .filter_map(|member| member.as_str().map(str::to_string))
                    .collect(),
                argument,
            };
        }

        // `ctx['@timestamp']` and `ctx.event.action` name the same kind of
        // thing.
        let term = &subject_path(term);
        if let Some((subject, kind)) = term.split_once(" instanceof ")
            && let Some(path) = subject
                .trim()
                .trim_start_matches('(')
                .trim()
                .strip_prefix("ctx.")
            && let Some(kind) = JsonKind::parse(kind.trim().trim_end_matches([')', ';']))
        {
            return Self::InstanceOf {
                path: clean_path(path),
                kind,
            };
        }
        if let Some((subject, literal)) = term.split_once(".contains(") {
            // A field argument is the append-once guard these scripts write.
            let argument = if let Some(text) = quoted_after(literal, "") {
                Argument::Literal(text)
            } else {
                let field = contains_tail(literal);
                let (field, lowered) = strip_case_fold(field);
                match field.strip_prefix("ctx.") {
                    Some(path) if lowered => Argument::LoweredField(clean_path(path)),
                    Some(path) => Argument::Field(clean_path(path)),
                    None => return Self::Never,
                }
            };
            let Some(path) = subject.trim().strip_prefix("ctx.") else {
                return Self::Never;
            };
            return Self::FieldContains {
                path: clean_path(path),
                argument,
            };
        }
        // Before the `==` pair, because `>=` and `<=` carry an `=` that the
        // split below would take for the start of an equality test.
        for (operator, greater) in [(">=", true), ("<=", false), (">", true), ("<", false)] {
            let Some((subject, threshold)) = term.split_once(operator) else {
                continue;
            };
            let Some(path) = subject.trim().strip_prefix("ctx.") else {
                continue;
            };
            let Some(than) = whole_number(threshold) else {
                continue;
            };
            return Self::Magnitude {
                path: clean_path(path),
                than,
                greater,
            };
        }
        for (operator, negated) in [("==", false), ("!=", true)] {
            let Some((subject, wanted)) = term.split_once(operator) else {
                continue;
            };
            // `ctx.<p>.toLowerCase() == "red"` compares the FOLDED value, and
            // reading the fold as part of the path named a field no event
            // carries -- gdacs's alert ladder took its else arm on every event.
            let (subject, lowered) = strip_case_fold(subject.trim());
            let Some(path) = subject.strip_prefix("ctx.") else {
                return Self::Never;
            };
            let wanted = wanted.trim();
            let wanted = if wanted == "null" {
                Wanted::Null
            } else if let Some(literal) = quoted_after(wanted, "") {
                Wanted::Text(literal)
            } else if let Some(other) = wanted.strip_prefix("ctx.") {
                // Read the same way as the subject above, so one side of a
                // comparison cannot resolve a spelling the other refuses.
                Wanted::Field(clean_path(other))
            } else {
                // `literal_value` reads strings and lists only, so a bare
                // `true` or a number needs the wider reader.
                crate::painless_common::painless_literal(wanted)
                    .map_or(Wanted::Unreadable, Wanted::Value)
            };
            return Self::Compare {
                path: clean_path(path),
                wanted,
                negated,
                lowered,
            };
        }
        // A BARE field is the test: Painless reads its boolean value. arista
        // gates its whole outcome ladder on `if (ctx.arista.blocked)`, and
        // answering false here took the else arm on every event.
        // Checked, because this is the LAST arm: an unchecked one made a
        // Truthy of any leftover text and hid 18 unreadable guards.
        if let Some(path) = subject_path(term).strip_prefix("ctx.")
            && path
                .chars()
                .all(|c| c.is_alphanumeric() || "._?@['\"]".contains(c))
        {
            return Self::Truthy(clean_path(path));
        }
        Self::Never
    }

    fn holds(&self, event: &Event) -> bool {
        match self {
            Self::Not(inner) => !inner.holds(event),
            Self::Never => false,
            Self::ListContains { members, argument } => {
                let Some(wanted) = argument.resolve(event) else {
                    return false;
                };
                members.iter().any(|member| member == wanted.as_ref())
            }
            Self::FieldContains { path, argument } => {
                let Some(wanted) = argument.resolve(event) else {
                    return false;
                };
                match event.get(path) {
                    Some(Value::Array(items)) => items
                        .iter()
                        .any(|item| item.as_str() == Some(wanted.as_ref())),
                    Some(Value::String(text)) => text.contains(wanted.as_ref()),
                    _ => false,
                }
            }
            Self::Compare {
                path,
                wanted,
                negated,
                lowered,
            } => {
                let held = event.get(path);
                let matched = match wanted {
                    Wanted::Null => held.is_none_or(Value::is_null),
                    Wanted::Text(text) if *lowered => {
                        held.and_then(Value::as_str).map(str::to_lowercase) == Some(text.clone())
                    }
                    Wanted::Text(text) => held.and_then(Value::as_str) == Some(text.as_str()),
                    Wanted::Value(value) => held == Some(value),
                    // Absent and explicitly null are one value and DO compare
                    // equal; two containers never do.
                    Wanted::Field(other) => match (held, event.get(other)) {
                        (Some(Value::Object(_) | Value::Array(_)), _)
                        | (_, Some(Value::Object(_) | Value::Array(_))) => false,
                        (left, right) => {
                            left.unwrap_or(&Value::Null) == right.unwrap_or(&Value::Null)
                        }
                    },
                    Wanted::Unreadable => false,
                };
                matched != *negated
            }
            Self::Truthy(path) => event.get(path).and_then(Value::as_bool) == Some(true),
            // An absent field is not an instance of anything, which is what
            // Painless answers for a null too.
            Self::InstanceOf { path, kind } => event.get(path).is_some_and(|v| kind.matches(v)),
            // Compared as a `long`, which is what the script cast it to, and
            // exactly: 1e18 is past the integer f64 represents without loss.
            // A field that is not a number is neither greater nor less.
            Self::Magnitude {
                path,
                than,
                greater,
            } => event.get(path).and_then(whole_of).is_some_and(|value| {
                if *greater {
                    value > *than
                } else {
                    value < *than
                }
            }),
        }
    }
}

impl Argument {
    /// Borrowed for the literal spelling, which is most of them, so the common
    /// term costs no allocation per event.
    fn resolve<'a>(&'a self, event: &Event) -> Option<Cow<'a, str>> {
        match self {
            Self::Literal(text) => Some(Cow::Borrowed(text)),
            Self::Field(path) => event.get_as_string(path).map(Cow::Owned),
            Self::LoweredField(path) => event
                .get_as_string(path)
                .map(|text| Cow::Owned(text.to_lowercase())),
        }
    }
}

/// One statement that writes a value the script already has to hand.
///
/// Four patterns, and the value is either a literal or another `ctx.` field:
/// `ctx.a.add(v)`, `ctx.a.put('k', v)`, `ctx.a = v`, and the `.put` and `.add`
/// forms with a copied source. Anything else is left alone.
/// The field a `ctx.<base>.remove('<leaf>')` prunes.
///
/// One reader for the gate and the handler both, because two hand-written
/// answers to "can this statement run" is the drift F53 named. `ctx.remove(k)`
/// prunes a top-level key and has no base.
fn removed_path(statement: &str) -> Option<String> {
    let (subject, argument) = statement.split_once(".remove(")?;
    let leaf = quoted_after(argument, "")?;
    // A `remove` on anything but the document is a list operation, which is a
    // different pattern with its own matcher.
    if argument
        .trim_start()
        .starts_with(|c: char| c.is_ascii_digit())
    {
        return None;
    }
    match subject.trim() {
        "ctx" => Some(leaf),
        subject => Some(format!(
            "{}.{leaf}",
            clean_path(subject.strip_prefix("ctx.")?)
        )),
    }
}

fn parse_literal_statement(statement: &str) -> Option<Literal> {
    if let Some(path) = removed_path(statement) {
        return Some(Literal::Remove { path });
    }
    if let Some((subject, argument)) = statement.split_once(".add(") {
        return Some(Literal::Append {
            path: clean_path(subject.trim().strip_prefix("ctx.")?),
            value: parse_rhs(argument)?,
        });
    }
    // `ctx.user.put("name", ctx.winlog.event_data.SubjectUserName)`. The
    // null-guard blocks these scripts open with -- `ctx.put("user", hm)` --
    // fall out here: the subject is bare `ctx` and the value is a local.
    if let Some((subject, arguments)) = statement.split_once(".put(") {
        let parent = clean_path(subject.trim().strip_prefix("ctx.")?);
        let key = quoted_after(arguments, "")?;
        let (_, rest) = arguments.split_once(',')?;
        return Some(Literal::Set {
            path: format!("{parent}.{key}"),
            value: parse_rhs(rest)?,
        });
    }
    let (subject, value) = split_assignment(statement)?;
    Some(Literal::Set {
        path: clean_path(subject.trim().strip_prefix("ctx.")?),
        value: parse_rhs(value)?,
    })
}

/// The value a statement writes: a literal, or a `ctx.` field to read off the
/// event.
fn parse_rhs(text: &str) -> Option<Rhs> {
    let text = text.trim().trim_end_matches(';').trim();
    if let Some(inner) = text
        .strip_prefix("String.valueOf(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        return Some(Rhs::Text(clean_path(inner.trim().strip_prefix("ctx.")?)));
    }
    // `ctx.a.size()`, before the bare-path read below takes the call for part
    // of the name.
    if let Some(path) = text.strip_suffix(".size()").and_then(|head| {
        subject_path(head.trim())
            .strip_prefix("ctx.")
            .map(str::to_owned)
    }) && !path.contains(['(', ')', ' '])
    {
        return Some(Rhs::SizeOf(clean_path(&path)));
    }
    // `ctx.a / 1000` and `ctx.a * 1000`, before the bare-path read below,
    // which would otherwise take the whole expression for a field name.
    for (operator, divide) in [('/', true), ('*', false)] {
        let Some((left, right)) = text.split_once(operator) else {
            continue;
        };
        let Some(path) = subject_path(left.trim())
            .strip_prefix("ctx.")
            .map(str::to_owned)
        else {
            continue;
        };
        let Some(factor) = whole_number(right) else {
            continue;
        };
        if factor == 0 || path.contains(['(', ')', ' ']) {
            continue;
        }
        return Some(Rhs::Scaled {
            path: clean_path(&path),
            factor,
            divide,
        });
    }
    if let Some(inner) = text
        .strip_prefix("(long)")
        .map(str::trim)
        .and_then(|rest| rest.strip_prefix("Double.parseDouble("))
        .and_then(|rest| rest.strip_suffix(')'))
        .map(|rest| rest.trim().trim_end_matches(".toString()").trim())
        .and_then(|rest| subject_path(rest).strip_prefix("ctx.").map(str::to_owned))
        && !inner.contains(['(', ')', ' '])
    {
        return Some(Rhs::LongOf(clean_path(&inner)));
    }
    // `[ctx.a.b]` before the bare form, or the brackets read as path syntax.
    if let Some(inner) = text
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .map(str::trim)
        .and_then(|inner| subject_path(inner).strip_prefix("ctx.").map(str::to_owned))
        && !inner.contains([',', '(', ' '])
    {
        return Some(Rhs::FieldInList(clean_path(&inner)));
    }
    let text = text.trim_end_matches(')').trim();
    if let Some(path) = text.strip_prefix("ctx.") {
        // A source path and nothing else. `ctx.a + ctx.b` and a method call
        // on one are different patterns with their own matchers.
        return path
            .chars()
            .all(|c| c.is_alphanumeric() || "._?@['\"]".contains(c))
            .then(|| Rhs::Field(clean_path(path)));
    }
    literal_value(text).map(Rhs::Literal)
}

/// Make one parsed write, reporting whether the event had a value for it.
fn run_literal(event: &mut Event, literal: &Literal) -> bool {
    match literal {
        Literal::Append { path, value } => {
            let Some(value) = resolve_rhs(event, value) else {
                return false;
            };
            add_to_list(event, path, value);
            true
        }
        Literal::Set { path, value } => {
            let Some(value) = resolve_rhs(event, value) else {
                return false;
            };
            let _ = event.set(path, value);
            true
        }
        // `Event::remove` is the funnel that uses `shift_remove`, so a prune
        // cannot reorder the document it is cleaning.
        Literal::Remove { path } => event.remove(path).is_some(),
    }
}

/// A write's value, read off the event where the script named a field.
fn resolve_rhs(event: &Event, value: &Rhs) -> Option<Value> {
    match value {
        Rhs::Text(path) => event.get(path).and_then(scalar_text).map(Value::String),
        Rhs::Field(path) => event.get(path).cloned(),
        Rhs::FieldInList(path) => event
            .get(path)
            .cloned()
            .map(|value| Value::Array(vec![value])),
        // A map counts its keys and a list its members, the way Painless does.
        // Anything else has no size and writes nothing.
        Rhs::SizeOf(path) => event.get(path).and_then(|value| match value {
            Value::Array(items) => Some(Value::from(items.len())),
            Value::Object(entries) => Some(Value::from(entries.len())),
            _ => None,
        }),
        // Integer arithmetic, as Painless does it on a `long`: the division
        // truncates rather than producing a fraction.
        Rhs::Scaled {
            path,
            factor,
            divide,
        } => event.get(path).and_then(|value| {
            // A whole number keeps integer arithmetic, exactly as Painless
            // does for `long * long`.
            if let Some(whole) = value.as_i64() {
                return Some(Value::from(if *divide {
                    whole / factor
                } else {
                    whole * factor
                }));
            }
            // A FRACTIONAL source widens to a double before multiplying.
            // Reading it as an integer declined and left the field unscaled --
            // gitlab times a request at 0.01969 seconds and publishes 19,690.
            let n = match value {
                Value::String(text) => text.trim().parse::<f64>().ok()?,
                other => other.as_f64()?,
            };
            #[allow(clippy::cast_precision_loss)]
            let factor = *factor as f64;
            Some(Value::from(if *divide { n / factor } else { n * factor }))
        }),
        // A value that will not parse is left alone, the way the vendor's
        // script throws and its processor's `on_failure` leaves the field.
        Rhs::LongOf(path) => event
            .get(path)
            .and_then(|value| match value {
                Value::Number(number) => number.as_f64(),
                Value::String(text) => text.trim().parse::<f64>().ok(),
                _ => None,
            })
            .map(|number| Value::from(number.trunc() as i64)),
        Rhs::Literal(value) => Some(value.clone()),
    }
}

/// A quoted string, or a bracketed list of them.
///
/// `ctx.event.type = ['info']` is as common as the scalar form and was read as
/// the bare string `info`, so the field came out a string where Elastic writes
/// a one-element array.
fn literal_value(text: &str) -> Option<Value> {
    let text = text.trim();
    // Painless reads a present-but-null field as null, and several pipelines
    // write one deliberately so their own drop-empty pass takes the field.
    if text == "null" {
        return Some(Value::Null);
    }
    let Some(inner) = text
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
    else {
        return quoted_after(text, "").map(Value::String);
    };
    let members: Vec<Value> = inner
        .split(',')
        .filter_map(|member| quoted_after(member, ""))
        .map(Value::String)
        .collect();
    (!members.is_empty()).then_some(Value::Array(members))
}

/// Fan an already-parsed key/value message out through a params table.
///
/// Cisco's FTD security events arrive as `Key: value, Key: value` pairs that an
/// earlier processor lifts into a map. Each vendor key has a row in params
/// naming the vendor field it becomes, the ECS fields it also feeds, and the
/// message ids it is evidence for -- and when the header carried no id, the id
/// with the most evidence is the message's. Nothing about which keys exist is
/// written here: it is all read from the params the pipeline ships.
///
/// This one script is the whole security-event family -- connection, file,
/// malware, intrusion and DNS -- 398 of `cisco_ftd`'s 432 corpus events.
fn try_keyed_message_table(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let script = expand_locals(script);
    let Some(source) = ctx_path_before(&script, ".entrySet()") else {
        return false;
    };
    let Some(message) = event.get(&source).and_then(Value::as_object).cloned() else {
        // The processor's `if` guards the map being absent, so nothing to read
        // means the path came out wrong rather than the event being quiet.
        return false;
    };

    // Which of the two maps a row lands in is decided by a list written into
    // the script, and each map's destination by the local it was assigned to.
    let writes = ctx_writes(&script);
    let Some(list_local) = identifier_before(&script, ".contains(param.target)") else {
        return false;
    };
    let listed = script_string_list(&script, &list_local);
    let Some(listed_local) = local_indexed_by(&script, "contains(param.target)){") else {
        return false;
    };
    let Some(other_local) = local_indexed_by(&script, "else{") else {
        return false;
    };
    let destination = |local: &str| {
        writes
            .iter()
            .find(|(_, rhs)| rhs == local)
            .map(|(path, _)| path.clone())
    };
    let (Some(listed_path), Some(other_path)) =
        (destination(&listed_local), destination(&other_local))
    else {
        return false;
    };

    let mut listed_map = Map::new();
    let mut other_map = Map::new();
    let mut counters: Vec<(String, usize)> = Vec::new();

    for (key, value) in &message {
        let Some(row) = params.get(key).and_then(Value::as_object) else {
            continue;
        };
        // Counted even for an empty value: the key's presence is the evidence.
        for id in row
            .get("id")
            .and_then(Value::as_array)
            .unwrap_or(&Vec::new())
        {
            let Some(id) = id.as_str() else { continue };
            match counters.iter_mut().find(|(name, _)| name == id) {
                Some((_, count)) => *count += 1,
                None => counters.push((id.to_string(), 1)),
            }
        }
        if is_painless_empty(value) {
            continue;
        }
        for field in row
            .get("ecs")
            .and_then(Value::as_array)
            .unwrap_or(&Vec::new())
        {
            if let Some(field) = field.as_str() {
                append_or_create(event, field, value.clone());
            }
        }
        let Some(target) = row.get("target").and_then(Value::as_str) else {
            continue;
        };
        if listed.iter().any(|name| name == target) {
            listed_map.insert(target.to_string(), value.clone());
        } else {
            other_map.insert(target.to_string(), value.clone());
        }
    }

    let _ = event.set(&listed_path, Value::Object(listed_map));
    let _ = event.set(&other_path, Value::Object(other_map));

    // The header's own id wins where there was one; the vote is the fallback.
    let Some((decided, _)) = writes.iter().find(|(_, rhs)| rhs.ends_with("getKey()")) else {
        return true;
    };
    if event
        .get_as_string(decided)
        .is_some_and(|current| !current.is_empty())
    {
        return true;
    }
    if let Some((best, _)) = counters.iter().max_by_key(|(_, count)| *count) {
        let _ = event.set(decided, best.clone());
    }
    true
}

/// Painless `isEmpty`: no members, or no characters.
fn is_painless_empty(value: &Value) -> bool {
    match value {
        Value::Array(items) => items.is_empty(),
        Value::String(text) => text.is_empty(),
        Value::Null => true,
        _ => false,
    }
}

/// Write `value` at `path`, growing a list where one is already there.
/// `.add()` on a list the script created above it, so the first member lands
/// in a ONE-ELEMENT ARRAY.
///
/// Different from [`append_or_create`], which is cisco's own `appendOrCreate`
/// helper and stores the first value BARE. Sharing one of them left
/// `related.ip` and `related.user` a string wherever a logon event contributed
/// exactly one of each.
fn add_to_list(event: &mut Event, path: &str, value: Value) {
    let grown = match event.get(path) {
        Some(Value::Array(existing)) => {
            let mut items = existing.clone();
            items.push(value);
            Value::Array(items)
        }
        Some(existing) => Value::Array(vec![existing.clone(), value]),
        None => Value::Array(vec![value]),
    };
    let _ = event.set(path, grown);
}

fn append_or_create(event: &mut Event, path: &str, value: Value) {
    let grown = match event.get(path) {
        None => value,
        Some(Value::Array(existing)) => {
            let mut items = existing.clone();
            items.push(value);
            Value::Array(items)
        }
        Some(existing) => Value::Array(vec![existing.clone(), value]),
    };
    let _ = event.set(path, grown);
}

/// The quoted strings of the `new ArrayList([...])` bound to `local`.
///
/// Named rather than positional: the helper the vendor defines above the loop
/// builds its own `new ArrayList([existing, value])`, and taking the first
/// literal in the script finds that one and reads an empty list out of it.
fn script_string_list(script: &str, local: &str) -> Vec<String> {
    let binding = format!("def {local} = new ArrayList([");
    let Some(start) = script.find(&binding) else {
        return Vec::new();
    };
    let tail = &script[start + binding.len()..];
    let end = tail.find("])").unwrap_or(tail.len());
    tail[..end]
        .split(',')
        .filter_map(|item| {
            let item = item.trim();
            item.strip_prefix('\'')
                .and_then(|item| item.strip_suffix('\''))
                .map(str::to_string)
        })
        .collect()
}

/// The identifier immediately preceding `marker`.
fn identifier_before(script: &str, marker: &str) -> Option<String> {
    let head = &script[..script.find(marker)?];
    let name: String = head
        .chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then(|| name.chars().rev().collect())
}

/// The local a `<local>[param.target]` write names, just after `marker`.
fn local_indexed_by(script: &str, marker: &str) -> Option<String> {
    let tail = &script[script.find(marker)? + marker.len()..];
    let name = tail[..tail.find("[param.target]")?].trim();
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| name.to_string())
}

/// A table read in either direction, depending on which side the field holds.
///
/// Cisco's ASA and FTD pipelines map `network.transport` to its IANA protocol
/// number, and if the device wrote the NUMBER there instead, put the number in
/// `network.iana_number` and the name back in `network.transport`. It is worth
/// its own matcher because it is the difference between a connection event
/// having a protocol and not: it was wrong in 414 of 512 ASA events.
///
/// Everything is read out of the script -- which field, which two destinations,
/// and the table itself -- so the vendor renaming or extending any of them is
/// picked up by regenerating rather than by editing this.
fn try_reversible_lookup(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let script = expand_locals(script);
    let Some(source) = ctx_path_between(&script, "params[ctx.", "]") else {
        return false;
    };
    let Some(forward_local) = local_bound_to(&script, "params[") else {
        return false;
    };
    let writes = ctx_writes(&script);

    let Some(forward_target) = writes
        .iter()
        .find(|(_, rhs)| *rhs == forward_local)
        .map(|(path, _)| path.clone())
    else {
        return false;
    };

    let Some(key) = event.get_as_string(&source) else {
        // The processor's own `if` guards the field being absent, so reaching
        // here with nothing to read means the path came out wrong.
        return false;
    };

    if let Some(value) = params.get(&key) {
        let _ = event.set(&forward_target, value.clone());
        return true;
    }

    // The reverse table is built by value, so the field already holds a number
    // and the name has to be put back. Both destinations come from the script:
    // the one assigned the field itself, and the one assigned the lookup.
    let Some(name) = params
        .iter()
        .find(|(_, value)| scalar_text(value).as_deref() == Some(key.as_str()))
        .map(|(name, _)| name.clone())
    else {
        return true;
    };
    let echo = format!("ctx.{source}");
    for (path, rhs) in &writes {
        if *rhs == echo {
            // What the field already held, moved across untouched.
            let _ = event.set(path, key.clone());
        } else if *rhs != forward_local
            && !rhs.is_empty()
            && rhs.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            // The only other local in play is the reverse lookup's result.
            let _ = event.set(path, name.clone());
        }
    }
    true
}

/// Rewrite `def x = ctx.a.b;` bindings back into the paths they alias.
///
/// The vendor binds a subtree to a local and then writes through it, so every
/// destination in the script reads `net['iana_number']` rather than a `ctx.`
/// path. Substituting the binding puts them all back in one form.
fn expand_locals(script: &str) -> String {
    let mut out = script.to_string();
    for statement in script.split(';') {
        // A statement carries the previous block's closing brace, so the
        // binding is found from the LAST `def ` in it, not the start.
        let Some(start) = statement.rfind("def ") else {
            continue;
        };
        let Some((name, bound)) = statement[start + "def ".len()..].split_once('=') else {
            continue;
        };
        let (name, bound) = (name.trim(), bound.trim());
        if !bound.starts_with("ctx.")
            || bound.contains('(')
            || bound.contains('[')
            || name.contains(|c: char| !c.is_alphanumeric() && c != '_')
        {
            continue;
        }
        out = out
            .replace(&format!("{name}."), &format!("{bound}."))
            .replace(&format!("{name}["), &format!("{bound}["));
    }
    out
}

/// The local a `def x = <marker>...` or typed `Integer x = <marker>...`
/// statement binds -- either way the name is the last word before the `=`.
/// A params row that is a LIST OF INSTRUCTIONS, each naming a destination and
/// either a literal value or a field to copy from.
///
/// cyberarkpas builds `_tmp.values` for a `foreach` to write, so the whole ECS
/// fan-out -- `event.category`, `event.outcome`, `user.target.name` -- hangs
/// off this one script. The member names are the helper's own and are required
/// here, so a package that diverged declines rather than being mis-read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InstructionRows {
    /// The field whose value keys the table.
    key: String,
    /// Where the built list lands.
    target: String,
}

fn parse_instruction_rows(script: &str) -> Option<InstructionRows> {
    if !script.contains("item.set") || !script.contains("item.from") {
        return None;
    }

    // `String msgID = ctx.event?.code;` -- the declared type varies, so the
    // binding is found by the local's name rather than by a `def` prefix.
    let local = script
        .split_once("params.get(")?
        .1
        .split(')')
        .next()?
        .trim();
    let bound = script.split_once(&format!(" {local} = ctx"))?.1;
    let key = clean_path(bound[..bound.find([';', '\n'])?].trim_start_matches(['?', '.']));

    // `ctx._tmp["values"] = values`, whose member is subscripted.
    let head = script[..script.rfind("= values")?]
        .trim()
        .trim_end_matches(']');
    let (parent, member) = match head.rsplit_once('[') {
        Some((parent, member)) => (
            parent,
            Some(member.trim().trim_matches(['"', '\'']).to_string()),
        ),
        None => (head, None),
    };
    let parent = clean_path(parent.trim().rsplit("ctx.").next()?);
    if parent.is_empty() || key.is_empty() {
        return None;
    }

    Some(InstructionRows {
        key,
        target: member.map_or_else(|| parent.clone(), |member| format!("{parent}.{member}")),
    })
}

fn run_instruction_rows(
    event: &mut Event,
    pattern: &InstructionRows,
    params: &Map<String, Value>,
) -> bool {
    let Some(key) = event.get_as_string(&pattern.key) else {
        return true;
    };
    let Some(Value::Array(actions)) = params.get(&key) else {
        // `if (actions == null) return;`
        return true;
    };

    let mut values = Vec::with_capacity(actions.len());
    for item in actions {
        // A literal value wins; otherwise the instruction names a field to read.
        let value = match item.get("value") {
            Some(literal) if !literal.is_null() => literal.clone(),
            _ => match item.get("from").and_then(Value::as_str).and_then(|from| {
                let from = from.to_string();
                event.get(&from).cloned()
            }) {
                Some(read) => read,
                None => continue,
            },
        };
        // `|| val == ""` -- an empty string contributes nothing.
        if value.as_str() == Some("") {
            continue;
        }
        let Some(to) = item.get("set") else {
            continue;
        };
        values.push(json!({ "to": to.clone(), "value": value }));
    }

    // `if (!values.isEmpty())` -- an empty list is not written at all.
    if !values.is_empty() {
        let _ = event.set(&pattern.target, Value::Array(values));
    }
    true
}

/// `params.getOrDefault(ctx.<key>, null)`, then a loop per named column
/// appending each of its members onto an ECS array.
///
/// `box_events` keys the whole params block by its event type and puts the two
/// columns under a `map` member. Nothing claimed it, so `event.category` and
/// `event.type` were never written on 147 of its 148 events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RowColumnAppends {
    /// The field whose value keys the table.
    key: String,
    /// A member the columns sit under, where the row nests them.
    inner: Option<String>,
    /// Each `(column, ctx target)`.
    appends: Vec<(String, String)>,
}

fn parse_row_column_appends(script: &str) -> Option<RowColumnAppends> {
    let key = ctx_path_between(script, "params.getOrDefault(ctx.", ",")?;
    let local = local_bound_to(script, "params.getOrDefault(")?;

    let mut inner = None;
    let mut appends = Vec::new();
    for (at, _) in script.match_indices(&format!("{local}.")) {
        let rest = &script[at + local.len() + 1..];
        let (member, tail) = match rest.strip_prefix("get('") {
            Some(tail) => (None, tail),
            None => match rest.split_once(".get('") {
                Some((member, tail)) => (Some(member.to_string()), tail),
                None => continue,
            },
        };
        let Some((column, tail)) = tail.split_once('\'') else {
            continue;
        };
        let Some(add_at) = tail.find(".add(") else {
            continue;
        };
        let Some((_, path)) = tail[..add_at].rsplit_once("ctx.") else {
            continue;
        };
        inner = inner.or(member);
        appends.push((column.to_string(), clean_path(path)));
    }

    (!appends.is_empty()).then_some(RowColumnAppends {
        key,
        inner,
        appends,
    })
}

fn run_row_column_appends(
    event: &mut Event,
    pattern: &RowColumnAppends,
    params: &Map<String, Value>,
) -> bool {
    let Some(key) = event.get_as_string(&pattern.key) else {
        return true;
    };
    let Some(row) = params.get(&key) else {
        // `getOrDefault(..., null)` and the script's own null check.
        return true;
    };
    let row = match &pattern.inner {
        Some(member) => match row.get(member) {
            Some(nested) => nested,
            None => return true,
        },
        None => row,
    };

    for (column, target) in &pattern.appends {
        let Some(Value::Array(items)) = row.get(column) else {
            continue;
        };
        for item in items.clone() {
            add_to_list(event, target, item);
        }
    }
    true
}

fn local_bound_to(script: &str, marker: &str) -> Option<String> {
    let head = &script[..script.find(marker)?];
    let statement = head.rsplit(';').next()?.trim();
    let before_eq = statement.strip_suffix('=')?.trim();
    let name = before_eq.rsplit(char::is_whitespace).next()?;
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| name.to_string())
}

/// Every `ctx.<path> = <expression>` in the script, as `(path, expression)`.
///
/// The path is normalised out of Painless's map syntax, so `ctx.network
/// ['iana_number']` and `ctx.network.iana_number` come back the same.
pub(crate) fn ctx_writes(script: &str) -> Vec<(String, String)> {
    let mut writes = Vec::new();
    for statement in script.split(';') {
        let Some((lhs, rhs)) = split_assignment(statement) else {
            continue;
        };
        let Some(start) = lhs.rfind("ctx.") else {
            continue;
        };
        let path = lhs[start + "ctx.".len()..]
            .replace("['", ".")
            .replace("[\"", ".")
            .replace("']", "")
            .replace("\"]", "");
        writes.push((clean_path(&path), rhs.trim().to_string()));
    }
    writes
}

/// [`ctx_writes`], plus writes whose ROOT is a bracket -- `ctx["a"] = v`.
///
/// Deliberately NOT folded into `ctx_writes`: nine other matchers read that
/// helper, and `carbonblack_edr`'s clone script spells `ctx["event"] = event`.
/// Teaching them all to see a bracket root gave `LookupMerge` a target it had
/// never had and cost two events on the corpus, against no gain -- so the
/// wider reading is scoped to the one parser that needs it.
fn ctx_writes_rooted_either_way(script: &str) -> Vec<(String, String)> {
    let mut writes = ctx_writes(script);
    for statement in script.split(';') {
        let Some((lhs, rhs)) = split_assignment(statement) else {
            continue;
        };
        // `ctx.a["b"]` already came back from `ctx_writes`.
        if lhs.contains("ctx.") {
            continue;
        }
        let Some(at) = lhs.rfind("ctx[") else {
            continue;
        };
        let path = lhs[at + "ctx".len()..]
            .replace("['", ".")
            .replace("[\"", ".")
            .replace("']", "")
            .replace("\"]", "");
        writes.push((
            clean_path(path.trim_start_matches('.')),
            rhs.trim().to_string(),
        ));
    }
    writes
}

/// Split a statement at its assignment, ignoring every comparison.
///
/// A guarded write reads `if (x != null) { ctx.a.b = c`, so the first `=` in
/// the text belongs to the comparison and splitting there loses the write.
fn split_assignment(statement: &str) -> Option<(&str, &str)> {
    let bytes = statement.as_bytes();
    let at = (0..bytes.len())
        .find(|at| bytes[*at] == b'=' && crate::painless_common::assigns_at(bytes, *at))?;
    Some((&statement[..at], &statement[at + 1..]))
}

/// A scalar's text, the way Painless would stringify it for a map key.
fn scalar_text(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// `ctx.<target>.put('<key>', params['<name>'][<numeric field>])`
///
/// The bounds check the vendor writes around it is the array's own length, so
/// an index outside it simply leaves the field unset.
fn try_indexed_lookup(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(container) = ctx_path_before(script, ".put(") else {
        return false;
    };
    let Some(key) = quoted_after(script, ".put(") else {
        return false;
    };
    let Some(Value::Array(table)) = params_indexed(script, params) else {
        return false;
    };
    // The index is the only ctx field the script reads, and the processor's own
    // `if` guards it being absent -- so a value we cannot read means the path
    // came out wrong, and saying "handled" would hide that.
    let Some(index_path) = ctx_path_before(script, ";") else {
        return false;
    };
    let Some(index) = event
        .get_as_string(&index_path)
        .and_then(|s| s.trim().parse::<usize>().ok())
    else {
        return false;
    };

    if let Some(value) = table.get(index).cloned() {
        let _ = event.set(&format!("{container}.{key}"), value);
    }
    true
}

/// How a scale script spells its multiply.
///
/// Both forms are in the vendored pipelines and the compound one carries no
/// separate target: aws s3access writes `ctx.event.duration *= params.MS_TO_NS`.
fn scale_marker(script: &str) -> Option<&'static str> {
    ["* params.", "*= params."]
        .into_iter()
        .find(|marker| script.contains(marker))
}

/// `ctx.<target> = ctx.<source> * params.<name>`, or `ctx.<f> *= params.<name>`
///
/// Source and target are usually the same field, but not always: cloudfront
/// scales `_tmp.time_taken` INTO `event.duration`, and reading the last path
/// before the multiply as both wrote the result back over the source and left
/// `event.duration` unset on 13 events.
fn try_scale(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(marker) = scale_marker(script) else {
        return false;
    };
    let Some((head, _)) = script.split_once(marker) else {
        return false;
    };
    let Some(source) = crate::painless_common::painless_path(head) else {
        return false;
    };
    // The assignment that owns the multiply. Without one the script scales in
    // place, which is azure's spelling and the compound form's only one.
    let path = crate::painless_common::last_assignment(head)
        .and_then(|at| crate::painless_common::painless_path(&head[..at]))
        .unwrap_or_else(|| source.clone());

    let Some(factor) = params_ref(script, params, marker).and_then(Value::as_f64) else {
        return false;
    };
    // The vendor wraps the read in `Float.parseFloat` where the field is text,
    // so a numeric string counts as a number here too.
    let Some(current) = event.get(&source).and_then(|held| {
        held.as_f64()
            .or_else(|| held.as_str().and_then(|text| text.trim().parse().ok()))
    }) else {
        return true;
    };

    let scaled = current * factor;
    // Whole results stay integers: a duration in nanoseconds is not a float.
    if scaled.fract() == 0.0 && scaled.abs() < 9.007_199_254_740_992e15 {
        #[allow(clippy::cast_possible_truncation)]
        let _ = event.set(&path, scaled as i64);
    } else {
        let _ = event.set(&path, scaled);
    }

    // The same script often derives an end instant from the scaled duration.
    #[allow(clippy::cast_possible_truncation)]
    if script.contains(".plusNanos(") {
        add_nanos(event, script, scaled as i64);
    }
    true
}

/// `ctx.<target> = ZonedDateTime.parse(<start>).plusNanos(nanos)`
///
/// The start instant is a ctx field bound to a local earlier in the script.
fn add_nanos(event: &mut Event, script: &str, nanos: i64) {
    use crate::painless_common::painless_path;

    use crate::painless_common::ctx_path_bound_to;

    let Some((head, _)) = script.split_once(".plusNanos(") else {
        return;
    };
    // `parse(<local>)`, where the local was bound to a ctx path earlier.
    let Some(source) = head
        .rsplit_once("parse(")
        .map(|(_, name)| name.trim_end_matches(')').trim())
        .and_then(|name| ctx_path_bound_to(script, name).or_else(|| painless_path(name)))
    else {
        return;
    };
    // The assignment target is whatever sits left of the `=` on this line.
    let Some(target) = head
        .rsplit_once('=')
        .and_then(|(lhs, _)| painless_path(lhs))
    else {
        return;
    };
    let Some(start) = event.get_as_string(&source) else {
        return;
    };
    let Ok(start) = chrono::DateTime::parse_from_rfc3339(&start) else {
        return;
    };

    let end = start + chrono::TimeDelta::nanoseconds(nanos);
    let _ = event.set(
        &target,
        Value::String(
            end.with_timezone(&chrono::Utc)
                .format("%Y-%m-%dT%H:%M:%S%.3f%:z")
                .to_string(),
        ),
    );
}

/// `ctx.<field> = ctx.<field>.replace(params.<name>, '<replacement>')`
fn try_replace(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(path) = ctx_path_before(script, ".replace(params.") else {
        return false;
    };
    let Some(needle) = params_ref(script, params, ".replace(params.").and_then(Value::as_str)
    else {
        return false;
    };
    let replacement = quoted_after(script, ",").unwrap_or_default();
    let Some(current) = event.get_str(&path) else {
        return true;
    };

    let replaced = current.replace(needle, &replacement);
    let _ = event.set(&path, replaced);
    true
}

/// The first single- or double-quoted string following `after`.
fn quoted_after(script: &str, after: &str) -> Option<String> {
    let tail = &script[script.find(after)? + after.len()..];
    let start = tail.find(['\'', '"'])?;
    let quote = tail.as_bytes()[start] as char;
    let end = tail[start + 1..].find(quote)?;
    Some(tail[start + 1..=start + end].to_string())
}

/// Resolve the expression inside `params.get(...)` to a table key.
///
/// It is either a `ctx.` path written inline or a `def` bound to one earlier in
/// the script, and either may be case-folded before the lookup.
fn resolve_key(event: &Event, script: &str, expr: &str) -> Option<String> {
    let (path, fold) = lookup_key_path(script, expr)?;
    let value = event.get_as_string(&path)?;
    Some(fold.apply(&value))
}

/// The `ctx.` path a lookup key expression names, and the fold applied to it.
///
/// Constant for a given script, so a caller that resolves once keeps this out
/// of the per-event path.
fn lookup_key_path(script: &str, expr: &str) -> Option<(String, Fold)> {
    let (expr, fold) = Fold::strip(expr);
    let expr = expr.trim();

    // Either spelling of the root. `ctx?.` reached the `def` branch below,
    // looked for a binding named after the whole path, found none and answered
    // "key absent" -- suricata's every lookup, on all 64 of its events.
    let root = expr
        .strip_prefix("ctx.")
        .or_else(|| expr.strip_prefix("ctx?."));
    let (path, fold) = if let Some(rest) = root {
        (rest.to_string(), fold)
    } else {
        let binding = format!("def {expr} = ctx.");
        let start = script.find(&binding)? + binding.len();
        let tail = &script[start..];
        let end = tail.find([';', '\n']).unwrap_or(tail.len());
        let (bound, bound_fold) = Fold::strip(&tail[..end]);
        // The call sits on whichever of the two the script spells it on, and
        // never on both.
        let fold = if fold == Fold::None { bound_fold } else { fold };
        (bound.trim().to_string(), fold)
    };

    Some((clean_path(&path), fold))
}

/// The params entry an indexed reference names, in either form Painless allows:
/// `params['LogLevel'][i]` or `params.LogLevel[i]`.
fn params_indexed<'a>(script: &str, params: &'a Map<String, Value>) -> Option<&'a Value> {
    if let Some(name) = quoted_after(script, "params[") {
        return params.get(&name);
    }
    params_ref(script, params, "params.")
}

/// The params entry a `params.<name>` reference names, given its lead-in text.
fn params_ref<'a>(script: &str, params: &'a Map<String, Value>, prefix: &str) -> Option<&'a Value> {
    let start = script.find(prefix)? + prefix.len();
    let tail = &script[start..];
    let end = tail
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(tail.len());
    params.get(&tail[..end])
}

/// The dotted `ctx.` path that immediately precedes `marker`.
///
/// The search runs BACKWARD from the marker, so an earlier binding in the same
/// statement wins whenever the path it finds is not the one the marker belongs
/// to -- `def p = ctx.a; def q = ctx.b; q.splitOnToken(' ')` handed back
/// everything from `b` to the marker. A path holding whitespace or a statement
/// terminator is that failure, and the caller is better told nothing than sold
/// a field name no event can hold: it declines instead of binding a pattern that
/// then writes nothing.
pub(crate) fn ctx_path_before(script: &str, marker: &str) -> Option<String> {
    let end = script.find(marker)?;
    let head = &script[..end];
    let start = head.rfind("ctx.")? + "ctx.".len();
    let path = clean_path(&head[start..]);
    (!path.contains([' ', '\t', '\n', ';'])).then_some(path)
}

/// The dotted `ctx.` path written between two markers.
///
/// Held to the same rule as [`ctx_path_before`]: the two markers need not sit
/// in one statement, so a close that only appears later spans the gap and the
/// path comes back carrying whatever lay between.
pub(crate) fn ctx_path_between(script: &str, open: &str, close: &str) -> Option<String> {
    let start = script.find(open)? + open.len();
    let tail = &script[start..];
    let end = tail.find(close)?;
    let path = clean_path(&tail[..end]);
    (!path.contains([' ', '\t', '\n', ';'])).then_some(path)
}

/// The argument of the LAST call to `name(`, balanced across nested parens.
///
/// The table lookup is often written twice -- once to null-check, once to use
/// -- and it is the second that feeds the merge.
fn last_call_argument(script: &str, name: &str) -> Option<String> {
    last_delimited(script, name, '(', ')')
}

/// The last `name[...]` subscript's contents, brackets balanced.
///
/// The bracket twin of [`last_call_argument`]: a Painless map reads the same
/// whether it is subscripted or `.get()`, and both spellings appear across the
/// integrations for the same job.
fn last_bracket_subscript(script: &str, name: &str) -> Option<String> {
    last_delimited(script, name, '[', ']')
}

/// What the LAST `name` opens, up to its matching close.
///
/// The two spellings differ only in the delimiter pair, so they share the
/// scan rather than the scan being written twice and drifting.
fn last_delimited(script: &str, name: &str, open: char, close: char) -> Option<String> {
    let start = script.rfind(name)? + name.len();
    let mut depth = 1usize;
    for (i, c) in script[start..].char_indices() {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(script[start..start + i].to_string());
            }
        }
    }
    None
}

/// Strip Painless null-safe navigation from a field path.
///
/// Allocates once where there is nothing to strip, which is most paths, and
/// twice where there is. Returning a `Cow` would borrow in the common case,
/// but 85 of the callers build a pattern struct that owns its paths, so the
/// signature change costs more than the runtime sites it would save.
pub(crate) fn clean_path(path: &str) -> String {
    let path = path.trim();
    if path.contains('?') {
        path.replace("?.", ".").replace('?', "")
    } else {
        path.to_string()
    }
}

/// A mutable reference to the value at a dotted path.
pub(crate) fn pointer_mut<'a>(event: &'a mut Event, path: &str) -> Option<&'a mut Value> {
    let mut pointer = String::with_capacity(path.len() + 1);
    for segment in path.split('.') {
        pointer.push('/');
        // JSON Pointer's own escapes, so a key holding one still resolves.
        for c in segment.chars() {
            match c {
                '~' => pointer.push_str("~0"),
                '/' => pointer.push_str("~1"),
                _ => pointer.push(c),
            }
        }
    }
    event.as_value_mut().pointer_mut(&pointer)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "painless_params_tests.rs"]
mod tests;
