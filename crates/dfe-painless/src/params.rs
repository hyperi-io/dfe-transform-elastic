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
//! [`crate::common::try_known_painless`], so a pattern with a params
//! block runs against the pipeline's real table rather than a transcribed copy.

use std::borrow::Cow;

use serde_json::{Map, Value, json};

use crate::helpers::{filetime_to_unix_ms, remove_sentinel_values};
use dfe_core::event::Event;

/// A processor's `params` block, parsed once per CALL SITE.
///
/// The block is emitted as a JSON string rather than a `json!` literal: the
/// macro expands once per key, and the o365 operation table alone is deep
/// enough to blow rustc's recursion limit. Parsing it behind a per-site
/// `OnceLock` costs one parse per process and nothing per event -- the same
/// pattern `cached_grok` uses, and for the same reason.
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
    let normalised = crate::common::normalise(script);
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
/// [`crate::plan::PainlessPlan`] rather than once per event. A pattern
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
    /// A scalar written from arithmetic that reads a scale out of `params`.
    ScalarExpression(Box<crate::expr::ScalarExpression>),
    /// A one-entry map whose key comes from a params lookup.
    KeyedByLookup(Box<KeyedByLookup>),
    /// Named fields parsed from hex text into numbers, in place.
    HexFields(crate::hex::HexFields),
    AwsEntity(Box<crate::entity::EntityScript>),
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
    RemapKeysThroughTable(Box<RemapKeysThroughTable>),
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
    /// A NAMED params table read with a default, in either of the two
    /// spellings the tree ships.
    ///
    /// `jamf_protect` writes the ternary one per telemetry field -- 40 sites
    /// over 13 files -- and qualys writes the `getOrDefault` one over its
    /// severity table. Sibling to [`ParamsPattern::UppercaseLookupDefault`],
    /// which folds case, reads the whole `params` map, and defaults to the KEY.
    TableLookupOrLiteral {
        source: String,
        table: String,
        target: String,
        default: TableDefault,
        gate: Option<TableGate>,
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
    /// The same flat table, where the key carries an ELVIS default and a key
    /// with no row writes a LITERAL rather than nothing.
    ///
    /// `(ctx.a.b ?: "").toString()` never throws on an absent field and never
    /// yields null, so the `else` arm runs on every event the table misses --
    /// which makes the target unconditional, where every other lookup in this
    /// ladder leaves it alone.
    StringifiedLookupOrLiteral {
        source: String,
        /// The key an absent or null source stands in as, off the elvis.
        absent_key: String,
        target: String,
        /// What the `else` arm writes.
        default: Value,
    },
    SentinelRemoval,
    FiletimeFieldList,
    BitFlags,
    FirstContainedMember,
    RenameKeys(Box<RenameKeys>),
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
    /// The flag names a field's own characters spell, read off the params keys
    /// as a substring test rather than a lookup.
    ContainsFlags(Box<ContainsFlags>),
    /// The literal writes the script makes on its own account travel with the
    /// pattern, so the four-hundred-line bodies are read once rather than per
    /// event.
    LookupMerge(Program),
    LookupColumns,
    /// An action named from the tail of a request URL.
    UrlTailAction(Box<crate::url_action::UrlTailAction>),
    LookupNormalise(LookupNormaliseScript, Program),
    /// A member moved to a sibling the params table NAMES.
    RenameMemberByLookup(Box<RenameMemberByLookup>),
    /// A params lookup written to a NAMED MEMBER of a container the script
    /// first guarantees exists.
    MemberLookup(MemberLookupScript),
    /// A params lookup that writes nothing when the table misses.
    GuardedLookup(GuardedLookupScript),
    /// A params row per id, collected into the field the script writes back.
    CollectParamsRows(Box<crate::collect_rows::CollectParamsRows>),
    /// Several fields unwrapped from a delimiter the params table carries.
    TrimDelimited(Box<crate::trim_delimited::TrimDelimited>),
    IndexedLookup,
    Scale,
    Replace,
    AddUniqueRow,
    FrameworkPreference,
    RowOrDefaults(Box<RowOrDefaults>),
    /// Literal arms answering the keys a params table does not carry, then the
    /// table.
    ArmedTable(Box<crate::named_arms::ArmedTable>),
    /// An issue classified from a params row and a block deadline.
    IssueLifecycle(Box<crate::issue_lifecycle::IssueLifecycle>),
    /// A guard chain picking ONE params row to write to a single target.
    GuardedParamsRow(Box<GuardedParamsRow>),
    /// Named MEMBERS of a params row, each to its own target.
    RowMembers(Box<RowMembers>),
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
    /// A map rebuilt from ONLY the keys the params table names, each renamed
    /// to the table's value.
    SelectRenameKeys(SelectRenameKeys),
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
    /// A run of null-guarded lookups, each field through its OWN named table,
    /// subscripted `params['<table>'][<key>]`.
    FieldTables(crate::field_tables::FieldTables),
    /// Every mapping the named params LIST describes, applied in order.
    ///
    /// The script names nothing: each spec in the list carries its own source
    /// object and key, its own destination object and key, and its own value
    /// map, so the table is the whole pattern. The `String` is the params key
    /// the list sits under.
    MemberMappings(String),
    /// Several scratch fields reparsed with a date pattern the DOCUMENT
    /// carries, into a sibling container.
    ConfiguredDateFormat(Box<ConfiguredDateFormat>),
    /// A delimited string's tokens named by POSITION out of the params table.
    SplitNamedByPosition(Box<SplitNamedByPosition>),
    /// An action's ECS block chosen by a four-tier lookup over `params`.
    ActionMapping(Box<crate::action_mapping::ActionMapping>),
    /// One params row naming both an ECS type literal and the path a second
    /// field's value belongs at.
    RowNamedTarget(Box<crate::row_named_target::RowNamedTarget>),
    /// A severity resolved to a label through one table, then scored through a
    /// second.
    LabelledScore(Box<crate::labelled_score::LabelledScore>),
}

/// The membership test a table lookup may sit inside.
///
/// `qualys_vmdr` maps `SEVERITY_LEVEL` to a word only for the vulnerability types
/// its table covers, and running the lookup regardless would rewrite every other
/// type's severity with a row that was never meant for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableGate {
    /// The field whose value has to be in the list.
    pub(crate) field: String,
    /// The params list it is tested against.
    pub(crate) list: String,
}

/// What a table lookup falls back to when the key has no row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TableDefault {
    /// A quoted literal, frozen into the script.
    Literal(String),
    /// Another ROW of the same table, named by its key, so the fallback moves
    /// with the table -- qualys's severity falls back to `vuln_level["0"]`.
    Row(String),
}

/// The one matcher this script's text triggers, or `None`.
///
/// The trigger order is load-bearing and each comment says why a branch sits
/// where it does; a script can spell several triggers and the FIRST wins,
/// exactly as the old inline dispatch behaved.
pub(crate) fn params_pattern(normalised: &str) -> Option<ParamsPattern> {
    // Pattern: NAMED MEMBERS of a params row, each to its own target. FIRST,
    // because every reader below takes a row WHOLE and so writes the members as
    // children of one field rather than to the ones the script names. The
    // trigger is the double subscript, and the parse is the real gate: it
    // demands every `params[` in the script be the same lookup.
    if normalised.contains("params.containsKey(ctx.")
        && normalised.contains("][")
        && let Some(pattern) = parse_row_members(normalised)
    {
        return Some(ParamsPattern::RowMembers(Box::new(pattern)));
    }

    // Pattern: aws cloudtrail's entity classifier, per-service enrichment
    // into TreeSets then classification through the params tables. First,
    // because its 950 lines spell half the other triggers somewhere. The
    // trigger IS the parse: a restructured script declines here and falls
    // through the dispatch unclaimed.
    if normalised.contains("enrichCtx")
        && normalised.contains("related.entity")
        && let Some(parsed) = crate::entity::EntityScript::parse(normalised)
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

    // Pattern: the same lookup through a NAMED table with a default -- a
    // literal in jamf_protect's telemetry, another row of the table in
    // qualys's severity. The trigger is the parse, with the two lookup
    // spellings as a cheap reject.
    if (normalised.contains(".containsKey(") || normalised.contains(".getOrDefault("))
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

    // Pattern: rename an object's keys, recursively, through a name map. The
    // trigger claims every script that spells the helper and the READER decides
    // what each does with the result, so a spelling it cannot place still lands
    // here rather than falling through to a matcher meant for something else.
    if normalised.contains("keyMap.containsKey(key)") {
        return Some(ParamsPattern::RenameKeys(Box::new(RenameKeys::parse(
            normalised,
        ))));
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

    // Pattern: a map's keys renamed through a table picked by a presence test.
    if normalised.contains("entrySet()")
        && normalised.contains("== null")
        && normalised.contains("] = dst")
        && let Some(pattern) = parse_remap_keys_through_table(normalised)
    {
        return Some(ParamsPattern::RemapKeysThroughTable(Box::new(pattern)));
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

    // Pattern: the flag names a field's own characters spell, read off the
    // params keys. Ahead of the reversible lookup below, whose bare
    // `params.entrySet()` trigger this also spells and which needs a
    // `params[ctx.` the substring test never writes -- claimed there, the
    // script reached nothing at all.
    if normalised.contains(".contains(entry.getKey())")
        && let Some(pattern) = parse_contains_flags(normalised)
    {
        return Some(ParamsPattern::ContainsFlags(Box::new(pattern)));
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

    // Pattern: an action named from the tail of a request URL, through a
    // two-level table with a computed fallback. Above `LookupColumns`, which
    // counts three `.get(` with no parse behind them and whose runner then
    // declines.
    if normalised.contains(".splitOnToken(")
        && normalised.contains("params.get(")
        && let Some(pattern) = crate::url_action::parse_url_tail_action(normalised)
    {
        return Some(ParamsPattern::UrlTailAction(Box::new(pattern)));
    }

    // Pattern: look a row up in a nested table and fan its columns out,
    // appending the list-valued ones rather than replacing them.
    if normalised.contains("params.get(") && normalised.matches(".get(").count() >= 3 {
        return Some(ParamsPattern::LookupColumns);
    }

    // Pattern: the lookup written to a NAMED MEMBER of a container the script
    // first guarantees exists. Ahead of `LookupNormalise`, which reads that
    // guarantee as the write and so targets the bare container: chrome stored
    // a string AT `event`, and every later `event.<sub>` write then failed on
    // it, scoring zero on six events whose every field was right.
    if normalised.contains("params.get(")
        && let Some(pattern) = parse_member_lookup(normalised)
    {
        return Some(ParamsPattern::MemberLookup(pattern));
    }

    // Pattern: a member MOVED to a sibling the table NAMES, rather than a row
    // written as a value. Ahead of `LookupNormalise`, which reads the naming
    // subscript as the destination and so stored the table's row under a key
    // called `type]` -- darktrace kept its raw `data` beside it on every event.
    if normalised.contains("[params.get(")
        && let Some(pattern) = parse_rename_member_by_lookup(normalised)
    {
        return Some(ParamsPattern::RenameMemberByLookup(Box::new(pattern)));
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

    // Pattern: the same flat table keyed by a field's STRING form, with the key
    // bound to a local and the guard written as a positive `containsKey`.
    // Above `IndexedLookup`, whose `.put(` plus `params` trigger claims the
    // script while its runner declines on it -- the table is a map, not an
    // array. cyberark_epm's two logon tables are 19 fields over 10 events that
    // reached no runner at all.
    if normalised.contains("params.containsKey(")
        && let Some(pattern) = parse_local_key_lookup(normalised)
    {
        return Some(pattern);
    }

    // Pattern: the same table, keyed through an ELVIS so an absent field is a
    // literal rather than a null, with an `else` arm writing a literal default.
    // The key expression carries no `params[ctx.` and no `.put(`, so every
    // other trigger in this ladder misses it.
    if normalised.contains("params.containsKey(")
        && normalised.contains("?:")
        && let Some(pattern) = parse_stringified_lookup_or_literal(normalised)
    {
        return Some(pattern);
    }

    // Pattern: a params row per id, collected into the field the script puts
    // it back into. Above `IndexedLookup`, which claims the same two keywords
    // with no parse behind them and whose runner then declines a table that is
    // a map rather than an array. Gated on the parse, so what it cannot read
    // falls through.
    if normalised.contains(".put(")
        && normalised.contains("params")
        && let Some(pattern) = crate::collect_rows::parse_collect_params_rows(normalised)
    {
        return Some(ParamsPattern::CollectParamsRows(Box::new(pattern)));
    }

    // Pattern: several fields unwrapped from a delimiter the table carries,
    // through a helper the script declares. Above `IndexedLookup`, which claims
    // the same two keywords with no parse behind them and whose runner then
    // declines a table that is a map rather than an array -- sentinel_one's
    // command lines kept their quotes on eight of eighteen events that way.
    if normalised.contains(".put(")
        && normalised.contains("params")
        && let Some(pattern) = crate::trim_delimited::parse_trim_delimited(normalised)
    {
        return Some(ParamsPattern::TrimDelimited(Box::new(pattern)));
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

    // Pattern: a guard chain choosing ONE params row for a single target.
    // Above `RowOrDefaults` only because that one reads a table keyed by a
    // field, which these scripts do not have -- the params block here is three
    // sentences and the guards decide which is written.
    if normalised.contains("} else {")
        && normalised.contains("= params.")
        && let Some(pattern) = parse_guarded_params_row(normalised)
    {
        return Some(ParamsPattern::GuardedParamsRow(Box::new(pattern)));
    }

    // Pattern: an issue classified from a params row, a security-domain table
    // and a block deadline measured against the event's own timestamp. Above
    // `RowOrDefaults`, which declines the script because its branch is not the
    // end of the text, leaving it to `GuardedCopy` in the ladder below.
    if normalised.contains(".isAfter(")
        && normalised.contains("params.")
        && let Some(pattern) = crate::issue_lifecycle::parse_issue_lifecycle(normalised)
    {
        return Some(ParamsPattern::IssueLifecycle(Box::new(pattern)));
    }

    // Pattern: literal arms answering the keys a params table does not carry,
    // then the table itself. Above `RowOrDefaults`, which reads the lookup
    // without the arms and cannot take the `getOrDefault` spelling.
    if normalised.contains(".getOrDefault(")
        && let Some(pattern) = crate::named_arms::parse_armed_table(normalised)
    {
        return Some(ParamsPattern::ArmedTable(Box::new(pattern)));
    }

    // Pattern: a NAMED params table's row fanned onto ctx, with literal
    // defaults where the key has no row. LAST, so every table matcher above
    // keeps the scripts it already claims.
    if let Some(pattern) = parse_row_or_defaults(normalised) {
        return Some(ParamsPattern::RowOrDefaults(Box::new(pattern)));
    }

    // Pattern: named fields parsed from hex text into numbers, the field names
    // coming from a params list rather than the script.
    if normalised.contains("for (key in params.")
        && let Some(pattern) = crate::hex::parse_hex_fields(normalised)
    {
        return Some(ParamsPattern::HexFields(pattern));
    }

    // Pattern: a one-entry map whose KEY is a params row's value, which is what
    // Painless's `[local: value]` literal builds.
    if normalised.contains("= params[ctx.")
        && let Some(pattern) = parse_keyed_by_lookup(normalised)
    {
        return Some(ParamsPattern::KeyedByLookup(Box::new(pattern)));
    }

    // Pattern: a scalar written from arithmetic where `params` supplies a
    // scale. The same reader the text-only lane uses, which declines these
    // because it is never handed a params block -- `event.duration =
    // ctx.ses.duration * params.S_TO_NS` is the common spelling. Last of all,
    // so every table matcher above keeps what it already claims.
    if normalised.contains("params.")
        && let Some(pattern) = crate::expr::parse_scalar_expression(normalised)
        && pattern.reads_params()
    {
        return Some(ParamsPattern::ScalarExpression(Box::new(pattern)));
    }

    // Pattern: a map rebuilt from ONLY the keys the table names, each renamed
    // to the table's value. LAST, so it takes only what nothing above took --
    // `RenameKeys` further up KEEPS a key the table misses where this one
    // DROPS it, and letting the general reader claim this script would carry
    // the vendor's own abbreviations into ECS beside the renamed ones.
    if normalised.contains("params.containsKey(")
        && let Some(pattern) = parse_select_rename_keys(normalised)
    {
        return Some(ParamsPattern::SelectRenameKeys(pattern));
    }

    // Pattern: a run of null-guarded lookups, each field through its OWN named
    // table, subscripted `params['<table>'][ctx.<field>]`. Late, because the
    // parse IS the trigger and it reads the WHOLE script: nothing above claims
    // gigamon's nine lookups today, so it needs to precede nothing.
    if let Some(pattern) = crate::field_tables::parse_field_tables(normalised) {
        return Some(ParamsPattern::FieldTables(pattern));
    }

    // Pattern: the same run of lookups in the DOTTED spelling, each keyed
    // through a local the block binds -- `params.<table>.containsKey(
    // k.toString())` then `params.<table>[k.toString()]`. After its sibling
    // above, because the parse IS the trigger and reads the whole script: all
    // three trend_micro spellings read `binding: []` before this arm.
    if let Some(pattern) = crate::field_tables::parse_named_table_lookups(normalised) {
        return Some(ParamsPattern::FieldTables(pattern));
    }

    // Pattern: a params LIST of mapping specs, each naming its own source
    // object and key, destination object and key, and value map. LAST, because
    // the parse IS the trigger and reads the whole loop: nothing above claims
    // iptables' script today, so it needs to precede nothing.
    if let Some(list) = parse_member_mappings(normalised) {
        return Some(ParamsPattern::MemberMappings(list));
    }

    // Pattern: each of the params-named fields reparsed with a date pattern the
    // DOCUMENT carries rather than the pipeline. LAST, because nothing above
    // claims citrix_adc's script and it needs to precede nothing.
    if normalised.contains("DateTimeFormatter.ofPattern(ctx.")
        && let Some(pattern) = parse_configured_date_format(normalised)
    {
        return Some(ParamsPattern::ConfiguredDateFormat(Box::new(pattern)));
    }

    // Pattern: a delimited string's tokens named by POSITION out of the params
    // table. LAST, because nothing above claims barracuda_waf's custom headers
    // and it needs to precede nothing; the parse is the gate, and the two
    // substrings are the cheap reject.
    if normalised.contains(".splitOnToken(")
        && normalised.contains("params[")
        && let Some(pattern) = parse_split_named_by_position(normalised)
    {
        return Some(ParamsPattern::SplitNamedByPosition(Box::new(pattern)));
    }

    // Pattern: an action's ECS block chosen by an exact table, then an ordered
    // prefix list, then one suffix rule, then a fallback. LAST, because nothing
    // above claims atlassian_cloud's classifier and it needs to precede
    // nothing; the two calls are the cheap reject and the parse -- which
    // demands all four tiers -- is the gate.
    if normalised.contains(".startsWith(")
        && normalised.contains(".endsWith(params.")
        && let Some(pattern) = crate::action_mapping::parse_action_mapping(normalised)
    {
        return Some(ParamsPattern::ActionMapping(Box::new(pattern)));
    }

    // Pattern: one params row naming both an ECS type literal and the path the
    // indicator's value belongs at. LAST, because nothing above claims the
    // three threat-intelligence sources that ship it and it needs to precede
    // nothing; the row-named destination is the gate and the two substrings are
    // the cheap reject.
    if normalised.contains("= params[ctx.")
        && normalised.contains("set(ctx, ")
        && let Some(pattern) = crate::row_named_target::parse_row_named_target(normalised)
    {
        return Some(ParamsPattern::RowNamedTarget(Box::new(pattern)));
    }

    // Pattern: a severity arriving as either a number or a word, resolved to
    // the word through one table and then scored through a second. LAST,
    // because `beyondtrust_isi` alone ships it and it needs to precede nothing;
    // the `instanceof Long` is the cheap reject -- four pipelines in the tree
    // spell it -- and the parse, which demands both tables and all three
    // writes, is the gate.
    if normalised.contains("instanceof Long")
        && normalised.contains("String.valueOf(")
        && let Some(pattern) = crate::labelled_score::parse_labelled_score(normalised)
    {
        return Some(ParamsPattern::LabelledScore(Box::new(pattern)));
    }

    None
}

/// A delimited string's tokens named by POSITION out of the params table.
///
/// ```painless
/// def headers = ctx._temp.raw_custom_headers.splitOnToken(' ');
/// if (ctx.barracuda.waf.custom_header == null) {
///     ctx.barracuda.waf.custom_header = new HashMap();
/// }
/// for (int i = 0; i < headers.length; i++) {
///   ctx.barracuda.waf.custom_header[params[(i+1).toString()]] = headers[i];
/// }
/// ```
///
/// The table names each POSITION rather than each value, so the script says
/// only that a header list arrives in a fixed order -- `barracuda_waf` ships
/// `{"1":"accept_encoding","2":"host","3":"connection", ...}` and its access
/// stream carries up to four of them per event. This is the whole of what the
/// source is missing: eight of its events turn on nothing else.
///
/// The vendor pads an absent header with `"-"`, so most positions arrive
/// written and the `drop_empty` at the end of the pipeline takes them back out
/// -- which is why the runner writes every named position rather than trying
/// to decide which ones mean anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SplitNamedByPosition {
    /// The `ctx.` path holding the delimited text.
    source: String,
    /// The token the split cuts on.
    separator: String,
    /// The `ctx.` container each named token is written into.
    target: String,
    /// What the loop index is offset by before it keys the table, which is 1
    /// where the table's first row is `"1"`.
    offset: usize,
}

/// Read the source, the separator, the container and the key offset, or decline.
///
/// Every part is read off the script: the loop variable is taken from the `for`
/// header the split's own local bounds, the key expression has to be that
/// variable offset by a literal, and the value has to be that same list at that
/// same variable. A loop writing anything else is a different script and
/// declines here rather than binding to a runner that would invent positions.
fn parse_split_named_by_position(script: &str) -> Option<SplitNamedByPosition> {
    // `def <list> = ctx.<source>.splitOnToken('<sep>');`
    let (bound, rest) = script.split_once(".splitOnToken(")?;
    let separator = literal_at(rest)?;
    let source = clean_path(bound.rsplit_once("ctx.")?.1.trim());
    let list = bound.rsplit_once(" = ")?.0.rsplit(' ').next()?.trim();
    if source.is_empty() || separator.is_empty() || list.is_empty() {
        return None;
    }

    // `for (int <var> = 0; <var> < <list>.length; <var>++)`, read off the bound
    // the loop walks so a second loop over some other list cannot supply it.
    let header = script.split_once(&format!("{list}.length"))?.0;
    let var = header.rsplit_once("for (int ")?.1.split_once('=')?.0.trim();
    if var.is_empty() {
        return None;
    }

    // `ctx.<target>[params[<key>]] = <list>[<var>];`
    let target = ctx_path_before(script, "[params[")?;
    let (key, after) = script.split_once("[params[")?.1.split_once("]]")?;
    let value = after.split_once(';')?.0.trim().strip_prefix('=')?.trim();
    if target.is_empty() || value != format!("{list}[{var}]") {
        return None;
    }

    // `(<var>+<n>).toString()`, or the bare `<var>.toString()` where the table
    // is keyed from zero. Anything else is a key this cannot resolve.
    let key = key.trim();
    let offset = if let Some(sum) = key
        .strip_prefix('(')
        .and_then(|key| key.strip_suffix(").toString()"))
    {
        let (left, right) = sum.split_once('+')?;
        if left.trim() != var {
            return None;
        }
        right.trim().parse().ok()?
    } else if key.strip_suffix(".toString()")? == var {
        0
    } else {
        return None;
    };

    Some(SplitNamedByPosition {
        source,
        separator,
        target,
        offset,
    })
}

/// Write each token of the split under the name its POSITION has in the table.
///
/// A position the table does not name writes nothing. Painless keys the map by
/// the null the lookup returned, which names no field the document can hold, so
/// writing anything there would be inventing a field rather than reproducing
/// one.
fn run_split_named_by_position(
    event: &mut Event,
    pattern: &SplitNamedByPosition,
    params: &Map<String, Value>,
) -> bool {
    // `splitOnToken` on an absent or non-string field throws, and a throw
    // writes nothing at all.
    let Some(text) = event.get_str(&pattern.source).map(str::to_owned) else {
        return true;
    };
    for (at, token) in text.split(pattern.separator.as_str()).enumerate() {
        let Some(name) = params
            .get(&(at + pattern.offset).to_string())
            .and_then(Value::as_str)
        else {
            continue;
        };
        let _ = event.set(&format!("{}.{name}", pattern.target), Value::from(token));
    }
    true
}

/// Several scratch fields reparsed with a date pattern the document supplies.
///
/// ```painless
/// def zone = ctx.event?.timezone != null ? ZoneId.of(ctx.event.timezone) : null;
/// def formatter = DateTimeFormatter.ofPattern(ctx._conf.custom_date_format);
/// def outFormatter = DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ss.SSSXXX");
/// params.fields.forEach(field -> {
///   if (!ctx._tmp?.containsKey(field)) { return true; }
///   try {
///     def localDateTime = LocalDateTime.parse(ctx._tmp[field], formatter);
///     ctx.citrix_adc.log[field] = outFormatter.format(ZonedDateTime.of(localDateTime, zone));
///   } catch (Exception e) { return true; }
/// });
/// ```
///
/// `citrix_adc` ships this over five fields at once, and its `_conf` carries
/// `dd/MM/yyyy:HH:mm:ss`. Unclaimed, the fallback date processors behind it
/// read `10/08/2024` with their own hard-coded `MM/dd/yyyy` and every such
/// event landed in October.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConfiguredDateFormat {
    /// The params key listing the field names.
    names: String,
    /// The document path holding the input pattern.
    format: String,
    /// The document path holding the zone, which the output is rendered in.
    zone: Option<String>,
    /// The container the values are read from.
    source: String,
    /// The container the parsed values are written to.
    target: String,
}

/// The one output pattern this matcher can render, which is what
/// [`dfe_core::date_formats::parse_date`] already writes: milliseconds, and `Z`
/// at a zero offset where Java's `XXX` writes `Z` too.
const ISO_OFFSET_PATTERN: &str = "yyyy-MM-dd'T'HH:mm:ss.SSSXXX";

fn parse_configured_date_format(script: &str) -> Option<ConfiguredDateFormat> {
    // The INPUT formatter is the one built from a ctx path; the output one is
    // a literal, and reading either for the other inverts the whole pattern.
    let format = clean_path(
        script
            .split_once("DateTimeFormatter.ofPattern(ctx.")?
            .1
            .split_once(')')?
            .0
            .trim(),
    );
    if !script.contains(ISO_OFFSET_PATTERN) {
        return None;
    }

    let (head, body) = script.split_once(".forEach(")?;
    let names = head.rsplit_once("params.")?.1.trim();
    if names.is_empty() || names.contains(char::is_whitespace) {
        return None;
    }

    // Both containers are subscripted by the loop's OWN variable; anything
    // else reads a different field from the one the loop is walking.
    let subscript = format!("[{}]", body.split_once("->")?.0.trim());
    let source = clean_path(
        body.split_once("LocalDateTime.parse(ctx.")?
            .1
            .split_once(subscript.as_str())?
            .0
            .trim(),
    );
    let target = clean_path(
        body.split_once(&format!("{subscript} = "))?
            .0
            .rsplit("ctx.")
            .next()?
            .trim(),
    );

    let zone = script
        .split_once("ZoneId.of(ctx.")
        .and_then(|(_, tail)| tail.split_once(')'))
        .map(|(path, _)| clean_path(path.trim()));

    (!format.is_empty() && !source.is_empty() && !target.is_empty()).then(|| ConfiguredDateFormat {
        names: names.to_owned(),
        format,
        zone,
        source,
        target,
    })
}

/// Reparse each named field with the document's own pattern.
///
/// A value the pattern cannot read is SKIPPED, which is the script's own
/// `catch`: the target stays absent and the date processor behind it -- the
/// one carrying the pipeline's hard-coded format list -- takes the field.
fn run_configured_date_format(
    event: &mut Event,
    pattern: &ConfiguredDateFormat,
    params: &Map<String, Value>,
) -> bool {
    let Some(format) = event.get_str(&pattern.format).map(str::to_owned) else {
        return true;
    };
    let Some(names) = params.get(&pattern.names).and_then(Value::as_array) else {
        return true;
    };
    let zone = pattern
        .zone
        .as_deref()
        .and_then(|path| event.get_str(path))
        .map(str::to_owned);

    for name in names.iter().filter_map(Value::as_str) {
        let Some(text) = event
            .get_str(&format!("{}.{name}", pattern.source))
            .map(str::to_owned)
        else {
            continue;
        };
        if let Some(parsed) = dfe_core::date_formats::parse_date(&text, &[&format], zone.as_deref())
        {
            let _ = event.set(&format!("{}.{name}", pattern.target), Value::String(parsed));
        }
    }
    true
}

/// The params key holding a list of mapping specs, or `None`.
///
/// ```painless
/// for (action in params.mappings) {
///   def src = ctx[action.source.object];
///   if (src != null) {
///     Map map = action.map;
///     String key = src[action.source.key];
///     String mapping = map[key];
///     if (mapping != null) {
///       Map dst = ctx[action.destination.object];
///       if (dst == null) {
///           dst = new HashMap();
///           ctx[action.destination.object] = dst;
///       }
///       dst[action.destination.key] = mapping;
///     }
///   }
/// }
/// ```
///
/// The loop's local is read off the `in params.` it binds over, and every one
/// of the five accessors the body needs is required against that local. A loop
/// over some other params list spells none of them and declines here.
fn parse_member_mappings(script: &str) -> Option<String> {
    let marker = " in params.";
    let local = identifier_before(script, marker)?;
    let list = leading_name(&script[script.find(marker)? + marker.len()..]);
    if list.is_empty() {
        return None;
    }
    let accessors = [
        format!("ctx[{local}.source.object]"),
        format!("{local}.source.key"),
        format!("ctx[{local}.destination.object]"),
        format!("{local}.destination.key"),
        format!("{local}.map"),
    ];
    accessors
        .iter()
        .all(|accessor| script.contains(accessor.as_str()))
        .then_some(list)
}

/// One member of a top-level object, the way `ctx[<object>][<key>]` reads it.
///
/// Both names are LITERAL keys rather than dotted paths: `ctx["a.b"]` is one
/// key in Painless where [`Event::get`] would walk two levels. Nothing is
/// joined either, so a spec costs no allocation on a path that runs at 20,000
/// events a batch.
fn member<'a>(event: &'a Event, object: &str, key: &str) -> Option<&'a Value> {
    event
        .as_value()
        .as_object()?
        .get(object)?
        .as_object()?
        .get(key)
}

/// Write one member of a top-level object, creating the object where the
/// script's own `new HashMap()` would.
///
/// A present non-map at `object` is a `ClassCastException` in Painless, so the
/// vendor's document fails there rather than growing a member: write nothing.
fn set_member(event: &mut Event, object: &str, key: &str, value: Value) {
    let Some(root) = event.as_value_mut().as_object_mut() else {
        return;
    };
    let vacant = match root.get(object) {
        Some(Value::Object(_)) => false,
        None | Some(Value::Null) => true,
        Some(_) => return,
    };
    if vacant {
        root.insert(object.to_owned(), Value::Object(Map::new()));
    }
    let Some(Value::Object(destination)) = root.get_mut(object) else {
        return;
    };
    if let Some(slot) = destination.get_mut(key) {
        *slot = value;
    } else {
        destination.insert(key.to_owned(), value);
    }
}

/// Apply every mapping the params list describes, in order.
///
/// Order is behaviour, because a spec sees what the specs before it wrote:
/// iptables folds `event.action` `d -> drop` in one spec and reads that `drop`
/// in the next to write `event.type = denied`.
///
/// A spec writes NOTHING where its source object is absent, where its source
/// member is not a string, or where the map has no row for that member. The
/// script declares `String key = src[...]`, so a non-string member is a
/// `ClassCastException` there rather than a key rendered into the table.
///
/// Returns false only where `params` carries no list under this name, which
/// says the block is not this pattern's and something else should have it.
fn run_member_mappings(event: &mut Event, list: &str, params: &Map<String, Value>) -> bool {
    let Some(specs) = params.get(list).and_then(Value::as_array) else {
        return false;
    };
    for spec in specs {
        let (Some(source), Some(destination), Some(table)) = (
            spec.get("source").and_then(Value::as_object),
            spec.get("destination").and_then(Value::as_object),
            spec.get("map").and_then(Value::as_object),
        ) else {
            continue;
        };
        let (Some(source_object), Some(source_key)) = (
            source.get("object").and_then(Value::as_str),
            source.get("key").and_then(Value::as_str),
        ) else {
            continue;
        };
        let (Some(destination_object), Some(destination_key)) = (
            destination.get("object").and_then(Value::as_str),
            destination.get("key").and_then(Value::as_str),
        ) else {
            continue;
        };
        let Some(mapped) = member(event, source_object, source_key)
            .and_then(Value::as_str)
            .and_then(|key| table.get(key))
            .cloned()
        else {
            continue;
        };
        set_member(event, destination_object, destination_key, mapped);
    }
    true
}

/// A one-entry map whose KEY comes from a params lookup.
///
/// ```painless
/// def key = params[ctx.json.Algorithm];
/// if (key == null) { throw new Exception("Unsupported hash algorithm ..."); }
/// def hashes = [key:ctx.json.Name];
/// ctx["_hashes"] = hashes;
/// ```
///
/// The subtlety is Painless's map literal: `[key: value]` evaluates `key` as an
/// EXPRESSION, so the map's key is the params row's value, not the word "key".
/// `ti_recordedfuture` turns `SHA-256` into `sha256` that way and the captured
/// output confirms it -- `{"sha256": "38e9..."}` -- after a `rename` moves the
/// map to `threat.indicator.file.hash`.
///
/// Unmatched it costs 17 of its 98 events, all on that one field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyedByLookup {
    /// The field whose value selects the params row.
    selector: String,
    /// The field whose value the map holds.
    value: String,
    /// Where the one-entry map lands.
    target: String,
}

impl KeyedByLookup {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(selector: String, value: String, target: String) -> Self {
        Self {
            selector,
            value,
            target,
        }
    }
}

/// Write the one-entry map, or nothing.
///
/// A selector with no row in the table writes nothing: the script THROWS there,
/// and its `on_failure` appends to `error.message` rather than storing a map
/// under some fallback key.
fn run_keyed_by_lookup(
    event: &mut Event,
    pattern: &KeyedByLookup,
    params: &Map<String, Value>,
) -> bool {
    let (Some(selector), Some(value)) = (
        event.get(&pattern.selector).and_then(Value::as_str),
        event.get(&pattern.value).cloned(),
    ) else {
        return true;
    };
    let Some(key) = params.get(selector).and_then(Value::as_str) else {
        return true;
    };
    let mut held = Map::new();
    held.insert(key.to_owned(), value);
    let _ = event.set(&pattern.target, Value::Object(held));
    true
}

/// `def k = params[ctx.<selector>]; .. def m = [k:ctx.<value>]; ctx["<target>"] = m;`
fn parse_keyed_by_lookup(script: &str) -> Option<KeyedByLookup> {
    let (head, tail) = script.split_once("= params[ctx.")?;
    let local = head.trim_end().rsplit(' ').next()?.trim();
    let selector = clean_path(tail.split(']').next()?.trim());
    if local.is_empty() || selector.is_empty() {
        return None;
    }

    // The map literal, whose key is the LOCAL rather than a string.
    let literal = format!("[{local}:ctx.");
    let (before, after) = script.split_once(&literal)?;
    let value = clean_path(after.split(']').next()?.trim());
    // `def hashes = ` -- the name is the token BEFORE the `=`, not the `=`.
    let map_local = before
        .trim_end()
        .strip_suffix('=')?
        .trim_end()
        .rsplit([' ', '\n'])
        .next()?
        .trim();
    if value.is_empty() || map_local.is_empty() {
        return None;
    }

    // The store, which names where the map lands.
    let stored = script.split_once("ctx[")?.1;
    let target = stored.split(']').next()?.trim().trim_matches(['\'', '"']);
    if target.is_empty() || !script.contains(&format!("= {map_local};")) {
        return None;
    }
    Some(KeyedByLookup::new(selector, value, target.to_owned()))
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

/// A params key expression stripped back to the term that names the value.
///
/// `obj.toString()`, `(ctx.ses.device_os_type_id).toString()` and
/// `String.valueOf(ctx.a.b)` are one term in three spellings -- the last is
/// Java's prefix form of the second -- and a reader that knows only one
/// declines the rest outright.
pub(crate) fn key_term(key: &str) -> Option<String> {
    let key = key.trim();
    let key = key
        .strip_prefix("String.valueOf(")
        .and_then(|inner| inner.strip_suffix(')'))
        .unwrap_or(key)
        .trim();
    let key = key.trim_end_matches(".toString()").trim();
    let key = key
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
        .unwrap_or(key)
        .trim();
    (!key.is_empty() && !key.contains(['(', ' ', '\n'])).then(|| key.to_string())
}

/// `if (params.containsKey(k)) { ctx.<target> = params.get(k); }`
fn parse_guarded_lookup(script: &str) -> Option<GuardedLookupScript> {
    // Balanced, because the key is `obj.toString()` and splitting on the first
    // `)` cuts inside that call rather than after the argument.
    let term = key_term(&last_call_argument(script, "params.containsKey(")?)?;
    // The key is written either inline as a path or bound to a local first.
    let key = if let Some(path) = term
        .strip_prefix("ctx.")
        .or_else(|| term.strip_prefix("ctx?."))
    {
        clean_path(path)
    } else if let Some((_, bound)) = script.split_once(&format!(" {term} = ctx.")) {
        clean_path(bound.split([';', '\n']).next()?.trim())
    } else {
        // The local is bound THROUGH a stringifying wrapper, which the runner
        // undoes anyway by reading the key with `Event::get_as_string`.
        let (_, bound) = script.split_once(&format!(" {term} = "))?;
        let path = key_term(bound.split([';', '\n']).next()?)?;
        clean_path(
            path.strip_prefix("ctx.")
                .or_else(|| path.strip_prefix("ctx?."))?,
        )
    };
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
        ParamsPattern::ScalarExpression(pattern) => {
            crate::expr::scalar_expression_params(event, pattern, params)
        }
        ParamsPattern::KeyedByLookup(pattern) => run_keyed_by_lookup(event, pattern, params),
        ParamsPattern::SplitNamedByPosition(pattern) => {
            run_split_named_by_position(event, pattern, params)
        }
        ParamsPattern::HexFields(pattern) => crate::hex::hex_fields(event, pattern, params),
        ParamsPattern::AwsEntity(script) => crate::entity::run_entity_script(event, script, params),
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
        ParamsPattern::SysmonRegistry => crate::windows::run_registry(event, params),
        ParamsPattern::MessageTable => crate::windows::run_message_table(event, params),
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
        ParamsPattern::ScheduledTask => crate::scheduled_task::run(event, params),
        ParamsPattern::ThreatIndicatorType(source) => {
            try_threat_indicator_type(event, source, params)
        }
        ParamsPattern::EvidenceCategories { source, key } => {
            run_evidence_categories(event, source, key, params)
        }
        ParamsPattern::MsgParts => run_msg_parts(event, params),
        ParamsPattern::MappingRow(pattern) => run_mapping_row(event, pattern, params),
        ParamsPattern::SecuritySddl => crate::sddl::run(event, params),
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
            gate,
        } => {
            // A record outside the gated list keeps the value it arrived with.
            if let Some(TableGate { field, list }) = gate
                && !gated_in(event, field, params.get(list))
            {
                return true;
            }
            // The script's own guard: an absent source writes nothing at all,
            // not the default.
            if let Some(key) = event.get_as_string(source) {
                let rows = params.get(table).and_then(Value::as_object);
                // A fallback ROW the table does not carry writes nothing --
                // Painless would store the null `getOrDefault` handed back, and
                // an explicit null is a field Elasticsearch's own prune removes.
                let value =
                    rows.and_then(|rows| rows.get(&key))
                        .cloned()
                        .or_else(|| match default {
                            TableDefault::Literal(text) => Some(json!(text)),
                            TableDefault::Row(row) => rows.and_then(|rows| rows.get(row)).cloned(),
                        });
                if let Some(value) = value {
                    let _ = event.set(target, value);
                }
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
        ParamsPattern::RemapKeysThroughTable(pattern) => {
            run_remap_keys_through_table(event, pattern, params)
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
        ParamsPattern::RenameKeys(pattern) => run_rename_keys(event, pattern, params),
        ParamsPattern::SelectRenameKeys(pattern) => run_select_rename_keys(event, pattern, params),
        ParamsPattern::FieldTables(pattern) => {
            crate::field_tables::field_tables(event, pattern, params)
        }
        ParamsPattern::MemberMappings(list) => run_member_mappings(event, list, params),
        ParamsPattern::ConfiguredDateFormat(pattern) => {
            run_configured_date_format(event, pattern, params)
        }
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
        ParamsPattern::ContainsFlags(pattern) => run_contains_flags(event, pattern, params),
        ParamsPattern::LookupMerge(literals) => {
            try_lookup_merge(event, normalised, params, literals)
        }
        ParamsPattern::LookupColumns => try_lookup_columns(event, normalised, params),
        ParamsPattern::ActionMapping(pattern) => {
            crate::action_mapping::run_action_mapping(event, pattern, params)
        }
        ParamsPattern::RowNamedTarget(pattern) => {
            crate::row_named_target::run_row_named_target(event, pattern, params)
        }
        ParamsPattern::LabelledScore(pattern) => {
            crate::labelled_score::run_labelled_score(event, pattern, params)
        }
        ParamsPattern::UrlTailAction(pattern) => {
            crate::url_action::url_tail_action(event, pattern, params)
        }
        ParamsPattern::NormalisedLookup {
            source,
            target,
            fold,
        } => run_normalised_lookup(event, source, target, *fold, params),
        ParamsPattern::StringifiedLookup { source, target } => {
            run_stringified_lookup(event, source, target, params)
        }
        ParamsPattern::StringifiedLookupOrLiteral {
            source,
            absent_key,
            target,
            default,
        } => run_stringified_lookup_or_literal(event, source, absent_key, target, default, params),
        ParamsPattern::LookupNormalise(pattern, literals) => {
            lookup_normalise(event, pattern, literals, params)
        }
        ParamsPattern::RenameMemberByLookup(pattern) => {
            run_rename_member_by_lookup(event, pattern, params)
        }
        ParamsPattern::MemberLookup(pattern) => member_lookup(event, pattern, params),
        ParamsPattern::GuardedLookup(pattern) => guarded_lookup(event, pattern, params),
        ParamsPattern::CollectParamsRows(pattern) => {
            crate::collect_rows::collect_params_rows(event, pattern, params)
        }
        ParamsPattern::TrimDelimited(pattern) => {
            crate::trim_delimited::trim_delimited(event, pattern, params)
        }
        ParamsPattern::IndexedLookup => try_indexed_lookup(event, normalised, params),
        ParamsPattern::Scale => try_scale(event, normalised, params),
        ParamsPattern::Replace => try_replace(event, normalised, params),
        ParamsPattern::AddUniqueRow => try_add_unique_row(event, normalised, params),
        ParamsPattern::FrameworkPreference => try_framework_preference(event, normalised, params),
        ParamsPattern::RowOrDefaults(pattern) => run_row_or_defaults(event, pattern, params),
        ParamsPattern::ArmedTable(pattern) => {
            crate::named_arms::armed_table(event, pattern, params)
        }
        ParamsPattern::IssueLifecycle(pattern) => {
            crate::issue_lifecycle::issue_lifecycle(event, pattern, params)
        }
        // The arms are tried in the order the script writes them, and the
        // fallback runs unguarded -- the vendor's `else` has no condition, so
        // the field is written on every event the processor reaches.
        // A key the table does not carry writes NOTHING, which is what the
        // script's own `containsKey` guard says.
        ParamsPattern::RowMembers(pattern) => {
            if let Some(row) = event
                .get_str(&pattern.key)
                .and_then(|key| params.get(key))
                .and_then(Value::as_object)
                .cloned()
            {
                for (member, target) in &pattern.writes {
                    if let Some(value) = row.get(member) {
                        let _ = event.set(target, value.clone());
                    }
                }
            }
            true
        }
        ParamsPattern::GuardedParamsRow(pattern) => {
            let chosen = pattern
                .arms
                .iter()
                .find(|(path, wanted, _)| match wanted {
                    None => event.has_value(path),
                    Some(value) => event.get(path).is_some_and(|held| held == value),
                })
                .map_or(pattern.fallback.as_str(), |(_, _, key)| key.as_str());
            if let Some(value) = params.get(chosen) {
                let _ = event.set(&pattern.target, value.clone());
            }
            true
        }
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

/// Whether the document's `field` is one the params `list` names.
///
/// A missing field or a list the params block does not carry is OUT, the same
/// way `List.contains` answers false for a null it was never given.
fn gated_in(event: &Event, field: &str, list: Option<&Value>) -> bool {
    let Some(value) = event.get(field) else {
        return false;
    };
    list.and_then(Value::as_array)
        .is_some_and(|names| names.contains(value))
}

/// One NAMED params table read with a default, in either spelling.
///
/// Two readers, one pattern: [`parse_ternary_table_lookup`] takes the
/// `containsKey` ternary and [`parse_table_lookup_or_row`] the `getOrDefault`.
fn parse_table_lookup_or_literal(script: &str) -> Option<ParamsPattern> {
    let lookup =
        parse_ternary_table_lookup(script).or_else(|| parse_table_lookup_or_row(script))?;
    Some(ParamsPattern::TableLookupOrLiteral {
        source: lookup.source,
        table: lookup.table,
        target: lookup.target,
        default: lookup.default,
        gate: lookup.gate,
    })
}

/// The parts either spelling of the lookup resolves to.
struct TableLookup {
    source: String,
    table: String,
    target: String,
    default: TableDefault,
    gate: Option<TableGate>,
}

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
fn parse_ternary_table_lookup(script: &str) -> Option<TableLookup> {
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
    let default = TableDefault::Literal(quoted_after(literal.split(';').next()?, "")?);

    // The LAST `ctx.` in the script is the write. The allocation line above it
    // names the root only, and `Event::set` builds that anyway.
    let assigned = script.rsplit_once("ctx.")?.1;
    let target = clean_path(assigned.split('=').next()?.trim());
    if target.is_empty() || target.contains(char::is_whitespace) {
        return None;
    }

    Some(TableLookup {
        source,
        table,
        target,
        default,
        gate: None,
    })
}

/// The same NAMED table, spelled `getOrDefault`, falling back to another ROW
/// of itself rather than to a literal.
///
/// The whole script, as qualys writes it over its severity table:
///
/// ```painless
/// if (!(ctx.<source> instanceof String)) { return; }
/// String level = ctx.<source>;
/// ctx.<target> = params.<table>.getOrDefault(level, params.<table>["0"]);
/// ```
///
/// The fallback has to name the SAME table and subscript it with a literal.
/// Every other `getOrDefault` in the tree defaults to `null`, to the key, or to
/// the field's own current value, and each of those is a different pattern.
///
/// `qualys_vmdr`'s `asset_host_detection` stream writes the same lookup inside
/// `if (params.vuln_types.contains(vuln_type)) { ... }`, which
/// [`parse_membership_gate`] reads and the runner then honours.
fn parse_table_lookup_or_row(script: &str) -> Option<TableLookup> {
    let (head, rest) = script.split_once(".getOrDefault(")?;

    // The assignment usually sits at the script's TOP LEVEL. Where it does not,
    // a params-list membership test is the only enclosing guard this reader
    // takes; anything else left open is a different script and declines.
    let gate = if head.matches('{').count() == head.matches('}').count() {
        None
    } else {
        Some(parse_membership_gate(script, head)?)
    };

    let table = head.rsplit("params.").next()?.trim().to_string();
    if table.is_empty() || table.contains(['.', ' ', '(', '[', '?']) {
        return None;
    }

    let (key, fallback) = rest.split_once(',')?;
    let key = key.trim();
    let row = fallback
        .trim_start()
        .strip_prefix(&format!("params.{table}["))?
        .split(']')
        .next()?
        .trim()
        .trim_matches(['\'', '"'])
        .to_owned();
    if row.is_empty() {
        return None;
    }

    // The key is either the ctx path itself or a local bound to one.
    let source = match key.strip_prefix("ctx.") {
        Some(path) => clean_path(path),
        None => clean_path(
            script
                .split_once(&format!(" {key} = ctx."))?
                .1
                .split([';', '\n'])
                .next()?
                .trim(),
        ),
    };
    if source.is_empty() || source.contains(char::is_whitespace) {
        return None;
    }

    // The assignment the lookup feeds is the `ctx.` immediately before it.
    let target = clean_path(head.rsplit("ctx.").next()?.split('=').next()?.trim());
    if target.is_empty() || target.contains(char::is_whitespace) {
        return None;
    }

    Some(TableLookup {
        source,
        table,
        target,
        default: TableDefault::Row(row),
        gate,
    })
}

/// `if (params.<list>.contains(<local>)) {` as the field to read and the list to
/// test it against.
///
/// `head` is the script up to the lookup, so the guard this reads is the last
/// one opened before it. Nothing else may be left open: two nested guards mean
/// a condition this does not model, and claiming the script would run the
/// lookup where the vendor does not.
fn parse_membership_gate(script: &str, head: &str) -> Option<TableGate> {
    let (before, after) = head.rsplit_once("if (params.")?;
    if before.matches('{').count() != before.matches('}').count() {
        return None;
    }

    let (list, rest) = after.split_once(".contains(")?;
    if list.is_empty() || list.contains(['.', ' ', '(', '[']) {
        return None;
    }

    // The tested value is either a ctx path or a local bound to one.
    let tested = rest.split_once(')')?.0.trim();
    let field = match tested.strip_prefix("ctx.") {
        Some(path) => clean_path(path),
        None => clean_path(
            script
                .split_once(&format!(" {tested} = ctx."))?
                .1
                .split([';', '\n'])
                .next()?
                .trim(),
        ),
    };
    if field.is_empty() || field.contains(char::is_whitespace) {
        return None;
    }

    Some(TableGate {
        field,
        list: list.to_owned(),
    })
}

/// `def name = ctx.<source>.toUpperCase(); ... ctx.<target> = params.getOrDefault(name, name);`
///
/// The default being the KEY itself is what makes this its own pattern: a name
/// the table does not abbreviate stands for itself rather than going missing.
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

/// `def <k> = Long.toString(ctx.<source>); if (params.containsKey(<k>)) {
/// ctx.<container>.put('<leaf>', params[<k>]); }`
///
/// One lookup, a second spelling: the key is bound to a LOCAL, the guard is
/// positive rather than an early return, and the write is a `.put`. The local
/// is what ties the three halves together, so a script guarding on one key and
/// writing another declines -- `symantec_endpoint_security` spells the same
/// guard over a set it built elsewhere, on 70 call sites.
fn parse_local_key_lookup(script: &str) -> Option<ParamsPattern> {
    const GUARD: &str = "params.containsKey(";
    // `!params.containsKey(` is the early-return spelling above, which reads
    // its key inline and must keep its own position in the ladder.
    let (at, _) = script
        .match_indices(GUARD)
        .find(|(at, _)| !script[..*at].ends_with('!'))?;
    let local = balanced_argument(&script[at + GUARD.len()..])?;
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    let binder = format!("def {local} = ");
    let bound = script.split_once(&binder)?.1;
    let source = stringified_key_path(bound.split(';').next()?)?;

    // The write must read the SAME key, or this is a different script that
    // happens to spell both halves.
    let subscript = format!("params[{local}]");
    let target = ctx_put_writes(script)
        .into_iter()
        .find_map(|(path, value)| (value == subscript).then_some(path))?;

    Some(ParamsPattern::StringifiedLookup { source, target })
}

/// The `ctx.` path a key expression names, through the calls Painless wraps a
/// stringify in.
///
/// [`key_path`] reads the trailing `.toString()` spelling already; these are
/// the ones written as a call around the path.
fn stringified_key_path(expr: &str) -> Option<String> {
    let expr = expr.trim();
    for call in ["Long.toString(", "Integer.toString(", "String.valueOf("] {
        if let Some(inner) = expr.strip_prefix(call) {
            return key_path(inner.strip_suffix(')')?);
        }
    }
    key_path(expr)
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

/// ```painless
/// def <local> = (ctx.<source> ?: '<absent>').toString();
/// if (params.containsKey(<local>)) { ctx.<target> = params[<local>]; }
/// else { ctx.<target> = <default>; }
/// ```
///
/// The `?:` is what separates this from [`parse_stringified_lookup`]: an
/// absent source is the elvis literal rather than a null, so the script always
/// reaches one arm or the other and the target is always written.
fn parse_stringified_lookup_or_literal(script: &str) -> Option<ParamsPattern> {
    let (statement, rest) = skip_trivia(script).strip_prefix("def ")?.split_once(';')?;
    let (local, expression) = statement.split_once('=')?;
    let local = identifier(local)?;
    let (source, absent_key) = elvis_key(expression)?;

    let rest = skip_trivia(rest).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    if without_spaces(test) != format!("params.containsKey({local})") {
        return None;
    }

    let (found, rest) = balanced(skip_trivia(rest), '{', '}')?;
    let (target, expression) = only_ctx_assignment(found)?;
    if without_spaces(expression) != format!("params[{local}]") {
        return None;
    }

    let rest = skip_trivia(rest).strip_prefix("else")?;
    let (missing, rest) = balanced(skip_trivia(rest), '{', '}')?;
    // The two arms must write the SAME field, or this is two lookups sharing a
    // key rather than one lookup with a fallback.
    let (fallback, literal) = only_ctx_assignment(missing)?;
    if fallback != target || !skip_trivia(rest).is_empty() {
        return None;
    }
    let default = crate::common::painless_literal(literal)?;

    Some(ParamsPattern::StringifiedLookupOrLiteral {
        source,
        absent_key,
        target,
        default,
    })
}

/// `(ctx.<path> ?: '<literal>').toString()` as the path and the literal.
///
/// Both halves are required: without the elvis, Painless throws on an absent
/// field, which is a different script and a different outcome.
fn elvis_key(expression: &str) -> Option<(String, String)> {
    let inner = skip_trivia(expression)
        .trim_end()
        .strip_suffix(".toString()")?;
    let (inner, rest) = balanced(skip_trivia(inner), '(', ')')?;
    if !rest.trim().is_empty() {
        return None;
    }
    let (path, default) = inner.split_once("?:")?;
    Some((ctx_path_plain(path)?, quoted_whole(default)?))
}

/// A whole trimmed text that is one single- or double-quoted literal.
///
/// Unlike the readers that hunt for the first quote, this admits the EMPTY
/// string, which is what the elvis defaults to.
fn quoted_whole(text: &str) -> Option<String> {
    let text = text.trim();
    for quote in ['\'', '"'] {
        if let Some(inner) = text
            .strip_prefix(quote)
            .and_then(|rest| rest.strip_suffix(quote))
            && !inner.contains(quote)
        {
            return Some(inner.to_string());
        }
    }
    None
}

/// The one `ctx.<path> = <expression>;` a block holds, and nothing else.
fn only_ctx_assignment(block: &str) -> Option<(String, &str)> {
    let (statement, tail) = skip_trivia(block).split_once(';')?;
    if !skip_trivia(tail).is_empty() {
        return None;
    }
    let (lhs, rhs) = statement.split_once('=')?;
    Some((ctx_path_plain(lhs)?, rhs))
}

/// A local's name, or `None` where the text is not one identifier.
fn identifier(text: &str) -> Option<&str> {
    let name = text.trim();
    (!name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
    .then_some(name)
}

/// An expression with every space removed, so spacing is not part of a match.
fn without_spaces(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Look the key up, falling back to the script's own literal.
///
/// An absent or null source is the elvis literal, so the target is written on
/// every event -- the miss writes the default rather than nothing.
fn run_stringified_lookup_or_literal(
    event: &mut Event,
    source: &str,
    absent_key: &str,
    target: &str,
    default: &Value,
    params: &Map<String, Value>,
) -> bool {
    // A CONTAINER is neither the elvis case nor a key: Painless stringifies it
    // to `[a, b]`, which no vendored table is keyed by. Standing the elvis
    // literal in for it would find a row the script cannot reach.
    let key = match event.get(source) {
        None | Some(Value::Null) => Some(absent_key.to_owned()),
        Some(_) => event.get_as_string(source),
    };
    let value = key
        .and_then(|key| params.get(&key).cloned())
        .unwrap_or_else(|| default.clone());
    let _ = event.set(target, value);
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

/// What a column writes when its row member is absent, null or an empty list.
///
/// The plain `ctx.<target> = <row>.<member>` form writes whatever the member
/// holds and skips only an explicit null. Where the script routes the member
/// through a list accumulator instead, three more answers become possible and
/// the difference is the whole value written: `jamf_pro`'s `event.type` reads
/// `['info']` from an EMPTY column and `event.category` is not written at all.
#[derive(Debug, Clone, PartialEq, Eq)]
enum EmptyColumn {
    /// Not accumulated: write the member as it stands, skip an explicit null.
    Verbatim,
    /// Accumulated, and an empty accumulator is written as the empty list.
    Empty,
    /// Accumulated, and an empty accumulator is not written --
    /// `if (!cats.isEmpty()) { ctx.event.category = cats; }`.
    Skip,
    /// Accumulated, and an empty accumulator is filled first --
    /// `if (types.isEmpty()) { types.add('info'); }`.
    Fill(Value),
}

/// One column a row fans out, and where it lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowColumn {
    target: String,
    member: String,
    /// The script guards the write on the target still being unset --
    /// `if (m.containsKey('outcome') && ctx.event.outcome == null)`.
    only_if_unset: bool,
    /// What the script writes where the member carries nothing.
    empty: EmptyColumn,
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
                empty: EmptyColumn::Verbatim,
            });
        }
        rest = after;
    }

    plain.push_str(rest);
    (columns, plain)
}

/// A list local the script builds from one row member before writing it.
struct Accumulator {
    /// The row member `addAll` feeds it, once one is seen.
    member: Option<String>,
    /// What an `isEmpty` fallback puts in it, once one is seen.
    fill: Option<Value>,
    /// Whether a write has consumed it, which is what makes the local
    /// accounted for.
    written: bool,
}

/// The local a `def <local> = new ArrayList();` statement opens.
///
/// The EMPTY parens are the whole trigger: `new ArrayList(m.category)` is the
/// kolide form, which copies a member rather than accumulating one, and
/// reading it as an accumulator would leave the copy unwritten.
fn opened_accumulator(statement: &str) -> Option<String> {
    let (local, value) = statement.trim().strip_prefix("def ")?.split_once('=')?;
    let local = local.trim();
    if value.trim() != "new ArrayList()"
        || local.is_empty()
        || !local.chars().all(|c| c.is_alphanumeric() || c == '_')
    {
        return None;
    }
    Some(local.to_string())
}

/// The value a `{ L.add(<literal>); ... }` block leaves in an empty `L`.
fn added_literals(block: &str, local: &str) -> Option<Value> {
    let opens = format!("{local}.add(");
    let mut members = Vec::new();
    for statement in block.split(';') {
        let statement = skip_trivia(statement);
        if statement.is_empty() {
            continue;
        }
        let inner = statement.strip_prefix(opens.as_str())?.strip_suffix(')')?;
        members.push(literal_value(inner)?);
    }
    (!members.is_empty()).then(|| Value::Array(members))
}

/// The columns a script accumulates into a list local before writing them.
///
/// ```painless
/// def cats = new ArrayList();
/// def types = new ArrayList();
/// if (entry.category != null) { cats.addAll(entry.category); }
/// if (entry.type != null) { types.addAll(entry.type); }
/// if (types.isEmpty()) { types.add('info'); }
/// if (!cats.isEmpty()) { ctx.event.category = cats; }
/// ctx.event.type = types;
/// ```
///
/// Every statement this reads is cut out of the returned block, so the plain
/// scan after it neither takes a write without its guard nor trips the
/// caller's `if (` bail-out on a guard already accounted for. It declines any
/// block it cannot account for whole, because a claim that reproduces some
/// columns and drops the rest leaves no error behind.
fn accumulated_columns(block: &str, row: &str) -> Option<(Vec<RowColumn>, String)> {
    let mut opened: Vec<(String, Accumulator)> = Vec::new();
    let mut columns = Vec::new();
    let mut plain = String::with_capacity(block.len());
    let mut rest = block;

    while !rest.is_empty() {
        let head = skip_trivia(rest);
        plain.push_str(&rest[..rest.len() - head.len()]);
        rest = head;
        if rest.is_empty() {
            break;
        }

        // `if (<test>) { <body> }`, which is where every accumulator statement
        // but the bare write lives.
        if let Some(after_if) = rest.strip_prefix("if")
            && let Some((test, after)) = balanced(skip_trivia(after_if), '(', ')')
            && let Some((body, after)) = balanced(skip_trivia(after), '{', '}')
        {
            let taken = read_accumulator_guard(test, body, row, &mut opened, &mut columns);
            if !taken {
                plain.push_str(&rest[..rest.len() - after.len()]);
            }
            rest = after;
            continue;
        }

        // Anything else is a statement: the `def <local> = new ArrayList();`
        // that opens an accumulator, or the unguarded `ctx.<t> = <local>;`
        // that closes one.
        let (statement, after) = match rest.split_once(';') {
            Some((statement, after)) => (statement, after),
            None => (rest, ""),
        };
        let taken = match opened_accumulator(statement) {
            Some(local) => {
                if opened.iter().any(|(name, _)| *name == local) {
                    return None;
                }
                opened.push((
                    local,
                    Accumulator {
                        member: None,
                        fill: None,
                        written: false,
                    },
                ));
                true
            }
            None => read_accumulator_write(statement, &mut opened, &mut columns, false),
        };
        if !taken {
            plain.push_str(&rest[..rest.len() - after.len()]);
        }
        rest = after;
    }

    // A local fed from the row but never written, or one the block still
    // names after the walk, means the model of the block is incomplete. A
    // local nothing ever fed is dead and drops without loss.
    for (local, state) in &opened {
        if (state.member.is_some() && !state.written) || plain.contains(local.as_str()) {
            return None;
        }
    }
    Some((columns, plain))
}

/// Read one `if (<test>) { <body> }` as an accumulator statement, reporting
/// whether it was one.
fn read_accumulator_guard(
    test: &str,
    body: &str,
    row: &str,
    opened: &mut [(String, Accumulator)],
    columns: &mut Vec<RowColumn>,
) -> bool {
    let test = test.trim();

    // `if (<row>.<member> != null) { <local>.addAll(<row>.<member>); }`
    if let Some(member) = test
        .strip_prefix(&format!("{row}."))
        .and_then(|rest| rest.strip_suffix("!= null").map(str::trim_end))
        .filter(|member| member.chars().all(|c| c.is_alphanumeric() || c == '_'))
        && let Some((local, _)) = skip_trivia(body).split_once(".addAll(")
        && skip_trivia(body).trim_end().trim_end_matches(';')
            == format!("{local}.addAll({row}.{member})")
        && let Some((_, state)) = opened.iter_mut().find(|(name, _)| name == local)
        && state.member.is_none()
    {
        state.member = Some(member.to_string());
        return true;
    }

    // `if (<local>.isEmpty()) { <local>.add('info'); }`
    if let Some(local) = test.strip_suffix(".isEmpty()")
        && let Some((_, state)) = opened.iter_mut().find(|(name, _)| name == local)
        && state.fill.is_none()
        && let Some(filled) = added_literals(body, local)
    {
        state.fill = Some(filled);
        return true;
    }

    // `if (!<local>.isEmpty()) { ctx.<target> = <local>; }`
    if let Some(local) = test
        .strip_prefix('!')
        .and_then(|rest| rest.trim_start().strip_suffix(".isEmpty()"))
        && opened.iter().any(|(name, _)| name == local)
    {
        return read_accumulator_write(body, opened, columns, true);
    }

    false
}

/// Read a `ctx.<target> = <local>` statement as an accumulator's write,
/// reporting whether it was one.
fn read_accumulator_write(
    statement: &str,
    opened: &mut [(String, Accumulator)],
    columns: &mut Vec<RowColumn>,
    guarded: bool,
) -> bool {
    let Some((target, local)) = statement.trim().trim_end_matches(';').split_once('=') else {
        return false;
    };
    let Some(target) = target.trim().strip_prefix("ctx.") else {
        return false;
    };
    let local = local.trim();
    let Some((_, state)) = opened.iter_mut().find(|(name, _)| name == local) else {
        return false;
    };
    // A local written before anything fed it from the row does not depend on
    // the row at all, and reproducing it as a column would invent a member.
    let (Some(member), false) = (state.member.clone(), state.written) else {
        return false;
    };

    columns.push(RowColumn {
        target: clean_path(target),
        member,
        only_if_unset: false,
        // A fill runs before the write, so it settles the empty case whatever
        // the guard says. Without one, the guard separates writing nothing
        // from writing `[]`.
        empty: match (state.fill.clone(), guarded) {
            (Some(filled), _) => EmptyColumn::Fill(filled),
            (None, true) => EmptyColumn::Skip,
            (None, false) => EmptyColumn::Empty,
        },
    });
    state.written = true;
    true
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
        None => crate::common::ctx_path_bound_to(script, &key)?,
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
    let (accumulated, plain) = accumulated_columns(&plain, &row)?;
    columns.extend(accumulated);
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
            empty: EmptyColumn::Verbatim,
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
        let held = row.get(&column.member).filter(|value| !value.is_null());
        let written = match &column.empty {
            EmptyColumn::Verbatim => held.cloned(),
            rule => accumulated_value(held, rule),
        };
        if let Some(written) = written {
            let _ = event.set(&column.target, written);
        }
    }
    true
}

/// What an accumulated column writes for the member it read.
///
/// `addAll` takes a collection, so the write is always a LIST -- a scalar
/// member is wrapped rather than dropped, which is the closest reading of a
/// table Painless itself would throw on.
fn accumulated_value(held: Option<&Value>, rule: &EmptyColumn) -> Option<Value> {
    let members = match held {
        Some(Value::Array(members)) => members.clone(),
        Some(other) => vec![other.clone()],
        None => Vec::new(),
    };
    if !members.is_empty() {
        return Some(Value::Array(members));
    }
    match rule {
        EmptyColumn::Fill(filled) => Some(filled.clone()),
        EmptyColumn::Empty => Some(Value::Array(members)),
        EmptyColumn::Skip | EmptyColumn::Verbatim => None,
    }
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
    let subject = crate::common::ctx_path_bound_to(head, key)?;
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

/// mimecast's scored log-type classifier, tables from params.
///
/// Keys lowercase into a Java `HashSet`; a `definite_positive` hit wins
/// outright, then candidate ELIMINATION through the `negative` table (a lone
/// survivor wins), then the `positive` table scores what remains and every
/// co-equal winner is listed. Iteration orders are Java's hash orders, which
/// [`crate::helpers::java_bucket`] reproduces -- the corpus's own
/// single-winner strings depend on them.
fn try_mimecast_log_type(event: &mut Event, params: &Map<String, Value>) -> bool {
    use crate::helpers::{java_bucket, java_table_size};

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
    let subject = crate::common::ctx_path_bound_to(script, local)?;

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

/// A map's keys renamed through a params table, chosen by a presence test.
///
/// `ti_recordedfuture` decodes its risklist CSV into `_tmp_.col0..col4`, then
/// needs to know which layout it read: four columns for url, domain and IP,
/// five for hash, with `Algorithm` inserted second. The absence of the last
/// column is the whole discriminator, and the chosen table maps each position
/// onto its name.
///
/// Without it the columns never become `json.Name` and the next processor
/// raises `field not found`, taking the whole event down the error path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RemapKeysThroughTable {
    source: String,
    target: String,
    /// The field whose ABSENCE picks the first table.
    selector: String,
    absent: String,
    present: String,
}

/// `def cols = params[ctx.a.x == null ? "d" : "h"]; ... dst[v] = src[k]; ctx.b = dst`
fn parse_remap_keys_through_table(script: &str) -> Option<RemapKeysThroughTable> {
    // The selector and the two table names, off the ternary that picks one.
    let (head, rest) = script.split_once("== null")?;
    let selector = clean_path(head.rsplit("ctx.").next()?.trim());
    let mut names = crate::common::quoted_members(rest.split(']').next()?);
    if names.len() < 2 || selector.is_empty() {
        return None;
    }
    let present = names.pop()?;
    let absent = names.pop()?;

    // The map read through the table, and the one built from it.
    let source = clean_path(
        script
            .split_once("= ctx.")?
            .1
            .split([';', '\n'])
            .next()?
            .trim(),
    );
    let (target_head, _) = script.split_once("] = dst")?;
    let target = clean_path(
        target_head
            .rsplit("ctx[")
            .next()?
            .trim()
            .trim_matches(['\'', '"']),
    );
    if source.is_empty() || target.is_empty() {
        return None;
    }
    Some(RemapKeysThroughTable {
        source,
        target,
        selector,
        absent,
        present,
    })
}

/// Build the renamed map, in the table's own order.
fn run_remap_keys_through_table(
    event: &mut Event,
    pattern: &RemapKeysThroughTable,
    params: &Map<String, Value>,
) -> bool {
    let Some(source) = event
        .get(&pattern.source)
        .and_then(Value::as_object)
        .cloned()
    else {
        return false;
    };
    // The LAST column's absence is the discriminator, nothing else.
    let table = if event.has_value(&pattern.selector) {
        &pattern.present
    } else {
        &pattern.absent
    };
    let Some(table) = params.get(table).and_then(Value::as_object) else {
        return false;
    };

    // In the table's declaration order, which is what `entrySet()` walks.
    let mut renamed = Map::with_capacity(table.len());
    for (from, to) in table {
        let Some(to) = to.as_str() else { continue };
        // A column the record does not carry is left out rather than written
        // as an explicit null, which the pipeline's own drop-empty would
        // remove a moment later anyway.
        if let Some(value) = source.get(from) {
            renamed.insert(to.to_owned(), value.clone());
        }
    }
    if renamed.is_empty() {
        return false;
    }
    let _ = event.set(&pattern.target, Value::Object(renamed));
    true
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
    let base = crate::common::ctx_path_bound_to(script, &local)?;

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
    let placeholders =
        crate::common::quoted_members(script.split_once("if (v == ")?.1.split_once(") {")?.0);
    if placeholders.is_empty() {
        return None;
    }

    // Both key lists come off the call, where they are still `params.<name>`.
    let arguments = script
        .split_once("processFieldValue(k, ")?
        .1
        .split_once(')')?
        .0;
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
            let mut literals = crate::common::quoted_members(condition).into_iter();
            let key = literals.next()?;
            let from = literals.next()?;
            let to = crate::common::quoted_members(body.split_once(';')?.0)
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
        let Some(path) = crate::common::ctx_path_bound_to(script, &var) else {
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
        crate::common::drop_empty_recursive(event, &crate::common::DropPolicy::read(script));
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
        for (path, value) in absent_row_writes(script) {
            let _ = event.set(&path, value);
        }
        return true;
    };
    for (k, v) in row.clone() {
        let path = match routes.iter().find(|(key, _)| *key == k) {
            Some((_, MergeRoute::At(path))) => path.clone(),
            Some((_, MergeRoute::Under(path))) => format!("{path}.{k}"),
            // Written once, after the loop, because the seed goes in front of
            // it and has to land whether or not the row carries the key.
            Some((_, MergeRoute::Collect(_))) => continue,
            None => format!("{target}.{k}"),
        };
        let _ = event.set(&path, v);
    }
    if let Some((key, local)) = routes.iter().find_map(|(key, route)| match route {
        MergeRoute::Collect(local) => Some((key.as_str(), local.as_str())),
        MergeRoute::At(_) | MergeRoute::Under(_) => None,
    }) && let Some((mut list, path)) = collected_list(script, local)
    {
        if let Some(Value::Array(members)) = row.get(key) {
            list.extend(members.iter().cloned());
        }
        let _ = event.set(&path, Value::Array(list));
    }
    true
}

/// The writes a leading `if (params.get(<key>) == null) { ... return; }` makes.
///
/// That branch is the script's answer for a key the table has no row for, and
/// the [`Program`] the pattern carries holds it under a guard the evaluator
/// reads as `Never` -- a `Program` never sees `params`, so it cannot tell
/// whether the row is there. Most of these scripts return and write nothing, and
/// for those this reads nothing; `beyondtrust_pra` writes a session category and
/// an info type first.
fn absent_row_writes(script: &str) -> Vec<(String, Value)> {
    let head = script
        .split_once("forEach")
        .map_or(script, |(head, _)| head);
    let Some(at) = head.find("if (params.get(") else {
        return Vec::new();
    };
    let Some((test, block, _)) = guard_and_block(&head[at + "if".len()..]) else {
        return Vec::new();
    };
    // The branch has to be the row-missing one and has to RETURN, or what
    // follows it is the merge rather than an alternative to it.
    if !test.contains("== null") || !block.contains("return") {
        return Vec::new();
    }
    literal_writes(block)
}

/// The seed a collected key starts from, and the path the finished list lands
/// at: `def <local> = new ArrayList([...]);` ... `ctx.<path> = <local>`.
fn collected_list(script: &str, local: &str) -> Option<(Vec<Value>, String)> {
    let seeded = format!(" {local} = new ArrayList(");
    let at = script.find(&seeded)? + seeded.len();
    let (argument, _) = balanced(&script[at - 1..], '(', ')')?;
    let Some(Value::Array(seed)) = literal_value(argument.trim()) else {
        return None;
    };

    // The LAST `... = <local>` that ends a statement, so the `addAll` inside the
    // lambda and the declaration itself are both passed over.
    let assigned = format!("= {local}");
    let end = script
        .match_indices(assigned.as_str())
        .filter(|(found, _)| {
            let after = script[found + assigned.len()..].trim_start();
            after.is_empty() || after.starts_with(';')
        })
        .last()?
        .0;
    let head = script[..end].trim_end();
    let start = head.rfind("ctx.")? + "ctx.".len();
    Some((seed, clean_path(&head[start..])))
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
    crate::common::ctx_path_bound_to(script, reference)
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
    /// `<local>.addAll(v)` -- appended to a seeded local the script writes out
    /// after the merge. The `String` is that local's name.
    Collect(String),
}

/// The key-routed arms of a merge lambda, each with the path it writes.
///
/// A row normally merges whole into one container, but `watchguard_firebox`
/// sends `log_type` to its own namespace and suricata sends `network_protocol`
/// to `network.protocol`. Both spellings of the guard appear across the
/// integrations, and so do both spellings of the write.
///
/// An arm that COLLECTS rather than writes names no path at all --
/// `beyondtrust_pra` adds `category` to a local the script seeded before the
/// merge and writes out after it. Left to the default, the seed was dropped and
/// a row without the key wrote nothing, so it is its own route.
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
    // Checked first: an arm that collects writes no path at all, and the two
    // readers below would fall through it to whatever the enclosing lambda
    // assigns.
    if let Some((head, _)) = arm.split_once(".addAll(v)") {
        let local = head
            .rsplit(|c: char| c.is_whitespace() || matches!(c, '{' | '}' | ';' | '(' | ')'))
            .next()?;
        if !local.is_empty() && !local.contains(['.', '[']) {
            return Some(MergeRoute::Collect(local.to_string()));
        }
    }
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

/// A member MOVED to a sibling the params table NAMES.
///
/// ```painless
/// def data = ctx.json.model.logic.data;
/// if (ctx.json.model.logic?.type != null) {
///   if (['componentList', 'weightedComponentList'].contains(ctx.json.model.logic?.type)) {
///     ctx["json"]["model"]["logic"][params.get(ctx.json.model.logic?.type)] = data;
///   } else {
///     ctx["json"]["model"]["logic"]["data_" + ctx.json.model.logic?.type] = data;
///   }
/// }
/// ctx.json.model.logic.remove("data");
/// ```
///
/// No lookup matcher in this ladder can express it, because the row is not the
/// VALUE written -- it is the NAME written under. Every one of them takes a path
/// the script spells and stores the row there, and here the path is what the
/// table decides. darktrace's model logic carries its payload under `data` and
/// names the column beside it, so one event holds a list of component ids and
/// the next a list of weighted components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RenameMemberByLookup {
    /// The container holding both the member read and the member written.
    container: String,
    /// The member the value comes from, which the script then prunes, as the
    /// whole path -- the script fixes it, so it is joined here rather than per
    /// event.
    source: String,
    /// The `ctx.` path naming the row, as the whole document spells it.
    key: String,
    /// The keys the script routes through the table. Anything else takes the
    /// prefix below, which is the script's own `else` arm rather than a
    /// fallback this invented.
    listed: Vec<String>,
    /// What an unlisted key is prefixed with to name its member.
    prefix: String,
}

/// Read the container, the member, the key and both naming arms, or decline.
///
/// Every part is read off the script and cross-checked: the lookup has to BE
/// the subscript, the two arms have to write the same container from the same
/// key, and the prune has to name that container's member. A script that says
/// anything else declines here rather than binding to a runner that would
/// invent a column name.
fn parse_rename_member_by_lookup(script: &str) -> Option<RenameMemberByLookup> {
    // ONE lookup, and it has to be the SUBSCRIPT. Either other spelling means
    // the row is written as a value somewhere, which is a different pattern.
    if script.matches("params.get(").count() != 1 || script.contains("params[") {
        return None;
    }

    // The prune names the container and the member, in the dotted spelling.
    let member = quoted_after(script, ".remove(")?;
    let container = ctx_path_before(script, ".remove(")?;

    // The local both arms write, read off the binding that takes the very
    // member the prune removes. Tying the two together is what says the script
    // MOVES that member rather than writing something else beside it.
    let bound = format!(" = ctx.{container}.{member};");
    let local = identifier_ending(script.split_once(&bound)?.0)?;

    // The key, off the lookup's own argument.
    let key = clean_path(
        last_call_argument(script, "params.get(")?
            .trim()
            .strip_prefix("ctx.")?,
    );
    if !is_ctx_path(&key) {
        return None;
    }

    // Arm one: `ctx[...][params.get(ctx.<key>)] = <local>`.
    let (head, after) = script.split_once("[params.get(")?;
    let written = after.split_once(")]")?.1.trim().strip_prefix('=')?;
    if subscripted_subject(head)? != container || assigned_local(written) != local {
        return None;
    }

    // Arm two names the member itself, and has to write the SAME container
    // from the SAME key -- a script keying on one field and naming another
    // says more than this pattern can.
    let (head, after) = script.split_once(" else {")?.1.split_once("] =")?;
    let (subject, key_expr) = head.rsplit_once('[')?;
    if subscripted_subject(subject)? != container || assigned_local(after) != local {
        return None;
    }
    let (prefix, concatenated) = key_expr.split_once('+')?;
    let prefix = literal_at(prefix)?;
    if clean_path(concatenated.trim().strip_prefix("ctx.")?) != key {
        return None;
    }

    // The allow-list guard the two arms hang off, over that same key.
    let (listed, argument) = script.split_once("].contains(")?;
    let Some(Value::Array(listed)) = literal_value(&format!("[{}]", listed.rsplit_once('[')?.1))
    else {
        return None;
    };
    if clean_path(argument.split(')').next()?.trim().strip_prefix("ctx.")?) != key {
        return None;
    }

    Some(RenameMemberByLookup {
        source: format!("{container}.{member}"),
        container,
        key,
        listed: listed
            .iter()
            .filter_map(|member| member.as_str().map(str::to_owned))
            .collect(),
        prefix,
    })
}

/// The bare name a write's value names, or empty text for anything else.
fn assigned_local(value: &str) -> &str {
    value.split(';').next().map_or("", str::trim)
}

/// The `ctx` path a trailing subscript is applied TO, read BACKWARDS from it.
///
/// `head` ends where the subscript's own `[` begins, so the answer is whatever
/// quoted subscripts and dotted segments precede it, back to the `ctx` root.
/// Reading FORWARDS from the last `ctx.` cannot answer this: in
/// `ctx["a"]["b"][params.get(ctx.c.d)]` that `ctx.` is the KEY expression, and
/// the forward read comes back with `c.d` where the destination is `a.b`.
///
/// A segment the text does not spell as a quoted literal is a key the EVENT
/// decides, so there is no path to return and this declines.
fn subscripted_subject(head: &str) -> Option<String> {
    let mut segments = Vec::new();
    let mut rest = head.trim_end();
    while let Some(inside) = rest.strip_suffix(']') {
        let (before, segment) = inside.rsplit_once('[')?;
        segments.push(whole_literal(segment)?);
        rest = before.trim_end();
    }
    // Whatever is left is the root, `ctx` itself or `ctx.<path>`.
    let root = rest
        .rsplit(|c: char| c.is_whitespace() || "({;".contains(c))
        .next()?
        .strip_prefix("ctx")?;
    if !root.is_empty() {
        segments.push(clean_path(
            root.strip_prefix('.').or_else(|| root.strip_prefix("?."))?,
        ));
    }
    segments.reverse();
    let path = segments.join(".");
    is_ctx_path(&path).then_some(path)
}

/// `text` as the ONE quoted literal it is, or `None` for anything else.
///
/// Distinct from [`literal_at`], which reads the literal a fragment OPENS with
/// and ignores what trails it: `"data_" + ctx.a.b` opens with a literal and is
/// not one, and reading it as `data_` would name a member the vendor computes.
fn whole_literal(text: &str) -> Option<String> {
    let text = text.trim();
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let inner = text.strip_prefix(quote)?.strip_suffix(quote)?;
    (!inner.contains(quote)).then(|| inner.to_owned())
}

/// Move the member under the name the table gives its key, then prune it.
///
/// A key the table misses takes the script's own `else` arm, and a key the
/// script's list DOES carry but the table does not writes nothing at all:
/// Painless would key the map by the lookup's null, which names no field the
/// document can hold. The prune runs either way, which is what the script does.
fn run_rename_member_by_lookup(
    event: &mut Event,
    pattern: &RenameMemberByLookup,
    params: &Map<String, Value>,
) -> bool {
    let value = event.get(&pattern.source).cloned();
    if let Some(value) = value
        && let Some(key) = event.get_as_string(&pattern.key)
    {
        let named = if pattern.listed.contains(&key) {
            params.get(&key).and_then(Value::as_str).map(str::to_owned)
        } else {
            Some(format!("{}{key}", pattern.prefix))
        };
        if let Some(named) = named {
            let _ = event.set(&format!("{}.{named}", pattern.container), value);
        }
    }
    event.remove(&pattern.source);
    true
}

/// Where a lookup written to a container's NAMED MEMBER reads and writes.
///
/// Distinct from [`LookupNormaliseScript`] on the two points that decide the
/// output: the container guarantee the script opens with (`ctx.event =
/// ctx.event ?: [:]`, or the `new HashMap()` spelling) is not a write, and the
/// member the row lands on is often ECS's array-typed `event.type` or
/// `event.category`, so the row is wrapped in a one-element list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MemberLookupScript {
    /// The `ctx.` path the lookup key comes from.
    key: String,
    fold: Fold,
    /// The member written, container and leaf joined into one dotted path.
    target: String,
    /// The write wraps the row in a one-element list.
    wrap: bool,
    miss: LookupMiss,
}

/// What a script does where the params table holds no row for its key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LookupMiss {
    /// `ctx.<container>.remove('<leaf>')` -- the member goes.
    Remove,
    /// The write sits behind a null guard the miss fails, so the member keeps
    /// whatever it already held.
    Keep,
    /// Nothing guards the write, so Painless stores the lookup's own null.
    Null,
}

/// Read the key, the member the row is written to, the list wrap and the miss
/// behaviour out of the script once.
///
/// `None` where the script is not this pattern after all, which leaves the text
/// to `LookupNormalise` below -- deliberately narrow, because that matcher gets
/// every direct `ctx.<path> = params.get(...)` right and only the member
/// spellings wrong.
fn parse_member_lookup(script: &str) -> Option<MemberLookupScript> {
    // ONE lookup, spelled with `get`, and no `containsKey` guard: a second read
    // or either of the other spellings means the script says more than this
    // pattern can, and claiming it would drop the rest.
    if script.matches("params.get(").count() != 1
        || script.contains("params[")
        || script.contains("params.containsKey(")
    {
        return None;
    }

    let lookup_at = script.find("params.get(")?;
    let statement = enclosing_statement(script, lookup_at);
    let (local, wrapped_at_binding) = lookup_binding(statement)?;

    // The one write that carries the lookup's value, and the list wrap where
    // the write rather than the binding spells it.
    let assignments = ctx_writes(script);
    let puts = ctx_put_writes(script);
    let mut carriers = assignments
        .iter()
        .chain(puts.iter())
        .filter(|(_, rhs)| local.as_deref() == Some(unwrapped_value(rhs).0));
    let (target, value) = match &local {
        Some(_) => carriers.next()?,
        // The lookup reaches the member with no local in between, so the write
        // is the statement it sits in.
        None => puts.iter().find(|(_, rhs)| rhs.contains("params.get("))?,
    };
    let wrap = wrapped_at_binding || unwrapped_value(value).1;

    // Everything else the script writes has to be a container guarantee, or
    // claiming it would silently drop a write -- qualys_gav puts a literal
    // `linux` on the same member from the other arm of a branch.
    let accounted = |path: &String, rhs: &String| {
        std::ptr::eq(path, target) || is_container_guarantee(path, rhs)
    };
    if !assignments
        .iter()
        .chain(puts.iter())
        .all(|(path, rhs)| accounted(path, rhs))
    {
        return None;
    }

    let (key, fold) = lookup_key_path(script, &last_call_argument(script, "params.get(")?)?;
    Some(MemberLookupScript {
        key,
        fold,
        target: target.clone(),
        wrap,
        miss: local.map_or(LookupMiss::Null, |local| {
            lookup_miss(script, &local, target)
        }),
    })
}

/// The statement `at` sits in, bounded by the semicolons either side of it.
fn enclosing_statement(script: &str, at: usize) -> &str {
    let start = script[..at].rfind(';').map_or(0, |semi| semi + 1);
    let end = script[at..]
        .find(';')
        .map_or(script.len(), |offset| at + offset);
    &script[start..end]
}

/// The local a lookup statement binds its row to, and whether that binding put
/// the row in a list.
///
/// `None` for a statement that binds nothing -- the `.put(` spelling reads the
/// table inline, and a direct `ctx.<path> = params.get(...)` is
/// `LookupNormalise`'s to claim.
fn lookup_binding(statement: &str) -> Option<(Option<String>, bool)> {
    // `def type = new ArrayList(); type.add(params.get(...));` -- data_studio.
    if let Some((head, _)) = statement.split_once(".add(params.get(") {
        let local = identifier_ending(head)?;
        return Some((Some(local), true));
    }
    // `def type = params.get(...);` -- the guarded spellings.
    if let Some((head, _)) = statement.split_once("= params.get(")
        && let Some(local) = identifier_ending(head)
    {
        return Some((Some(local), false));
    }
    // The lookup is the `.put` value itself, with nothing bound.
    statement.contains(".put(").then_some((None, false))
}

/// The bare identifier `head` ends with, or `None` where it ends with anything
/// else -- a dotted `ctx.` path, a subscript, a call.
pub(crate) fn identifier_ending(head: &str) -> Option<String> {
    let head = head.trim_end();
    let start = head
        .char_indices()
        .rev()
        .find(|(_, c)| !(c.is_alphanumeric() || *c == '_'))
        .map_or(0, |(at, c)| at + c.len_utf8());
    // A name reached through a dot, a subscript or a call is the tail of a
    // PATH, not a local: reading one as a local turns
    // `ctx.event.severity = params.get(...)` into a binding named `severity`.
    if head[..start].ends_with(['.', ']', ')']) {
        return None;
    }
    let name = &head[start..];
    (!name.is_empty() && !name.starts_with(|c: char| c.is_ascii_digit())).then(|| name.to_string())
}

/// A write's value with a one-element list unwrapped, and whether it was one.
fn unwrapped_value(value: &str) -> (&str, bool) {
    let value = value.trim();
    value
        .strip_prefix('[')
        .and_then(|inner| inner.strip_suffix(']'))
        .map_or((value, false), |inner| (inner.trim(), true))
}

/// Every `ctx.<container>.put(<leaf>, <value>)` in the script, the member as one
/// dotted path.
///
/// [`ctx_writes`] cannot see these: a `put` carries no `=`, so a script whose
/// only write is one reads to that helper as writing nothing at all.
fn ctx_put_writes(script: &str) -> Vec<(String, String)> {
    let mut writes = Vec::new();
    for (at, _) in script.match_indices(".put(") {
        let Some(container) = ctx_path_ending(&script[..at]) else {
            continue;
        };
        // Balanced, because the value is often a call of its own.
        let Some(arguments) = delimited_from(script, at + ".put".len()) else {
            continue;
        };
        let arguments = arguments.trim_start();
        let Some(quote) = arguments.chars().next().filter(|c| *c == '\'' || *c == '"') else {
            continue;
        };
        let Some((leaf, rest)) = arguments[quote.len_utf8()..].split_once(quote) else {
            continue;
        };
        let Some(value) = rest.trim_start().strip_prefix(',') else {
            continue;
        };
        writes.push((format!("{container}.{leaf}"), value.trim().to_string()));
    }
    writes
}

/// The `ctx.` path `head` ends with, validated as a plain dotted path.
fn ctx_path_ending(head: &str) -> Option<String> {
    let at = head.rfind("ctx.")?;
    let path = clean_path(&head[at + "ctx.".len()..]);
    (!path.is_empty()
        && path
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '_'))
    .then_some(path)
}

/// What the parenthesis at `open` encloses, up to its matching close.
fn delimited_from(script: &str, open: usize) -> Option<&str> {
    let mut depth = 0usize;
    for (offset, c) in script[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&script[open + 1..open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

/// Whether a write makes a container EXIST rather than putting a value in it.
///
/// The three spellings the integrations use, plus the `?:` guarantee of the
/// same path. Reading one of these as the script's write is what put a bare
/// string at `event`.
fn is_container_guarantee(path: &str, rhs: &str) -> bool {
    const EMPTY: [&str; 3] = ["[:]", "new HashMap()", "new ArrayList()"];
    let rhs = rhs.trim();
    EMPTY.contains(&rhs)
        || rhs
            .strip_prefix(&format!("ctx.{path}"))
            .and_then(|rest| rest.trim_start().strip_prefix("?:"))
            .is_some_and(|rest| EMPTY.contains(&rest.trim()))
}

/// What the script does where the table has no row, read off the guard it puts
/// around the write.
fn lookup_miss(script: &str, local: &str, target: &str) -> LookupMiss {
    if script.contains(&format!("{local} == null")) {
        // `if (<local> == null) { ctx.<container>.remove('<leaf>'); }` --
        // google_workspace drops the member rather than leaving a stale one.
        if parse_removes(script).iter().any(|path| path == target) {
            return LookupMiss::Remove;
        }
        // The other spelling returns, so the member is left as it was.
        return LookupMiss::Keep;
    }
    if script.contains(&format!("{local} != null")) {
        return LookupMiss::Keep;
    }
    LookupMiss::Null
}

/// Write the table's row to the member, in the form the script's own write
/// spells and with the miss behaviour it declares.
fn member_lookup(
    event: &mut Event,
    pattern: &MemberLookupScript,
    params: &Map<String, Value>,
) -> bool {
    // The key field absent is where Painless would throw on the dereference and
    // the processor's `on_failure` would run, so writing nothing is right.
    let Some(raw) = event.get_as_string(&pattern.key) else {
        return true;
    };
    let wrap = |value: Value| {
        if pattern.wrap {
            Value::Array(vec![value])
        } else {
            value
        }
    };

    match params.get(&pattern.fold.apply(&raw)) {
        Some(row) => {
            let _ = event.set(&pattern.target, wrap(row.clone()));
        }
        None => match pattern.miss {
            LookupMiss::Remove => {
                event.remove(&pattern.target);
            }
            LookupMiss::Keep => {}
            LookupMiss::Null => {
                let _ = event.set(&pattern.target, wrap(Value::Null));
            }
        },
    }
    true
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

/// Every `<declaration> <local> = ctx.<path>[.toLowerCase()];` binding, in
/// script order, with whether each was folded DOWN rather than up.
///
/// The script offers its sources as an `if / else if` chain, so all of them are
/// read and the caller takes the first the event actually carries.
///
/// The declaration keyword is read from [`DECLARATIONS`] rather than assumed to
/// be `String`: `island_browser` spells the same subject `def`, and a reader
/// keyed on one keyword finds no subject at all and declines the whole script.
/// A `for (String x: ...)` loop header declares a loop variable, not a binding,
/// and is rejected because its left side is no identifier.
fn subjects_of_bindings(script: &str) -> Vec<(String, bool)> {
    let mut subjects = Vec::new();
    let mut at = 0;
    while let Some(declaration) = next_declaration(script, at) {
        at = declaration;
        let Some((name, bound)) = script[declaration..]
            .split(';')
            .next()
            .and_then(|statement| statement.split_once('='))
        else {
            continue;
        };
        if identifier(name).is_none() {
            continue;
        }
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
///
/// Both halves come off the SAME call, which `akamai_siem` needs: it opens with a
/// `.put(<local>, ...)` whose key is no literal at all.
fn put_target(script: &str) -> Option<String> {
    let (at, key) = literal_call(script, ".put(")?;
    let path = ctx_path_at_end(&script[..at])?;
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
///
/// Every script shipping the helper spells the same recursion and differs only
/// in what it does with the result, so the whole reading happens ONCE per call
/// site and the runner reads this rather than the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RenameKeys {
    /// Where the name table comes from.
    names: NameTable,
    /// The retyping lists a five-argument call carries.
    coerce: Option<Coercions>,
    /// A key the helper renames on top of the table, in its SCALAR branch only.
    rename_key: Option<(String, String)>,
    write: RenameWrite,
}

/// The block a call reads its name table out of.
#[derive(Debug, Clone, PartialEq, Eq)]
enum NameTable {
    /// The whole `params` block, which the two-argument call passes bare.
    Params,
    /// One named member of it.
    Member(String),
}

/// The params members naming the fields the vendor's own converters retype.
///
/// claroty's asset feed is the only spelling. The lists are keyed by the
/// RENAMED name, and a key the table does not name is never retyped -- the
/// retyping arms sit inside the helper's `containsKey` branch.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Coercions {
    longs: String,
    strings: String,
    bools: String,
}

/// Where the renamed value goes.
#[derive(Debug, Clone, PartialEq, Eq)]
enum RenameWrite {
    /// `ctx.<path> = renameKeys(ctx.<path>, params)` -- the map in place.
    InPlace(String),
    /// One renamed element per member of a ctx list. `google_scc` alone ships
    /// sixteen of these.
    FanOut(Box<FanOut>),
    /// A spelling no reader here places. Writing nothing is correct; guessing
    /// at a target is a corruption that leaves no error behind.
    Unreadable,
}

/// The list fan-out: read a ctx list, rename each member, store the result.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FanOut {
    /// The ctx list each element is read from.
    source: String,
    /// The ctx path the renamed list is written to.
    target: String,
    /// A `ctx.<path>.remove('<key>')` the script makes first. It may name a
    /// field the write does not -- two of `google_scc`'s drop
    /// `json.asset.orgPolicy` and write `google_scc.asset.organization_policy`.
    drop: Option<String>,
}

impl RenameKeys {
    /// Read the whole rename off the script text.
    ///
    /// Never declines, because the ladder arm above claims every script that
    /// spells the helper: a text no reader places comes back as
    /// [`RenameWrite::Unreadable`] rather than falling through to a later arm.
    fn parse(script: &str) -> Self {
        Self::read(script).unwrap_or(Self {
            names: NameTable::Params,
            coerce: None,
            rename_key: None,
            write: RenameWrite::Unreadable,
        })
    }

    fn read(script: &str) -> Option<Self> {
        let (write, arguments) = match rename_fan_out(script) {
            Some((fan, arguments)) => (RenameWrite::FanOut(Box::new(fan)), arguments),
            None => rename_in_place(script)?,
        };
        let (names, coerce) = rename_name_table(&arguments)?;
        Some(Self {
            names,
            coerce,
            rename_key: rename_key_guard(script),
            write,
        })
    }
}

/// `ctx.<path> = renameKeys(ctx.<path>, params)`, and the call's arguments.
fn rename_in_place(script: &str) -> Option<(RenameWrite, Vec<String>)> {
    // The helper's own `dst[keyMap[key]] = renameKeys(value, keyMap)` is not a
    // ctx write, so the only statement left is the one the script ends on.
    let (path, rhs) = ctx_writes(script)
        .into_iter()
        .find(|(_, rhs)| rhs.starts_with("renameKeys("))?;
    let (inside, _) = balanced(&rhs["renameKeys".len()..], '(', ')')?;
    Some((RenameWrite::InPlace(path), call_arguments(inside)))
}

/// The list fan-out: the loop, the call inside it, and where the built list
/// goes.
///
/// The LOOP is read first and the call found by its element variable. The
/// helper's body calls itself, so matching `renameKeys(` alone answers with the
/// recursion rather than with the one call the loop makes.
fn rename_fan_out(script: &str) -> Option<(FanOut, Vec<String>)> {
    let (element, source) = rename_loop(script)?;
    let (at, arguments) = rename_call(script, &element)?;
    let target = rename_fan_out_target(script, at)?;
    Some((
        FanOut {
            source,
            target,
            drop: rename_dropped_field(script),
        },
        arguments,
    ))
}

/// A `for (<element> in ctx.<list>)` header, as its element name and list path.
///
/// Both spacings ship: `google_scc` writes `for(entity in ...` and claroty
/// `for (child in ...`.
fn rename_loop(script: &str) -> Option<(String, String)> {
    let mut at = 0;
    while let Some(found) = script[at..].find("for") {
        at += found + "for".len();
        let Some((inside, _)) = balanced(script[at..].trim_start(), '(', ')') else {
            continue;
        };
        // The helper's own walks are `for (def entry: src.entrySet())`, so the
        // `in` keyword is what separates the fan-out from the recursion.
        let Some((element, list)) = inside.split_once(" in ") else {
            continue;
        };
        if let Some(element) = identifier(element)
            && let Some(list) = ctx_path_term(list)
            && is_ctx_path(&list)
        {
            return Some((element.to_owned(), list));
        }
    }
    None
}

/// The `renameKeys(<first>, ...)` call taking `first`, as its offset and its
/// arguments.
///
/// A call whose first argument is anything else is the helper's own definition
/// or one of its recursions, both of which take the helper's parameter names.
fn rename_call(script: &str, first: &str) -> Option<(usize, Vec<String>)> {
    let mut at = 0;
    while let Some(found) = script[at..].find("renameKeys") {
        let start = at + found;
        at = start + "renameKeys".len();
        let Some((inside, _)) = balanced(&script[at..], '(', ')') else {
            continue;
        };
        let arguments = call_arguments(inside);
        if arguments
            .first()
            .is_some_and(|argument| argument.as_str() == first)
        {
            return Some((start, arguments));
        }
    }
    None
}

/// One call's arguments, trimmed. Each is a bare name or a `params.<member>`,
/// so a comma split is the whole parse.
fn call_arguments(inside: &str) -> Vec<String> {
    inside
        .split(',')
        .map(|argument| argument.trim().to_owned())
        .collect()
}

/// The ctx path the loop's own `.add(` builds into.
///
/// Two spellings: the renamed element is appended to a LOCAL the script then
/// stores, or straight onto a ctx path the script pre-created.
fn rename_fan_out_target(script: &str, call: usize) -> Option<String> {
    let receiver = rename_add_receiver(script, call)?;
    if let Some(path) = ctx_path_term(&receiver) {
        return is_ctx_path(&path).then_some(path);
    }
    let local = identifier(&receiver)?;
    // The list is stored either by assignment or by a `.put(` onto a container.
    ctx_writes(script)
        .into_iter()
        .find(|(_, rhs)| rhs.as_str() == local)
        .map(|(path, _)| path)
        .or_else(|| rename_put_target(script, local))
}

/// What the loop appends the renamed element to.
fn rename_add_receiver(script: &str, call: usize) -> Option<String> {
    // `<receiver>.add(renameKeys(<element>, ...))` -- the receiver opens the
    // same statement the call sits in.
    let statement = script[..call].rsplit([';', '{', '\n']).next()?.trim();
    if let Some(head) = statement.strip_suffix(".add(") {
        return Some(head.trim().to_owned());
    }
    // `def <local> = renameKeys(...); <receiver>.add(<local>);`
    let local = identifier_ending(statement.strip_suffix('=')?)?;
    let add = format!(".add({local})");
    let head = &script[..script.find(&add)?];
    Some(head.rsplit([';', '{', '\n']).next()?.trim().to_owned())
}

/// The field a `ctx.<path>.put('<key>', <local>)` writes.
fn rename_put_target(script: &str, local: &str) -> Option<String> {
    let mut at = 0;
    while let Some(found) = script[at..].find(".put(") {
        let start = at + found;
        at = start + ".put(".len();
        let Some((inside, _)) = balanced(&script[start + ".put".len()..], '(', ')') else {
            continue;
        };
        let arguments = call_arguments(inside);
        let [key, value] = arguments.as_slice() else {
            continue;
        };
        if value.as_str() != local {
            continue;
        }
        let Some(key) = literal_at(key) else {
            continue;
        };
        let Some(path) = ctx_path_at_end(&script[..start]) else {
            continue;
        };
        let field = format!("{path}.{key}");
        if is_ctx_path(&field) {
            return Some(field);
        }
    }
    None
}

/// A `ctx.<path>.remove('<key>')` the script makes, as the field it takes out.
///
/// Anchored on the ctx path, because the helper spells the same call on its own
/// local map -- `updatedJson.remove('location')` names no ctx field at all.
fn rename_dropped_field(script: &str) -> Option<String> {
    let mut at = 0;
    while let Some(found) = script[at..].find(".remove(") {
        let start = at + found;
        at = start + ".remove(".len();
        let Some(key) = literal_at(&script[at..]) else {
            continue;
        };
        let Some(path) = ctx_path_at_end(&script[..start]) else {
            continue;
        };
        let field = format!("{path}.{key}");
        if is_ctx_path(&field) {
            return Some(field);
        }
    }
    None
}

/// The params member holding the name table, and the retyping lists a
/// five-argument call adds.
///
/// The two-argument call passes the whole `params` block as the table; the
/// five-argument one names four members. Any other arity is a helper this
/// cannot run.
fn rename_name_table(arguments: &[String]) -> Option<(NameTable, Option<Coercions>)> {
    match arguments {
        [_, table] => Some((params_member(table)?, None)),
        [_, table, longs, strings, bools] => Some((
            params_member(table)?,
            Some(Coercions {
                longs: named_params_member(longs)?,
                strings: named_params_member(strings)?,
                bools: named_params_member(bools)?,
            }),
        )),
        _ => None,
    }
}

/// The block an argument names, whole or by member.
fn params_member(argument: &str) -> Option<NameTable> {
    if argument == "params" {
        return Some(NameTable::Params);
    }
    named_params_member(argument).map(NameTable::Member)
}

/// The params member `params.<name>` names.
fn named_params_member(argument: &str) -> Option<String> {
    identifier(argument.strip_prefix("params.")?).map(str::to_owned)
}

/// The helper's own `if (key == '<from>') { <dst>['<to>'] = value; }`, which
/// renames one key on top of the table.
///
/// `microsoft_defender_cloud` and `ti_threatconnect` both move a scalar
/// `location` to `location_value` that way. The guard sits inside the
/// recursion, so it applies at every depth, and inside the SCALAR branch, so a
/// `location` object keeps its own name.
fn rename_key_guard(script: &str) -> Option<(String, String)> {
    let guard = key_equality_guard(script)?;
    let (from, rest) = next_quoted(guard)?;
    // The one write the guard opens, and the removal that takes the original
    // key back out. entityanalytics guards the same way to rewrite a VALUE, and
    // declines here on both counts.
    let (to, rest) = next_quoted(rest.trim_start().strip_prefix(") {")?)?;
    rest.trim_start().strip_prefix("] = value;")?;
    script
        .contains(&format!(".remove('{from}')"))
        .then_some((from, to))
}

/// The text after an `if (key` that tests the key itself.
///
/// `if (keyMap.containsKey(key))` opens with the same characters and is the
/// helper's table lookup, so the character after the word decides.
fn key_equality_guard(script: &str) -> Option<&str> {
    let mut at = 0;
    while let Some(found) = script[at..].find("if (key") {
        at += found + "if (key".len();
        let rest = &script[at..];
        if !rest.starts_with(|c: char| c.is_alphanumeric() || c == '_') {
            return Some(rest);
        }
    }
    None
}

/// The next quoted literal in `text`, and what follows its closing quote.
fn next_quoted(text: &str) -> Option<(String, &str)> {
    let start = text.find(['\'', '"'])?;
    let quote = text[start..].chars().next()?;
    let rest = &text[start + quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some((rest[..end].to_owned(), &rest[end + quote.len_utf8()..]))
}

/// Rename an object's keys through the params table, in place or over a list.
fn run_rename_keys(event: &mut Event, pattern: &RenameKeys, params: &Map<String, Value>) -> bool {
    let names = match &pattern.names {
        NameTable::Params => params,
        // A table the block does not carry is a script that cannot run at all.
        NameTable::Member(member) => match params.get(member).and_then(Value::as_object) {
            Some(table) => table,
            None => return false,
        },
    };
    // A retyping list the block does not carry names no field, so the arm it
    // guards never fires and the value stands as it is.
    let (longs, strings, bools) = match &pattern.coerce {
        Some(coerce) => (
            params_list(params, &coerce.longs),
            params_list(params, &coerce.strings),
            params_list(params, &coerce.bools),
        ),
        None => (&[][..], &[][..], &[][..]),
    };
    let renamer = Renamer {
        names,
        longs,
        strings,
        bools,
        rename_key: pattern
            .rename_key
            .as_ref()
            .map(|(from, to)| (from.as_str(), to.as_str())),
    };

    match &pattern.write {
        RenameWrite::InPlace(path) => {
            let Some(subject) = event.get(path).cloned() else {
                // The processor's own `if` guards the object being absent.
                return false;
            };
            let _ = event.set(path, renamer.apply(&subject));
            true
        }
        RenameWrite::FanOut(fan) => run_rename_fan_out(event, fan, &renamer),
        RenameWrite::Unreadable => false,
    }
}

/// The named params member as a list, empty where the block does not carry it.
fn params_list<'a>(params: &'a Map<String, Value>, member: &str) -> &'a [Value] {
    params
        .get(member)
        .and_then(Value::as_array)
        .map_or(&[], Vec::as_slice)
}

/// One renamed element per member of the source list, stored where the script
/// puts it.
///
/// The removal is honoured before the write: `remove` then `put` on the SAME
/// key re-appends it at the END, and `preserve_order` is what parity rests on.
fn run_rename_fan_out(event: &mut Event, fan: &FanOut, renamer: &Renamer<'_>) -> bool {
    // Built off a BORROW of the source rather than a clone of it: every member
    // is rebuilt anyway, so copying the list first buys nothing.
    let list = {
        let Some(Value::Array(members)) = event.get(&fan.source) else {
            // The processor's own `if` guards the list being absent.
            return false;
        };
        let mut list = Vec::with_capacity(members.len());
        list.extend(members.iter().map(|member| renamer.apply(member)));
        list
    };

    if let Some(field) = &fan.drop {
        event.remove(field);
    }
    let _ = event.set(&fan.target, Value::Array(list));
    true
}

/// The rename as the vendor helper performs it: the name table, the retyping
/// lists, and the one extra key rename two of the scripts add.
struct Renamer<'a> {
    names: &'a Map<String, Value>,
    longs: &'a [Value],
    strings: &'a [Value],
    bools: &'a [Value],
    rename_key: Option<(&'a str, &'a str)>,
}

impl Renamer<'_> {
    /// `value` with every key the table names replaced, at any depth.
    fn apply(&self, value: &Value) -> Value {
        match value {
            Value::Object(members) => {
                let mut renamed = Map::with_capacity(members.len());
                for (key, member) in members {
                    let table = self.names.get(key).and_then(Value::as_str);
                    let name = table.unwrap_or(key);
                    if member.is_object() || member.is_array() {
                        renamed.insert(name.to_owned(), self.apply(member));
                        continue;
                    }
                    // Retyping is inside the helper's `containsKey` branch, so a
                    // key the table does not name keeps its value as it stands.
                    let scalar = match table {
                        Some(_) => self.retyped(name, member),
                        None => member.clone(),
                    };
                    renamed.insert(name.to_owned(), scalar);

                    if let Some((from, to)) = self.rename_key
                        && key == from
                    {
                        renamed.insert(to.to_owned(), member.clone());
                        // `shift_remove`, never `remove`: under `preserve_order`
                        // the plain one drops the LAST key into the freed slot.
                        renamed.shift_remove(from);
                    }
                }
                Value::Object(renamed)
            }
            Value::Array(members) => {
                Value::Array(members.iter().map(|member| self.apply(member)).collect())
            }
            other => other.clone(),
        }
    }

    /// The scalar retyped the way the vendor's own converters do, keyed by the
    /// RENAMED name.
    ///
    /// A conversion Painless would throw on leaves the value as it stands: the
    /// throw belongs to the processor's `on_failure`, which this cannot reach.
    fn retyped(&self, name: &str, value: &Value) -> Value {
        if names_field(self.longs, name) {
            return crate::coercion::as_long(value).map_or_else(|| value.clone(), Value::from);
        }
        // The vendor guards the stringify on the value being present, so a null
        // falls through to the boolean arm and then to the value itself.
        if names_field(self.strings, name) && !value.is_null() {
            return scalar_text(value).map_or_else(|| value.clone(), Value::String);
        }
        if names_field(self.bools, name) {
            return as_boolean(value).unwrap_or_else(|| value.clone());
        }
        value.clone()
    }
}

/// Whether a params list names this field.
fn names_field(list: &[Value], name: &str) -> bool {
    list.iter().any(|entry| entry.as_str() == Some(name))
}

/// The boolean a `Boolean.parseBoolean` yields, or `None` where Painless would
/// have thrown.
///
/// `parseBoolean` is true for the word `true` in any case and false for
/// everything else a String can hold, so it never fails on text.
fn as_boolean(value: &Value) -> Option<Value> {
    match value {
        Value::String(text) => Some(Value::Bool(text.eq_ignore_ascii_case("true"))),
        Value::Bool(_) => Some(value.clone()),
        _ => None,
    }
}

/// A map rebuilt from ONLY the keys the params table names, each key renamed
/// to the table's value.
///
/// `cisco_asa` splits a validated certificate's distinguished name into its
/// `CN`, `OU`, `O` and `C` abbreviations, folds those onto their ECS names,
/// and only then renames the map to `tls.server.x509.subject`. A key the table
/// does not name is DROPPED, which is how the vendor keeps its own
/// abbreviations out of ECS -- the near neighbour [`ParamsPattern::RenameKeys`]
/// KEEPS them, so the two are not interchangeable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SelectRenameKeys {
    /// The `ctx.` map read and written back in place.
    subject: String,
    /// The script wraps a value that is not already a list in a one-element
    /// list, which is what ECS's array-typed x509 subject fields want.
    wrap_in_list: bool,
}

/// Read the select-and-rename fold: the map walked, and whether each value is
/// wrapped in a list on the way.
///
/// The spelling, from `pipelines/cisco/asa/default.yml`:
///
/// ```text
/// def <out> = [:];
/// ctx.<subject>.forEach((k,v) -> {
///   if (params.containsKey(k)) {
///     <out>[params[k]] = (v instanceof List) ? v : [v];
///   } else {
///     return false;
///   }
/// });
/// ctx.<subject> = <out>;
/// ```
///
/// The stored expression is READ rather than assumed: `v` and
/// `(v instanceof List) ? v : [v]` store different things, and a third
/// expression is a behaviour this cannot say, so it declines.
fn parse_select_rename_keys(script: &str) -> Option<SelectRenameKeys> {
    // The local map the fold builds, and the map it walks.
    let declaration = script.find("= [:];")?;
    let out = script[..declaration].split_whitespace().next_back()?;
    let subject = ctx_path_before(script, ".forEach((")?;

    // Written back over the map it walked. A fold storing its result anywhere
    // else does more than this pattern.
    if !ctx_writes(script)
        .iter()
        .any(|(path, rhs)| path == &subject && rhs == out)
    {
        return None;
    }

    // The lambda's key and value names, and nothing but those two.
    let (names, body) = script.split_once(".forEach((")?.1.split_once("->")?;
    let mut names = names.trim().trim_end_matches(')').split(',');
    let (Some(key), Some(value), None) = (names.next(), names.next(), names.next()) else {
        return None;
    };
    let (key, value) = (key.trim(), value.trim());
    if key.is_empty() || value.is_empty() {
        return None;
    }

    // The table guard, and the one write it guards.
    if !body.contains(&format!("params.containsKey({key})")) {
        return None;
    }
    let write = format!("{out}[params[{key}]] = ");
    let at = body.find(&write)? + write.len();
    let stored = body[at..].split_once(';')?.0.trim();

    let wrap_in_list = stored == format!("({value} instanceof List) ? {value} : [{value}]");
    (wrap_in_list || stored == value).then_some(SelectRenameKeys {
        subject,
        wrap_in_list,
    })
}

/// Rebuild the map from the keys the table names, in the map's OWN order.
///
/// A key the table does not name is dropped -- that is the script's `else {
/// return false; }` arm, which adds nothing for it. An empty result is still
/// stored, because the script assigns unconditionally once the fold has run.
fn run_select_rename_keys(
    event: &mut Event,
    pattern: &SelectRenameKeys,
    params: &Map<String, Value>,
) -> bool {
    let Some(subject) = event
        .get(&pattern.subject)
        .and_then(Value::as_object)
        .cloned()
    else {
        // The script's own `== null` guard returns before the fold, so nothing
        // was written and nothing is counted as having run.
        return false;
    };

    let mut renamed = Map::with_capacity(subject.len());
    for (key, value) in &subject {
        let Some(name) = params.get(key).and_then(Value::as_str) else {
            continue;
        };
        let value = if pattern.wrap_in_list && !value.is_array() {
            Value::Array(vec![value.clone()])
        } else {
            value.clone()
        };
        renamed.insert(name.to_owned(), value);
    }
    let _ = event.set(&pattern.subject, Value::Object(renamed));
    true
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
    /// `ctx.<path>.addAll(ctx.<source>)` -- EVERY member of one list appended
    /// to another, which is a different write from [`Literal::Append`]: that
    /// one adds the source as a single member, and nesting a list inside the
    /// target is not what the vendor wrote. teleport folds a certificate's
    /// logins, participants and database users into `related.user` this way.
    AppendAll {
        path: String,
        source: String,
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
    /// `Long.decode(ctx.<path>)` -- Java's radix-sensitive read of a numeric
    /// string, which is why the vendor spells it this way rather than
    /// `parseLong`: Windows writes a pid as `0x1f4` and a port as `3389`, and
    /// only `decode` reads both.
    Decoded(String),
    /// `'<literal>' + ctx.<path> + ...` -- a string spliced together from
    /// literal text and the event's own fields, in the order the script writes
    /// them. `first_epss` builds a CVE lookup URL this way, wiz an issue URL,
    /// `jamf_protect` an English sentence and tychon a CPE name. Overlaps
    /// `KnownPattern::ConcatAssignment` -- see [`parse_concat`].
    ///
    /// The fold is the case call on the JOIN, which only a parenthesised
    /// expression can carry -- `digital_guardian`'s
    /// `(ctx.<a> + "-" + ctx.<b>).toLowerCase()`. Unparenthesised, the call
    /// binds to the last term alone and is a different expression.
    Concat {
        pieces: Vec<ConcatPiece>,
        fold: Fold,
    },
    /// `ctx.<path>.splitOnToken("<sep>")[<n>]` -- one part of a split,
    /// optionally case-folded. `cisco_ise` takes the word before the first colon
    /// of a message description as `event.action` at 24 sites, and okta's admin
    /// URL gives up its tail after an API prefix at four more.
    SplitPart {
        path: String,
        separator: String,
        index: usize,
        fold: Fold,
    },
    /// `ctx.<path>.splitOnToken("<sep>")[-1]` -- the LAST part, which is how a
    /// script takes the leaf of a path. Windows reads `process.name` and
    /// `process.parent.name` off an executable path this way.
    ///
    /// Apart from [`Rhs::SplitPart`] because that index is a `usize` and
    /// Painless counts `-1` back from the end. `-1` is the only negative
    /// subscript the vendor tree spells, so it is the only one read.
    SplitLast {
        path: String,
        separator: String,
        fold: Fold,
    },
    Literal(Value),
}

/// One term of a [`Rhs::Concat`].
///
/// [`crate::common`] holds the same two cases for cloudfront's `+=`
/// assembly. They are not one type because that one is a run of `+=`
/// statements each guarded by its own `if`, and this is a single expression.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ConcatPiece {
    Literal(String),
    /// Read off the event and rendered as text. A field the event does not
    /// carry declines the whole write -- see [`resolve_rhs`].
    Field(String),
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

/// Substitute a local bound to a `splitOnToken` call, where every use of it is
/// a subscript.
///
/// ```painless
/// def parts = ctx.process.executable.splitOnToken("\\");
/// ctx.process.put("name", parts[-1]);
/// ```
///
/// Windows takes the leaf of a path this way, for `process.name` and
/// `process.parent.name`, across its security streams. Until the local is
/// followed, `parts[-1]` names something no event carries and the write is
/// dropped -- while the `remove` after it is read, so the field the script was
/// pruning went and the field it was setting never arrived.
///
/// **Every use has to be a subscript, and that is a correctness condition
/// rather than caution.** A split local's other common use is `parts.length` in
/// a guard. Inlining there moves an expression no reader resolves INTO a guard,
/// where the branch is then decided by an unreadable comparison's default
/// polarity instead of by the event -- `aws_bedrock_agentcore` writes both its
/// names under `if (parts.length == 2)`, and running those writes unguarded is
/// a wrong value where today there is a missing one. Leaving that local alone
/// keeps such a script exactly as unreadable as it already is.
///
/// The split is a pure function of the event, so evaluating it once per use
/// rather than once per script is the same answer.
fn inline_split_locals(body: &str) -> Cow<'_, str> {
    if !body.contains(".splitOnToken(") {
        return Cow::Borrowed(body);
    }

    let mut out = body.to_string();
    let mut changed = false;
    let mut at = 0;

    while let Some(declaration) = next_declaration(&out, at) {
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
        if !is_split_call(&value)
            || !assigned_once(&out, &name)
            || !every_use_subscripts(&out, &name)
        {
            continue;
        }

        // Forward of the declaration only. Rewriting the binding too would
        // leave `def <expression> = <expression>;` behind, which
        // `drop_inlined_declaration` can no longer find by name.
        let after = value_start + terminator + 1;
        let rewritten = replace_word(&out[after..], &name, &value);
        if rewritten != out[after..] {
            changed = true;
        }
        out = out[..after].to_string() + &rewritten;
        at = after;

        // The binding is dead text once every use of it carries the call.
        if let Some(without) = drop_inlined_declaration(&out, &name) {
            at = at.min(without.len());
            out = without;
            changed = true;
        }
    }

    if changed {
        Cow::Owned(out)
    } else {
        Cow::Borrowed(body)
    }
}

/// Whether `value` is exactly `ctx.<path>.splitOnToken("<sep>")`.
///
/// The whole value, not a call somewhere inside one: a local bound to
/// `parts.splitOnToken(",")[0].trim()` is a different expression, and
/// substituting it where a subscript follows would build a second subscript
/// this reader never wrote.
fn is_split_call(value: &str) -> bool {
    let Some((subject, rest)) = value.split_once(".splitOnToken(") else {
        return false;
    };
    let subject = subject.trim_end_matches(['.', '?']);
    let readable = subject
        .strip_prefix("ctx.")
        .or_else(|| subject.strip_prefix("ctx?."))
        .is_some_and(|path| {
            !path.is_empty()
                && path
                    .chars()
                    .all(|c| c.is_alphanumeric() || "._?['\"]".contains(c))
        });
    readable
        && quoted_literal(rest)
            .is_some_and(|(separator, after)| !separator.is_empty() && after.trim() == ")")
}

/// Whether every whole-word use of `name` is immediately subscripted by a
/// whole number -- `name[0]`, `name[-1]`.
///
/// The declaration subscripts nothing and is skipped, on a single `=`. A `==`
/// is a COMPARISON and is a use like any other, so `if (parts == null)` declines
/// the whole local rather than being read as its binding -- inlining around it
/// would leave the guard naming a local the substitution had just deleted.
fn every_use_subscripts(body: &str, name: &str) -> bool {
    fn word(c: u8) -> bool {
        c.is_ascii_alphanumeric() || c == b'_'
    }

    let bytes = body.as_bytes();
    let mut uses = 0usize;
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
        // The declaration is the one use that is an assignment rather than a
        // read, and it is the text being inlined.
        let tail = body[end..].trim_start();
        if tail.starts_with('=') && !tail.starts_with("==") {
            continue;
        }
        let Some(subscript) = body[end..]
            .strip_prefix('[')
            .and_then(|rest| rest.split_once(']'))
            .map(|(inside, _)| inside)
        else {
            return false;
        };
        if subscript.trim().parse::<i64>().is_err() {
            return false;
        }
        uses += 1;
    }
    uses > 0
}

impl Program {
    /// Read a body into the tree the per-event walk runs.
    pub(crate) fn parse(body: &str) -> Self {
        // Aliases first: a local list's members are literals, but a local
        // NAMING a field has to be followed before the list can be asked about
        // it.
        let body = inline_ctx_aliases(body);
        let body = inline_local_lists(&body);
        // Last, so a split whose subject is itself a local reads the path the
        // alias pass resolved rather than the local's name.
        let body = inline_split_locals(&body);
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
            || crate::common::painless_literal(wanted).is_some())
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
    FieldContains {
        path: String,
        argument: Argument,
        /// `ctx.<p>.toLowerCase().contains('x')` searches the FOLDED value, the
        /// same way [`Term::Compare`] below compares one. The script's own
        /// literal is already lower case, so only the event's side folds.
        lowered: bool,
    },
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
    /// The three container kinds and `boolean`, because JSON settles each of
    /// those exactly. `instanceof long` is NOT read: a JSON number carries no
    /// width, so a `long` test cannot be told from a `double` one here.
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
    /// JSON has one boolean and Painless has two spellings of it, so the test
    /// is exact -- unlike `long`, where the width the name asks about is not in
    /// the document. `jamf_protect`'s telemetry gates six `event.outcome` writes
    /// on `<field> instanceof boolean`, and while that was unread the term
    /// parsed to [`Term::Never`], the success branch was dead, and the script's
    /// own trailing default wrote `unknown` over all six.
    Bool,
}

impl JsonKind {
    /// The Painless type names that settle to one JSON kind.
    fn parse(name: &str) -> Option<Self> {
        match name.trim() {
            "List" | "Collection" | "ArrayList" => Some(Self::List),
            "Map" | "HashMap" => Some(Self::Map),
            "String" => Some(Self::Text),
            "boolean" | "Boolean" => Some(Self::Bool),
            _ => None,
        }
    }

    fn matches(self, value: &Value) -> bool {
        match self {
            Self::List => value.is_array(),
            Self::Map => value.is_object(),
            Self::Text => value.is_string(),
            Self::Bool => value.is_boolean(),
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
/// The branch resolver in [`crate::common`] rewrites a script per
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
            // The SUBJECT folds too, which the argument side above has always
            // read and this side had not. Leaving the fold on the path named a
            // field no event carries, so both arms of prisma_cloud's audit
            // ladder were false and its trailing else wrote `unknown` over
            // five failures.
            let (subject, lowered) = strip_case_fold(subject.trim());
            let Some(path) = subject.strip_prefix("ctx.") else {
                return Self::Never;
            };
            return Self::FieldContains {
                path: clean_path(path),
                argument,
                lowered,
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
                crate::common::painless_literal(wanted).map_or(Wanted::Unreadable, Wanted::Value)
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
            Self::FieldContains {
                path,
                argument,
                lowered,
            } => {
                let Some(wanted) = argument.resolve(event) else {
                    return false;
                };
                match event.get(path) {
                    Some(Value::Array(items)) => items
                        .iter()
                        .any(|item| item.as_str() == Some(wanted.as_ref())),
                    // `toLowerCase()` is a String method, so the list arm above
                    // never carries the fold. The allocation is what the
                    // script's own call costs, and it is only paid where the
                    // text spells one.
                    Some(Value::String(text)) if *lowered => {
                        text.to_lowercase().contains(wanted.as_ref())
                    }
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
    // Ahead of `.add(` below, which never sees this text -- `.addAll(` has no
    // `.add(` in it -- but reads as the neighbouring write and belongs beside
    // it. The argument has to be a `ctx.` path: a local holds a value this
    // cannot resolve, so the whole statement is declined rather than half read.
    if let Some((subject, argument)) = statement.split_once(".addAll(") {
        let source = argument
            .trim()
            .trim_end_matches(';')
            .trim()
            .trim_end_matches(')')
            .trim()
            .strip_prefix("ctx.")?;
        if !is_ctx_path(source) {
            return None;
        }
        return Some(Literal::AppendAll {
            path: clean_path(subject.trim().strip_prefix("ctx.")?),
            source: clean_path(source),
        });
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
        // The key has to OPEN the arguments. Windows builds one by
        // concatenation -- `ctx.winlog.event_data.put(Sd + "Owner", ...)` -- and
        // taking the first literal in the argument list called that key `Owner`.
        let key = literal_at(arguments)?;
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
    // A concatenation is read BEFORE every reader below, because the literal
    // one at the bottom answers the leading literal ALONE and reports the
    // write done. first_epss's whole source is one such expression, and all
    // nine of its events carried the bare prefix of the URL where the CVE id
    // belonged.
    if let Some(pieces) = parse_concat(text) {
        return Some(Rhs::Concat {
            pieces,
            fold: Fold::None,
        });
    }
    // The same join wrapped in parentheses and case-folded as a whole. The
    // parentheses are what make the fold the JOIN's rather than the last term's,
    // so a bare `ctx.a + ctx.b.toLowerCase()` is left to the readers below.
    if let Some((pieces, fold)) = parse_folded_concat(text) {
        return Some(Rhs::Concat { pieces, fold });
    }
    // A part of a split, before the bare-path read below takes the whole
    // expression for a field name.
    if let Some(part) = parse_split_part(text) {
        return Some(part);
    }
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
    // `Long.decode(ctx.<path>)`, before the bare-path read below takes the call
    // for part of a field name. Every trailing parenthesis goes: the argument is
    // a bare path, so the only ones there are this call's own close and whatever
    // enclosing call the statement sits in -- `ctx.a.put("k", Long.decode(ctx.b))`
    // hands this reader both.
    if let Some(rest) = text.strip_prefix("Long.decode(")
        && rest.ends_with(')')
    {
        let argument = subject_path(rest.trim_end_matches(')').trim());
        if let Some(path) = argument.strip_prefix("ctx.")
            && !path.is_empty()
            && !path.contains(['(', ')', ',', ' '])
        {
            return Some(Rhs::Decoded(clean_path(path)));
        }
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
            .then(|| Rhs::Field(dotted_subscripts(&clean_path(path))));
    }
    literal_value(text).map(Rhs::Literal)
}

/// `'<literal>' + ctx.<path> + ...` as its terms, in order.
///
/// Deliberately narrow: every term is a quoted literal or a bare `ctx.` path,
/// they alternate with `+`, and there is at least one of each. Anything else
/// answers `None` and leaves the readers in [`parse_rhs`] to decide, so a
/// plain literal assignment -- the bulk of what `PlainAssignments` claims --
/// reads exactly as it did.
///
/// A literal is required because all-fields is ambiguous: Painless
/// concatenates two strings and ADDS two numbers, and the text alone does not
/// say which the event will hold. All-literals is a constant, which the
/// literal reader already answers.
///
/// **This overlaps `KnownPattern::ConcatAssignment`, and the overlap is
/// deliberate.** That arm reads the same join, but only where it is the WHOLE
/// script -- one statement, optionally one `if`-wrapped block. This one reads
/// a join wherever a statement sits, so it covers the forms that arm declines:
/// a statement with no terminator (tychon's `package.cpe`), and a join inside
/// a longer script. Where both can read a script the ladder decides, and a
/// `Program` that can write promotes `GuardedCopy` above `ConcatAssignment` --
/// fortimanager's date join goes that way. That costs nothing because the two
/// agree on the answer, including writing NOTHING when a part is absent; it
/// would cost a source the day they stop agreeing, which is what the pins in
/// `tests/which_matcher.rs` are for.
fn parse_concat(text: &str) -> Option<Vec<ConcatPiece>> {
    // This reader runs ahead of every other, so the overwhelmingly common
    // right-hand side -- one quoted literal -- gets a byte scan rather than a
    // parse and a discarded allocation.
    if !text.contains('+') {
        return None;
    }

    let mut pieces = Vec::new();
    let mut rest = text.trim();
    loop {
        if let Some((literal, tail)) = quoted_literal(rest) {
            pieces.push(ConcatPiece::Literal(literal));
            rest = tail.trim_start();
        } else {
            // A term is everything up to the next `+`. One inside a literal
            // cannot reach here, because the literal arm above consumed it.
            let end = rest.find('+').unwrap_or(rest.len());
            let path = subject_path(rest[..end].trim())
                .strip_prefix("ctx.")
                .map(str::to_owned)?;
            if path.is_empty()
                || !path
                    .chars()
                    .all(|c| c.is_alphanumeric() || "._?@".contains(c))
            {
                return None;
            }
            pieces.push(ConcatPiece::Field(clean_path(&path)));
            rest = rest[end..].trim_start();
        }
        let Some(tail) = rest.strip_prefix('+') else {
            break;
        };
        rest = tail.trim_start();
    }

    let fields = pieces
        .iter()
        .filter(|piece| matches!(piece, ConcatPiece::Field(_)))
        .count();
    (rest.is_empty() && fields > 0 && fields < pieces.len()).then_some(pieces)
}

/// `(<join>).toLowerCase()` -- the join [`parse_concat`] reads, parenthesised
/// and case-folded as a whole.
///
/// The parentheses are the whole point and are required: without them the call
/// binds to the LAST TERM, so `ctx.a + ctx.b.toUpperCase()` folds only `b` and
/// is not this pattern. They must also wrap the entire remainder -- a stray
/// `(ctx.a) + ctx.b` closes early and is declined -- which is what the depth
/// walk checks.
///
/// `digital_guardian` is the source: `ctx.event.action` is
/// `(dg_utype + "-" + inc_state).toLowerCase()` on every one of its events.
fn parse_folded_concat(text: &str) -> Option<(Vec<ConcatPiece>, Fold)> {
    let text = text.trim();
    let (head, fold) = text
        .strip_suffix(".toLowerCase()")
        .map(|head| (head, Fold::Lower))
        .or_else(|| {
            text.strip_suffix(".toUpperCase()")
                .map(|head| (head, Fold::Upper))
        })?;

    let inner = head.trim().strip_prefix('(')?.strip_suffix(')')?;
    // The opening paren has to be the one the closing paren matches, or this is
    // a call on something else that happens to end in a bracket.
    let mut depth = 0usize;
    for c in inner.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.checked_sub(1)?,
            _ => {}
        }
    }
    if depth != 0 {
        return None;
    }

    parse_concat(inner).map(|pieces| (pieces, fold))
}

/// Named MEMBERS of a params row, each written to its own target.
///
/// zeek expands a connection state code through a table whose rows carry two
/// members, and takes one for the vendor field and the other for ECS:
///
/// ```painless
/// if (ctx.zeek?.connection?.state == null) { return; }
/// if (params.containsKey(ctx.zeek.connection.state)) {
///   ctx.zeek.connection.state_message = params[ctx.zeek.connection.state]["conn_str"];
///   ctx.event.type = params[ctx.zeek.connection.state]["types"];
/// }
/// ```
///
/// A reader that takes the row WHOLE writes both members as children of the
/// first target, which is why `state_message.conn_str` and
/// `state_message.types` read as wrong on 18 events beside `state_message`
/// itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RowMembers {
    /// The `ctx` path holding the row's key.
    key: String,
    /// `(member of the row, target)`, in the order the script writes them.
    writes: Vec<(String, String)>,
}

/// Read the member writes, or decline them.
///
/// Every write has to subscript the SAME lookup: a script that reads two rows,
/// or writes anything the row does not supply, is a different one.
fn parse_row_members(script: &str) -> Option<RowMembers> {
    let key = clean_path(
        script
            .split_once("params.containsKey(ctx.")?
            .1
            .split_once(')')?
            .0
            .trim(),
    );
    if key.is_empty() || key.contains(['(', ' ']) {
        return None;
    }

    let lookup = format!("params[ctx.{key}][");
    let mut writes = Vec::new();
    for statement in script.split(';') {
        let Some((target, value)) = statement.split_once('=') else {
            continue;
        };
        // The LAST `ctx.` on the left, because the first statement of a block
        // carries the `if` header in front of its assignment.
        let Some((_, target)) = target.rsplit_once("ctx.") else {
            continue;
        };
        let value = value.trim().replace("?.", ".");
        let Some(member) = value
            .strip_prefix(&lookup)
            .and_then(|rest| rest.strip_suffix(']'))
        else {
            // A statement that assigns something else means the script does
            // more than these writes, and running only these would leave it
            // half done.
            if value.contains("params[") {
                return None;
            }
            continue;
        };
        writes.push((
            member.trim().trim_matches(['"', '\'']).to_owned(),
            clean_path(target.trim()),
        ));
    }
    (!writes.is_empty()).then_some(RowMembers { key, writes })
}

/// A guard chain choosing ONE params row to write to a single target.
///
/// Every threat-intel package decides an expiry reason the same way, and the
/// params block is three sentences rather than a table keyed by a field:
///
/// ```painless
/// if (ctx.<ns>.valid_until != null) {
///   ctx.<ns>.ioc_expiration_reason = params.valid_until;
/// } else if (ctx.<ns>.revoked != null && ctx.<ns>.revoked == true) {
///   ctx.<ns>.ioc_expiration_reason = params.revoked;
/// } else {
///   ctx.<ns>.ioc_expiration_reason = params.default;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GuardedParamsRow {
    target: String,
    /// In order: the field that must be present, the literal it must equal
    /// where the guard names one, and the params key to write when it holds.
    arms: Vec<(String, Option<Value>, String)>,
    /// Written when no arm holds.
    fallback: String,
}

/// Read the chain, or decline it.
///
/// Every arm has to write the SAME target, and every value has to be a params
/// key rather than an expression: a chain that writes two fields, or computes
/// one of its values, is a different script and running half of it would put a
/// value where the vendor puts another.
fn parse_guarded_params_row(script: &str) -> Option<GuardedParamsRow> {
    let (head, tail) = script.trim().split_once("} else {")?;
    let otherwise = tail.split_once('}')?.0;
    let fallback = params_key(otherwise)?;

    // The `else` writes the same field as every arm, or the chain decides
    // between two targets and running it would put a value where the vendor
    // puts another.
    let mut target = Some(clean_path(
        otherwise.split_once('=')?.0.trim().strip_prefix("ctx.")?,
    ));
    let mut arms = Vec::new();
    for block in head.split("} else if") {
        let (guard, body) = block
            .trim()
            .trim_start_matches("if")
            .trim()
            .split_once('{')?;
        let guard = guard
            .trim()
            .strip_prefix('(')?
            .trim_end()
            .strip_suffix(')')?;
        let (path, wanted) = guard_terms(guard)?;
        let key = params_key(body)?;

        let written = clean_path(body.split_once('=')?.0.trim().strip_prefix("ctx.")?);
        if *target.get_or_insert(written.clone()) != written {
            return None;
        }
        arms.push((path, wanted, key));
    }
    (!arms.is_empty()).then(|| GuardedParamsRow {
        target: target.unwrap_or_default(),
        arms,
        fallback,
    })
}

/// `ctx.<path> != null`, optionally joined by `&&` to `ctx.<same path> ==
/// <literal>`.
fn guard_terms(guard: &str) -> Option<(String, Option<Value>)> {
    let mut terms = guard.split("&&").map(str::trim);
    let path = clean_path(
        terms
            .next()?
            .strip_suffix("!= null")?
            .trim()
            .strip_prefix("ctx.")?,
    );
    if path.is_empty() {
        return None;
    }
    let Some(second) = terms.next() else {
        return Some((path, None));
    };
    if terms.next().is_some() {
        return None;
    }
    // The second term has to test the SAME field, or the guard reads two.
    let (subject, value) = second.split_once("==")?;
    if clean_path(subject.trim().strip_prefix("ctx.")?) != path {
        return None;
    }
    let wanted = match value.trim() {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        text => literal_value(text)?,
    };
    Some((path, Some(wanted)))
}

/// `ctx.<path> = params.<key>;` as its key alone.
fn params_key(body: &str) -> Option<String> {
    let key = body
        .split_once("= params.")?
        .1
        .trim()
        .trim_end_matches(';')
        .trim();
    (!key.is_empty() && key.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| key.to_owned())
}

/// `ctx.<path>.splitOnToken("<sep>")[<n>]`, with `?.` and a trailing case fold.
///
/// Painless splits on a literal token rather than a pattern, so the separator is
/// taken as text and `str::split` reproduces it -- both keep the empty parts a
/// repeated separator leaves. An empty separator is declined: Painless throws on
/// one where `str::split` would answer with a boundary at every character.
fn parse_split_part(text: &str) -> Option<Rhs> {
    let (head, fold) = match text.trim() {
        rest if rest.ends_with(".toLowerCase()") => (
            rest.trim_end_matches(".toLowerCase()")
                .trim_end_matches('?'),
            Fold::Lower,
        ),
        rest if rest.ends_with(".toUpperCase()") => (
            rest.trim_end_matches(".toUpperCase()")
                .trim_end_matches('?'),
            Fold::Upper,
        ),
        rest => (rest, Fold::None),
    };

    let (subject, rest) = head.split_once("splitOnToken(")?;
    let subject = subject.trim_end_matches(['.', '?']);
    let path = clean_path(
        subject
            .strip_prefix("ctx.")
            .or_else(|| subject.strip_prefix("ctx?."))?,
    );
    if path.is_empty() || path.contains(['(', ' ']) {
        return None;
    }

    let (separator, after) = quoted_literal(rest)?;
    if separator.is_empty() {
        return None;
    }
    let (index, tail) = after
        .trim_start()
        .strip_prefix(')')?
        .trim_start()
        .strip_prefix('[')?
        .split_once(']')?;
    // Only the enclosing call's own closing parenthesis may follow the
    // subscript: `ctx.a.put("k", ctx.b.splitOnToken("/")[0])` hands this reader
    // the `put`'s close along with the expression. Anything else -- a `+ ".0"`,
    // a second subscript -- is a different expression and is declined.
    if !tail.trim().trim_end_matches(')').trim().is_empty() {
        return None;
    }

    // Painless counts `-1` back from the end, which a `usize` index cannot
    // hold. No other negative subscript is spelled in the tree, so none is read.
    if index.trim() == "-1" {
        return Some(Rhs::SplitLast {
            path,
            separator,
            fold,
        });
    }

    Some(Rhs::SplitPart {
        path,
        separator,
        index: index.trim().parse().ok()?,
        fold,
    })
}

/// A quoted string literal at the head of `text`, as its body and whatever
/// follows the closing quote.
///
/// Either quote character opens one, and a backslash escapes the character
/// after it rather than ending the literal. The call site's own escaping is
/// already resolved by [`crate::common::normalise`] -- wiz's URL
/// reaches the generated module as `\"https://...\"` and arrives here with
/// plain quotes -- so a backslash here is one the vendor wrote.
fn quoted_literal(text: &str) -> Option<(String, &str)> {
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let body = &text[quote.len_utf8()..];
    let mut out = String::with_capacity(body.len());
    let mut chars = body.char_indices();
    while let Some((at, c)) = chars.next() {
        if c == quote {
            return Some((out, &body[at + c.len_utf8()..]));
        }
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some((_, 'n')) => out.push('\n'),
            Some((_, 't')) => out.push('\t'),
            Some((_, 'r')) => out.push('\r'),
            Some((_, escaped)) => out.push(escaped),
            // An unterminated literal is not a literal.
            None => return None,
        }
    }
    None
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
        // Painless appends every member without deduplicating, which is the
        // difference from the `append` PROCESSOR beside these scripts and its
        // `allow_duplicates: false`. A source that is not a list is nothing
        // `addAll` could take, so nothing is written.
        //
        // Grown and written ONCE rather than through [`add_to_list`] per
        // member, which clones the target on every call and is quadratic in
        // the list -- o365 folds its whole parsed action list back this way.
        Literal::AppendAll { path, source } => {
            let Some(Value::Array(items)) = event.get(source) else {
                return false;
            };
            if items.is_empty() {
                return false;
            }
            let mut grown = match event.get(path) {
                Some(Value::Array(held)) => {
                    let mut grown = Vec::with_capacity(held.len() + items.len());
                    grown.extend(held.iter().cloned());
                    grown
                }
                // A scalar already there becomes the list's first member, the
                // way Painless's own `add` on it would.
                Some(held) => {
                    let mut grown = Vec::with_capacity(items.len() + 1);
                    grown.push(held.clone());
                    grown
                }
                None => Vec::with_capacity(items.len()),
            };
            grown.extend(items.iter().cloned());
            let _ = event.set(path, Value::Array(grown));
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
        // Painless hands `Long.decode` a String and THROWS on anything else, so
        // a field that is not text writes nothing rather than being coerced --
        // and text it cannot read throws too, which the vendor's own
        // `on_failure` handler is there for.
        Rhs::Decoded(path) => event
            .get_str(path)
            .and_then(crate::coercion::java_decode)
            .map(Value::from),
        // A field the event does not carry writes NOTHING, where Painless
        // would splice in the text `null`. Every site is guarded by its
        // processor's own `if` on that field, so an absent one means this is
        // not the event the script was written for -- and a URL with `null`
        // inside it is a worse answer than no URL.
        Rhs::Concat { pieces, fold } => {
            // The literals' length is known here; the fields are whatever the
            // event holds, so the growth left is theirs alone.
            let literals: usize = pieces
                .iter()
                .map(|piece| match piece {
                    ConcatPiece::Literal(text) => text.len(),
                    ConcatPiece::Field(_) => 0,
                })
                .sum();
            let mut built = String::with_capacity(literals);
            for piece in pieces {
                match piece {
                    ConcatPiece::Literal(text) => built.push_str(text),
                    ConcatPiece::Field(path) => match event.get(path)? {
                        Value::String(text) => built.push_str(text),
                        Value::Number(number) => built.push_str(&number.to_string()),
                        Value::Bool(flag) => built.push_str(if *flag { "true" } else { "false" }),
                        // A container has no text rendering Elasticsearch and
                        // this would agree on.
                        Value::Null | Value::Array(_) | Value::Object(_) => return None,
                    },
                }
            }
            Some(Value::String(fold.apply(&built)))
        }
        // A field the split does not reach that far into writes NOTHING, the
        // way Painless throws and the processor's `on_failure` leaves the
        // field.
        Rhs::SplitPart {
            path,
            separator,
            index,
            fold,
        } => event
            .get_str(path)
            .and_then(|text| text.split(separator.as_str()).nth(*index))
            .map(|part| Value::String(fold.apply(part))),
        // A string holding no separator splits into ONE part, and that part is
        // its own last -- which is what makes a bare `cmd.exe` come out as the
        // process name rather than nothing.
        Rhs::SplitLast {
            path,
            separator,
            fold,
        } => event
            .get_str(path)
            .and_then(|text| text.rsplit(separator.as_str()).next())
            .map(|part| Value::String(fold.apply(part))),
        Rhs::Literal(value) => Some(value.clone()),
    }
}

/// A quoted string, or a bracketed list of them.
///
/// `ctx.event.type = ['info']` is as common as the scalar form and was read as
/// the bare string `info`, so the field came out a string where Elastic writes
/// a one-element array.
///
/// **The WHOLE text has to be that literal.** This is the last reader
/// [`parse_rhs`] tries, so an expression none of the others could read arrives
/// here intact -- and a scan for the first quoted run ANYWHERE answers such an
/// expression with a fragment of itself. `digital_guardian` writes
/// `ctx.event.action = (ctx.<a> + "-" + ctx.<b>).toLowerCase()`, and every one
/// of its events carried the literal `-` as its action, with `PlainAssignments`
/// reporting the write done.
pub(crate) fn literal_value(text: &str) -> Option<Value> {
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
        let (literal, tail) = quoted_literal(text)?;
        return tail.trim().is_empty().then_some(Value::String(literal));
    };
    // EVERY member has to be a literal, on the same reading as the scalar arm.
    // Skipping the ones this cannot read wrote a SHORTER list than the vendor's
    // -- `['ok', ctx.a]` came out as one element -- which is a wrong value
    // rather than a missing one.
    let mut members = Vec::new();
    for member in inner.split(',') {
        let (literal, tail) = quoted_literal(member.trim())?;
        if !tail.trim().is_empty() {
            return None;
        }
        members.push(Value::String(literal));
    }
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
pub(crate) fn add_to_list(event: &mut Event, path: &str, value: Value) {
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

/// The flag whose name depends on another field carrying a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContainsFlagsBranch {
    /// The literal the source is tested for.
    marker: String,
    /// The field whose non-empty value chooses between the two names.
    witness: String,
    /// The name added where `witness` holds a value, and where it does not.
    present: String,
    absent: String,
}

/// Flags named by which of a params table's KEYS a field CONTAINS.
///
/// ```painless
/// ArrayList hf = new ArrayList();
/// for (entry in params.entrySet()) {
///   if (ctx.infoblox_nios.log.dns.header_flags.contains(entry.getKey())) {
///     hf.add(entry.getValue());
///   }
/// }
/// if (ctx.dns?.response_code != null && ctx.dns.response_code != '') {
///   if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) { hf.add('RA') }
/// } else {
///   if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) { hf.add('RD') }
/// }
/// if (hf.length == 0) { return; }
/// if (ctx.dns == null) { HashMap hm = new HashMap(); ctx.put('dns', hm); }
/// ctx.dns.put('header_flags', hf);
/// ```
///
/// `infoblox_nios`'s named(8) log packs the DNS header flags into one token --
/// `+AED` -- and the table turns each character into its ECS name. The `+` is
/// the odd one out: it means recursion DESIRED on a query and recursion
/// AVAILABLE on a response, so which name it takes is decided by whether the
/// event carries a response code.
///
/// The table is read as a SUBSTRING test rather than a lookup, which is what
/// separates it from the lookups below: they index `params` by the field's
/// whole value, and this one asks which of the params keys the value holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContainsFlags {
    /// The string tested against every params key.
    source: String,
    /// Where the list of names lands.
    target: String,
    branch: ContainsFlagsBranch,
}

fn parse_contains_flags(script: &str) -> Option<ContainsFlags> {
    let flat = crate::common::one_space(script);

    // The accumulator, and the field every membership test reads.
    let bucket = flat
        .split_once(" = new ArrayList()")?
        .0
        .rsplit(' ')
        .next()
        .filter(|local| !local.is_empty())?;
    let at = flat.find(".contains(entry.getKey())")?;
    let source = clean_path(flat[..at].rsplit_once("ctx.")?.1);
    if source.is_empty() || !flat.contains(&format!("{bucket}.add(entry.getValue());")) {
        return None;
    }

    // The two arms, and the field that chooses between them.
    let witness = clean_path(flat.split_once(" != null && ")?.0.rsplit_once("ctx.")?.1);
    let (present_arm, absent_arm) = flat.split_once("} else {")?;
    // Every read here is anchored on the literal's own opening quote. The walk
    // above the branch spells both `{source}.contains(` and `{bucket}.add(`
    // with a non-literal argument, so an unanchored read finds those first and
    // then runs on to whatever quote comes next.
    let add = format!("{bucket}.add(");
    let (quote, marker) = opening_literal(present_arm, &format!("{source}.contains("))?;
    let branch = ContainsFlagsBranch {
        present: opening_literal(present_arm, &add)?.1,
        absent: opening_literal(absent_arm, &add)?.1,
        witness,
        marker,
    };
    // Both arms test the same literal against the same field, or they are two
    // different rules and only one of them would be run. An empty marker would
    // be held by every value there is, so it is no test at all.
    let test = format!("{source}.contains({quote}{}{quote})", branch.marker);
    if branch.witness.is_empty()
        || branch.marker.is_empty()
        || !present_arm.contains(test.as_str())
        || !absent_arm.contains(test.as_str())
    {
        return None;
    }

    // An empty list is not written at all, and the only other statement that
    // may write the document is the vendor creating the target's parent.
    let empty = [
        format!("if ({bucket}.length == 0) {{ return; }}"),
        format!("if ({bucket}.size() == 0) {{ return; }}"),
    ];
    let put = flat.rfind(".put(")?;
    let parent = clean_path(flat[..put].rsplit_once("ctx.")?.1);
    let key = quoted_after(&flat[put..], ".put(")?;
    if !empty.iter().any(|form| flat.contains(form.as_str()))
        || flat.matches(&format!("{bucket}.add(")).count() != 3
        || !flat[put..].contains(&format!(", {bucket})"))
        || flat.matches(".put(").count() > 2
        || (flat.matches(".put(").count() == 2 && !flat.contains(&format!("ctx.put('{parent}',")))
        || parent.is_empty()
        || key.is_empty()
    {
        return None;
    }

    Some(ContainsFlags {
        source,
        target: format!("{parent}.{key}"),
        branch,
    })
}

fn run_contains_flags(
    event: &mut Event,
    pattern: &ContainsFlags,
    params: &Map<String, Value>,
) -> bool {
    // The processor's own `if` guards the field, so nothing to read here means
    // the path came out wrong.
    let Some(text) = event.get_string(&pattern.source) else {
        return false;
    };

    // The params block is insertion-ordered, and the order it is written in is
    // the order Elasticsearch appends the names in.
    let mut flags: Vec<Value> = params
        .iter()
        .filter(|(key, _)| text.contains(key.as_str()))
        .map(|(_, value)| value.clone())
        .collect();

    if text.contains(pattern.branch.marker.as_str()) {
        let carried = event.has_value(&pattern.branch.witness)
            && event.get_str(&pattern.branch.witness) != Some("");
        flags.push(Value::String(if carried {
            pattern.branch.present.clone()
        } else {
            pattern.branch.absent.clone()
        }));
    }

    // The script's own early return: an empty list leaves the target alone.
    if flags.is_empty() {
        return true;
    }
    let _ = event.set(&pattern.target, Value::Array(flags));
    true
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
///
/// A write SUBSCRIPTED by an expression is not one of these and is skipped: the
/// destination there is the path the subscript is applied to plus a key the
/// event decides, and no `(path, expression)` pair can say that. Reading the
/// last `ctx.` on such a line lands INSIDE the subscript's own argument --
/// `ctx["json"]["model"]["logic"][params.get(ctx.json.model.logic?.type)]` came
/// back as `json.model.logic.type]`, and `LookupNormalise` then stored the
/// table's row under a key called `type]` on every darktrace event. Writing
/// nothing is the answer; guessing the container instead would put a bare
/// string where the container belongs.
pub(crate) fn ctx_writes(script: &str) -> Vec<(String, String)> {
    let mut writes = Vec::new();
    for statement in script.split(';') {
        let Some((lhs, rhs)) = split_assignment(statement) else {
            continue;
        };
        let Some(start) = lhs.rfind("ctx.") else {
            continue;
        };
        if open_subscripts(&lhs[..start]) > 0 {
            continue;
        }
        let path = lhs[start + "ctx.".len()..]
            .replace("['", ".")
            .replace("[\"", ".")
            .replace("']", "")
            .replace("\"]", "");
        writes.push((clean_path(&path), rhs.trim().to_string()));
    }
    writes
}

/// How many subscripts `head` leaves open.
fn open_subscripts(head: &str) -> usize {
    head.bytes().fold(0usize, |depth, byte| match byte {
        b'[' => depth + 1,
        b']' => depth.saturating_sub(1),
        _ => depth,
    })
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
    let at =
        (0..bytes.len()).find(|at| bytes[*at] == b'=' && crate::common::assigns_at(bytes, *at))?;
    Some((&statement[..at], &statement[at + 1..]))
}

/// A scalar's text, the way Painless would stringify it for a map key.
pub(crate) fn scalar_text(value: &Value) -> Option<String> {
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
    // Container and key off the SAME `.put(`, and only one whose key is a
    // literal. cisco_asa, cisco_ftd, fireeye_nx and akamai_siem each open with a
    // `.put(` taking a local, so an unanchored key lands in another statement.
    let Some((at, key)) = literal_call(script, ".put(") else {
        return false;
    };
    let Some(container) = ctx_path_at_end(&script[..at]) else {
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

    // The script says which end it counts from. `params['t'][value - 1]` is
    // 1-based and reading it as 0-based returns the NEXT row, which is a
    // plausible value rather than a miss: checkpoint_email's severity 2 wrote
    // `Medium` where the vendor writes `Low`, in every event.
    let Some(index) = index.checked_sub(subscript_offset(script)) else {
        return true;
    };

    if let Some(value) = table.get(index).cloned() {
        let _ = event.set(&format!("{container}.{key}"), value);
    }
    true
}

/// The constant a params subscript subtracts from its index, or zero.
///
/// Read off the script rather than assumed either way: `checkpoint_email` and
/// `cyberark_epm` spell `[value - 1]`, and other pipelines subscript with no
/// offset at all.
fn subscript_offset(script: &str) -> usize {
    let Some((_, after)) = script.split_once('[') else {
        return 0;
    };
    let Some((subscript, _)) = after.rsplit_once(']') else {
        return 0;
    };
    // The LAST subscript in the statement is the row index; an earlier one
    // names the table (`params['severity'][value - 1]`).
    let subscript = subscript.rsplit('[').next().unwrap_or(subscript);
    let Some((_, offset)) = subscript.rsplit_once('-') else {
        return 0;
    };
    offset.trim().parse().unwrap_or(0)
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
    let Some(source) = crate::common::painless_path(head) else {
        return false;
    };
    // The assignment that owns the multiply. Without one the script scales in
    // place, which is azure's spelling and the compound form's only one.
    let path = crate::common::last_assignment(head)
        .and_then(|at| crate::common::painless_path(&head[..at]))
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
    // `Math.round(<field> * params.<name>)`, which ten sources spell over
    // `event.duration` -- zeek's connection stream scales 0.10412883758544922
    // seconds to the 104128838 Elasticsearch writes, on 8 of its 18 events.
    // Java rounds a half TOWARDS positive infinity, which `f64::round` does not
    // do for a negative operand.
    if script.contains("Math.round(") {
        #[allow(clippy::cast_possible_truncation)]
        let _ = event.set(&path, (scaled + 0.5).floor() as i64);
    } else if scaled.fract() == 0.0 && scaled.abs() < 9.007_199_254_740_992e15 {
        // Whole results stay integers: a duration in nanoseconds is not a float.
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
    use crate::common::painless_path;

    use crate::common::ctx_path_bound_to;

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
    // Anchored on the call the replacement belongs to: a bare `,` is the first
    // comma in the SCRIPT, and is this call's only where nothing else has one.
    let replacement = script
        .split_once(".replace(params.")
        .and_then(|(_, tail)| tail.split_once(','))
        .and_then(|(_, after)| literal_at(after))
        .unwrap_or_default();
    let Some(current) = event.get_str(&path) else {
        return true;
    };

    let replaced = current.replace(needle, &replacement);
    let _ = event.set(&path, replaced);
    true
}

/// The first single- or double-quoted string following `after`.
///
/// UNANCHORED, and that is the whole hazard: the quote it answers with may sit
/// past the end of the call, the statement or the function. Safe only where the
/// slice handed in is already cut to the call -- `&flat[put..]` starts AT the
/// call, so the anchor is the first thing in it. Where the slice is a whole
/// script, reach for [`literal_at`], [`literal_call`] or [`opening_literal`],
/// which make the opening quote part of what is searched for.
fn quoted_after(script: &str, after: &str) -> Option<String> {
    let tail = &script[script.find(after)? + after.len()..];
    let start = tail.find(['\'', '"'])?;
    let quote = tail.as_bytes()[start] as char;
    let end = tail[start + 1..].find(quote)?;
    Some(tail[start + 1..=start + end].to_string())
}

/// The quoted literal `text` OPENS with, over any leading space.
///
/// Nothing is searched for, which is the point: an argument that is not a
/// literal answers None here rather than running on to the next quote in the
/// file. `ctx.winlog.event_data.put(Sd + "Owner", ...)` is the case in hand --
/// the key is a concatenation, and the unanchored read called it `Owner`.
fn literal_at(text: &str) -> Option<String> {
    let text = text.trim_start();
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let rest = &text[quote.len_utf8()..];
    rest.find(quote).map(|end| rest[..end].to_string())
}

/// The first `call` whose argument is a LITERAL, as its offset and that literal.
///
/// The offset is what separates this from [`opening_literal`]: a caller reading
/// both halves of one call -- the subject path and the key -- gets them off the
/// SAME occurrence. Pairing `ctx_path_before(script, ".put(")` with a key read
/// from a later `.put(` names a field neither statement writes.
pub(crate) fn literal_call(script: &str, call: &str) -> Option<(usize, String)> {
    let mut at = 0;
    while let Some(found) = script[at..].find(call) {
        let start = at + found;
        at = start + call.len();
        if let Some(text) = literal_at(&script[at..]) {
            return Some((start, text));
        }
    }
    None
}

/// The literal a call opens with, where `anchor` ends at its `(`, and the
/// quote character it was written in.
///
/// The quote is searched for as part of the anchor, which is the difference
/// from [`quoted_after`]: that finds `after` and then the next quote anywhere
/// past it, over the end of the call and the end of the statement alike. On
/// `infoblox_nios`'s flag arms it answered with the empty string of a `!= ''`
/// test two statements away. Making the quote part of the anchor also steps
/// over a call whose argument is not a literal, so `hf.add(` reads past the
/// `hf.add(entry.getValue())` of the walk above.
fn opening_literal(script: &str, anchor: &str) -> Option<(char, String)> {
    ['\'', '"'].into_iter().find_map(|quote| {
        let open = format!("{anchor}{quote}");
        let at = script.find(&open)?;
        let tail = &script[at + open.len()..];
        tail.find(quote).map(|end| (quote, tail[..end].to_string()))
    })
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
///
/// Anchored on the bracket's own opening quote, because `params[<ctx path>]` is
/// a different form entirely -- the whole block is the table there, and no name
/// is being spelled. `ti_crowdstrike_ioc`'s `params[ctx.ti_crowdstrike.ioc.type]`
/// is followed by a `'domain'` its block really holds, so an unanchored read
/// indexes a table the script never named.
fn params_indexed<'a>(script: &str, params: &'a Map<String, Value>) -> Option<&'a Value> {
    if let Some((_, name)) = opening_literal(script, "params[") {
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
    ctx_path_at_end(&script[..script.find(marker)?])
}

/// The dotted `ctx.` path that ENDS at `head`.
///
/// Split out of [`ctx_path_before`] so a caller that has already located its
/// call -- by offset, because the key it read had to come off that same call --
/// can read the subject without searching for the marker a second time and
/// landing on an EARLIER one.
pub(crate) fn ctx_path_at_end(head: &str) -> Option<String> {
    // `ctx?.` is the same root written null-safe, and juniper_srx writes every
    // one of its paths that way. Reading only the plain spelling left its
    // sentinel sweep bound to a runner that could not find the map.
    let start = match (head.rfind("ctx."), head.rfind("ctx?.")) {
        (Some(plain), Some(safe)) if safe > plain => safe + "ctx?.".len(),
        (Some(plain), _) => plain + "ctx.".len(),
        (None, Some(safe)) => safe + "ctx?.".len(),
        (None, None) => return None,
    };
    // A path read backwards out of a CALL carries that call's CLOSING paren and
    // whatever trails it: `Integer.parseInt(ctx.a.b)-1` yields `a.b)-1`. No
    // field name holds one, so the path ends where it begins -- reading it whole
    // cost bitwarden and symantec_endpoint_security every indexed lookup they
    // spell.
    let read = &head[start..];
    let read = read.find(')').map_or(read, |at| &read[..at]);
    let path = clean_path(read);
    // An OPENING paren left over is the other case: the subject is a method
    // call, `ctx.a.b.entrySet()` rather than a field, and the path names nothing
    // the document holds.
    (!path.contains([' ', '\t', '\n', ';', '('])).then_some(path)
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
pub(crate) fn last_delimited(script: &str, name: &str, open: char, close: char) -> Option<String> {
    // The name ends AT its own open, so the region starts one char back and
    // [`balanced`] sees the opener it expects to be handed.
    let at = script.rfind(name)? + name.len() - open.len_utf8();
    balanced(&script[at..], open, close).map(|(inside, _)| inside.to_string())
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

/// A path with Painless's NUMERIC list subscripts respelled as segments.
///
/// `facts[0].name` names the value the document spells `facts.0.name`:
/// `Event::get` walks dotted segments and reads a numeric one on an array as an
/// index, and has no reading at all for a bracket. The bare-path reader in
/// [`parse_rhs`] admits `[` and `]`, so a script carrying one was CLAIMED and
/// then handed a path nothing could resolve -- the write answered `None` and was
/// skipped in silence. `jamf_protect`'s alerts lost `event.action`, `rule.name`,
/// `event.reason` and `rule.description` on all eleven events that way, with
/// the bracket sitting in plain view in the binding dump.
///
/// The NUMERIC subscript only. A quoted one -- `a['b.c']` -- is a key that may
/// hold dots of its own, which is [`crate::common::painless_path`]'s question
/// and a different answer; one is left exactly as it arrived, so a path this
/// cannot respell reads no worse than it did.
fn dotted_subscripts(path: &str) -> String {
    if !path.contains('[') {
        return path.to_owned();
    }
    let mut out = String::with_capacity(path.len());
    let mut rest = path;
    while let Some(open) = rest.find('[') {
        let Some(close) = rest[open + 1..].find(']').map(|at| open + 1 + at) else {
            break;
        };
        let index = &rest[open + 1..close];
        if index.is_empty() || !index.bytes().all(|byte| byte.is_ascii_digit()) {
            break;
        }
        out.push_str(&rest[..open]);
        out.push('.');
        out.push_str(index);
        rest = &rest[close + 1..];
    }
    out.push_str(rest);
    out
}

/// Whether a string could be a field path at all.
///
/// A reader that slices between two markers has no guarantee they sit in one
/// statement, and what comes back is then SOURCE rather than a path -- ece's
/// `event.action` script put two hundred characters of its own body into a
/// `FirstElement` take, `Event::set` failed on it, and the error was swallowed.
/// A slash and a colon are legal: azure's SAML claims are keyed by URI.
pub(crate) fn is_ctx_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains([
            ' ', '\t', '\n', '\r', ';', '(', ')', '\'', '"', '{', '}', '=', '!', ',', '+',
        ])
}

/// The `ctx` path each `def <name> = ctx?...;` line reads, keyed by the local.
///
/// The prefix must be `ctx.` or `ctx?.` in full. Stripping a bare `ctx` and
/// then trimming the punctuation reads `ctxfoo.bar` as the path `foo.bar`,
/// which binds a local to a field the script never named.
///
/// A call or a subscript is declined: the value is not a path this can resolve.
pub(crate) fn ctx_locals(script: &str) -> Vec<(String, String)> {
    script
        .split(';')
        .filter_map(|statement| {
            let (name, value) = statement.trim().strip_prefix("def ")?.split_once('=')?;
            let value = value.trim();
            let path = value
                .strip_prefix("ctx?.")
                .or_else(|| value.strip_prefix("ctx."))?;
            let path = clean_path(path);
            (!path.is_empty() && !path.contains(['(', ' ', '[']))
                .then(|| (name.trim().to_owned(), path))
        })
        .collect()
}

/// One `ctx.<path>` TERM as a dotted path, declining a call or a subscript.
///
/// Accepts any other character, `@timestamp` included, and refuses only what
/// says the term is not a bare path: parentheses, brackets and whitespace.
///
/// The strict counterpart below refuses `@timestamp` as well, and the two are
/// deliberately NOT one reader --
/// `the_readers_differ_on_the_at_sign_and_that_is_deliberate` pins why.
/// Widening the strict one opens matchers that have never been exposed to a
/// bad path; narrowing this one refuses ECS fields that are real. Either is a
/// parity change rather than a tidy-up.
pub(crate) fn ctx_path_term(term: &str) -> Option<String> {
    let path = clean_path(term);
    let path = path.strip_prefix("ctx.")?;
    (!path.is_empty() && !path.contains(['(', ')', '[', ']', ' ', '\t'])).then(|| path.to_owned())
}

/// One `ctx.<path>` term, admitting only alphanumerics, `_` and `.`.
///
/// So `@timestamp` is REFUSED here and accepted by `ctx_path_term` -- see the
/// note there before making them agree.
pub(crate) fn ctx_path_plain(text: &str) -> Option<String> {
    let path = clean_path(text);
    let path = path.strip_prefix("ctx.")?;
    (!path.is_empty()
        && path
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.'))
    .then(|| path.to_string())
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
#[path = "params_tests.rs"]
mod tests;
