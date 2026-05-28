// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! ANTLR4 Painless parse tree → IR visitor.
//!
//! Walks the ANTLR-generated parse tree and builds `ir::Expr` / `ir::Stmt`
//! nodes. The visitor uses a slot-based return pattern (inherited from the
//! predecessor VRL transpiler) because ANTLR's `ParseTreeVisitor` trait
//! forces a uniform return type across all visit methods.

use std::ops::Deref;
use std::rc::Rc;

use antlr_rust::parser::ParserNodeType;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Tree, VisitChildren};
use anyhow::anyhow;
use serde_json::json;

use super::ir::*;
use super::parser::painlessparser::*;
use super::parser::painlessparservisitor::PainlessParserVisitor;

/// An IR node returned from a visitor method.
///
/// The ANTLR visitor trait forces a uniform return type, so we wrap
/// both expressions and statements in this enum.
#[derive(Debug, Clone)]
enum IrNode {
    Expr(Expr),
    Stmt(Stmt),
    Stmts(Vec<Stmt>),
    /// A raw string for type names that don't map to IR.
    TypeName(String),
}

impl IrNode {
    fn into_expr(self) -> anyhow::Result<Expr> {
        match self {
            IrNode::Expr(e) => Ok(e),
            other => Err(anyhow!(
                "expected Expr, got {:?}",
                std::mem::discriminant(&other)
            )),
        }
    }

    fn into_stmt(self) -> anyhow::Result<Stmt> {
        match self {
            IrNode::Stmt(s) => Ok(s),
            IrNode::Expr(e) => Ok(Stmt::Expr(e)),
            other => Err(anyhow!(
                "expected Stmt, got {:?}",
                std::mem::discriminant(&other)
            )),
        }
    }

    fn into_stmts(self) -> anyhow::Result<Vec<Stmt>> {
        match self {
            IrNode::Stmts(ss) => Ok(ss),
            IrNode::Stmt(s) => Ok(vec![s]),
            other => Err(anyhow!(
                "expected Stmts, got {:?}",
                std::mem::discriminant(&other)
            )),
        }
    }

    fn into_type_name(self) -> anyhow::Result<String> {
        match self {
            IrNode::TypeName(s) => Ok(s),
            other => Err(anyhow!(
                "expected TypeName, got {:?}",
                std::mem::discriminant(&other)
            )),
        }
    }
}

/// Visitor that builds IR from the ANTLR4 Painless parse tree.
pub struct RustVisitor {
    /// Slot for returning values from visitor methods.
    program: Option<anyhow::Result<IrNode>>,
    /// Collected function definitions.
    functions: Vec<Stmt>,
    /// Whether an error has been reported (to avoid duplicate logging).
    error_reported: bool,
}

impl Default for RustVisitor {
    fn default() -> Self {
        Self {
            program: None,
            functions: Vec::new(),
            error_reported: false,
        }
    }
}

impl RustVisitor {
    /// Visit a node and extract the result from the slot.
    fn visit_ir(
        &mut self,
        node: &<PainlessParserContextType as ParserNodeType>::Type,
    ) -> anyhow::Result<IrNode> {
        self.visit_node(node);
        self.program
            .take()
            .unwrap_or_else(|| Err(anyhow!("visitor produced no result")))
    }

    /// Visit a node expecting an expression result.
    fn visit_expr_node(
        &mut self,
        node: &<PainlessParserContextType as ParserNodeType>::Type,
    ) -> anyhow::Result<Expr> {
        self.visit_ir(node)?.into_expr()
    }

    /// Visit a node expecting a statement result.
    fn visit_stmt_node(
        &mut self,
        node: &<PainlessParserContextType as ParserNodeType>::Type,
    ) -> anyhow::Result<Stmt> {
        self.visit_ir(node)?.into_stmt()
    }

    /// Store a result in the return slot.
    fn returns(&mut self, result: anyhow::Result<IrNode>) {
        if let Err(ref err) = result {
            if !self.error_reported {
                tracing::error!(error = %err, "painless visitor error");
                self.error_reported = true;
            }
        }
        self.program = Some(result);
    }

    /// Delegate to a child node (pass-through).
    fn delegate(&mut self, node: &<PainlessParserContextType as ParserNodeType>::Type) {
        self.visit_node(node);
    }

    /// Visit a sequence of statement nodes and collect them.
    fn visit_statement_list(
        &mut self,
        statements: &[Rc<StatementContextAll<'_>>],
        trailing: Option<Rc<DstatementContextAll<'_>>>,
    ) -> anyhow::Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        for s in statements {
            let stmt = self.visit_stmt_node(s.deref())?;
            match stmt {
                Stmt::Block(inner) => stmts.extend(inner),
                other => stmts.push(other),
            }
        }
        if let Some(ds) = trailing {
            let stmt = self.visit_stmt_node(ds.deref())?;
            match stmt {
                Stmt::Block(inner) => stmts.extend(inner),
                other => stmts.push(other),
            }
        }
        Ok(stmts)
    }

    /// Transpile a Painless source string to IR.
    ///
    /// This is the main entry point. Parses the source and visits the tree.
    pub fn transpile(source: &str) -> anyhow::Result<PainlessScript> {
        use super::parser::painlesslexer::PainlessLexer;
        use antlr_rust::InputStream;
        use antlr_rust::common_token_stream::CommonTokenStream;

        let input = InputStream::new(source);
        let lexer = PainlessLexer::new(input);
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = PainlessParser::new(token_stream);
        let tree = parser
            .source()
            .map_err(|e| anyhow!("parse error: {:?}", e))?;

        let mut visitor = RustVisitor::default();
        let result = visitor.visit_ir(&*tree)?;
        let body = result.into_stmts()?;

        Ok(PainlessScript {
            functions: visitor.functions,
            body,
        })
    }
}

impl<'input> ParseTreeVisitor<'input, PainlessParserContextType> for RustVisitor {}

impl<'input> PainlessParserVisitor<'input> for RustVisitor {
    /// Grammar: function* statement* EOF
    fn visit_source(&mut self, ctx: &SourceContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            // Collect function definitions
            for func in ctx.function_all() {
                let name = func.ID().unwrap().get_text();
                let block_stmt = self.visit_stmt_node(&*func.block().unwrap())?;

                let params: Vec<FunctionParam> = if let Some(parameters) = func.parameters() {
                    let ids = parameters.ID_all();
                    let decltypes = parameters.decltype_all();
                    ids.iter()
                        .enumerate()
                        .map(|(i, id)| {
                            let param_name = id.get_text();
                            let type_name = decltypes.get(i).map(|dt| dt.get_text());
                            FunctionParam {
                                name: param_name,
                                type_name,
                            }
                        })
                        .collect()
                } else {
                    vec![]
                };

                self.functions.push(Stmt::FunctionDef {
                    name,
                    params,
                    body: Box::new(block_stmt),
                });
            }

            // Collect body statements
            let stmts = self.visit_statement_list(&ctx.statement_all(), None)?;

            Ok(IrNode::Stmts(stmts))
        })();
        self.returns(result);
    }

    fn visit_function(&mut self, ctx: &FunctionContext<'_>) {
        self.returns(Err(anyhow!(
            "visit_function should not be called directly: {}",
            ctx.get_text()
        )));
    }

    fn visit_parameters(&mut self, ctx: &ParametersContext<'_>) {
        self.returns(Err(anyhow!(
            "visit_parameters should not be called directly: {}",
            ctx.get_text()
        )));
    }

    /// Grammar: (rstatement | dstatement (SEMICOLON | EOF))
    fn visit_statement(&mut self, ctx: &StatementContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: IF LP expression RP trailer (ELSE trailer)?
    fn visit_if(&mut self, ctx: &IfContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let cond = self.visit_expr_node(&*ctx.expression().unwrap())?;
            let trailers = ctx.trailer_all();

            let then_body = self.visit_stmt_node(&*trailers[0])?;
            let else_body = if trailers.len() > 1 {
                Some(Box::new(self.visit_stmt_node(&*trailers[1])?))
            } else {
                None
            };

            Ok(IrNode::Stmt(Stmt::If {
                cond,
                then_body: Box::new(then_body),
                else_body,
            }))
        })();
        self.returns(result);
    }

    fn visit_while(&mut self, ctx: &WhileContext<'_>) {
        self.returns(Err(anyhow!("unsupported: while loop: {}", ctx.get_text())));
    }

    /// Grammar: FOR LP initializer? SEMICOLON expression? SEMICOLON afterthought? RP (trailer | empty)
    fn visit_for(&mut self, ctx: &ForContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let init = if let Some(initializer) = ctx.initializer() {
                self.visit_stmt_node(&*initializer)?
            } else {
                Stmt::Empty
            };

            let cond = if let Some(expr) = ctx.expression() {
                self.visit_expr_node(&*expr)?
            } else {
                Expr::Literal { value: json!(true) }
            };

            let update = if let Some(afterthought) = ctx.afterthought() {
                self.visit_expr_node(&*afterthought.expression().unwrap())?
            } else {
                Expr::Literal { value: json!(true) }
            };

            let body = if let Some(trailer) = ctx.trailer() {
                self.visit_stmt_node(&*trailer)?
            } else {
                Stmt::Empty
            };

            Ok(IrNode::Stmt(Stmt::ForC {
                init: Box::new(init),
                cond,
                update,
                body: Box::new(body),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: FOR LP decltype ID COLON expression RP trailer
    fn visit_each(&mut self, ctx: &EachContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let var = ctx.ID().unwrap().get_text();
            let iter = self.visit_expr_node(&*ctx.expression().unwrap())?;
            let body = self.visit_stmt_node(&*ctx.trailer().unwrap())?;

            Ok(IrNode::Stmt(Stmt::ForEach {
                var,
                iter,
                body: Box::new(body),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: FOR LP ID IN expression RP trailer
    fn visit_ineach(&mut self, ctx: &IneachContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let var = ctx.ID().unwrap().get_text();
            let iter = self.visit_expr_node(&*ctx.expression().unwrap())?;
            let body = self.visit_stmt_node(&*ctx.trailer().unwrap())?;

            Ok(IrNode::Stmt(Stmt::ForEach {
                var,
                iter,
                body: Box::new(body),
            }))
        })();
        self.returns(result);
    }

    fn visit_try(&mut self, ctx: &TryContext<'_>) {
        // Transpile try as just the body block (ignore catch)
        let result: anyhow::Result<IrNode> = (|| {
            let body = self.visit_stmt_node(&*ctx.block().unwrap())?;
            Ok(IrNode::Stmt(body))
        })();
        self.returns(result);
    }

    fn visit_do(&mut self, ctx: &DoContext<'_>) {
        self.returns(Err(anyhow!("unsupported: do loop: {}", ctx.get_text())));
    }

    /// Grammar: declaration (SEMICOLON | EOF)
    fn visit_decl(&mut self, ctx: &DeclContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    fn visit_continue(&mut self, _ctx: &ContinueContext<'_>) {
        self.returns(Ok(IrNode::Stmt(Stmt::Empty)));
    }

    fn visit_break(&mut self, _ctx: &BreakContext<'_>) {
        self.returns(Ok(IrNode::Stmt(Stmt::Empty)));
    }

    /// Grammar: RETURN expression?
    fn visit_return(&mut self, ctx: &ReturnContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let value = ctx
                .expression()
                .map(|e| self.visit_expr_node(&*e))
                .transpose()?;
            Ok(IrNode::Stmt(Stmt::Return { value }))
        })();
        self.returns(result);
    }

    fn visit_throw(&mut self, _ctx: &ThrowContext<'_>) {
        // Throw becomes a return (Painless throws are usually error signalling)
        self.returns(Ok(IrNode::Stmt(Stmt::Return { value: None })));
    }

    /// Grammar: expression (dstatement terminal)
    fn visit_expr(&mut self, ctx: &ExprContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: block | statement
    fn visit_trailer(&mut self, ctx: &TrailerContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: LBRACK statement* dstatement? RBRACK
    fn visit_block(&mut self, ctx: &BlockContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let stmts = self.visit_statement_list(&ctx.statement_all(), ctx.dstatement())?;
            Ok(IrNode::Stmt(Stmt::Block(stmts)))
        })();
        self.returns(result);
    }

    fn visit_empty(&mut self, _ctx: &EmptyContext<'_>) {
        self.returns(Ok(IrNode::Stmt(Stmt::Empty)));
    }

    /// Grammar: declaration | expression
    fn visit_initializer(&mut self, ctx: &InitializerContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: expression
    fn visit_afterthought(&mut self, ctx: &AfterthoughtContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: decltype declvar (COMMA declvar)*
    fn visit_declaration(&mut self, ctx: &DeclarationContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let type_name = ctx.decltype().map(|dt| dt.get_text());
            let declvars = ctx.declvar_all();

            if declvars.len() == 1 {
                let dv = &declvars[0];
                let name = dv.ID().unwrap().get_text();
                let value = dv
                    .expression()
                    .map(|e| self.visit_expr_node(&*e))
                    .transpose()?;
                Ok(IrNode::Stmt(Stmt::VarDecl {
                    name,
                    type_name,
                    value,
                }))
            } else {
                // Multiple declarations: emit as a block
                let mut stmts = Vec::new();
                for dv in &declvars {
                    let name = dv.ID().unwrap().get_text();
                    let value = dv
                        .expression()
                        .map(|e| self.visit_expr_node(&*e))
                        .transpose()?;
                    stmts.push(Stmt::VarDecl {
                        name,
                        type_name: type_name.clone(),
                        value,
                    });
                }
                Ok(IrNode::Stmt(Stmt::Block(stmts)))
            }
        })();
        self.returns(result);
    }

    /// Grammar: typeid (LBRACE RBRACE)*
    fn visit_decltype(&mut self, ctx: &DecltypeContext<'_>) {
        self.returns(Ok(IrNode::TypeName(ctx.get_text())));
    }

    /// Grammar: (DEF | PRIMITIVE | ID (DOT DOTID)*)
    fn visit_typeid(&mut self, ctx: &TypeidContext<'_>) {
        self.returns(Ok(IrNode::TypeName(ctx.get_text())));
    }

    /// Grammar: ID (ASSIGN expression)?
    fn visit_declvar(&mut self, ctx: &DeclvarContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let name = ctx.ID().unwrap().get_text();
            let value = ctx
                .expression()
                .map(|e| self.visit_expr_node(&*e))
                .transpose()?;
            Ok(IrNode::Stmt(Stmt::VarDecl {
                name,
                type_name: None,
                value,
            }))
        })();
        self.returns(result);
    }

    fn visit_trap(&mut self, _ctx: &TrapContext<'_>) {
        self.returns(Ok(IrNode::Stmt(Stmt::Empty)));
    }

    /// Grammar: unary (pass-through for single noncondexpression)
    fn visit_single(&mut self, ctx: &SingleContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: noncondexpression (LT|LTE|GT|GTE|EQ|EQR|NE|NER) noncondexpression
    fn visit_comp(&mut self, ctx: &CompContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let children: Vec<_> = ctx.get_children().collect();
            if children.len() != 3 {
                return Err(anyhow!("comp must have 3 children, got {}", children.len()));
            }

            let left = self.visit_expr_node(&*children[0])?;
            let op_text = children[1].get_text();
            let right = self.visit_expr_node(&*children[2])?;

            let op = match op_text.as_str() {
                "==" | "===" => BinOp::Eq,
                "!=" | "!==" => BinOp::Ne,
                "<" => BinOp::Lt,
                "<=" => BinOp::Le,
                ">" => BinOp::Gt,
                ">=" => BinOp::Ge,
                other => return Err(anyhow!("unknown comparison operator: {other}")),
            };

            Ok(IrNode::Expr(Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: noncondexpression (BOOLAND|BOOLOR) noncondexpression
    fn visit_bool(&mut self, ctx: &BoolContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let children: Vec<_> = ctx.get_children().collect();
            if children.len() != 3 {
                return Err(anyhow!("bool must have 3 children, got {}", children.len()));
            }

            let left = self.visit_expr_node(&*children[0])?;
            let op = match children[1].get_text().as_str() {
                "&&" => BinOp::And,
                "||" => BinOp::Or,
                other => return Err(anyhow!("unknown boolean operator: {other}")),
            };
            let right = self.visit_expr_node(&*children[2])?;

            Ok(IrNode::Expr(Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: noncondexpression (MUL|DIV|REM|ADD|SUB|LSH|RSH|USH|BWAND|XOR|BWOR|FIND|MATCH) noncondexpression
    fn visit_binary(&mut self, ctx: &BinaryContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let children: Vec<_> = ctx.get_children().collect();
            if children.len() != 3 {
                return Err(anyhow!(
                    "binary must have 3 children, got {}",
                    children.len()
                ));
            }

            let left = self.visit_expr_node(&*children[0])?;
            let op_text = children[1].get_text();
            let right = self.visit_expr_node(&*children[2])?;

            let op = match op_text.as_str() {
                "+" => BinOp::Add,
                "-" => BinOp::Sub,
                "*" => BinOp::Mul,
                "/" => BinOp::Div,
                "%" => BinOp::Mod,
                "&" => BinOp::BitAnd,
                "|" => BinOp::BitOr,
                "^" => BinOp::BitXor,
                "<<" => BinOp::Shl,
                ">>" => BinOp::Shr,
                ">>>" => BinOp::Ushr,
                "=~" => BinOp::Find,
                "==~" => BinOp::Match,
                other => return Err(anyhow!("unknown binary operator: {other}")),
            };

            Ok(IrNode::Expr(Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: noncondexpression ELVIS noncondexpression
    fn visit_elvis(&mut self, ctx: &ElvisContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let children: Vec<_> = ctx.get_children().collect();
            if children.len() != 3 {
                return Err(anyhow!("elvis must have 3 children"));
            }
            let left = self.visit_expr_node(&*children[0])?;
            let right = self.visit_expr_node(&*children[2])?;

            // Elvis `a ?: b` → if a != null then a else b
            Ok(IrNode::Expr(Expr::Ternary {
                cond: Box::new(Expr::BinaryOp {
                    left: Box::new(left.clone()),
                    op: BinOp::Ne,
                    right: Box::new(Expr::Literal {
                        value: serde_json::Value::Null,
                    }),
                }),
                then_expr: Box::new(left),
                else_expr: Box::new(right),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: noncondexpression INSTANCEOF decltype
    fn visit_instanceof(&mut self, ctx: &InstanceofContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let expr = self.visit_expr_node(&*ctx.noncondexpression().unwrap())?;
            let type_name = ctx.decltype().unwrap().get_text();

            Ok(IrNode::Expr(Expr::InstanceOf {
                expr: Box::new(expr),
                type_name,
            }))
        })();
        self.returns(result);
    }

    /// Grammar: noncondexpression (pass-through)
    fn visit_nonconditional(&mut self, ctx: &NonconditionalContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: noncondexpression COND expression COLON expression
    fn visit_conditional(&mut self, ctx: &ConditionalContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let children: Vec<_> = ctx.get_children().collect();
            // cond ? then : else → 5 children
            if children.len() != 5 {
                return Err(anyhow!("conditional must have 5 children"));
            }
            let cond = self.visit_expr_node(&*children[0])?;
            let then_expr = self.visit_expr_node(&*children[2])?;
            let else_expr = self.visit_expr_node(&*children[4])?;

            Ok(IrNode::Expr(Expr::Ternary {
                cond: Box::new(cond),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: noncondexpression (ASSIGN|AADD|ASUB|...) expression
    fn visit_assignment(&mut self, ctx: &AssignmentContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let children: Vec<_> = ctx.get_children().collect();
            if children.len() != 3 {
                return Err(anyhow!(
                    "assignment must have 3 children, got {}",
                    children.len()
                ));
            }

            let left = self.visit_expr_node(&*children[0])?;
            let right = self.visit_expr_node(&*children[2])?;
            let op_text = children[1].get_text();

            if op_text == "=" {
                // Plain assignment
                match &left {
                    Expr::CtxAccess { path } => {
                        // ctx.field = value → CtxAssign
                        Ok(IrNode::Expr(Expr::CtxAssign {
                            path: path.clone(),
                            value: Box::new(right),
                        }))
                    }
                    _ => Ok(IrNode::Expr(Expr::Assign {
                        target: Box::new(left),
                        value: Box::new(right),
                    })),
                }
            } else {
                // Compound assignment
                let op = match op_text.as_str() {
                    "+=" => CompoundOp::AddAssign,
                    "-=" => CompoundOp::SubAssign,
                    "*=" => CompoundOp::MulAssign,
                    "/=" => CompoundOp::DivAssign,
                    "%=" => CompoundOp::ModAssign,
                    "&=" => CompoundOp::BitAndAssign,
                    "^=" => CompoundOp::BitXorAssign,
                    "|=" => CompoundOp::BitOrAssign,
                    "<<=" => CompoundOp::ShlAssign,
                    ">>=" => CompoundOp::ShrAssign,
                    ">>>=" => CompoundOp::UshrAssign,
                    other => return Err(anyhow!("unknown assignment operator: {other}")),
                };

                Ok(IrNode::Expr(Expr::CompoundAssign {
                    target: Box::new(left),
                    op,
                    value: Box::new(right),
                }))
            }
        })();
        self.returns(result);
    }

    /// Grammar: (INCR|DECR) chain
    fn visit_pre(&mut self, ctx: &PreContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let chain = self.visit_expr_node(&*ctx.chain().unwrap())?;
            let op = if ctx.INCR().is_some() {
                UnaryOp::PreInc
            } else {
                UnaryOp::PreDec
            };
            Ok(IrNode::Expr(Expr::UnaryOp {
                op,
                operand: Box::new(chain),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: (ADD|SUB) unary
    fn visit_addsub(&mut self, ctx: &AddsubContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let children: Vec<_> = ctx.get_children().collect();
            if children.len() != 2 {
                return Err(anyhow!("addsub must have 2 children"));
            }
            let op_text = children[0].get_text();
            let operand = self.visit_expr_node(&*children[1])?;

            if op_text == "-" {
                Ok(IrNode::Expr(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    operand: Box::new(operand),
                }))
            } else {
                // Unary + is a no-op
                Ok(IrNode::Expr(operand))
            }
        })();
        self.returns(result);
    }

    /// Grammar: unarynotaddsub (pass-through)
    fn visit_notaddsub(&mut self, ctx: &NotaddsubContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: chain (pass-through for unarynotaddsub → read)
    fn visit_read(&mut self, ctx: &ReadContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: chain (INCR | DECR)
    fn visit_post(&mut self, ctx: &PostContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let chain = self.visit_expr_node(&*ctx.chain().unwrap())?;
            let op = if ctx.INCR().is_some() {
                UnaryOp::PostInc
            } else {
                UnaryOp::PostDec
            };
            Ok(IrNode::Expr(Expr::UnaryOp {
                op,
                operand: Box::new(chain),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: (BOOLNOT | BWNOT) unary
    fn visit_not(&mut self, ctx: &NotContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let operand = self.visit_expr_node(&*ctx.unary().unwrap())?;

            if ctx.BWNOT().is_some() {
                // Bitwise NOT: ~x → -(x + 1) in two's complement
                Ok(IrNode::Expr(Expr::BinaryOp {
                    left: Box::new(Expr::UnaryOp {
                        op: UnaryOp::Neg,
                        operand: Box::new(Expr::BinaryOp {
                            left: Box::new(operand),
                            op: BinOp::Add,
                            right: Box::new(Expr::Literal { value: json!(1) }),
                        }),
                    }),
                    op: BinOp::Sub,
                    right: Box::new(Expr::Literal { value: json!(0) }),
                }))
            } else {
                Ok(IrNode::Expr(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                }))
            }
        })();
        self.returns(result);
    }

    /// Grammar: castexpression (pass-through)
    fn visit_cast(&mut self, ctx: &CastContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: LP primordefcasttype RP unary
    fn visit_primordefcast(&mut self, ctx: &PrimordefcastContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let type_name = ctx.primordefcasttype().unwrap().get_text();
            let expr = self.visit_expr_node(&*ctx.unary().unwrap())?;

            Ok(IrNode::Expr(Expr::Cast {
                type_name,
                expr: Box::new(expr),
            }))
        })();
        self.returns(result);
    }

    /// Grammar: LP refcasttype RP unarynotaddsub
    fn visit_refcast(&mut self, ctx: &RefcastContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let type_name = ctx.refcasttype().unwrap().get_text();
            let expr = self.visit_expr_node(&*ctx.unarynotaddsub().unwrap())?;

            Ok(IrNode::Expr(Expr::Cast {
                type_name,
                expr: Box::new(expr),
            }))
        })();
        self.returns(result);
    }

    fn visit_primordefcasttype(&mut self, ctx: &PrimordefcasttypeContext<'_>) {
        self.returns(Ok(IrNode::TypeName(ctx.get_text())));
    }

    fn visit_refcasttype(&mut self, ctx: &RefcasttypeContext<'_>) {
        self.returns(Ok(IrNode::TypeName(ctx.get_text())));
    }

    /// Grammar: primary postfix*
    ///
    /// This is the most complex visitor method. It handles:
    /// - `ctx.field.subfield` → CtxAccess
    /// - `ctx.field = value` (handled at assignment level)
    /// - `params.key` → ParamAccess
    /// - method calls: `.replace()`, `.toLowerCase()`, etc.
    /// - bracket access: `map[key]`
    /// - field access: `.field`
    fn visit_dynamic(&mut self, ctx: &DynamicContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let primary_node = self.visit_ir(&*ctx.primary().unwrap())?;
            let mut expr = primary_node.into_expr()?;

            let primary_text = ctx.primary().unwrap().get_text();
            let postfixes = ctx.postfix_all();

            // Identify ctx / params access chains
            if primary_text == "ctx" {
                // Build a path from subsequent field accesses
                let mut path = Vec::new();
                let mut i = 0;

                while i < postfixes.len() {
                    let pf = &postfixes[i];
                    if let Some(fa) = pf.fieldaccess() {
                        if let Some(dotid) = fa.DOTID() {
                            let field = dotid.get_text();
                            // ctx.remove('field') is handled below as method call
                            path.push(PathSegment::Static(field));
                            i += 1;
                            continue;
                        }
                        if let Some(dotint) = fa.DOTINTEGER() {
                            path.push(PathSegment::Static(dotint.get_text()));
                            i += 1;
                            continue;
                        }
                    }
                    if let Some(ba) = pf.braceaccess() {
                        let index_expr = self.visit_expr_node(&*ba.expression().unwrap())?;
                        // If the index is a string literal, make it static
                        if let Expr::Literal {
                            value: serde_json::Value::String(s),
                        } = &index_expr
                        {
                            path.push(PathSegment::Static(s.clone()));
                        } else {
                            path.push(PathSegment::Dynamic(Box::new(index_expr)));
                        }
                        i += 1;
                        continue;
                    }
                    // Hit a method call — stop building the path
                    break;
                }

                if i < postfixes.len() {
                    // There are remaining postfixes (method calls, etc.)
                    // Build CtxAccess for the path so far, then handle remaining
                    if !path.is_empty() {
                        expr = Expr::CtxAccess { path };
                    } else {
                        // Direct method on ctx (e.g., ctx.remove)
                        expr = Expr::CtxAccess { path: vec![] };
                    }

                    // Process remaining postfixes
                    for j in i..postfixes.len() {
                        expr = self.process_postfix(expr, &postfixes[j])?;
                    }
                } else if !path.is_empty() {
                    expr = Expr::CtxAccess { path };
                } else {
                    // Just "ctx" alone — represents the root
                    expr = Expr::CtxAccess { path: vec![] };
                }

                return Ok(IrNode::Expr(expr));
            }

            if primary_text == "params" {
                // Build params path
                let mut path = Vec::new();
                let mut i = 0;

                while i < postfixes.len() {
                    let pf = &postfixes[i];
                    if let Some(fa) = pf.fieldaccess() {
                        if let Some(dotid) = fa.DOTID() {
                            path.push(PathSegment::Static(dotid.get_text()));
                            i += 1;
                            continue;
                        }
                    }
                    if let Some(ba) = pf.braceaccess() {
                        let index_expr = self.visit_expr_node(&*ba.expression().unwrap())?;
                        if let Expr::Literal {
                            value: serde_json::Value::String(s),
                        } = &index_expr
                        {
                            path.push(PathSegment::Static(s.clone()));
                        } else {
                            path.push(PathSegment::Dynamic(Box::new(index_expr)));
                        }
                        i += 1;
                        continue;
                    }
                    break;
                }

                expr = Expr::ParamAccess { path };

                // Process remaining postfixes
                for j in i..postfixes.len() {
                    expr = self.process_postfix(expr, &postfixes[j])?;
                }

                return Ok(IrNode::Expr(expr));
            }

            // General case: process all postfixes
            // Check if primary is a class name for static calls
            let is_class_name = primary_text
                .chars()
                .next()
                .map_or(false, |c| c.is_uppercase())
                && !matches!(primary_text.as_str(), "true" | "false" | "null");

            if is_class_name && !postfixes.is_empty() {
                // Check if first postfix is a method call → static call
                if let Some(ci) = postfixes[0].callinvoke() {
                    let method = ci.DOTID().unwrap().get_text();
                    let args = self.collect_arguments(&ci.arguments().unwrap())?;

                    expr = Expr::StaticCall {
                        class: primary_text.clone(),
                        method,
                        args,
                    };

                    // Process remaining postfixes
                    for j in 1..postfixes.len() {
                        expr = self.process_postfix(expr, &postfixes[j])?;
                    }

                    return Ok(IrNode::Expr(expr));
                }
            }

            // Process all postfixes in order
            for pf in &postfixes {
                expr = self.process_postfix(expr, pf)?;
            }

            Ok(IrNode::Expr(expr))
        })();
        self.returns(result);
    }

    /// Grammar: arrayinitializer
    fn visit_newarray(&mut self, ctx: &NewarrayContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: LP expression RP
    fn visit_precedence(&mut self, ctx: &PrecedenceContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let expr = self.visit_expr_node(&*ctx.expression().unwrap())?;
            Ok(IrNode::Expr(expr))
        })();
        self.returns(result);
    }

    /// Grammar: OCTAL | HEX | INTEGER | DECIMAL
    fn visit_numeric(&mut self, ctx: &NumericContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            if let Some(dec) = ctx.DECIMAL() {
                let text = dec.get_text();
                let cleaned = text.trim_end_matches(['f', 'F', 'd', 'D', 'l', 'L']);
                let f: f64 = cleaned
                    .parse()
                    .map_err(|_| anyhow!("failed to parse decimal: {text}"))?;
                Ok(IrNode::Expr(Expr::Literal {
                    value: serde_json::Value::Number(
                        serde_json::Number::from_f64(f).unwrap_or_else(|| 0.into()),
                    ),
                }))
            } else if let Some(hex) = ctx.HEX() {
                let text = hex.get_text();
                let cleaned = text.trim_end_matches(['l', 'L']);
                let val = i64::from_str_radix(&cleaned[2..], 16)
                    .map_err(|_| anyhow!("failed to parse hex: {text}"))?;
                Ok(IrNode::Expr(Expr::Literal { value: json!(val) }))
            } else if let Some(octal) = ctx.OCTAL() {
                let text = octal.get_text();
                let cleaned = text.trim_end_matches(['l', 'L']);
                let val = i64::from_str_radix(&cleaned[1..], 8)
                    .map_err(|_| anyhow!("failed to parse octal: {text}"))?;
                Ok(IrNode::Expr(Expr::Literal { value: json!(val) }))
            } else if let Some(integer) = ctx.INTEGER() {
                let text = integer.get_text();
                let cleaned = text.trim_end_matches(['l', 'L', 'f', 'F', 'd', 'D']);
                let val: i64 = cleaned
                    .parse()
                    .map_err(|_| anyhow!("failed to parse integer: {text}"))?;
                Ok(IrNode::Expr(Expr::Literal { value: json!(val) }))
            } else {
                Err(anyhow!("unknown numeric type"))
            }
        })();
        self.returns(result);
    }

    fn visit_true(&mut self, _ctx: &TrueContext<'_>) {
        self.returns(Ok(IrNode::Expr(Expr::Literal { value: json!(true) })));
    }

    fn visit_false(&mut self, _ctx: &FalseContext<'_>) {
        self.returns(Ok(IrNode::Expr(Expr::Literal {
            value: json!(false),
        })));
    }

    fn visit_null(&mut self, _ctx: &NullContext<'_>) {
        self.returns(Ok(IrNode::Expr(Expr::Literal {
            value: serde_json::Value::Null,
        })));
    }

    /// Grammar: STRING (single-quoted in Painless)
    fn visit_string(&mut self, ctx: &StringContext<'_>) {
        let text = ctx.get_text();
        // Strip surrounding quotes (single or double)
        let inner = if (text.starts_with('\'') && text.ends_with('\''))
            || (text.starts_with('"') && text.ends_with('"'))
        {
            &text[1..text.len() - 1]
        } else {
            &text
        };
        // Unescape basic sequences
        let unescaped = inner
            .replace("\\'", "'")
            .replace("\\\"", "\"")
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
            .replace("\\\\", "\\");
        self.returns(Ok(IrNode::Expr(Expr::Literal {
            value: json!(unescaped),
        })));
    }

    /// Grammar: REGEX
    fn visit_regex(&mut self, ctx: &RegexContext<'_>) {
        let text = ctx.get_text();
        // Strip surrounding /slashes/
        let pattern = if text.starts_with('/') && text.ends_with('/') {
            text[1..text.len() - 1].to_string()
        } else {
            text
        };
        self.returns(Ok(IrNode::Expr(Expr::Regex { pattern })));
    }

    /// Grammar: listinitializer
    fn visit_listinit(&mut self, ctx: &ListinitContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: mapinitializer
    fn visit_mapinit(&mut self, ctx: &MapinitContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    /// Grammar: ID
    fn visit_variable(&mut self, ctx: &VariableContext<'_>) {
        let name = ctx.ID().unwrap().get_text();
        self.returns(Ok(IrNode::Expr(Expr::LocalVar { name })));
    }

    /// Grammar: (ID | DOLLAR) arguments
    fn visit_calllocal(&mut self, ctx: &CalllocalContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let name = ctx.ID().unwrap().get_text();
            let args = self.collect_arguments(&ctx.arguments().unwrap())?;

            Ok(IrNode::Expr(Expr::FunctionCall { name, args }))
        })();
        self.returns(result);
    }

    /// Grammar: NEW typeid arguments
    fn visit_newobject(&mut self, ctx: &NewobjectContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let type_name = ctx.typeid().unwrap().get_text();
            let arguments = ctx.arguments().unwrap().argument_all();

            match type_name.as_str() {
                "HashMap" | "LinkedHashMap" => Ok(IrNode::Expr(Expr::NewMap)),
                "ArrayList" | "LinkedList" => {
                    if arguments.is_empty() {
                        Ok(IrNode::Expr(Expr::NewList))
                    } else if arguments.len() == 1 {
                        // ArrayList(collection) → clone the collection
                        let arg = self.visit_expr_node(&*arguments[0])?;
                        Ok(IrNode::Expr(arg))
                    } else {
                        Err(anyhow!(
                            "unsupported ArrayList constructor with {} args",
                            arguments.len()
                        ))
                    }
                }
                other => Err(anyhow!("unsupported new object type: {other}")),
            }
        })();
        self.returns(result);
    }

    /// Grammar: (callinvoke | fieldaccess | braceaccess)
    fn visit_postfix(&mut self, ctx: &PostfixContext<'_>) {
        self.delegate(&*ctx.get_child(0).unwrap());
    }

    fn visit_postdot(&mut self, ctx: &PostdotContext<'_>) {
        self.returns(Err(anyhow!(
            "visit_postdot should not be called directly: {}",
            ctx.get_text()
        )));
    }

    fn visit_callinvoke(&mut self, ctx: &CallinvokeContext<'_>) {
        // Handled by visit_dynamic / process_postfix
        self.returns(Err(anyhow!(
            "visit_callinvoke should not be called directly: {}",
            ctx.get_text()
        )));
    }

    fn visit_fieldaccess(&mut self, ctx: &FieldaccessContext<'_>) {
        // Handled by visit_dynamic / process_postfix
        self.returns(Err(anyhow!(
            "visit_fieldaccess should not be called directly: {}",
            ctx.get_text()
        )));
    }

    fn visit_braceaccess(&mut self, ctx: &BraceaccessContext<'_>) {
        // Handled by visit_dynamic / process_postfix
        self.returns(Err(anyhow!(
            "visit_braceaccess should not be called directly: {}",
            ctx.get_text()
        )));
    }

    fn visit_newstandardarray(&mut self, _ctx: &NewstandardarrayContext<'_>) {
        self.returns(Ok(IrNode::Expr(Expr::ArrayLiteral { elements: vec![] })));
    }

    fn visit_newinitializedarray(&mut self, ctx: &NewinitializedarrayContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let args = ctx.expression_all();
            let mut elements = Vec::new();
            for arg in &args {
                elements.push(self.visit_expr_node(arg.deref())?);
            }
            Ok(IrNode::Expr(Expr::ArrayLiteral { elements }))
        })();
        self.returns(result);
    }

    /// Grammar: LBRACE expression (COMMA expression)* RBRACE
    fn visit_listinitializer(&mut self, ctx: &ListinitializerContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let args = ctx.expression_all();
            let mut elements = Vec::new();
            for arg in &args {
                elements.push(self.visit_expr_node(arg.deref())?);
            }
            Ok(IrNode::Expr(Expr::ArrayLiteral { elements }))
        })();
        self.returns(result);
    }

    /// Grammar: LBRACK (maptoken (COMMA maptoken)*)? (COLON)? RBRACK
    fn visit_mapinitializer(&mut self, ctx: &MapinitializerContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let tokens = ctx.maptoken_all();
            if tokens.is_empty() {
                return Ok(IrNode::Expr(Expr::MapLiteral { entries: vec![] }));
            }

            let mut entries = Vec::new();
            for mt in &tokens {
                let exprs = mt.expression_all();
                if exprs.len() == 2 {
                    let key = self.visit_expr_node(&*exprs[0])?;
                    let val = self.visit_expr_node(&*exprs[1])?;
                    entries.push((key, val));
                }
            }
            Ok(IrNode::Expr(Expr::MapLiteral { entries }))
        })();
        self.returns(result);
    }

    fn visit_maptoken(&mut self, _ctx: &MaptokenContext<'_>) {
        // Handled by visit_mapinitializer
        self.returns(Err(anyhow!("visit_maptoken should not be called directly")));
    }

    fn visit_arguments(&mut self, _ctx: &ArgumentsContext<'_>) {
        // Handled by collect_arguments helper method
        self.returns(Err(anyhow!(
            "visit_arguments should not be called directly"
        )));
    }

    fn visit_argument(&mut self, ctx: &ArgumentContext<'_>) {
        // An argument is either an expression or a lambda
        if let Some(lambda) = ctx.lambda() {
            self.delegate(&*lambda);
        } else {
            self.delegate(&*ctx.get_child(0).unwrap());
        }
    }

    /// Grammar: (lamtype | LP (lamtype (COMMA lamtype)*)? RP) ARROW (block | expression)
    fn visit_lambda(&mut self, ctx: &LambdaContext<'_>) {
        let result: anyhow::Result<IrNode> = (|| {
            let params: Vec<String> = ctx
                .lamtype_all()
                .iter()
                .map(|lt| lt.ID().unwrap().get_text())
                .collect();

            let body = if let Some(block) = ctx.block() {
                self.visit_stmt_node(&*block)?
            } else if let Some(expr) = ctx.expression() {
                let e = self.visit_expr_node(&*expr)?;
                Stmt::Return { value: Some(e) }
            } else {
                Stmt::Empty
            };

            Ok(IrNode::Expr(Expr::Lambda {
                params,
                body: Box::new(body),
            }))
        })();
        self.returns(result);
    }

    fn visit_lamtype(&mut self, ctx: &LamtypeContext<'_>) {
        self.returns(Ok(IrNode::TypeName(ctx.ID().unwrap().get_text())));
    }

    fn visit_classfuncref(&mut self, ctx: &ClassfuncrefContext<'_>) {
        self.returns(Err(anyhow!(
            "unsupported: class method reference: {}",
            ctx.get_text()
        )));
    }

    fn visit_constructorfuncref(&mut self, ctx: &ConstructorfuncrefContext<'_>) {
        self.returns(Err(anyhow!(
            "unsupported: constructor reference: {}",
            ctx.get_text()
        )));
    }

    fn visit_localfuncref(&mut self, ctx: &LocalfuncrefContext<'_>) {
        self.returns(Err(anyhow!(
            "unsupported: local method reference: {}",
            ctx.get_text()
        )));
    }
}

// Helper methods not part of the visitor trait
impl RustVisitor {
    /// Collect arguments from an ArgumentsContext into a list of expressions.
    fn collect_arguments(&mut self, ctx: &ArgumentsContext<'_>) -> anyhow::Result<Vec<Expr>> {
        let mut args = Vec::new();
        for arg in ctx.argument_all() {
            if let Some(lambda) = arg.lambda() {
                let params: Vec<String> = lambda
                    .lamtype_all()
                    .iter()
                    .map(|lt| lt.ID().unwrap().get_text())
                    .collect();

                let body = if let Some(block) = lambda.block() {
                    self.visit_stmt_node(&*block)?
                } else if let Some(expr) = lambda.expression() {
                    let e = self.visit_expr_node(&*expr)?;
                    Stmt::Return { value: Some(e) }
                } else {
                    Stmt::Empty
                };

                args.push(Expr::Lambda {
                    params,
                    body: Box::new(body),
                });
            } else {
                let expr = self.visit_expr_node(&*arg)?;
                args.push(expr);
            }
        }
        Ok(args)
    }

    /// Process a single postfix (field access, method call, or bracket access)
    /// applied to an existing expression.
    fn process_postfix(
        &mut self,
        mut expr: Expr,
        postfix: &PostfixContextAll<'_>,
    ) -> anyhow::Result<Expr> {
        if let Some(ci) = postfix.callinvoke() {
            let method = ci.DOTID().unwrap().get_text();
            let is_null_safe = ci.NSDOT().is_some();
            let args = self.collect_arguments(&ci.arguments().unwrap())?;

            // Special handling for ctx.remove()
            if method == "remove" {
                if let Expr::CtxAccess { path } = &expr {
                    if path.is_empty() && args.len() == 1 {
                        // ctx.remove('field') → CtxRemove
                        let field = match &args[0] {
                            Expr::Literal {
                                value: serde_json::Value::String(s),
                            } => {
                                vec![PathSegment::Static(s.clone())]
                            }
                            other => vec![PathSegment::Dynamic(Box::new(other.clone()))],
                        };
                        return Ok(Expr::CtxRemove { path: field });
                    }
                }
            }

            if is_null_safe {
                // Null-safe method call: emit as conditional
                expr = Expr::MethodCall {
                    receiver: Box::new(expr),
                    method,
                    args,
                };
            } else {
                expr = Expr::MethodCall {
                    receiver: Box::new(expr),
                    method,
                    args,
                };
            }
        } else if let Some(fa) = postfix.fieldaccess() {
            let is_null_safe = fa.NSDOT().is_some();

            if let Some(dotid) = fa.DOTID() {
                let field = dotid.get_text();
                if is_null_safe {
                    expr = Expr::NullSafeAccess {
                        base: Box::new(expr),
                        field,
                    };
                } else {
                    // Regular field access on a non-ctx expression
                    // Treat as method-like for things like entry.getKey()
                    // or as bracket access for map.field
                    expr = Expr::BraceAccess {
                        base: Box::new(expr),
                        index: Box::new(Expr::Literal {
                            value: json!(field),
                        }),
                    };
                }
            } else if let Some(dotint) = fa.DOTINTEGER() {
                let idx_text = dotint.get_text();
                let idx: i64 = idx_text.parse().unwrap_or(0);
                expr = Expr::BraceAccess {
                    base: Box::new(expr),
                    index: Box::new(Expr::Literal { value: json!(idx) }),
                };
            }
        } else if let Some(ba) = postfix.braceaccess() {
            let index = self.visit_expr_node(&*ba.expression().unwrap())?;
            expr = Expr::BraceAccess {
                base: Box::new(expr),
                index: Box::new(index),
            };
        }

        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transpile(source: &str) -> anyhow::Result<PainlessScript> {
        RustVisitor::transpile(source)
    }

    #[test]
    fn simple_assignment() {
        let script = transpile("ctx.event.kind = 'event';").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::Expr(Expr::CtxAssign { path, .. }) => {
                assert_eq!(path.len(), 2);
            }
            other => panic!("expected CtxAssign, got {other:?}"),
        }
    }

    #[test]
    fn if_null_check() {
        let script = transpile("if (ctx.field != null) { ctx.result = 'yes'; }").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::If { cond, .. } => {
                assert!(matches!(cond, Expr::BinaryOp { op: BinOp::Ne, .. }));
            }
            other => panic!("expected If, got {other:?}"),
        }
    }

    #[test]
    fn arithmetic() {
        let script = transpile("ctx.event.duration = ctx.event.duration * 1000000;").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::Expr(Expr::CtxAssign { value, .. }) => {
                assert!(matches!(
                    value.as_ref(),
                    Expr::BinaryOp { op: BinOp::Mul, .. }
                ));
            }
            other => panic!("expected CtxAssign with Mul, got {other:?}"),
        }
    }

    #[test]
    fn variable_declaration() {
        let script = transpile("def x = 42;").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::VarDecl {
                name,
                value: Some(Expr::Literal { value }),
                ..
            } => {
                assert_eq!(name, "x");
                assert_eq!(*value, json!(42));
            }
            other => panic!("expected VarDecl, got {other:?}"),
        }
    }

    #[test]
    fn string_method_call() {
        let script = transpile("ctx.field = ctx.field.replace('-', '.');").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::Expr(Expr::CtxAssign { value, .. }) => {
                assert!(
                    matches!(value.as_ref(), Expr::MethodCall { method, .. } if method == "replace")
                );
            }
            other => panic!("expected CtxAssign with replace, got {other:?}"),
        }
    }

    #[test]
    fn for_each_loop() {
        let script = transpile("for (def entry : ctx.items) { ctx.result = entry; }").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::ForEach { var, .. } => {
                assert_eq!(var, "entry");
            }
            other => panic!("expected ForEach, got {other:?}"),
        }
    }

    #[test]
    fn null_literal() {
        let script = transpile("ctx.field = null;").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::Expr(Expr::CtxAssign { value, .. }) => {
                assert!(matches!(
                    value.as_ref(),
                    Expr::Literal {
                        value: serde_json::Value::Null
                    }
                ));
            }
            other => panic!("expected CtxAssign with null, got {other:?}"),
        }
    }

    #[test]
    fn boolean_expression() {
        let script = transpile("if (ctx.a != null && ctx.b != null) { ctx.c = 'ok'; }").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::If { cond, .. } => {
                assert!(matches!(cond, Expr::BinaryOp { op: BinOp::And, .. }));
            }
            other => panic!("expected If with And, got {other:?}"),
        }
    }

    #[test]
    fn ctx_remove() {
        let script = transpile("ctx.remove('old_field');").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::Expr(Expr::CtxRemove { path }) => {
                assert_eq!(path.len(), 1);
                match &path[0] {
                    PathSegment::Static(s) => assert_eq!(s, "old_field"),
                    other => panic!("expected Static path, got {other:?}"),
                }
            }
            other => panic!("expected CtxRemove, got {other:?}"),
        }
    }

    #[test]
    fn params_access() {
        let script = transpile("def x = params.nano;").unwrap();
        assert_eq!(script.body.len(), 1);
        match &script.body[0] {
            Stmt::VarDecl {
                value: Some(Expr::ParamAccess { path }),
                ..
            } => {
                assert_eq!(path.len(), 1);
            }
            other => panic!("expected ParamAccess, got {other:?}"),
        }
    }

    #[test]
    fn new_hashmap() {
        let script = transpile("def m = new HashMap();").unwrap();
        match &script.body[0] {
            Stmt::VarDecl {
                value: Some(Expr::NewMap),
                ..
            } => {}
            other => panic!("expected NewMap, got {other:?}"),
        }
    }

    #[test]
    fn numeric_hex() {
        let script = transpile("def x = 0xFF;").unwrap();
        match &script.body[0] {
            Stmt::VarDecl {
                value: Some(Expr::Literal { value }),
                ..
            } => {
                assert_eq!(*value, json!(255));
            }
            other => panic!("expected Literal 255, got {other:?}"),
        }
    }

    #[test]
    fn ternary_expression() {
        let script = transpile("ctx.x = ctx.a != null ? ctx.a : 'default';").unwrap();
        match &script.body[0] {
            Stmt::Expr(Expr::CtxAssign { value, .. }) => {
                assert!(matches!(value.as_ref(), Expr::Ternary { .. }));
            }
            other => panic!("expected Ternary, got {other:?}"),
        }
    }

    #[test]
    fn instanceof_check() {
        let script =
            transpile("if (ctx.field instanceof String) { ctx.result = 'string'; }").unwrap();
        match &script.body[0] {
            Stmt::If { cond, .. } => {
                assert!(
                    matches!(cond, Expr::InstanceOf { type_name, .. } if type_name == "String")
                );
            }
            other => panic!("expected InstanceOf, got {other:?}"),
        }
    }

    #[test]
    fn compound_assignment() {
        let script = transpile("ctx.count += 1;").unwrap();
        match &script.body[0] {
            Stmt::Expr(Expr::CompoundAssign {
                op: CompoundOp::AddAssign,
                ..
            }) => {}
            other => panic!("expected CompoundAssign AddAssign, got {other:?}"),
        }
    }

    #[test]
    fn static_method_call() {
        let script = transpile("def x = Long.parseLong(ctx.field);").unwrap();
        match &script.body[0] {
            Stmt::VarDecl {
                value: Some(Expr::StaticCall { class, method, .. }),
                ..
            } => {
                assert_eq!(class, "Long");
                assert_eq!(method, "parseLong");
            }
            other => panic!("expected StaticCall, got {other:?}"),
        }
    }

    /// End-to-end test: Painless source → IR → Rust source string.
    #[test]
    fn e2e_duration_multiply() {
        use super::super::emitter;

        let source = "ctx.event.duration = ctx.event.duration * params.nano;";
        let script = transpile(source).unwrap();
        let (_, body) = emitter::emit_script(&script, "    ");

        // Should contain event.get for reading
        assert!(body.contains("event.get("), "missing event.get in: {body}");
        // Should contain event.set for writing
        assert!(body.contains("event.set("), "missing event.set in: {body}");
        // Should use painless_mul for multiplication
        assert!(
            body.contains("painless_mul"),
            "missing painless_mul in: {body}"
        );
        // Should reference PARAMS for params access
        assert!(body.contains("PARAMS"), "missing PARAMS in: {body}");
    }

    #[test]
    fn e2e_if_null_guard() {
        use super::super::emitter;

        let source = r#"if (ctx.message != null) { ctx.event.original = ctx.message; }"#;
        let script = transpile(source).unwrap();
        let (_, body) = emitter::emit_script(&script, "    ");

        // Should emit a null check
        assert!(body.contains("is_null()"), "missing null check in: {body}");
        // Should emit event.set
        assert!(body.contains("event.set("), "missing event.set in: {body}");
    }
}
