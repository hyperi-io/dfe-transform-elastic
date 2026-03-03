use std::{collections::HashMap, error::Error, ops::Deref, rc::Rc};

use antlr_rust::{
    parser::ParserNodeType,
    tree::{ParseTree, ParseTreeVisitor, Tree, VisitChildren},
};
use anyhow::{anyhow, Context};
use lazy_static::lazy_static;
use regex::Regex;
use tracing::instrument;

use crate::pipeline::script_template::ScriptTemplate;

use super::parser::{painlessparser::*, painlessparservisitor::PainlessParserVisitor};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: String,
}

#[derive(Debug)]
pub struct Transpiler {
    pub program: Option<anyhow::Result<String>>,
    pub functions: HashMap<String, Function>,
    pub control_flow_flags: u32,
    pub error_reported: bool,
    pub root: bool,
}

impl Default for Transpiler {
    fn default() -> Self {
        Transpiler {
            program: None,
            functions: HashMap::new(),
            control_flow_flags: 0,
            error_reported: false,
            root: true,
        }
    }
}

impl Transpiler {
    fn visit(&mut self, node: &<PainlessParserContextType as ParserNodeType>::Type) -> anyhow::Result<String> {
        self.visit_node(node);
        self.program.take().expect("tried to take an empty value")
    }

    fn returns(&mut self, result: anyhow::Result<String>) {
        assert!(
            self.program.is_none(),
            "return slot already contains value {:#?}",
            self.program
        );

        if let Err(err) = &result {
            if !self.error_reported {
                tracing::error!(error = err.as_ref() as &dyn Error);
                self.error_reported = true;
            }
        };

        self.program.replace(result);
    }

    fn delegate(&mut self, node: &<PainlessParserContextType as ParserNodeType>::Type) {
        self.visit_node(node);
    }
}

impl<'input> ParseTreeVisitor<'input, PainlessParserContextType> for Transpiler {}

lazy_static! {
    static ref INITIALISER_PATTERN: Regex = Regex::new("def(?P<id>[a-zA-Z0-9_]+)=0").unwrap();
    // The type widening is making this get out of hand
    static ref CONDITION_PATTERN: Regex = Regex::new(r#"(?s)left = (?<id>[^\s]+).*<.*right = \{temp = (?<target>[^\n]+)"#).unwrap();
}

pub enum CombinedStatementContext<'a> {
    RStatementContext(Rc<RstatementContextAll<'a>>),
    DStatementContext(Rc<DstatementContextAll<'a>>),
}

impl<'a> From<Rc<RstatementContextAll<'a>>> for CombinedStatementContext<'a> {
    fn from(value: Rc<RstatementContextAll<'a>>) -> Self {
        Self::RStatementContext(value)
    }
}

impl<'a> From<Rc<DstatementContextAll<'a>>> for CombinedStatementContext<'a> {
    fn from(value: Rc<DstatementContextAll<'a>>) -> Self {
        Self::DStatementContext(value)
    }
}

impl<'a> From<Rc<StatementContextAll<'a>>> for CombinedStatementContext<'a> {
    fn from(value: Rc<StatementContextAll<'a>>) -> Self {
        if let Some(rstatement) = value.rstatement() {
            return Self::RStatementContext(rstatement);
        }

        if let Some(dstatement) = value.dstatement() {
            return Self::DStatementContext(dstatement);
        }

        unreachable!()
    }
}

impl Transpiler {
    /** visit_statements is for special handling of return / continue statements. We need to intercept these
      at this high level so we can adjust the control flow appropriately that would not be possible at the
      visit_if node
    */
    #[instrument(skip_all)]
    pub fn visit_statements<'i, Iter>(&mut self, iter: &mut Iter) -> anyhow::Result<String>
    where
        Iter: Iterator,
        <Iter as Iterator>::Item: Into<CombinedStatementContext<'i>>,
    {
        let mut output_statements = Vec::<String>::new();

        // if statements need to consume the rest of the iterator so we can't use a regular for loop
        while let Some(statement) = iter.next() {
            match statement.into() {
                CombinedStatementContext::DStatementContext(dstatement) => {
                    output_statements.push(self.visit(&*dstatement)?);
                }
                CombinedStatementContext::RStatementContext(rstatement) => {
                    match rstatement.deref() {
                        // IF LP expression RP trailer (ELSE trailer | { _input.LA(1) != ELSE }? )
                        RstatementContextAll::IfContext(ctx) => {
                            // We only want to declare the control variables at the root
                            let mut reset_root = false;

                            if self.root {
                                reset_root = true;
                                self.root = false;
                            }

                            let condition = self.visit(&*ctx.expression().unwrap())?;
                            let trailers = ctx.trailer_all();

                            let initial_skip_id = self.control_flow_flags;

                            let first = self.visit(&*trailers[0])?;
                            let second = trailers.get(1).map(|second| self.visit(&**second)).transpose()?;

                            let final_skip_id = self.control_flow_flags;

                            if reset_root {
                                self.root = true;
                            }

                            let rest: String = self.visit_statements(iter)?;

                            let statements = ScriptTemplate::render(
                                r#"
                                {{ if .root }}{{ range .ids }}
                                _shouldSkip_{{ . }} = false
                                {{ end }}{{ end }}
                                {{ if and .ids .rest -}} return_{{ index .ids 0 }} = {{ end -}} if {{ .condition }} {{ .first }} {{ if .second }} else {{ .second }} {{ end }}
                                {{ if and .ids .rest }}if !({{ range .ids }} _shouldSkip_{{ . }} || {{ end }} false) { {{ end }}
                                    {{ .rest }}
                                {{ if and .ids .rest }} } else {
                                    return_{{ index .ids 0 }}
                                }
                                {{ end }}
                            "#,
                                HashMap::<String, gtmpl::Value>::from([
                                    (
                                        "ids".into(),
                                        ((initial_skip_id + 1)..=final_skip_id).collect::<Vec<_>>().into(),
                                    ),
                                    ("condition".into(), condition.into()),
                                    ("first".into(), first.into()),
                                    ("second".into(), second.into()),
                                    ("rest".into(), rest.into()),
                                    ("root".into(), self.root.into()),
                                    ("newline".into(), "\n".into()),
                                ]),
                            )?
                            .0;

                            output_statements.push(statements);
                        }
                        other => output_statements.push(self.visit(other)?),
                    }
                }
            }
        }

        Ok(output_statements.join("\n"))
    }
}

/**
 VisitorCompat forces a uniform return type across the entire AST meaning we have to do things
 stringly typed which is gross. Might be worth considering eschewing the generic visitor for a
 custom per node traversal that could have heterogeneous return types, need to figure out how it's
 doing its trait object downcasting first.

 Most of visitor nodes are `unimplemented!("{}", ctx.get_text())` because we want to fail early when we encounter
 nodes that we haven't explicitly supported transforming to VRL yet.
*/
impl<'input> PainlessParserVisitor<'input> for Transpiler {
    // Grammar: function* statement* EOF
    #[instrument(skip_all)]
    fn visit_source(&mut self, ctx: &SourceContext<'_>) {
        let result = try {
            self.functions = ctx
                .function_all()
                .into_iter()
                .map(|it| {
                    // Grammar: decltype ID parameters block
                    let name = it.ID().unwrap().get_text();

                    let body = self.visit(&*it.block().unwrap())?;

                    // Grammar: LP (decltype ID (COMMA decltype ID)*)? RP
                    let args = it
                        .parameters()
                        .unwrap()
                        .ID_all()
                        .into_iter()
                        .map(|param| param.get_text())
                        .filter(|param| !matches!(param.as_str(), "(" | ")" | ","))
                        .collect();

                    Ok((name.clone(), Function { name, body, args }))
                })
                .collect::<anyhow::Result<HashMap<_, _>>>()?;

            self.visit_statements(&mut ctx.statement_all().into_iter())?
        };

        self.returns(result)
    }

    // Grammar: decltype ID parameters block
    #[instrument(skip_all)]
    fn visit_function(&mut self, ctx: &FunctionContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_function {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_parameters(&mut self, ctx: &ParametersContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_parameters {}", ctx.get_text())));
    }

    /**
     Grammar: (rstatement | dstatement (SEMICOLON | EOF))
     VRL doesn't have statements, so we don't care about semicolons
     and can just grab the first child
    */
    #[instrument(skip_all)]
    fn visit_statement(&mut self, ctx: &StatementContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    // Grammar: IF LP expression RP trailer (ELSE trailer | { _input.LA(1) != ELSE }? )
    #[instrument(skip_all)]
    fn visit_if(&mut self, ctx: &IfContext<'_>) {
        let result = try {
            let condition = self.visit(&*ctx.expression().unwrap())?;
            let trailers = ctx.trailer_all();

            match trailers.len() {
                1 => {
                    format!("if {condition} {trailer}", trailer = self.visit(&*trailers[0])?)
                }
                2 => {
                    format!(
                        "if {condition} {first} else {second}",
                        first = self.visit(&*trailers[0])?,
                        second = self.visit(&*trailers[1])?
                    )
                }
                len => Err(anyhow!(
                    "encountered if statement with unsupported number of trailers `{len}"
                ))?,
            }
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_while(&mut self, ctx: &WhileContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_while {}", ctx.get_text())));
    }

    // FOR LP initializer? SEMICOLON expression? SEMICOLON afterthought? RP (trailer | empty)
    #[instrument(skip_all)]
    fn visit_for(&mut self, ctx: &ForContext<'_>) {
        let result = try {
            let variable_id = INITIALISER_PATTERN
                .captures(
                    &ctx.initializer()
                        .ok_or_else(|| anyhow!("for loops must contain an initialiser"))?
                        .get_text(),
                )
                .ok_or_else(|| anyhow!("for loop initialiser failed to match expected pattern"))?
                .get(1)
                .ok_or_else(|| anyhow!("failed to extract initialiser variable from initialiser"))?
                .as_str()
                .to_owned();

            let expression_text = self.visit(&*ctx.expression().unwrap())?;

            println!("{expression_text}");

            let expression = CONDITION_PATTERN
                .captures(&expression_text)
                .ok_or_else(|| anyhow!("for loop condition failed to match expected pattern: {expression_text}"))?;

            if variable_id != expression.get(1).unwrap().as_str() {
                Err(anyhow!("condition variable did not match initialiser variable"))?
            }

            let target = expression.get(2).unwrap().as_str();

            let afterthought = ctx.afterthought().unwrap().get_text();
            if afterthought != format!("{variable_id}++") {
                Err(anyhow!(
                    "afterthought must be a post increment of the initialiser variable"
                ))?
            }

            format!(
                "for_each(array!({target})) -> |{variable_id}, _value|  {{ {trailer} }}",
                trailer = self.visit(&*ctx.trailer().unwrap())?
            )
        };

        self.returns(result);
    }

    // FOR LP decltype ID COLON expression RP trailer
    #[instrument(skip_all)]
    fn visit_each(&mut self, ctx: &EachContext<'_>) {
        let result = try {
            let identifier = ctx.ID().unwrap().get_text();
            let expression = self.visit(&*ctx.expression().unwrap())?;
            let trailer = self.visit(&*ctx.trailer().unwrap())?;

            format!(
                "for_each({{\narr = {expression}\nif false {{ arr = null }}\narray!(arr)}}) -> |_index, {identifier}| \
                 {trailer}"
            )
        };

        self.returns(result);
    }

    // Grammar: FOR LP ID IN expression RP trailer
    #[instrument(skip_all)]
    fn visit_ineach(&mut self, ctx: &IneachContext<'_>) {
        let result = try {
            let identifier = ctx.ID().unwrap().get_text();
            let expression = self.visit(&*ctx.expression().unwrap())?;
            let trailer = self.visit(&*ctx.trailer().unwrap())?;

            format!("for_each(array!({{ expr = {expression}\nif false {{ expr = null }}\nexpr}})) -> |_index, {identifier}| {{{trailer}}}")
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_try(&mut self, ctx: &TryContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_try {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_do(&mut self, ctx: &DoContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_do {}", ctx.get_text())));
    }

    // Grammar: declaration
    #[instrument(skip_all)]
    fn visit_decl(&mut self, ctx: &DeclContext<'_>) {
        assert!(ctx.get_child_count() == 1);
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    #[instrument(skip_all)]
    fn visit_continue(&mut self, ctx: &ContinueContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_continue {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_break(&mut self, ctx: &BreakContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_break {}", ctx.get_text())));
    }

    // RETURN expression?
    #[instrument(skip_all)]
    fn visit_return(&mut self, ctx: &ReturnContext<'_>) {
        let result = try {
            let expr = ctx
                .expression()
                .map(|expr| self.visit(&*expr))
                .transpose()?
                .unwrap_or_else(|| "null".into());

            self.control_flow_flags += 1;

            format!("_shouldSkip_{id} = true\n{expr}", id = self.control_flow_flags)
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_throw(&mut self, ctx: &ThrowContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_throw {}", ctx.get_text())));
    }

    // Grammar: expression
    #[instrument(skip_all)]
    fn visit_expr(&mut self, ctx: &ExprContext<'_>) {
        assert_eq!(1, ctx.get_child_count());
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    // Grammar: block | statement
    #[instrument(skip_all)]
    fn visit_trailer(&mut self, ctx: &TrailerContext<'_>) {
        assert!(1 == ctx.get_child_count());

        self.delegate(&*ctx.get_child(0).unwrap());
    }

    // Grammar: LBRACK statement* dstatement? RBRACK
    #[instrument(skip_all)]
    fn visit_block(&mut self, ctx: &BlockContext<'_>) {
        let result = try {
            let mut statements = ctx
                .statement_all()
                .into_iter()
                .map(CombinedStatementContext::from)
                .chain(ctx.dstatement().into_iter().map(CombinedStatementContext::from));

            let statements = self.visit_statements(&mut statements)?;

            format!("{{ {statements}\n }}")
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_empty(&mut self, ctx: &EmptyContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_empty {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_initializer(&mut self, ctx: &InitializerContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_initializer {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_afterthought(&mut self, ctx: &AfterthoughtContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_afterthought {}", ctx.get_text())));
    }

    // Grammar: decltype declvar (COMMA declvar)*
    #[instrument(skip_all)]
    fn visit_declaration(&mut self, ctx: &DeclarationContext<'_>) {
        if ctx.get_child_count() > 2 {
            self.returns(Err(anyhow!(
                "declaration currently only supports a single declaration at a time"
            )));
            return;
        }

        self.delegate(&*ctx.get_child(1).unwrap());
    }

    // No idea what the repeating LBRACE RBRACE is supposed to represent
    // Grammar: typeid (LBRACE RBRACE)*
    #[instrument(skip_all)]
    fn visit_decltype(&mut self, ctx: &DecltypeContext<'_>) {
        self.returns(Ok(ctx.typeid().unwrap().get_text()))
    }

    // Grammar: (DEF | PRIMITIVE | ID (DOT DOTID)*)
    #[instrument(skip_all)]
    fn visit_typeid(&mut self, ctx: &TypeidContext<'_>) {
        self.returns(Ok(ctx.get_text()))
    }

    // Grammar: ID (ASSIGN expression)?
    #[instrument(skip_all)]
    fn visit_declvar(&mut self, ctx: &DeclvarContext<'_>) {
        let result = try {
            let child_count = ctx.get_child_count();
            if child_count != 3 {
                Err(anyhow!("declvar must have 3 children, found {child_count}"))?;
            }

            let identifier = ctx.ID().unwrap().get_text();
            let expression = self.visit(&*ctx.expression().unwrap())?;

            format!("{identifier} = {expression}")
        };

        self.returns(result)
    }

    #[instrument(skip_all)]
    fn visit_trap(&mut self, ctx: &TrapContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_trap {}", ctx.get_text())));
    }

    // Grammar: unary
    #[instrument(skip_all)]
    fn visit_single(&mut self, ctx: &SingleContext<'_>) {
        assert_eq!(1, ctx.get_child_count());
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    /*
       Grammar: noncondexpression (LT | LTE | GT | GTE) noncondexpression
                noncondexpression (EQ | EQR | NE | NER) noncondexpression
    */
    #[instrument(skip_all)]
    fn visit_comp(&mut self, ctx: &CompContext<'_>) {
        let result = try {
            let exprs = ctx.get_children().collect::<Vec<_>>();
            assert!(exprs.len() == 3);

            let mut left = self.visit(&*exprs[0])?;
            let mut right = self.visit(&*exprs[2])?;

            match exprs[1].get_text().as_str() {
                // EQ | NE don't require coercions and doing so actually causes incorrect behaviour where null == 0
                // because to_int!(null) == 0 instead of an error
                op @ ("==" | "!=") => {
                    format!("{left} {op} {right}")
                }
                op => {
                    left = format!("{{\nleft = {left}\nif false {{left=null}}\nleft}}");
                    right = format!("{{\nright = {right}\nif false {{right=null}}\nright}}");

                    format!("int!({left}) {op} int!({right})")
                }
            }
        };

        self.returns(result);
    }

    /*
        Grammar: noncondexpression BOOLAND noncondexpression
                 noncondexpression BOOLOR  noncondexpression
    */
    #[instrument(skip_all)]
    fn visit_bool(&mut self, ctx: &BoolContext<'_>) {
        let exprs = ctx.get_children().collect::<Vec<_>>();
        assert!(exprs.len() == 3);

        let result: anyhow::Result<String> = try {
            let left = self.visit(&*exprs[0])?;
            let right = self.visit(&*exprs[2])?;

            format!("{left} {} {right}", exprs[1].get_text(),)
        };

        self.returns(result);
    }

    /*
       Grammar: noncondexpression (MUL | DIV | REM) noncondexpression
                noncondexpression (ADD | SUB) noncondexpression
                noncondexpression (FIND | MATCH) noncondexpression
                noncondexpression (LSH | RSH | USH) noncondexpression
                noncondexpression (BWAND) noncondexpression
                noncondexpression (XOR) noncondexpression
                noncondexpression (BWOR) noncondexpression
    */
    #[instrument(skip_all)]
    fn visit_binary(&mut self, ctx: &BinaryContext<'_>) {
        let result = try {
            let children = ctx.get_children().collect::<Vec<_>>();
            if children.len() != 3 {
                Err(anyhow!(
                    "found {} children, visit_binary must have 3 children",
                    children.len()
                ))?;
            }

            let left = self.visit(&*children[0])?;
            let right = self.visit(&*children[2])?;

            match children[1].get_text().as_str() {
                op @ ("*" | "/" | "-" | "+") => {
                    ScriptTemplate::render(
                        r#"{
                                expr, err = {
                                    expr = {{ .left }}
                                    if false {
                                        expr = null
                                    }
                                    expr {{ .op }} {{ .right }}
                                }
                                assert!(err == null, to_string(err))
                                {{ if eq .op "/" }}
                                    to_int(expr)
                                {{ else }}
                                    expr
                                {{ end }}
                            }"#,
                        HashMap::<String, gtmpl::Value>::from([
                            ("left".into(), left.into()),
                            ("op".into(), op.into()),
                            ("right".into(), right.into()),
                        ]),
                    )?
                    .0
                }
                "&" => {
                    // It is very difficult to implement a general & operator without binary operations
                    // some special cases can be implemented like `& 0x7 (0b0000_0111)`` as `% 8 (0b0000_1000)`
                    let supported_masks = [("7", 8)];

                    let (_, modulus) = supported_masks
                        .into_iter()
                        .find(|(key, _)| *key == right)
                        .ok_or_else(|| anyhow!("unable to find modulo for bit mask {right}"))?;

                    format!("mod(int!({left}), {modulus})",)
                }
                ">>" => {
                    let divisor = 2_u64.pow(right.parse::<u32>()?);

                    format!("to_int(floor(int!({left}) / {divisor}))")
                }
                other => Err(anyhow!(
                    "found `{}` in visit_binary, this operand is not currently implemented",
                    other
                ))?,
            }
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_elvis(&mut self, ctx: &ElvisContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_elvis {}", ctx.get_text())));
    }

    // Grammar: noncondexpression INSTANCEOF decltype
    #[instrument(skip_all)]
    fn visit_instanceof(&mut self, ctx: &InstanceofContext<'_>) {
        let result = try {
            let expr = self.visit(&*ctx.noncondexpression().unwrap())?;
            match self.visit(&*ctx.decltype().unwrap())?.as_ref() {
                "String" => format!("is_string({expr})"),
                "List" => format!("is_array({expr})"),
                "Map" => format!("is_object({expr})"),
                "long" => format!("is_integer({expr})"),
                ty => Err(anyhow!("\"{ty}\" is an unsupported instanceof type"))?,
            }
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_nonconditional(&mut self, ctx: &NonconditionalContext<'_>) {
        assert_eq!(1, ctx.get_child_count());
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    // Grammar
    #[instrument(skip_all)]
    fn visit_conditional(&mut self, ctx: &ConditionalContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_conditional {}", ctx.get_text())));
    }

    // Grammar: noncondexpression (ASSIGN | AADD | ASUB | AMUL | ADIV | AREM | AAND | AXOR | AOR | ALSH | ARSH | AUSH) expression
    #[instrument(skip_all)]
    fn visit_assignment(&mut self, ctx: &AssignmentContext<'_>) {
        let result = try {
            let children = ctx.get_children().collect::<Vec<_>>();
            if children.len() != 3 {
                Err(anyhow!(
                    "visit_assignment has the wrong number of children {}",
                    children.len()
                ))?;
            }

            if ctx.ASSIGN().is_some() {
                let left_dynamic_assignment = DynamicVisitor::visit(&*children[0])?;

                let right = self.visit(&*children[2])?;

                match &left_dynamic_assignment.is_static() {
                    true => format!("{} = {}", self.visit(&*children[0])?, right),
                    false => format!(
                        "{primary} = set!({primary}, {path:?}, {right})",
                        primary = left_dynamic_assignment.primary,
                        path = left_dynamic_assignment.path,
                        right = right
                    ),
                }
            } else if ctx.AADD().is_some() {
                let left = self.visit(&*children[0])?;
                let right = self.visit(&*children[2])?;

                format!("{left} = int!({{tmp = {left}\nif false {{ tmp = null }}\ntmp}}) + int!({{tmp = {right}\nif false {{ tmp = null }}\ntmp}})")
            } else {
                Err(anyhow!(
                    "unimplemented visit_assignment operator ({op}), full expression: {expr}",
                    op = ctx.get_child(1).unwrap().get_text(),
                    expr = ctx.get_text()
                ))?
            }
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_pre(&mut self, ctx: &PreContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_pre {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_addsub(&mut self, ctx: &AddsubContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_addsub {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_notaddsub(&mut self, ctx: &NotaddsubContext<'_>) {
        assert_eq!(1, ctx.get_child_count());
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    #[instrument(skip_all)]
    fn visit_read(&mut self, ctx: &ReadContext<'_>) {
        assert_eq!(1, ctx.get_child_count());
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    // Grammar: chain (INCR | DECR)
    #[instrument(skip_all)]
    fn visit_post(&mut self, ctx: &PostContext<'_>) {
        let result = try {
            let chain = self.visit(&*ctx.chain().unwrap())?;
            if ctx.INCR().is_none() {
                Err(anyhow!(
                    "post decrement is not currently supported by the transpiler {}",
                    ctx.get_text()
                ))?
            }

            format!("{{tmp = {chain}\n{chain} = {chain} + 1\ntmp\n}}")
        };

        self.returns(result);
    }

    // (BOOLNOT | BWNOT) unary
    #[instrument(skip_all)]
    fn visit_not(&mut self, ctx: &NotContext<'_>) {
        let result = try {
            if ctx.BWNOT().is_some() {
                Err(anyhow!("bitwise-not not currently implemented: {}", ctx.get_text()))?;
            }

            format!(
                "{} {}",
                ctx.BOOLNOT().unwrap().get_text(),
                self.visit(&*ctx.unary().unwrap())?
            )
        };

        self.returns(result);
    }

    // Grammar: castexpression
    #[instrument(skip_all)]
    fn visit_cast(&mut self, ctx: &CastContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    // Grammar: LP primordefcasttype RP unary
    #[instrument(skip_all)]
    fn visit_primordefcast(&mut self, ctx: &PrimordefcastContext<'_>) {
        let result = try {
            let cast = ctx.primordefcasttype().unwrap().get_text();

            match cast.as_str() {
                "int" => {
                    format!("to_int!({unary})", unary = self.visit(&*ctx.unary().unwrap())?)
                }
                other => Err(anyhow!("unsupported primitive cast type ({other})"))?,
            }
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_refcast(&mut self, ctx: &RefcastContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_refcast {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_primordefcasttype(&mut self, ctx: &PrimordefcasttypeContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_primordefcasttype {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_refcasttype(&mut self, ctx: &RefcasttypeContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_refcasttype {}", ctx.get_text())));
    }

    // Grammar: primary postfix*
    #[instrument(skip_all)]
    fn visit_dynamic(&mut self, ctx: &DynamicContext<'_>) {
        let result = try {
            let mut primary = self.visit(&*ctx.primary().unwrap())?;

            // Painless ctx is equivalent to the bare `.` in VRL
            if primary == "ctx" {
                primary = "".into();
            }

            let mut expr = primary;

            let mut iter = ctx.postfix_all().into_iter().peekable();

            // Grammar: callinvoke | fieldaccess | postfix
            while let Some(postfix) = iter.next() {
                if let Some(fncall) = postfix.callinvoke() {
                    match fncall.DOTID().unwrap().get_text().as_str() {
                        "endsWith" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            expr = format!("ends_with(string!({}), {})", expr, self.visit(&*arguments[0])?)
                        }
                        "toLowerCase" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("downcase(string!({}))", expr)
                        }
                        "valueOf" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            match expr.as_str() {
                                "String" => {
                                    expr = format!(
                                        "to_string!({{expr={arg}\nif false{{expr={{}}}}\nexpr}})",
                                        arg = self.visit(&*arguments[0])?
                                    )
                                }
                                other => Err(anyhow!("unsupported valueOf type coercion: {other}"))?,
                            }
                        }
                        "remove" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            if expr.is_empty() {
                                expr = ".".into()
                            }

                            expr = format!("{expr} = remove!(object({expr}), [{}])", self.visit(&*arguments[0])?)
                        }
                        "replace" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 2);

                            let failable = if expr.starts_with('.') { "!" } else { "" };

                            expr = format!(
                                "replace{failable}({expr}, {}, {})",
                                self.visit(&*arguments[0])?,
                                self.visit(&*arguments[1])?
                            );
                        }
                        "keySet" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("keys!({expr})")
                        }
                        "get" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            expr = format!("get!({expr}, [ {} ])", self.visit(&*arguments[0])?)
                        }
                        "contains" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            let value = self.visit(&*arguments[0])?;

                            // Need to do some type widening to get around VRLs strictness with error handling
                            expr = format!(
                                "{{
                                    expr = {expr}
                                    value = {value}
                                    if false {{
                                        expr = null
                                        value = null
                                    }}
                                    if is_array(expr) {{
                                        includes(array!(expr), value)
                                    }} else if is_string(expr) {{
                                        contains(string!(expr), string!(value))
                                    }} else {{
                                        assert!(false, \"contains only works on strings and array\")
                                    }}
                                }}",
                            );
                        }
                        "entrySet" => {
                            let next = iter
                                .peek()
                                .and_then(|it| it.callinvoke())
                                .map(|it| it.DOTID().unwrap().get_text());

                            match next.as_deref() {
                                Some("removeIf") => {
                                    if !fncall.arguments().unwrap().argument_all().is_empty() {
                                        Err(anyhow!("entrySet does not support arguments"))?
                                    }

                                    let arguments = iter
                                        .next()
                                        .unwrap()
                                        .callinvoke()
                                        .unwrap()
                                        .arguments()
                                        .unwrap()
                                        .argument_all();

                                    if arguments.len() != 1 {
                                        Err(anyhow!("removeIf must have a single argument: {arguments:?}"))?
                                    }

                                    // Grammar: ( lamtype | LP ( lamtype ( COMMA lamtype )* )? RP ) ARROW ( block | expression )
                                    let lambda = arguments[0]
                                        .lambda()
                                        .ok_or_else(|| anyhow!("removeIf only supports lambdas"))?;

                                    let lambda_args = lambda.lamtype_all();
                                    if lambda_args.len() != 1 {
                                        Err(anyhow!("removeIf only supports lambdas with one argument"))?
                                    }

                                    let lambda_arg_name = lambda_args[0].ID().unwrap().get_text();

                                    if lambda.block().is_some() {
                                        Err(anyhow!("removeIf only supports lambdas with a single expression"))?
                                    }

                                    let filter_expr = self.visit(&*lambda.expression().unwrap())?;

                                    expr = format!("{expr} = filter(object!({expr})) -> |key, value| {{\n{lambda_arg_name} = {{ \"key\": key, \"value\": value }}\n!({filter_expr})\n}}");
                                }
                                _ => {
                                    let arguments = fncall.arguments().unwrap().argument_all();
                                    assert!(arguments.is_empty());

                                    expr = format!(
                                        "{{
                                            result = []
                                            for_each(object!({expr})) -> |key, value| {{
                                                result = push(result, {{ \"key\": key, \"value\": value }})
                                            }}
                                            result
                                        }}"
                                    );
                                }
                            }
                        }
                        "getKey" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("{expr}.key")
                        }
                        "getValue" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("{expr}.value")
                        }
                        "put" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 2);

                            let key = self.visit(&*arguments[0])?;
                            let value = self.visit(&*arguments[1])?;

                            expr = format!("{expr} = set!({expr}, [{key}], {value})")
                        }
                        "containsKey" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            let key_expr = self.visit(&*arguments[0])?;

                            expr = format!("(get!({expr}, [{key_expr}]) != null)");
                        }
                        "clone" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            // clone is a no-op in VRL, all operations are clone by default
                            // however this means that scripts that rely upon mutation of shared references will not work properly
                        }
                        "add" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            let value = self.visit(&*arguments[0])?;

                            // Need to widen the type here because VRL is really pedantic with failability
                            expr = format!("if false {{ {expr} = null }}\n{expr} = push!({expr}, {value})");
                        }
                        "startsWith" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            let value = self.visit(&*arguments[0])?;

                            expr = format!("starts_with!({{expr = {expr}\nif false {{ expr = null }}\nexpr}}, {value})")
                        }
                        "toUpperCase" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("upcase!({expr})")
                        }
                        "substring" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            // substring can take two arguments, but we only support one for now
                            if arguments.len() != 1 {
                                Err(anyhow!(
                                    "substring currently only supports one argument, got ({})",
                                    arguments.len()
                                ))?
                            }

                            let index = self.visit(&*arguments[0])?;

                            expr = format!("slice!({expr}, {index})")
                        }
                        "length" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("length!({{expr = {expr}\nif false {{expr=null}}\nexpr}})")
                        }
                        "size" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("length!({expr})")
                        }
                        "isEmpty" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("(length!({expr}) == 0)")
                        }
                        "parseLong" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);
                            if expr != "Long" {
                                Err(anyhow!(
                                    "parseLong currently is only supported as a class method on Long"
                                ))?
                            }

                            let argument = self.visit(&*arguments[0])?;

                            expr = format!("parse_int!({argument})");
                        }
                        "compile" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);
                            if expr != "Pattern" {
                                Err(anyhow!(
                                    "compile currently is only supported as a class method on Pattern",
                                ))?
                            }

                            let pattern = self.visit(&*arguments[0])?;

                            expr = format!("to_regex!({pattern})");
                        }
                        "splitOnToken" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.len() == 1);

                            let token = self.visit(&*arguments[0])?;

                            expr = format!("split!({{expr={expr}\nif false{{expr=null}}\nexpr}}, {token})")
                        }
                        "trim" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            assert!(arguments.is_empty());

                            expr = format!("strip_whitespace!({{expr = {expr}\nif false{{expr = null}}\nexpr}})")
                        }
                        "removeIf" => {
                            let arguments = fncall.arguments().unwrap().argument_all();
                            if arguments.len() != 1 {
                                Err(anyhow!("removeIf must have a single argument: {arguments:?}"))?
                            }

                            // Grammar: ( lamtype | LP ( lamtype ( COMMA lamtype )* )? RP ) ARROW ( block | expression )
                            let lambda = arguments[0]
                                .lambda()
                                .ok_or_else(|| anyhow!("removeIf only supports lambdas"))?;

                            let lambda_args = lambda.lamtype_all();
                            if lambda_args.len() != 1 {
                                Err(anyhow!("removeIf only supports lambdas with one argument"))?
                            }

                            let lambda_arg_name = lambda_args[0].ID().unwrap().get_text();

                            if lambda.block().is_some() {
                                Err(anyhow!("removeIf only supports lambdas with a single expression"))?
                            }

                            let filter_expr = self.visit(&*lambda.expression().unwrap())?;

                            expr =
                                format!("{expr} = filter(array!({{expr={expr}\nif false{{expr=null}}\nexpr}})) -> |_index, {lambda_arg_name}| {{\n!({filter_expr})\n}}")
                        }
                        "asList" => {
                            if expr != "Arrays" {
                                Err(anyhow!(
                                    "asList currently is only supported as a class method on Arrays"
                                ))?
                            }

                            let arguments = fncall.arguments().unwrap().argument_all();
                            if arguments.len() != 1 {
                                Err(anyhow!("asList only supports a single argument"))?
                            }

                            expr = self.visit(&*arguments[0])?;
                        }
                        other => Err(anyhow!("`{other}` postfix function is not currently implemented"))?,
                    }
                }

                if let Some(brace_access) = postfix.braceaccess() {
                    expr = format!("get!({expr}, [{}])", self.visit(&*brace_access.expression().unwrap())?);
                }

                if let Some(fieldaccess) = postfix.fieldaccess() {
                    // Grammar: (DOT | NSDOT) (DOTID | DOTINTEGER)

                    if let Some(field) = fieldaccess.DOTID() {
                        match field.get_text().as_str() {
                            "length" => expr = format!("{{temp = {expr}\nif false {{temp = null}}\nlength!(temp)}}"),
                            field => expr = format!("{expr}.{field}"),
                        }
                    }

                    if let Some(index) = fieldaccess.DOTINTEGER() {
                        expr = format!("{expr}[{index}]", index = index.get_text())
                    }
                }
            }

            expr
        };

        self.returns(result)
    }

    // Grammar: arrayinitializer
    #[instrument(skip_all)]
    fn visit_newarray(&mut self, ctx: &NewarrayContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    // Grammar: LP expression RP
    #[instrument(skip_all)]
    fn visit_precedence(&mut self, ctx: &PrecedenceContext<'_>) {
        let result = try {
            let expr = self.visit(&*ctx.expression().unwrap())?;

            format!("({expr})")
        };

        self.returns(result);
    }

    // Grammar:
    // OCTAL: '0' [0-7]+ [lL]?;
    // HEX: '0' [xX] [0-9a-fA-F]+ [lL]?;
    // INTEGER: ( '0' | [1-9] [0-9]* ) [lLfFdD]?;
    // DECIMAL: ( '0' | [1-9] [0-9]* ) (DOT [0-9]+)? ( [eE] [+\-]? [0-9]+ )? [fFdD]?;
    #[instrument(skip_all)]
    fn visit_numeric(&mut self, ctx: &NumericContext<'_>) {
        let result = try {
            if let Some(dec) = ctx.DECIMAL() {
                dec.get_text()
            } else if let Some(hex) = ctx.HEX() {
                u64::from_str_radix(&hex.get_text().replace('L', "")[2..], 16)
                    .with_context(|| anyhow!("failed to parse hex value: {}", hex.get_text()))?
                    .to_string()
            } else if let Some(octal) = ctx.OCTAL() {
                u64::from_str_radix(&octal.get_text()[1..], 8)
                    .with_context(|| anyhow!("failed to parse octal value: {}", octal.get_text()))?
                    .to_string()
            } else if let Some(integer) = ctx.INTEGER() {
                integer
                    .get_text()
                    .replace('L', "")
                    .parse::<u64>()
                    .with_context(|| anyhow!("failed to parse integer value: {}", integer.get_text()))?
                    .to_string()
            } else {
                unreachable!()
            }
        };

        self.returns(result);
    }

    // Grammar: TRUE
    #[instrument(skip_all)]
    fn visit_true(&mut self, _ctx: &TrueContext<'_>) {
        self.returns(Ok("true".into()));
    }

    // Grammar: FALSE
    #[instrument(skip_all)]
    fn visit_false(&mut self, _ctx: &FalseContext<'_>) {
        self.returns(Ok("false".into()));
    }

    // null is the same between painless and vrl
    // Grammar: NULL
    #[instrument(skip_all)]
    fn visit_null(&mut self, ctx: &NullContext<'_>) {
        self.returns(Ok(ctx.get_text()))
    }

    // Strings in VRL must be quoted with ", not '
    // Grammar: STRING
    #[instrument(skip_all)]
    fn visit_string(&mut self, ctx: &StringContext<'_>) {
        self.returns(Ok(ctx.get_text().replace('\'', "\"")))
    }

    #[instrument(skip_all)]
    fn visit_regex(&mut self, ctx: &RegexContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_regex {}", ctx.get_text())));
    }

    // Grammar: listinitializer
    #[instrument(skip_all)]
    fn visit_listinit(&mut self, ctx: &ListinitContext<'_>) {
        assert!(ctx.get_child_count() == 1);
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    // Grammar: mapinitializer
    #[instrument(skip_all)]
    fn visit_mapinit(&mut self, ctx: &MapinitContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    // Grammar: ID
    #[instrument(skip_all)]
    fn visit_variable(&mut self, ctx: &VariableContext<'_>) {
        self.returns(Ok(ctx.ID().unwrap().get_text()))
    }

    // Grammar: (ID, DOLLAR) arguments
    #[instrument(skip_all, fields(functions = ?self.functions))]
    fn visit_calllocal(&mut self, ctx: &CalllocalContext<'_>) {
        let result = try {
            let name = ctx.ID().unwrap().get_text();

            let function = self
                .functions
                .get(&name)
                .ok_or_else(|| anyhow!("attempt to call undefined function: {name}"))?
                .clone();

            let provided_args = ctx
                .arguments()
                .unwrap()
                .argument_all()
                .into_iter()
                .map(|expr| self.visit(&*expr))
                .collect::<anyhow::Result<Vec<String>>>()?;

            if function.args.len() != provided_args.len() {
                Err(anyhow!(
                    "function {name} expects {expected} arguments, but {provided} were provided",
                    expected = function.args.len(),
                    provided = provided_args.len(),
                ))?
            }

            ScriptTemplate::render(
                r#"
                    {{ $func_args := .func_args }}
                    {{ $provided_args := .provided_args }}
                    {
                        {{ range $index, $_value := .func_args}}
                            {{ index $func_args $index }} = {{ index $provided_args $index }}
                        {{ end }}
                        {{ .body }}
                    }
                "#,
                HashMap::<String, gtmpl::Value>::from([
                    ("body".into(), function.body.into()),
                    ("func_args".into(), function.args.into()),
                    ("provided_args".into(), provided_args.into()),
                ]),
            )?
            .0
        };

        self.returns(result);
    }

    // Grammar: NEW typeid arguments
    #[instrument(skip_all)]
    fn visit_newobject(&mut self, ctx: &NewobjectContext<'_>) {
        let result = try {
            // Grammar: ( LP ( argument ( COMMA argument )* )? RP )
            let arguments = ctx.arguments().unwrap().argument_all();

            match ctx.typeid().unwrap().get_text().as_str() {
                "HashMap" => {
                    if !arguments.is_empty() {
                        Err(anyhow!(
                            "HashMap constructor doens't currently support arguments, found {arguments:?}"
                        ))?
                    }

                    "{}".to_string()
                }
                "ArrayList" => match arguments.len() {
                    0 => "[]".to_string(),
                    // ArrayList constructor takes another collection as an argument
                    // Since VRL only has one array type, it should just be a transparent evalutation of the first argument
                    1 => self.visit(&*arguments[0])?,
                    _ => Err(anyhow!(
                        "ArrayList constructor only support 0 or 1 arguments, found {arguments:?}"
                    ))?,
                },
                object => Err(anyhow!("unsupported object type {object}"))?,
            }
        };

        self.returns(result)
    }

    // Grammar: (callinvoke | fieldaccess | braceaccess)
    #[instrument(skip_all)]
    fn visit_postfix(&mut self, ctx: &PostfixContext<'_>) {
        assert_eq!(1, ctx.get_child_count());
        self.delegate(&*ctx.get_child(0).unwrap())
    }

    #[instrument(skip_all)]
    fn visit_postdot(&mut self, ctx: &PostdotContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_postdot {}", ctx.get_text())));
    }

    /**
        visit_callinvoke is for postfix method calls e.g. `it.endsWith("suffix")`,
        unfortunately we only see `.endsWith("suffix")` at this point in the visitor
        tree and so we can't transform it into a vrl function calls `endsWith(it, "suffix")`
        it must be intercepted higher in the visitor tree at `visit_dynamic`

        Grammar: (DOT | NSDOT) DOTID arguments
    */
    #[instrument(skip_all)]
    fn visit_callinvoke(&mut self, ctx: &CallinvokeContext<'_>) {
        self.returns(Err(anyhow!("callinvoke must not be reached: {}", ctx.get_text())));
    }

    // Grammar: (DOT | NSDOT) (DOTID | DOTINTEGER)
    #[instrument(skip_all)]
    fn visit_fieldaccess(&mut self, ctx: &FieldaccessContext<'_>) {
        let result = try {
            if ctx.DOTINTEGER().is_some() {
                Err(anyhow!("DOTINTERGER not yet supported"))?;
            }

            // VRL has null coalescing built into the `.` operator, so we don't care if its DOT or NSDOT
            format!(".{}", ctx.DOTID().unwrap().get_text())
        };

        self.returns(result);
    }

    /**
       much like visit_callinvoke, there is not enough information here to properly
       transform the code, `visit_dynamic` should intercept this
    */
    #[instrument(skip_all)]
    fn visit_braceaccess(&mut self, _ctx: &BraceaccessContext<'_>) {
        self.returns(Err(anyhow!("braceaccess must not be reached",)));
    }

    #[instrument(skip_all)]
    fn visit_newstandardarray(&mut self, ctx: &NewstandardarrayContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_newstandardarray {}", ctx.get_text())));
    }

    // NEW type LBRACE RBRACE LBRACK ( expression ( COMMA expression )* )? RBRACK postfix*
    #[instrument(skip_all)]
    fn visit_newinitializedarray(&mut self, ctx: &NewinitializedarrayContext<'_>) {
        let result = try {
            if !ctx.postfix_all().is_empty() {
                Err(anyhow!(
                    "postfix currently not supported in visit_newinitializedarray, found \"{}\"",
                    ctx.get_text()
                ))?
            }

            let elements = ctx
                .expression_all()
                .into_iter()
                .map(|expr| self.visit(&*expr))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");

            format!("[ {elements} ]")
        };

        self.returns(result);
    }
    // Grammar: LBRACE expression (COMMA expression)* RBRACE
    //        | LBRACE LBRACE
    #[instrument(skip_all)]
    fn visit_listinitializer(&mut self, ctx: &ListinitializerContext<'_>) {
        let result = try {
            let expressions = ctx
                .expression_all()
                .into_iter()
                .map(|expr| self.visit(&*expr))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");

            format!("[ {expressions} ]")
        };

        self.returns(result);
    }

    // Grammar:     : LBRACE maptoken ( COMMA maptoken )* RBRACE
    //              | LBRACE COLON RBRACE
    #[instrument(skip_all)]
    fn visit_mapinitializer(&mut self, ctx: &MapinitializerContext<'_>) {
        let result = try {
            let map_tokens = ctx
                .maptoken_all()
                .into_iter()
                .map(|map_token| self.visit(&*map_token))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");

            format!("{{{map_tokens}}}")
        };

        self.returns(result);
    }

    // Grammar: expression COLON expression
    #[instrument(skip_all)]
    fn visit_maptoken(&mut self, ctx: &MaptokenContext<'_>) {
        let result = try {
            let exprs = ctx
                .expression_all()
                .into_iter()
                .map(|expr| self.visit(&*expr))
                .collect::<Result<Vec<_>, _>>()?;

            format!("{key}: {value}", key = exprs[0], value = exprs[1])
        };

        self.returns(result);
    }

    #[instrument(skip_all)]
    fn visit_arguments(&mut self, ctx: &ArgumentsContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_arguments {}", ctx.get_text())));
    }

    // Grammar: (expression | lambda | funcref)
    #[instrument(skip_all)]
    fn visit_argument(&mut self, ctx: &ArgumentContext<'_>) {
        assert!(ctx.get_child_count() == 1);

        self.delegate(&*ctx.get_child(0).unwrap())
    }

    #[instrument(skip_all)]
    fn visit_lambda(&mut self, ctx: &LambdaContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_lambda {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_lamtype(&mut self, ctx: &LamtypeContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_lamtype {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_classfuncref(&mut self, ctx: &ClassfuncrefContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_classfuncref {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_constructorfuncref(&mut self, ctx: &ConstructorfuncrefContext<'_>) {
        self.returns(Err(anyhow!("gevisit_constructorfuncref {}", ctx.get_text())));
    }

    #[instrument(skip_all)]
    fn visit_localfuncref(&mut self, ctx: &LocalfuncrefContext<'_>) {
        self.returns(Err(anyhow!("unimplemented visit_localfuncref {}", ctx.get_text())));
    }
}

enum QuerySegment {
    Dynamic(String),
    Static(String),
}

impl std::fmt::Debug for QuerySegment {
    fn fmt(&self, f: &mut vrl::prelude::fmt::Formatter<'_>) -> vrl::prelude::fmt::Result {
        match self {
            QuerySegment::Dynamic(str) => write!(f, "{str}"),
            QuerySegment::Static(str) => write!(f, "\"{str}\""),
        }
    }
}

#[derive(Default, Debug)]
struct Dynamic {
    path: Vec<QuerySegment>,
    primary: String,
}

impl Dynamic {
    fn is_static(&self) -> bool {
        self.path.iter().all(|item| matches!(item, QuerySegment::Static(_)))
    }
}

#[derive(Debug)]
struct DynamicVisitor(anyhow::Result<Dynamic>);
impl Default for DynamicVisitor {
    fn default() -> Self {
        Self(Ok(Default::default()))
    }
}

impl DynamicVisitor {
    fn visit<'input>(node: &(dyn PainlessParserContext<'input> + 'input)) -> anyhow::Result<Dynamic> {
        let mut visitor = DynamicVisitor::default();
        visitor.visit_node(node);

        visitor.0
    }
}

impl<'input> ParseTreeVisitor<'input, PainlessParserContextType> for DynamicVisitor {}
impl<'input> PainlessParserVisitor<'input> for DynamicVisitor {
    // LBRACE expression RBACE
    #[instrument(skip_all)]
    fn visit_braceaccess(&mut self, ctx: &BraceaccessContext<'input>) {
        if let Ok(value) = &mut self.0 {
            match Transpiler::default().visit(&*ctx.expression().unwrap()) {
                Ok(expr) => value.path.push(QuerySegment::Dynamic(expr)),
                Err(err) => self.0 = Err(err),
            };
        }
    }

    // ( DOT | NSDOT ) ( DOTID | DOTINTEGER )
    #[instrument(skip_all)]
    fn visit_fieldaccess(&mut self, ctx: &FieldaccessContext<'input>) {
        if let Ok(value) = &mut self.0 {
            value.path.push(QuerySegment::Static(ctx.DOTID().unwrap().get_text()));
        }
    }

    #[instrument(skip_all)]
    fn visit_dynamic(&mut self, ctx: &DynamicContext<'input>) {
        if let Ok(value) = &mut self.0 {
            value.primary = ctx.primary().unwrap().get_text();
            if value.primary == "ctx" {
                value.primary = ".".into();
            }

            self.visit_children(ctx);
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{painless::Script, test_utils};
    use anyhow::anyhow;
    use proptest::{prop_assert_eq, proptest};
    use test_utils::assert_eq;

    #[test]
    pub fn for_each_in() -> anyhow::Result<()> {
        let expressions = r#"
            def tmp = [:];
            for (item in ctx.properties) {
                tmp.map[item.key].value = item.value;
            }

            ctx.properties = tmp;
        "#;

        assert_eq!(
            vrl::value!({ properties: { map: { "foo": { value: "foo" }, "bar": { value: "bar" } } } }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({
                    properties: [{ key: "foo", value: "foo" }, { key: "bar", value: "bar" }]
                }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn for_loop() -> anyhow::Result<()> {
        let expressions = r#" 
            ctx.sum = 0;

            for (def i = 0; i < ctx.input.length; i++) {
                ctx.sum += ctx.input[i];
            }
        "#;

        assert_eq!(
            vrl::value!({
                input: [1, 2, 3, 4, 5],
                sum:  15
            }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({
                    input: [1, 2, 3, 4, 5]
                }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn replace() -> anyhow::Result<()> {
        let expressions = r#"
        if (ctx.azure.activitylogs.identity.claims != null) {
            ctx.temp_claims = new HashMap();
            for (String key : ctx.azure.activitylogs.identity.claims.keySet()) {
                ctx.temp_claims[key.replace('.', '_')] = ctx.azure.activitylogs.identity.claims.get(key);
            }
            ctx.azure.activitylogs.identity.claims = ctx.temp_claims; ctx.remove('temp_claims');
        }
        "#;

        assert_eq!(
            vrl::value!({ azure: { activitylogs: { identity: { claims: { "i_am_sam": "hello" } } } } }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({
                    azure: { activitylogs: { identity: { claims: { "i.am.sam": "hello" } } } }
                }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn array_contains() -> anyhow::Result<()> {
        let expressions = r#"
            ['foo', 'bar'].contains(ctx.type)
        "#;

        assert_eq!(
            vrl::value!(true),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ "type": "foo" }))?
                .result
                .unwrap()
        );

        Ok(())
    }

    #[test]
    pub fn string_contains() -> anyhow::Result<()> {
        let expressions = r#"
            "foobar".contains(ctx.type)
        "#;

        assert_eq!(
            vrl::value!(true),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ "type": "foo" }))?
                .result
                .unwrap()
        );

        Ok(())
    }

    #[test]
    pub fn boolean_algebra() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            (ctx.a == true) && (ctx.b == true) && (((ctx.c == true) && (ctx.d == true)) || ((ctx.e == true) && (ctx.f == true)))
        "#;

        for (input, output) in [
            (
                vrl::value!({ a: true, b: true, c: true, d: true, e: true, f: true}),
                vrl::value!(true),
            ),
            (
                vrl::value!({ a: false, b: true, c: true, d: true, e: true, f: true}),
                vrl::value!(false),
            ),
            (
                vrl::value!({ a: true, b: false, c: true, d: true, e: true, f: true}),
                vrl::value!(false),
            ),
            (
                vrl::value!({ a: true, b: true, c: false, d: true, e: true, f: true}),
                vrl::value!(true),
            ),
            (
                vrl::value!({ a: true, b: true, c: false, d: true, e: false, f: true}),
                vrl::value!(false),
            ),
        ] {
            assert_eq!(
                output,
                Script::parse(expressions)
                    .transpile()?
                    .compile()?
                    .run(input)?
                    .result
                    .unwrap()
            );
        }

        Ok(())
    }

    #[test]
    pub fn entry_set() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            for (entry in ctx.input.entrySet()) {
                ctx.output[entry.getKey()] = entry.getValue()
            }
        "#;

        assert_eq!(
            vrl::value!({
                input: {
                    "key1": "value1",
                    "key2": "value2"
                },
                output: {
                    "key1": "value1",
                    "key2": "value2"
                }
            }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({
                   input: {
                       "key1": "value1",
                       "key2": "value2"
                   }
                }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn binary_mask() -> anyhow::Result<()> {
        test_utils::init();

        let expression = "ctx.input & 0x7";

        let program = Script::parse(expression).transpile()?.compile()?;

        proptest!(|(x in 0i64..)| {
            prop_assert_eq!(
                x & 0x7,
                program
                    .run(vrl::value!({ input: x }))
                    .unwrap()
                    .result
                    .unwrap()
                    .as_integer()
                    .unwrap()
            );
        });

        Ok(())
    }

    #[test]
    pub fn bit_shift() -> anyhow::Result<()> {
        test_utils::init();

        proptest!(|(x in 0_u32.., shift in 0_u32..31_u32)| {
            let expression = format!("ctx.input >> {shift}");
            let program = Script::parse(&expression).transpile().unwrap().compile().unwrap();

            prop_assert_eq!(
                (x >> shift) as i64,
                program
                    .run(vrl::value!({ input: x }))
                    .unwrap()
                    .result
                    .unwrap()
                    .as_integer()
                    .unwrap()
            );
        });

        Ok(())
    }

    #[test]
    pub fn early_return() -> anyhow::Result<()> {
        test_utils::init();

        let script = r#"
            if (ctx?.message == "foo") {
                ctx?.message = "early_return";
                return;
            }

            ctx?.message = "bar";
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        for (input, output) in [
            (
                vrl::value!({ message: "foo" }),
                vrl::value!({ message: "early_return" }),
            ),
            (vrl::value!({ message: "baz" }), vrl::value!({ message: "bar" })),
        ] {
            assert_eq!(output, program.run(input)?.target);
        }

        Ok(())
    }

    #[test]
    pub fn consecutive_returns() -> anyhow::Result<()> {
        let script = r#"
            if (ctx?.message == "foo") {
                ctx?.message = "early_return";
                return;
            }

            if (ctx?.message == "bar") {
                ctx?.message = "second_return";
                return;
            }

            ctx?.message = "bar";
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        for (input, output) in [
            (
                vrl::value!({ message: "foo" }),
                vrl::value!({ message: "early_return" }),
            ),
            (
                vrl::value!({ message: "bar" }),
                vrl::value!({ message: "second_return" }),
            ),
            (vrl::value!({ message: "baz" }), vrl::value!({ message: "bar" })),
        ] {
            assert_eq!(output, program.run(input)?.target);
        }

        Ok(())
    }

    #[test]
    pub fn elif_returns() -> anyhow::Result<()> {
        let script = r#"
            if (ctx?.message == "foo") {
                ctx?.message = "early_return";
                return;
            } else if (ctx?.message == "bar") {
                ctx?.message = "second_return";
                return;
            } else if (ctx?.message == "baz") {
                ctx?.message = "third_return";
                return;
            } else {
                ctx?.message = "final_clause";
            }

            ctx?.congratulations = true;
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        for (input, output) in [
            (
                vrl::value!({ message: "foo" }),
                vrl::value!({ message: "early_return" }),
            ),
            (
                vrl::value!({ message: "bar" }),
                vrl::value!({ message: "second_return" }),
            ),
            (
                vrl::value!({ message: "baz" }),
                vrl::value!({ message: "third_return" }),
            ),
            (
                vrl::value!({ message: "qux" }),
                vrl::value!({ message: "final_clause", congratulations: true }),
            ),
        ] {
            assert_eq!(output, program.run(input)?.target);
        }

        Ok(())
    }

    #[test]
    pub fn return_with_else_if_complex() -> anyhow::Result<()> {
        let script = r#"
            if (ctx?.message == "foo") {
                ctx?.message = "early_return";
                return;
            } else if (ctx?.message == "bar")  {
                ctx?.message = "second_condition";
            } else {
                ctx?.error = "else";
                return
            }

            ctx?.message = "bar";
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        for (input, output) in [
            (
                vrl::value!({ message: "foo" }),
                vrl::value!({ message: "early_return" }),
            ),
            (vrl::value!({ message: "bar" }), vrl::value!({ message: "bar" })),
            (
                vrl::value!({ message: "baz" }),
                vrl::value!({ message: "baz", error: "else" }),
            ),
        ] {
            assert_eq!(output, program.run(input)?.target);
        }

        Ok(())
    }

    #[test]
    pub fn nested() -> anyhow::Result<()> {
        let script = r#"
            if (ctx?.message == "foo") {
                ctx?.message = "early_return";

                if (ctx?.count == 42) {
                    if (ctx?.reposte == true) {
                        ctx.error = "no you";
                        return;
                    }

                    ctx.error = "no jokes";
                    return;
                }

                ctx?.error = "more jokes";
            }

            ctx?.message = "bar";
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        for (input, output) in [
            (
                vrl::value!({ message: "foo" }),
                vrl::value!({ message: "bar", error: "more jokes" }),
            ),
            (
                vrl::value!({ message: "foo", count: 42 }),
                vrl::value!({ message: "early_return", count: 42, error: "no jokes" }),
            ),
            (
                vrl::value!({ message: "foo", count: 42, reposte: true }),
                vrl::value!({ message: "early_return", count: 42, error: "no you", reposte: true }),
            ),
            (
                vrl::value!({ message: "baz", count: 42 }),
                vrl::value!({ message: "bar", count: 42 }),
            ),
        ] {
            assert_eq!(output, program.run(input)?.target);
        }

        Ok(())
    }

    #[test]
    pub fn if_without_return() -> anyhow::Result<()> {
        let script = r#"
            if (ctx?.message == "foo") {
                ctx?.message = "early_return";
            } 

            ctx?.error = "bar";
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        for (input, output) in [
            (
                vrl::value!({ message: "foo" }),
                vrl::value!({ message: "early_return", error: "bar" }),
            ),
            (
                vrl::value!({ message: "bar" }),
                vrl::value!({ message: "bar", error: "bar" }),
            ),
        ] {
            assert_eq!(output, program.run(input)?.target);
        }

        Ok(())
    }

    #[test]
    pub fn int_cast() -> anyhow::Result<()> {
        let script = r#"
            ctx.message = (int) ctx?.message
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        assert_eq!(
            vrl::value!({ message: 42 }),
            program.run(vrl::value!({ message: "42" }))?.target
        );

        Ok(())
    }

    #[test]
    pub fn division() -> anyhow::Result<()> {
        let script = r#"
            ctx.value = ctx?.value / 8;
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        assert_eq!(
            vrl::value!({ value: 2 }),
            program.run(vrl::value!({ value: 16 }))?.target
        );

        Ok(())
    }

    #[test]
    pub fn subtraction() -> anyhow::Result<()> {
        let script = r#"
            ctx.value = ctx?.value - 8;
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        assert_eq!(
            vrl::value!({ value: 8 }),
            program.run(vrl::value!({ value: 16 }))?.target
        );

        Ok(())
    }

    #[test]
    pub fn string_to_uppercase() -> anyhow::Result<()> {
        let expressions = r#"
            ctx.message = ctx?.message.toUpperCase()
        "#;

        assert_eq!(
            vrl::value!({ message: "FOOBAR" }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ message: "foobar" }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn array_length() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.error_count = ctx?.errors?.length
        "#;

        assert_eq!(
            vrl::value!({ errors: ["foo", "bar"], error_count: 2 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ errors: ["foo", "bar"] }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn assignment_add() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.count += 1
        "#;

        assert_eq!(
            vrl::value!({ count: 2 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ count: 1 }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn null_not_equal_to_zero() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.missing_field == 0
        "#;

        assert_eq!(
            vrl::value!(false),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({}))?
                .result
                .ok_or_else(|| anyhow!("failed to produce a result"))?
        );

        Ok(())
    }

    #[test]
    pub fn substring() -> anyhow::Result<()> {
        let script = r#"
            ctx.message = ctx?.message.substring(ctx?.index)
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        for (input, output) in [
            (
                vrl::value!({ message: "foobar", index: 0 }),
                vrl::value!({ message: "foobar", index: 0 }),
            ),
            (
                vrl::value!({ message: "foobar", index: 1 }),
                vrl::value!({ message: "oobar", index: 1 }),
            ),
            (
                vrl::value!({ message: "foobar", index: 5 }),
                vrl::value!({ message: "r", index: 5 }),
            ),
        ] {
            assert_eq!(output, program.run(input)?.target);
        }

        Ok(())
    }

    #[test]
    pub fn string_length() -> anyhow::Result<()> {
        let script = r#"
            ctx.str_length = ctx?.message.length()
        "#;

        let program = Script::parse(script).transpile()?.compile()?;

        for (input, output) in [
            (
                vrl::value!({ message: "foobar" }),
                vrl::value!({ message: "foobar", str_length: 6 }),
            ),
            (
                vrl::value!({ message: "f" }),
                vrl::value!({ message: "f", str_length: 1 }),
            ),
            (
                vrl::value!({ message: "" }),
                vrl::value!({ message: "", str_length: 0 }),
            ),
        ] {
            assert_eq!(output, program.run(input)?.target);
        }

        Ok(())
    }

    #[test]
    pub fn arraylist_constructor() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.array = new ArrayList();
        "#;

        assert_eq!(
            vrl::value!({ array: [] }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({}))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn funcdef_basic() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            void fooBar() {
                ctx.message = ctx?.message.toUpperCase();
            }

            fooBar();
        "#;

        assert_eq!(
            vrl::value!({ message: "FOOBAR" }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ message: "foobar" }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn funcdef_arguments() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            void fooBar(def a, def b) {
                ctx.sum = 0;
                ctx.sum += a;
                ctx.sum += b;
            }

            fooBar(ctx?.foo, ctx?.bar);
        "#;

        assert_eq!(
            vrl::value!({ sum: 3, foo: 1, bar: 2 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ foo: 1, bar: 2 }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn funcdef_return() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            int add(def a, def b) {
                return a + b
            }

            ctx.first = add(ctx?.foo, ctx?.bar);
            ctx.second = add(ctx?.bar, ctx?.baz);
        "#;

        assert_eq!(
            vrl::value!({ foo: 1, bar: 2, baz: 3, first: 3, second: 5 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ foo: 1, bar: 2, baz: 3 }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn funcdef_early_return() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            int foo(def a, def b) {
                if (a == 1) {
                    return a
                }
                return a + b
            }

            ctx.first = foo(ctx?.foo, ctx?.bar);
            ctx.second = foo(ctx?.bar, ctx?.baz);
        "#;

        assert_eq!(
            vrl::value!({ foo: 1, bar: 2, baz: 3, first: 1, second: 5 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ foo: 1, bar: 2, baz: 3 }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn funcdef_nested_early_return() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            int foo(def a, def b) {
                if (a == 1) {
                    if (b == 2) {
                        return b
                    }
                    return a
                }
                return a + b
            }

            ctx.first = foo(ctx?.foo, ctx?.bar);
            ctx.second = foo(ctx?.bar, ctx?.baz);
        "#;

        assert_eq!(
            vrl::value!({ foo: 1, bar: 2, baz: 3, first: 2, second: 5 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ foo: 1, bar: 2, baz: 3 }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn postincrement() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            def foo = 0
            ctx.first = foo++;
            ctx.second = foo++;
        "#;

        assert_eq!(
            vrl::value!({ first: 0, second: 1 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({}))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn instanceof_object() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.target instanceof Map
        "#;

        for (input, output) in [
            (vrl::value!({ target: { foo: "bar" } }), vrl::value!(true)),
            (vrl::value!({ target: [] }), vrl::value!(false)),
            (vrl::value!({ target: "foobar" }), vrl::value!(false)),
        ] {
            assert_eq!(
                output,
                Script::parse(expressions)
                    .transpile()?
                    .compile()?
                    .run(input)?
                    .result
                    .unwrap()
            );
        }

        Ok(())
    }

    #[test]
    pub fn map_size() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.target.size()
        "#;

        for (input, output) in [
            (vrl::value!({ target: { foo: "bar" } }), vrl::value!(1)),
            (vrl::value!({ target: {} }), vrl::value!(0)),
            (vrl::value!({ target: { foo: "foo", bar: "bar" } }), vrl::value!(2)),
        ] {
            assert_eq!(
                output,
                Script::parse(expressions)
                    .transpile()?
                    .compile()?
                    .run(input)?
                    .result
                    .unwrap()
            );
        }

        Ok(())
    }

    #[test]
    pub fn parenthetical_division() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.count = (ctx?.a - ctx?.b) / 8;
        "#;

        assert_eq!(
            vrl::value!({ a: 12, b: 4, count: 1 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ a: 12, b: 4 }))?
                .target
        );

        Ok(())
    }

    #[ignore]
    #[test]
    pub fn conditional() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.target instanceof Map ? "map" : "not a map";
        "#;

        for (input, output) in [
            (vrl::value!({ target: { foo: "bar" } }), vrl::value!("map")),
            (vrl::value!({ target: [] }), vrl::value!("not a map")),
            (vrl::value!({ target: "foobar" }), vrl::value!("not a map")),
        ] {
            assert_eq!(
                output,
                Script::parse(expressions)
                    .transpile()?
                    .compile()?
                    .run(input)?
                    .result
                    .unwrap()
            );
        }

        Ok(())
    }

    #[test]
    pub fn postfix_isempty() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            !ctx?.target.isEmpty();
        "#;

        for (input, output) in [
            (vrl::value!({ target: "hello" }), vrl::value!(true)),
            (vrl::value!({ target: "" }), vrl::value!(false)),
        ] {
            assert_eq!(
                output,
                Script::parse(expressions)
                    .transpile()?
                    .compile()?
                    .run(input)?
                    .result
                    .unwrap()
            );
        }

        Ok(())
    }

    #[test]
    pub fn escaped_backslash() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx?.target?.contains("\\\\")
        "#;

        for (input, output) in [
            (vrl::value!({ target: "\\" }), vrl::value!(false)),
            (vrl::value!({ target: "\\\\" }), vrl::value!(true)),
        ] {
            assert_eq!(
                output,
                Script::parse(expressions)
                    .transpile()?
                    .compile()?
                    .run(input)?
                    .result
                    .unwrap()
            );
        }

        Ok(())
    }

    #[test]
    pub fn array_index() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx?.target.0
        "#;

        assert_eq!(
            vrl::value!("foo"),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ target: ["foo", "bar"] }))?
                .result
                .unwrap()
        );

        Ok(())
    }

    #[test]
    pub fn parse_long() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            Long.parseLong("1234567890123456789")
        "#;

        assert_eq!(
            vrl::value!(1234567890123456789_i64),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({}))?
                .result
                .unwrap()
        );

        Ok(())
    }

    #[test]
    pub fn infailable_multiplication() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.event.duration = Long.parseLong(ctx.fortinet.firewall.duration) * 1000000000
        "#;

        assert_eq!(
            vrl::value!({ event: { duration: 10_000_000_000_i64 }, fortinet: { firewall: { duration: "10" } } }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ fortinet: { firewall: { duration: "10" }}}))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn pattern_compile() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            Pattern.compile("\\d+");
        "#;

        assert_eq!(
            r#"r'\d+'"#,
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({}))?
                .result
                .unwrap()
                .to_string()
        );

        Ok(())
    }

    #[test]
    pub fn instanceof_long() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.target instanceof long
        "#;

        for (input, output) in [
            (vrl::value!({ target: { foo: "bar" } }), vrl::value!(false)),
            (vrl::value!({ target: 42 }), vrl::value!(true)),
            (vrl::value!({ target: "foobar" }), vrl::value!(false)),
        ] {
            assert_eq!(
                output,
                Script::parse(expressions)
                    .transpile()?
                    .compile()?
                    .run(input)?
                    .result
                    .unwrap()
            );
        }

        Ok(())
    }

    #[test]
    pub fn split_on_token() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.message.splitOnToken('-');
        "#;

        assert_eq!(
            vrl::value!(["Hello", "World"]),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ message: "Hello-World"}))?
                .result
                .unwrap()
        );

        Ok(())
    }

    #[test]
    pub fn trim() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.message.trim();
        "#;

        assert_eq!(
            vrl::value!("Hello, World"),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ message: " Hello, World    "}))?
                .result
                .unwrap()
        );

        Ok(())
    }

    #[test]
    pub fn remove_if() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.tags.removeIf(arg -> arg == "");
        "#;

        assert_eq!(
            vrl::value!({ tags: ["Hello", "World"] }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ tags: ["", "Hello", "", "World", ""]}))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn entryset_removeif() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.metadata.entrySet().removeIf(entry -> entry.getValue() == "N/A");
        "#;

        assert_eq!(
            vrl::value!({ metadata: { foo: "foo", baz: "baz" } }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({ metadata: { foo: "foo", "bar": "N/A", baz: "baz" } }))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn map_initiliasiation() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.metadata = [
                'foo': 'foo',
                'bar': 'bar',
                'baz': 'baz'
            ]
        "#;

        assert_eq!(
            vrl::value!({ metadata: { foo: "foo", bar: "bar", baz: "baz" } }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({}))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn long_hex() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.value = 0x0100000000000000L
        "#;

        assert_eq!(
            vrl::value!({ value: 0x0100000000000000_i64 }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({}))?
                .target
        );

        Ok(())
    }

    #[test]
    pub fn non_empty_array_initialisation() -> anyhow::Result<()> {
        test_utils::init();

        let expressions = r#"
            ctx.list = new ArrayList(Arrays.asList(ctx?.message?.splitOnToken(" ")));
        "#;

        assert_eq!(
            vrl::value!({ message: "Hello World", list: ["Hello", "World"] }),
            Script::parse(expressions)
                .transpile()?
                .compile()?
                .run(vrl::value!({
                    message: "Hello World"
                }))?
                .target
        );

        Ok(())
    }
}
