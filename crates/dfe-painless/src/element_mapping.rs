// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! An ECS block built from ONE element of a vendor list, with the parent maps
//! the script creates for itself.
//!
//! `jamf_protect`'s alerts stream ships three of these, each described by the
//! vendor as "one scripting processor to capture all the related X": the first
//! related file becomes `file.*`, the first related process -- plus the first
//! binary, the second process and the last -- becomes `process.*`, and the
//! first related group becomes `group.*`. Between them they are 150 of that
//! stream's 199 wrong fields, and five more follow for free, because
//! `related.hash` is four `append` processors gated on the `file.hash.*` and
//! `process.hash.*` these write.
//!
//! **This is an interpreter over a closed grammar, not a reader for one
//! script.** The three differ in every detail that matters -- one takes the
//! last element as well as the first, one coerces with `String.valueOf` and
//! another with `.toString()`, one renders an epoch through
//! `Instant.ofEpochSecond` -- so a matcher keyed to any one of them writes a
//! fraction of the other two. A statement outside the grammar declines the
//! WHOLE script: writing part of `process.*` and leaving the rest missing makes
//! the source read as needing polish rather than a different matcher.
//!
//! Three semantics carry most of the parity, and each was settled against the
//! corpus rather than assumed:
//!
//! - **`= new HashMap()` REPLACES and `?: new HashMap()` does not.** The
//!   process script builds `process.group_leader` twice: the first holds `pid`
//!   and `group.id` off the current process, the second replaces it with the
//!   last process in the list, and Elasticsearch's own output carries no
//!   `group_leader.group` because of it. Reading both spellings as
//!   create-if-absent leaves behind a member the vendor deleted.
//! - **Painless will not create a parent**, so a script assigning into
//!   `ctx.file` without building it first is a different script, and one that
//!   throws. A script with no map creation is declined rather than claimed.
//! - **An absent read lands as an explicit null**, which is what Painless
//!   assigns, and the pipeline's own drop-empty prunes it. `String.valueOf` of
//!   one is the four-character word `null` -- Java's rendering, not a
//!   convenience.
//!
//! What this deliberately does NOT reproduce: `.toString()` on an absent value
//! throws in Java and lands the document on the pipeline's `on_failure`. A
//! runner cannot raise from here, so it writes nothing and carries on. Every
//! element in the corpus carries the members the unguarded calls read -- `uid`
//! on a process, `gid` on a group -- so no capture reaches it.

use serde_json::{Map, Value};

use dfe_core::Event;

/// Below this a script is a single-member take, which `FirstElement` and
/// `LastElementMember` already read. Claiming one here would take a script
/// from a matcher that expresses it better.
const MIN_MEMBER_WRITES: usize = 2;

// -- The grammar ---------------------------------------------------------

/// Which element of a list a local names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ElementIndex {
    /// A literal subscript -- `[0]`, `[1]`.
    At(usize),
    /// `[<the same list>.size() - 1]`, which jamf calls the process group
    /// leader and which is the same element as `[0]` on a one-item list.
    Last,
}

/// A local bound to one element of a `ctx.` list.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Local {
    /// The name the script gave it, kept so a second binding of the same name
    /// can be refused.
    name: String,
    /// The `ctx.` path of the list it indexes.
    list: String,
    index: ElementIndex,
}

/// Something the script reads.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Read {
    /// `ctx.<path>`, with `?.` navigation flattened.
    Field(String),
    /// A bound local, and the members walked off it. `?.` and `.` are one walk
    /// here: both answer nothing for an absent member.
    Member { local: usize, members: Vec<String> },
}

/// How a read is rendered before it lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Render {
    /// Assigned as it stands.
    Direct,
    /// `String.valueOf(<read>)`.
    StringValueOf,
    /// `<read>.toString()`.
    ToString,
    /// `Instant.ofEpochSecond(<read>).toString()`.
    EpochSecond,
    /// `<read> ?: new ArrayList()`.
    OrEmptyList,
}

/// The condition of an `if`, or one conjunct of it.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Test {
    /// `<read> != null`.
    NotNull(Read),
    /// `ctx.<list>.size() > <least>`.
    Longer { list: String, least: usize },
    /// `<local>.containsKey('<member>')`, which an explicit null still passes.
    ContainsKey { local: usize, member: String },
    /// Every conjunct holds.
    All(Vec<Test>),
}

/// One statement of the restricted Painless this module reads.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Stmt {
    /// `ctx.<path> = new HashMap();` -- REPLACES whatever is there.
    ResetMap(String),
    /// `ctx.<path> = ctx.<path> ?: new HashMap();` -- creates only where the
    /// path holds nothing.
    EnsureMap(String),
    /// `ctx.<path> = <render applied to read>;`
    Write {
        path: String,
        read: Read,
        render: Render,
    },
    /// `if (<test>) { <body> }`. No `else` arm: none of the three has one, and
    /// reading the `if` half of one writes the wrong branch on every event
    /// that takes the other.
    If { test: Test, body: Vec<Stmt> },
}

/// The whole script: the elements it binds, and the statements that write off
/// them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementMapping {
    locals: Vec<Local>,
    body: Vec<Stmt>,
}

impl ElementMapping {
    /// Whether the script builds a parent map for itself.
    ///
    /// Painless throws on `ctx.a.b = x` with `ctx.a` absent, so a script
    /// carrying no creation cannot be one of these.
    fn creates_a_parent(&self) -> bool {
        fn walk(body: &[Stmt]) -> bool {
            body.iter().any(|stmt| match stmt {
                Stmt::ResetMap(_) | Stmt::EnsureMap(_) => true,
                Stmt::If { body, .. } => walk(body),
                Stmt::Write { .. } => false,
            })
        }
        walk(&self.body)
    }

    /// How many writes take their value off a bound element.
    fn member_writes(&self) -> usize {
        fn walk(body: &[Stmt]) -> usize {
            body.iter()
                .map(|stmt| match stmt {
                    Stmt::Write {
                        read: Read::Member { .. },
                        ..
                    } => 1,
                    Stmt::If { body, .. } => walk(body),
                    Stmt::ResetMap(_) | Stmt::EnsureMap(_) | Stmt::Write { .. } => 0,
                })
                .sum()
        }
        walk(&self.body)
    }
}

// -- Reading the script --------------------------------------------------

/// A cursor over the normalised script text.
struct Reader<'a> {
    text: &'a str,
    at: usize,
    locals: Vec<Local>,
}

impl<'a> Reader<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            at: 0,
            locals: Vec::new(),
        }
    }

    fn rest(&self) -> &'a str {
        &self.text[self.at..]
    }

    /// Whitespace and line comments, which the file capture carries mid-line.
    fn trivia(&mut self) {
        loop {
            let trimmed = self.rest().trim_start();
            self.at = self.text.len() - trimmed.len();
            if !trimmed.starts_with("//") {
                return;
            }
            self.at += trimmed.find('\n').unwrap_or(trimmed.len());
        }
    }

    /// Consume `token` where it sits at the cursor, trivia not skipped.
    fn eat(&mut self, token: &str) -> bool {
        if self.rest().starts_with(token) {
            self.at += token.len();
            return true;
        }
        false
    }

    /// Consume `token`, and the trivia before it.
    ///
    /// Punctuation only: a keyword needs [`Self::keyword`], which will not
    /// read `def` out of `default`.
    fn take(&mut self, token: &str) -> bool {
        self.trivia();
        self.eat(token)
    }

    /// Consume `word` where the identifier ENDS there.
    fn keyword(&mut self, word: &str) -> bool {
        self.trivia();
        let rest = self.rest();
        if !rest.starts_with(word)
            || rest[word.len()..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            return false;
        }
        self.at += word.len();
        true
    }

    /// The identifier at the cursor, without consuming it.
    fn peek_ident(&self) -> Option<&'a str> {
        let rest = self.rest().trim_start();
        if !rest.starts_with(|c: char| c.is_alphabetic() || c == '_') {
            return None;
        }
        let end = rest
            .find(|c: char| !(c.is_alphanumeric() || c == '_'))
            .unwrap_or(rest.len());
        Some(&rest[..end])
    }

    fn ident(&mut self) -> Option<&'a str> {
        self.trivia();
        let word = self.peek_ident()?;
        self.at += word.len();
        Some(word)
    }

    fn number(&mut self) -> Option<usize> {
        self.trivia();
        let rest = self.rest();
        let end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        let value: usize = rest[..end].parse().ok()?;
        self.at += end;
        Some(value)
    }

    /// The body of a `'...'` or `"..."` literal.
    fn quoted(&mut self) -> Option<&'a str> {
        self.trivia();
        let rest = self.rest();
        let quote = rest.chars().next().filter(|c| *c == '\'' || *c == '"')?;
        let end = rest[1..].find(quote)? + 1;
        self.at += end + 1;
        Some(&rest[1..end])
    }

    // -- Paths and values ------------------------------------------------

    /// `ctx` then its dotted path, `?.` flattened.
    ///
    /// A segment IMMEDIATELY followed by `(` is a method call rather than a
    /// member, so `.size()` and `.toString()` end the path and stay unread for
    /// whoever asked next.
    fn ctx_path(&mut self) -> Option<String> {
        if !self.keyword("ctx") {
            return None;
        }
        let mut path = String::new();
        while let Some(segment) = self.next_segment() {
            if !path.is_empty() {
                path.push('.');
            }
            path.push_str(segment);
        }
        (!path.is_empty()).then_some(path)
    }

    /// The next `.member` / `?.member` of a walk, or nothing where it ends.
    fn next_segment(&mut self) -> Option<&'a str> {
        let save = self.at;
        self.trivia();
        if !(self.eat("?.") || self.eat(".")) {
            self.at = save;
            return None;
        }
        let Some(segment) = self.ident() else {
            self.at = save;
            return None;
        };
        if self.rest().starts_with('(') {
            self.at = save;
            return None;
        }
        Some(segment)
    }

    /// A `ctx.` path, or a bound local and the members walked off it.
    fn read(&mut self) -> Option<Read> {
        self.trivia();
        if self.peek_ident() == Some("ctx") {
            return self.ctx_path().map(Read::Field);
        }
        let name = self.ident()?;
        let local = self.locals.iter().position(|held| held.name == name)?;
        let mut members = Vec::new();
        while let Some(segment) = self.next_segment() {
            members.push(segment.to_owned());
        }
        Some(Read::Member { local, members })
    }

    /// `ctx.<list>[<index>]`, the take a `def` binds.
    fn element(&mut self) -> Option<(String, ElementIndex)> {
        let list = self.ctx_path()?;
        if !self.take("[") {
            return None;
        }
        let index = self.subscript(&list)?;
        self.take("]").then_some((list, index))
    }

    /// A literal subscript, or the script's own `size() - 1`.
    ///
    /// The `size()` has to be over the SAME list. A count taken off another
    /// list indexes this one by a length that has nothing to do with it.
    fn subscript(&mut self, list: &str) -> Option<ElementIndex> {
        if let Some(literal) = self.number() {
            return Some(ElementIndex::At(literal));
        }
        let counted = self.ctx_path()?;
        if counted != list || !self.size_call() || !self.take("-") {
            return None;
        }
        // Only the last element. A `size() - 2` is a different take, and this
        // has no vendor spelling to check one against.
        (self.number()? == 1).then_some(ElementIndex::Last)
    }

    /// `.size()` where it sits next.
    fn size_call(&mut self) -> bool {
        self.method("size")
    }

    /// `.<name>()` where it sits next.
    fn method(&mut self, name: &str) -> bool {
        let save = self.at;
        if self.take(".") && self.keyword(name) && self.take("(") && self.take(")") {
            return true;
        }
        self.at = save;
        false
    }

    /// `<owner>.<name>(` where it sits next -- the opening of a static call.
    fn static_call(&mut self, owner: &str, name: &str) -> bool {
        let save = self.at;
        if self.keyword(owner) && self.take(".") && self.keyword(name) && self.take("(") {
            return true;
        }
        self.at = save;
        false
    }

    /// `new <name>()` where it sits next.
    fn construct(&mut self, name: &str) -> bool {
        let save = self.at;
        if self.keyword("new") && self.keyword(name) && self.take("(") && self.take(")") {
            return true;
        }
        self.at = save;
        false
    }

    // -- Statements ------------------------------------------------------

    /// Statements until the block's closing brace or the end of the script.
    fn block(&mut self) -> Option<Vec<Stmt>> {
        let mut body = Vec::new();
        loop {
            self.trivia();
            if self.rest().is_empty() || self.rest().starts_with('}') {
                return Some(body);
            }
            if let Some(stmt) = self.statement()? {
                body.push(stmt);
            }
        }
    }

    /// One statement. `None` declines the script; `Some(None)` is a `def`,
    /// which binds a local and runs nothing per event.
    fn statement(&mut self) -> Option<Option<Stmt>> {
        if self.keyword("if") {
            return self.conditional().map(Some);
        }
        if self.keyword("def") {
            return self.binding().map(|()| None);
        }
        let path = self.ctx_path()?;
        // One `=`, so `==`, `!=` and the compound assignments all decline.
        if !self.take("=") || self.rest().starts_with('=') {
            return None;
        }
        let stmt = self.assignment(path)?;
        self.take(";").then_some(Some(stmt))
    }

    fn conditional(&mut self) -> Option<Stmt> {
        if !self.take("(") {
            return None;
        }
        let test = self.test()?;
        if !self.take(")") || !self.take("{") {
            return None;
        }
        let body = self.block()?;
        if !self.take("}") || self.peek_ident() == Some("else") {
            return None;
        }
        Some(Stmt::If { test, body })
    }

    /// `def <name> = ctx.<list>[<index>];`
    fn binding(&mut self) -> Option<()> {
        let name = self.ident()?.to_owned();
        if !self.take("=") {
            return None;
        }
        let (list, index) = self.element()?;
        if !self.take(";") || self.locals.iter().any(|held| held.name == name) {
            return None;
        }
        self.locals.push(Local { name, list, index });
        Some(())
    }

    /// Everything to the right of a `ctx.<path> =`.
    fn assignment(&mut self, path: String) -> Option<Stmt> {
        if self.construct("HashMap") {
            return Some(Stmt::ResetMap(path));
        }
        if let Some(stmt) = self.ensure_map(&path) {
            return Some(stmt);
        }
        if self.static_call("String", "valueOf") {
            let read = self.read()?;
            return self.take(")").then_some(Stmt::Write {
                path,
                read,
                render: Render::StringValueOf,
            });
        }
        if self.static_call("Instant", "ofEpochSecond") {
            let read = self.read()?;
            if !self.take(")") || !self.method("toString") {
                return None;
            }
            return Some(Stmt::Write {
                path,
                read,
                render: Render::EpochSecond,
            });
        }
        let read = self.read()?;
        Some(Stmt::Write {
            path,
            read,
            render: self.trailing_render()?,
        })
    }

    /// `ctx.<the same path> ?: new HashMap()`.
    ///
    /// Both sides must name the SAME path: `ctx.a = ctx.b ?: new HashMap()`
    /// copies `b` where it is there, which is a different statement.
    fn ensure_map(&mut self, path: &str) -> Option<Stmt> {
        let save = self.at;
        if self.peek_ident() == Some("ctx")
            && let Some(other) = self.ctx_path()
            && other == path
            && self.take("?:")
            && self.construct("HashMap")
        {
            return Some(Stmt::EnsureMap(path.to_owned()));
        }
        self.at = save;
        None
    }

    /// The coercion trailing a read, if any.
    fn trailing_render(&mut self) -> Option<Render> {
        if self.method("toString") {
            return Some(Render::ToString);
        }
        if self.take("?:") {
            // A default this cannot name is a value this cannot write.
            return self.construct("ArrayList").then_some(Render::OrEmptyList);
        }
        Some(Render::Direct)
    }

    // -- Conditions ------------------------------------------------------

    /// A conjunction. `||` is outside the grammar: a disjunction changes which
    /// half of the body runs, and none of the three spells one.
    fn test(&mut self) -> Option<Test> {
        let mut atoms = vec![self.atom()?];
        while self.take("&&") {
            atoms.push(self.atom()?);
        }
        Some(if atoms.len() == 1 {
            atoms.pop()?
        } else {
            Test::All(atoms)
        })
    }

    fn atom(&mut self) -> Option<Test> {
        if let Some(test) = self.contains_key() {
            return Some(test);
        }
        let read = self.read()?;
        if let Read::Field(list) = &read
            && self.size_call()
        {
            let list = list.clone();
            // `>` alone. `>=` counts one element fewer and reads the same.
            if !self.take(">") || self.rest().starts_with('=') {
                return None;
            }
            return Some(Test::Longer {
                list,
                least: self.number()?,
            });
        }
        if !self.take("!=") {
            return None;
        }
        self.keyword("null").then_some(Test::NotNull(read))
    }

    /// `<local>.containsKey('<member>')`.
    fn contains_key(&mut self) -> Option<Test> {
        let save = self.at;
        let local = self
            .peek_ident()
            .and_then(|name| self.locals.iter().position(|held| held.name == name));
        if let Some(local) = local
            && self.ident().is_some()
            && self.take(".")
            && self.keyword("containsKey")
            && self.take("(")
            && let Some(member) = self.quoted()
        {
            let member = member.to_owned();
            if self.take(")") {
                return Some(Test::ContainsKey { local, member });
            }
        }
        self.at = save;
        None
    }
}

/// Read the whole script, or decline it.
///
/// Declines on the first statement outside the grammar, and on any text left
/// over after the block: a partial read writes part of an ECS block and leaves
/// the rest missing, which is the failure this module exists to avoid.
#[must_use]
pub fn parse_element_mapping(script: &str) -> Option<ElementMapping> {
    let mut reader = Reader::new(script);
    let body = reader.block()?;
    reader.trivia();
    if !reader.rest().is_empty() || reader.locals.is_empty() {
        return None;
    }
    let mapping = ElementMapping {
        locals: reader.locals,
        body,
    };
    (mapping.creates_a_parent() && mapping.member_writes() >= MIN_MEMBER_WRITES).then_some(mapping)
}

// -- Running it ----------------------------------------------------------

/// Apply the mapping, or leave the document alone.
///
/// Always claims the script: the guards are the vendor's own, so a batch whose
/// events carry no such list is a script that correctly wrote nothing rather
/// than a matcher that declined.
pub fn element_mapping(event: &mut Event, pattern: &ElementMapping) -> bool {
    run_block(event, pattern, &pattern.body);
    true
}

fn run_block(event: &mut Event, pattern: &ElementMapping, body: &[Stmt]) {
    for stmt in body {
        match stmt {
            Stmt::ResetMap(path) => {
                let _ = event.set(path, Value::Object(Map::new()));
            }
            Stmt::EnsureMap(path) => {
                if !event.has_value(path) {
                    let _ = event.set(path, Value::Object(Map::new()));
                }
            }
            Stmt::Write { path, read, render } => {
                // Cloned before the write, because the read borrows the same
                // document the write mutates -- and it is the LEAF rather than
                // the element, so the cost is one scalar an assignment.
                let value = resolve(event, pattern, read).cloned();
                if let Some(value) = render.apply(value) {
                    let _ = event.set(path, value);
                }
            }
            Stmt::If { test, body } => {
                if holds(event, pattern, test) {
                    run_block(event, pattern, body);
                }
            }
        }
    }
}

/// The element a local names, or nothing where the list is absent or short.
fn element<'a>(event: &'a Event, local: &Local) -> Option<&'a Value> {
    let items = event.get(&local.list)?.as_array()?;
    let index = match local.index {
        ElementIndex::At(literal) => literal,
        ElementIndex::Last => items.len().checked_sub(1)?,
    };
    items.get(index)
}

fn resolve<'a>(event: &'a Event, pattern: &ElementMapping, read: &Read) -> Option<&'a Value> {
    match read {
        Read::Field(path) => event.get(path),
        Read::Member { local, members } => {
            let mut current = element(event, pattern.locals.get(*local)?)?;
            for member in members {
                current = current.get(member.as_str())?;
            }
            Some(current)
        }
    }
}

fn holds(event: &Event, pattern: &ElementMapping, test: &Test) -> bool {
    match test {
        Test::NotNull(read) => !matches!(resolve(event, pattern, read), None | Some(Value::Null)),
        Test::Longer { list, least } => event
            .get(list)
            .and_then(Value::as_array)
            .is_some_and(|items| items.len() > *least),
        Test::ContainsKey { local, member } => pattern
            .locals
            .get(*local)
            .and_then(|local| element(event, local))
            .and_then(Value::as_object)
            .is_some_and(|entries| entries.contains_key(member)),
        Test::All(tests) => tests.iter().all(|test| holds(event, pattern, test)),
    }
}

impl Render {
    /// The value that lands, or nothing where the script writes none.
    fn apply(self, value: Option<Value>) -> Option<Value> {
        match self {
            // Painless assigns null for an absent read rather than skipping
            // the statement, and the pipeline's own drop-empty prunes it.
            Self::Direct => Some(value.unwrap_or(Value::Null)),
            Self::StringValueOf => match value {
                None => Some(Value::String("null".to_owned())),
                Some(read) => java_string(&read).map(Value::String),
            },
            // Java throws on a null receiver. Writing nothing is the closest a
            // runner gets; see the module docs.
            Self::ToString => match value {
                None | Some(Value::Null) => None,
                Some(read) => java_string(&read).map(Value::String),
            },
            Self::EpochSecond => value.as_ref().and_then(epoch_second).map(Value::String),
            Self::OrEmptyList => Some(
                value
                    .filter(|held| !held.is_null())
                    .unwrap_or_else(|| Value::Array(Vec::new())),
            ),
        }
    }
}

/// Java's rendering of a scalar.
///
/// A container answers nothing: Java writes `[a, b]` for a list and `{k=v}`
/// for a map, no vendor call site here reads one, and inventing a rendering
/// would be a guess that leaves no error behind.
fn java_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => Some("null".to_owned()),
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        Value::Array(_) | Value::Object(_) => None,
    }
}

/// `Instant.ofEpochSecond(<seconds>).toString()`.
///
/// `ofEpochSecond` takes a long, so the nanoseconds are always zero and the
/// rendering is always seconds-precision -- Java 12 and later print the
/// seconds field whether or not it is zero.
fn epoch_second(value: &Value) -> Option<String> {
    let seconds = match value {
        Value::Number(number) => number
            .as_i64()
            .or_else(|| number.as_f64().map(|s| s as i64))?,
        Value::String(text) => text.trim().parse::<i64>().ok()?,
        Value::Null | Value::Bool(_) | Value::Array(_) | Value::Object(_) => return None,
    };
    Some(
        chrono::DateTime::from_timestamp(seconds, 0)?
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string(),
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "element_mapping_tests.rs"]
mod tests;
