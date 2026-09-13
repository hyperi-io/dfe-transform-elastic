// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Words collected into `HashSet` accumulators and written out as lists.
//!
//! `trend_micro_vision_one` classifies an event by reading one or two of its own
//! fields and adding ECS words to a pair of sets, then writing each set that
//! collected anything:
//!
//! ```painless
//! def eventCategory = new HashSet();
//! def eventType = new HashSet();
//! def description = ctx.trend_micro_vision_one.alert.description.toLowerCase();
//! if (description.contains('logon')) {
//!   eventCategory.add('authentication');
//!   eventCategory.add('host');
//!   eventType.add('info');
//! } else if (description.contains('email')) {
//!   eventCategory.add('email');
//!   eventType.add('info');
//! } else {
//!   eventCategory.add('malware');
//!   eventType.add('info');
//! }
//! if (!eventCategory.isEmpty()) { ctx.event.category = eventCategory; }
//! if (!eventType.isEmpty()) { ctx.event.type = eventType; }
//! ```
//!
//! Its `audit` stream writes the same pattern with two subjects, independent
//! `if`s rather than one chain, list-membership tests, and one arm carrying a
//! nested chain -- so the reader walks arbitrary chains rather than a single
//! ladder, and gates on having read the WHOLE script.
//!
//! **The written order is Java's, not the script's.** `ctx.event.category`
//! receives the `HashSet` itself, and Elasticsearch serialises it by walking the
//! hash table -- so `add('authentication')` then `add('host')` comes out as
//! `["host", "authentication"]`. [`crate::helpers::java_bucket`] is what
//! reproduces that.
//!
//! **A vendor literal is compared as written.** `trend_micro`'s audit list holds
//! `'Notifications'` among sixteen lower-case words while the subject is
//! lower-cased before the test, so that one member can never match. Folding it
//! would write a category Elasticsearch does not.

use dfe_core::Event;
use serde_json::Value;

use crate::common::{if_block, matching_brace, vocabulary_tested};
use crate::params::clean_path;

/// The subjects the arms read, the accumulators they fill, and the chains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetLadder {
    /// Indexed by every [`Test`]'s `subject`.
    subjects: Vec<Subject>,
    /// The path each accumulator is written to, indexed by every arm's adds.
    targets: Vec<String>,
    /// Top-level chains, run in script order.
    chains: Vec<Chain>,
}

/// A local bound to a document field, and how the script read it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Subject {
    path: String,
    /// The script bound it through `.toLowerCase()`.
    folded: bool,
}

/// One `if` / `else if` / `else`, of which at most one arm runs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Chain {
    arms: Vec<Arm>,
}

/// One arm of a chain: what admits it, what it adds, what it guards.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Arm {
    /// `None` for a closing `else`, which always admits.
    test: Option<Test>,
    /// `(accumulator, word)` in the order the arm adds them.
    adds: Vec<(usize, String)>,
    /// Chains nested inside this arm's body.
    nested: Vec<Chain>,
}

/// The three ways these scripts test a subject.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Test {
    /// `<subject>.contains('<word>')` -- the word is a SUBSTRING of the field.
    Holds { subject: usize, word: String },
    /// `['a', 'b'].contains(<subject>)` -- the field is one of the words.
    OneOf { subject: usize, words: Vec<String> },
    /// `<subject> == '<word>'`
    Is { subject: usize, word: String },
}

impl Test {
    /// Whether this arm's guard admits the values read from the document.
    fn admits(&self, read: &[String]) -> bool {
        match self {
            Self::Holds { subject, word } => read[*subject].contains(word.as_str()),
            Self::OneOf { subject, words } => words.iter().any(|word| word == &read[*subject]),
            Self::Is { subject, word } => &read[*subject] == word,
        }
    }
}

/// Read the accumulators, their subjects and their chains, or decline.
///
/// A WHOLE-SCRIPT match: every statement must be one of the four this reader
/// reproduces -- a `new HashSet()` declaration, a subject binding, a chain whose
/// every arm body is adds and nested chains, or an `isEmpty`-guarded write-back.
/// Anything else declines, so a script doing this alongside other work is left
/// for a reader that covers the rest rather than claimed and half-run.
pub fn parse_set_ladder(script: &str) -> Option<SetLadder> {
    let mut sets: Vec<String> = Vec::new();
    let mut targets: Vec<Option<String>> = Vec::new();
    let mut locals: Vec<String> = Vec::new();
    let mut subjects: Vec<Subject> = Vec::new();
    let mut chains: Vec<Chain> = Vec::new();

    for statement in statements(script)? {
        if let Some(name) = declares_set(statement) {
            sets.push(name);
            targets.push(None);
            continue;
        }
        if let Some((name, subject)) = declares_subject(statement) {
            locals.push(name);
            subjects.push(subject);
            continue;
        }
        if let Some((at, path)) = write_back(statement, &sets) {
            // Two write-backs for one accumulator is a script this reader has
            // not read, not a target to pick between.
            if targets[at].replace(path).is_some() {
                return None;
            }
            continue;
        }
        chains.push(parse_chain(statement, &locals, &sets)?);
    }

    // An accumulator with no write-back is work the script does and this reader
    // would drop, so the whole parse declines rather than the one set.
    let targets: Option<Vec<String>> = targets.into_iter().collect();
    let targets = targets?;
    if targets.is_empty() || subjects.is_empty() || chains.is_empty() {
        return None;
    }
    Some(SetLadder {
        subjects,
        targets,
        chains,
    })
}

/// Fill each accumulator, then write the ones that collected anything.
///
/// An absent subject writes nothing: every one of these scripts is a processor
/// guarded on its subjects being present and non-empty, so reaching here without
/// one means the guard held and Painless never ran the body.
pub fn set_ladder(event: &mut Event, pattern: &SetLadder) -> bool {
    let mut read: Vec<String> = Vec::with_capacity(pattern.subjects.len());
    for subject in &pattern.subjects {
        let Some(held) = event.get_str(&subject.path) else {
            return true;
        };
        read.push(if subject.folded {
            held.to_lowercase()
        } else {
            held.to_owned()
        });
    }

    let mut collected: Vec<Vec<String>> = vec![Vec::new(); pattern.targets.len()];
    run_chains(&pattern.chains, &read, &mut collected);

    for (target, words) in pattern.targets.iter().zip(collected) {
        if words.is_empty() {
            continue;
        }
        let _ = event.set(target, Value::Array(java_set_order(words)));
    }
    true
}

/// Run each chain's first admitting arm, and whatever that arm guards.
fn run_chains(chains: &[Chain], read: &[String], collected: &mut [Vec<String>]) {
    for chain in chains {
        for arm in &chain.arms {
            if !arm.test.as_ref().is_none_or(|test| test.admits(read)) {
                continue;
            }
            for (at, word) in &arm.adds {
                // `Set.add` of a word already held is a no-op, and the first
                // add is the one that decides its place in the table.
                if !collected[*at].iter().any(|held| held == word) {
                    collected[*at].push(word.clone());
                }
            }
            run_chains(&arm.nested, read, collected);
            break;
        }
    }
}

/// The order a `HashSet` hands its words to the serialiser: bucket index
/// ascending, insertion order within a bucket.
fn java_set_order(mut words: Vec<String>) -> Vec<Value> {
    let table = crate::helpers::java_table_size(words.len());
    words.sort_by_key(|word| crate::helpers::java_bucket(word, table));
    words.into_iter().map(Value::String).collect()
}

/// The statements of a block, an `if ... else ...` CHAIN counting as ONE.
///
/// [`crate::common::block_statements`] stops at the `if`, so its `after` starts
/// at the `else` and the next split lands inside that block's body. These
/// scripts are chains throughout, so the walk has to take the whole chain.
fn statements(block: &str) -> Option<Vec<&str>> {
    let mut out = Vec::new();
    let mut rest = block.trim();
    while !rest.is_empty() {
        if let Some(end) = chain_end(rest) {
            out.push(rest[..end].trim());
            rest = rest[end..]
                .trim_start()
                .strip_prefix(';')
                .unwrap_or(&rest[end..]);
            rest = rest.trim();
            continue;
        }
        let end = rest.find(';')?;
        let statement = rest[..end].trim();
        if !statement.is_empty() {
            out.push(statement);
        }
        rest = rest[end + 1..].trim();
    }
    Some(out)
}

/// The offset just past the `if ... else ...` chain `text` opens with.
fn chain_end(text: &str) -> Option<usize> {
    let mut rest = text;
    loop {
        let (_, _, after) = if_block(rest)?;
        let Some(next) = after.trim_start().strip_prefix("else") else {
            return Some(text.len() - after.len());
        };
        // `elsewhere` opens with the same four letters and is not an `else`.
        if next.starts_with(|c: char| c.is_alphanumeric() || c == '_') {
            return Some(text.len() - after.len());
        }
        let next = next.trim_start();
        if if_block(next).is_some() {
            rest = next;
            continue;
        }
        let close = matching_brace(next)?;
        return Some(text.len() - next.len() + close + 1);
    }
}

/// One chain, arms in the order the script tests them.
fn parse_chain(text: &str, locals: &[String], sets: &[String]) -> Option<Chain> {
    let mut arms = Vec::new();
    let mut rest = text.trim();
    loop {
        let (condition, body, after) = if_block(rest)?;
        let (adds, nested) = parse_body(body, locals, sets)?;
        arms.push(Arm {
            test: Some(parse_test(condition, locals)?),
            adds,
            nested,
        });

        let Some(next) = after.trim_start().strip_prefix("else") else {
            break;
        };
        if next.starts_with(|c: char| c.is_alphanumeric() || c == '_') {
            break;
        }
        let next = next.trim_start();
        if if_block(next).is_some() {
            rest = next;
            continue;
        }
        let close = matching_brace(next)?;
        let (adds, nested) = parse_body(&next[1..close], locals, sets)?;
        if !next[close + 1..].trim().is_empty() {
            return None;
        }
        arms.push(Arm {
            test: None,
            adds,
            nested,
        });
        break;
    }
    Some(Chain { arms })
}

/// An arm's body: adds into the accumulators, and the chains it guards.
fn parse_body(
    body: &str,
    locals: &[String],
    sets: &[String],
) -> Option<(Vec<(usize, String)>, Vec<Chain>)> {
    let mut adds = Vec::new();
    let mut nested = Vec::new();
    for statement in statements(body)? {
        if chain_end(statement).is_some() {
            nested.push(parse_chain(statement, locals, sets)?);
            continue;
        }
        let (receiver, argument) = statement.split_once(".add(")?;
        let at = sets.iter().position(|name| name == receiver.trim())?;
        adds.push((at, quoted(argument.trim().strip_suffix(')')?)?));
    }
    Some((adds, nested))
}

/// The guard on one arm, against the locals the script bound.
fn parse_test(condition: &str, locals: &[String]) -> Option<Test> {
    let condition = condition.trim();
    // The list membership is read FIRST: `['a'].contains(x)` carries
    // `.contains(` too, and reading it as the substring test would take the
    // list's opening bracket for a subject.
    if let Some((list, tail)) = condition.split_once("].contains(") {
        let named = tail.trim().strip_suffix(')')?.trim();
        let subject = locals.iter().position(|name| name == named)?;
        let words = vocabulary_tested(&format!("{list}].contains({named})"), named)?;
        return Some(Test::OneOf { subject, words });
    }
    if let Some((named, tail)) = condition.split_once(".contains(") {
        let subject = locals.iter().position(|name| name == named.trim())?;
        return Some(Test::Holds {
            subject,
            word: quoted(tail.trim().strip_suffix(')')?)?,
        });
    }
    let (named, literal) = condition.split_once("==")?;
    let subject = locals.iter().position(|name| name == named.trim())?;
    Some(Test::Is {
        subject,
        word: quoted(literal.trim())?,
    })
}

/// `def <name> = new HashSet();`, as the local it binds.
fn declares_set(statement: &str) -> Option<String> {
    let (head, tail) = statement.split_once('=')?;
    (tail.trim() == "new HashSet()")
        .then(|| local(head))
        .flatten()
}

/// `def <name> = ctx.<path>[.toLowerCase()];`, as the local and what it reads.
fn declares_subject(statement: &str) -> Option<(String, Subject)> {
    let (head, tail) = statement.split_once('=')?;
    let name = local(head)?;
    let expression = tail.trim();
    let (read, folded) = match expression.strip_suffix(".toLowerCase()") {
        Some(head) => (head, true),
        None => (expression, false),
    };
    let path = clean_path(read.trim().strip_prefix("ctx.")?.trim());
    (!path.is_empty() && !path.contains(char::is_whitespace))
        .then_some((name, Subject { path, folded }))
}

/// `if (!<set>.isEmpty()) { ctx.<path> = <set>; }`, as the set and its target.
fn write_back(statement: &str, sets: &[String]) -> Option<(usize, String)> {
    let (condition, body, after) = if_block(statement)?;
    if !after.trim().is_empty() {
        return None;
    }
    let named = condition
        .trim()
        .strip_prefix('!')?
        .trim()
        .strip_suffix(".isEmpty()")?
        .trim();
    let at = sets.iter().position(|name| name == named)?;
    let (target, written) = body.trim().strip_suffix(';')?.split_once('=')?;
    if written.trim() != named {
        return None;
    }
    let path = clean_path(target.trim().strip_prefix("ctx.")?.trim());
    (!path.is_empty() && !path.contains(char::is_whitespace)).then_some((at, path))
}

/// The name a `def <name>` declaration binds.
fn local(declaration: &str) -> Option<String> {
    let mut words = declaration.split_whitespace();
    let kind = words.next()?;
    let name = words.next()?;
    if words.next().is_some() || !matches!(kind, "def" | "Set" | "HashSet" | "String") {
        return None;
    }
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| name.to_owned())
}

/// The text inside a quoted literal that is the WHOLE of `text`.
///
/// A concatenation or a bare local is not a literal, and taking the first quoted
/// run out of one would add a word the script never adds.
fn quoted(text: &str) -> Option<String> {
    let text = text.trim();
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let inner = text.strip_prefix(quote)?.strip_suffix(quote)?;
    (!inner.is_empty() && !inner.contains(quote)).then(|| inner.to_owned())
}

#[cfg(test)]
#[path = "set_ladder_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
