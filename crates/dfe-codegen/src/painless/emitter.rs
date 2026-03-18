// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless IR to Rust source string emitter.
//!
//! Renders `ir::Expr` and `ir::Stmt` nodes into Rust source code that
//! operates on `dfe_runtime::event::Event` via the prelude API.

use super::ir::{BinOp, CompoundOp, Expr, PainlessScript, PathSegment, Stmt, UnaryOp};

/// Emit a complete Painless script as Rust source code.
///
/// Returns two strings:
/// - `helpers`: Rust helper functions to emit before the transform method
/// - `body`: Rust code to emit inline in the transform method body
pub fn emit_script(script: &PainlessScript, indent: &str) -> (String, String) {
    let mut helpers = String::new();
    for func in &script.functions {
        helpers.push_str(&emit_stmt(func, ""));
        helpers.push('\n');
    }

    let mut body = String::new();
    for stmt in &script.body {
        body.push_str(&emit_stmt(stmt, indent));
    }

    (helpers, body)
}

/// Emit a statement as Rust source.
pub(crate) fn emit_stmt(stmt: &Stmt, pad: &str) -> String {
    match stmt {
        Stmt::Block(stmts) => {
            let mut out = format!("{pad}{{\n");
            let inner = format!("{pad}    ");
            for s in stmts {
                out.push_str(&emit_stmt(s, &inner));
            }
            out.push_str(&format!("{pad}}}\n"));
            out
        }

        Stmt::Expr(expr) => {
            let e = emit_expr(expr);
            format!("{pad}{e};\n")
        }

        Stmt::VarDecl { name, value, .. } => {
            let safe_name = sanitize_ident(name);
            if let Some(val) = value {
                let v = emit_expr(val);
                format!("{pad}let mut {safe_name} = {v};\n")
            } else {
                format!("{pad}let mut {safe_name} = Value::Null;\n")
            }
        }

        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            let c = emit_condition(cond);
            let mut out = format!("{pad}if {c} {{\n");
            let inner = format!("{pad}    ");
            out.push_str(&emit_stmt_body(then_body, &inner));
            if let Some(else_stmt) = else_body {
                out.push_str(&format!("{pad}}} else {{\n"));
                out.push_str(&emit_stmt_body(else_stmt, &inner));
            }
            out.push_str(&format!("{pad}}}\n"));
            out
        }

        Stmt::ForC {
            init,
            cond,
            update,
            body,
        } => {
            let mut out = String::new();
            // Emit init as a statement
            out.push_str(&emit_stmt(init, pad));
            let c = emit_condition(cond);
            out.push_str(&format!("{pad}while {c} {{\n"));
            let inner = format!("{pad}    ");
            out.push_str(&emit_stmt_body(body, &inner));
            // Emit update at end of loop
            let u = emit_expr(update);
            out.push_str(&format!("{inner}{u};\n"));
            out.push_str(&format!("{pad}}}\n"));
            out
        }

        Stmt::ForEach { var, iter, body } => {
            let safe_var = sanitize_ident(var);
            let iter_expr = emit_expr(iter);
            let inner = format!("{pad}    ");
            let mut out = format!(
                "{pad}if let Some(iter_arr__) = ({iter_expr}).as_array() {{\n\
                 {inner}for {safe_var}_ref__ in iter_arr__ {{\n\
                 {inner}    let mut {safe_var} = {safe_var}_ref__.clone();\n"
            );
            let body_inner = format!("{inner}    ");
            out.push_str(&emit_stmt_body(body, &body_inner));
            out.push_str(&format!("{inner}}}\n"));
            out.push_str(&format!("{pad}}}\n"));
            out
        }

        Stmt::Return { value } => {
            if let Some(val) = value {
                let v = emit_expr(val);
                format!("{pad}return {v};\n")
            } else {
                format!("{pad}return;\n")
            }
        }

        Stmt::FunctionDef { name, params, body } => {
            let safe_name = sanitize_ident(name);
            let param_list: Vec<String> = params
                .iter()
                .map(|p| format!("{}: &Value", sanitize_ident(&p.name)))
                .collect();
            let params_str = param_list.join(", ");
            let mut out = format!("{pad}fn painless_{safe_name}({params_str}) -> Value {{\n");
            let inner = format!("{pad}    ");
            out.push_str(&emit_stmt_body(body, &inner));
            out.push_str(&format!("{inner}Value::Null\n"));
            out.push_str(&format!("{pad}}}\n"));
            out
        }

        Stmt::Empty => String::new(),
    }
}

/// Emit the inner statements of a body (unwrapping Block if present).
fn emit_stmt_body(stmt: &Stmt, pad: &str) -> String {
    match stmt {
        Stmt::Block(stmts) => {
            let mut out = String::new();
            for s in stmts {
                out.push_str(&emit_stmt(s, pad));
            }
            out
        }
        other => emit_stmt(other, pad),
    }
}

/// Emit an expression as Rust source.
pub(crate) fn emit_expr(expr: &Expr) -> String {
    match expr {
        Expr::CtxAccess { path } => {
            let dotted = static_path_to_dotted(path);
            if dotted.is_some() {
                let p = dotted.as_deref().unwrap_or("");
                format!("event.get(\"{p}\").cloned().unwrap_or(Value::Null)")
            } else {
                let built = emit_dynamic_path(path);
                format!("event.get(&{built}).cloned().unwrap_or(Value::Null)")
            }
        }

        Expr::CtxAssign { path, value } => {
            let v = emit_expr(value);
            let dotted = static_path_to_dotted(path);
            if let Some(p) = dotted {
                format!("event.set(\"{p}\", {v})")
            } else {
                let built = emit_dynamic_path(path);
                format!("event.set(&{built}, {v})")
            }
        }

        Expr::CtxRemove { path } => {
            let dotted = static_path_to_dotted(path);
            if let Some(p) = dotted {
                format!("event.remove(\"{p}\")")
            } else {
                let built = emit_dynamic_path(path);
                format!("event.remove(&{built})")
            }
        }

        Expr::ParamAccess { path } => {
            // Params are inlined at codegen time via the params module.
            // This fallback emits access to a PARAMS lazy_static.
            let dotted = static_path_to_dotted(path);
            if let Some(p) = dotted {
                let parts: Vec<&str> = p.split('.').collect();
                let mut access = "PARAMS".to_string();
                for part in parts {
                    access = format!("{access}[\"{}\"]\n", part);
                }
                format!("{access}.clone()")
            } else {
                "Value::Null /* dynamic param access */".to_string()
            }
        }

        Expr::LocalVar { name } => sanitize_ident(name),

        Expr::Literal { value } => emit_literal(value),

        Expr::BinaryOp { left, op, right } => emit_binary(left, *op, right),

        Expr::UnaryOp { op, operand } => {
            let o = emit_expr(operand);
            match op {
                UnaryOp::Not => format!("!painless_truthy(&{o})"),
                UnaryOp::Neg => format!("json!(-painless_to_i64(&{o}))"),
                UnaryOp::PreInc => {
                    format!("{{ {o} = painless_add(&{o}, &json!(1)); {o}.clone() }}")
                }
                UnaryOp::PreDec => {
                    format!("{{ {o} = painless_sub(&{o}, &json!(1)); {o}.clone() }}")
                }
                UnaryOp::PostInc => {
                    format!(
                        "{{ let tmp__ = {o}.clone(); {o} = painless_add(&{o}, &json!(1)); tmp__ }}"
                    )
                }
                UnaryOp::PostDec => {
                    format!(
                        "{{ let tmp__ = {o}.clone(); {o} = painless_sub(&{o}, &json!(1)); tmp__ }}"
                    )
                }
            }
        }

        Expr::MethodCall {
            receiver,
            method,
            args,
        } => emit_method_call(receiver, method, args),

        Expr::StaticCall {
            class,
            method,
            args,
        } => emit_static_call(class, method, args),

        Expr::InstanceOf { expr, type_name } => {
            let e = emit_expr(expr);
            match type_name.as_str() {
                "Map" | "HashMap" => format!("({e}).is_object()"),
                "List" | "ArrayList" => format!("({e}).is_array()"),
                "String" => format!("({e}).is_string()"),
                "long" | "int" | "Long" | "Integer" => {
                    format!("(({e}).is_i64() || ({e}).is_u64())")
                }
                "double" | "float" | "Double" | "Float" => format!("({e}).is_f64()"),
                "boolean" | "Boolean" => format!("({e}).is_boolean()"),
                _ => format!("/* instanceof {type_name} */ true"),
            }
        }

        Expr::Cast { type_name, expr } => {
            let e = emit_expr(expr);
            match type_name.as_str() {
                "int" | "long" | "Long" | "Integer" => {
                    format!("json!(painless_to_i64(&{e}))")
                }
                "double" | "float" | "Double" | "Float" => {
                    format!("json!(painless_to_f64(&{e}))")
                }
                "String" => format!("json!(painless_to_string(&{e}))"),
                "char" => {
                    format!(
                        "json!(painless_to_string(&{e}).chars().next().unwrap_or('\\0').to_string())"
                    )
                }
                _ => e,
            }
        }

        Expr::NewMap => "Value::Object(serde_json::Map::new())".to_string(),

        Expr::NewList => "Value::Array(vec![])".to_string(),

        Expr::MapLiteral { entries } => {
            if entries.is_empty() {
                return "Value::Object(serde_json::Map::new())".to_string();
            }
            let mut out = "{\n    let mut map__ = serde_json::Map::new();\n".to_string();
            for (k, v) in entries {
                let key = emit_expr(k);
                let val = emit_expr(v);
                out.push_str(&format!(
                    "    map__.insert(painless_to_string(&{key}), {val});\n"
                ));
            }
            out.push_str("    Value::Object(map__)\n}");
            out
        }

        Expr::ArrayLiteral { elements } => {
            let elems: Vec<String> = elements.iter().map(emit_expr).collect();
            format!("json!([{}])", elems.join(", "))
        }

        Expr::Lambda { params, body } => {
            let param_list: Vec<String> = params
                .iter()
                .map(|p| format!("{}", sanitize_ident(p)))
                .collect();
            let body_str = emit_stmt(body, "");
            format!("|{}| {{ {} }}", param_list.join(", "), body_str.trim())
        }

        Expr::FunctionCall { name, args } => {
            let safe_name = sanitize_ident(name);
            let arg_list: Vec<String> = args.iter().map(|a| format!("&{}", emit_expr(a))).collect();
            format!("painless_{safe_name}({})", arg_list.join(", "))
        }

        Expr::Regex { pattern } => {
            let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"");
            format!(
                "regex::Regex::new(\"{escaped}\").unwrap_or_else(|_| regex::Regex::new(\".\").unwrap())"
            )
        }

        Expr::NullSafeAccess { base, field } => {
            let b = emit_expr(base);
            format!(
                "({b}).as_object().and_then(|o__| o__.get(\"{field}\")).cloned().unwrap_or(Value::Null)"
            )
        }

        Expr::BraceAccess { base, index } => {
            let b = emit_expr(base);
            let i = emit_expr(index);
            format!(
                "if let Some(s__) = ({i}).as_str() {{ \
                    ({b}).as_object().and_then(|o__| o__.get(s__)).cloned().unwrap_or(Value::Null) \
                }} else if let Some(n__) = ({i}).as_u64() {{ \
                    ({b}).as_array().and_then(|a__| a__.get(n__ as usize)).cloned().unwrap_or(Value::Null) \
                }} else {{ Value::Null }}"
            )
        }

        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => {
            let c = emit_condition(cond);
            let t = emit_expr(then_expr);
            let e = emit_expr(else_expr);
            format!("if {c} {{ {t} }} else {{ {e} }}")
        }

        Expr::CompoundAssign { target, op, value } => {
            let t = emit_expr(target);
            let v = emit_expr(value);
            let helper = match op {
                CompoundOp::AddAssign => "painless_add",
                CompoundOp::SubAssign => "painless_sub",
                CompoundOp::MulAssign => "painless_mul",
                CompoundOp::DivAssign => "painless_div",
                CompoundOp::ModAssign => "painless_mod",
                CompoundOp::BitAndAssign => {
                    return format!("{t} = json!(painless_to_i64(&{t}) & painless_to_i64(&{v}))");
                }
                CompoundOp::BitOrAssign => {
                    return format!("{t} = json!(painless_to_i64(&{t}) | painless_to_i64(&{v}))");
                }
                CompoundOp::BitXorAssign => {
                    return format!("{t} = json!(painless_to_i64(&{t}) ^ painless_to_i64(&{v}))");
                }
                CompoundOp::ShlAssign => {
                    return format!("{t} = json!(painless_to_i64(&{t}) << painless_to_i64(&{v}))");
                }
                CompoundOp::ShrAssign => {
                    return format!("{t} = json!(painless_to_i64(&{t}) >> painless_to_i64(&{v}))");
                }
                CompoundOp::UshrAssign => {
                    return format!(
                        "{t} = json!((painless_to_i64(&{t}) as u64 >> painless_to_i64(&{v}) as u64) as i64)"
                    );
                }
            };
            format!("{t} = {helper}(&{t}, &{v})")
        }

        Expr::Assign { target, value } => {
            let t = emit_expr(target);
            let v = emit_expr(value);
            format!("{t} = {v}")
        }
    }
}

/// Emit a binary operation.
fn emit_binary(left: &Expr, op: BinOp, right: &Expr) -> String {
    let l = emit_expr(left);
    let r = emit_expr(right);

    match op {
        BinOp::Add => format!("painless_add(&{l}, &{r})"),
        BinOp::Sub => format!("painless_sub(&{l}, &{r})"),
        BinOp::Mul => format!("painless_mul(&{l}, &{r})"),
        BinOp::Div => format!("painless_div(&{l}, &{r})"),
        BinOp::Mod => format!("painless_mod(&{l}, &{r})"),

        BinOp::BitAnd => format!("json!(painless_to_i64(&{l}) & painless_to_i64(&{r}))"),
        BinOp::BitOr => format!("json!(painless_to_i64(&{l}) | painless_to_i64(&{r}))"),
        BinOp::BitXor => format!("json!(painless_to_i64(&{l}) ^ painless_to_i64(&{r}))"),
        BinOp::Shl => format!("json!(painless_to_i64(&{l}) << painless_to_i64(&{r}))"),
        BinOp::Shr => format!("json!(painless_to_i64(&{l}) >> painless_to_i64(&{r}))"),
        BinOp::Ushr => {
            format!("json!((painless_to_i64(&{l}) as u64 >> painless_to_i64(&{r}) as u64) as i64)")
        }

        BinOp::Eq => format!("json!(painless_eq(&{l}, &{r}))"),
        BinOp::Ne => format!("json!(!painless_eq(&{l}, &{r}))"),
        BinOp::Lt => format!("json!(painless_cmp(&{l}, &{r}).map_or(false, |o| o.is_lt()))"),
        BinOp::Le => format!("json!(painless_cmp(&{l}, &{r}).map_or(false, |o| !o.is_gt()))"),
        BinOp::Gt => format!("json!(painless_cmp(&{l}, &{r}).map_or(false, |o| o.is_gt()))"),
        BinOp::Ge => format!("json!(painless_cmp(&{l}, &{r}).map_or(false, |o| !o.is_lt()))"),

        BinOp::And => format!("json!(painless_truthy(&{l}) && painless_truthy(&{r}))"),
        BinOp::Or => format!("json!(painless_truthy(&{l}) || painless_truthy(&{r}))"),

        BinOp::Find | BinOp::Match => {
            format!("/* regex op */ json!(false)")
        }
    }
}

/// Emit a condition expression for use in `if` statements.
/// Wraps the expression in `painless_truthy` if needed.
fn emit_condition(expr: &Expr) -> String {
    match expr {
        // Direct boolean comparisons don't need wrapping
        Expr::BinaryOp {
            op: op @ (BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge),
            left,
            right,
        } => {
            let l = emit_expr(left);
            let r = emit_expr(right);
            match op {
                BinOp::Eq => {
                    // Special case: comparison to null
                    if matches!(right.as_ref(), Expr::Literal { value } if value.is_null()) {
                        return format!("({l}).is_null()");
                    }
                    if matches!(left.as_ref(), Expr::Literal { value } if value.is_null()) {
                        return format!("({r}).is_null()");
                    }
                    format!("painless_eq(&{l}, &{r})")
                }
                BinOp::Ne => {
                    if matches!(right.as_ref(), Expr::Literal { value } if value.is_null()) {
                        return format!("!({l}).is_null()");
                    }
                    if matches!(left.as_ref(), Expr::Literal { value } if value.is_null()) {
                        return format!("!({r}).is_null()");
                    }
                    format!("!painless_eq(&{l}, &{r})")
                }
                BinOp::Lt => format!("painless_cmp(&{l}, &{r}).map_or(false, |o| o.is_lt())"),
                BinOp::Le => format!("painless_cmp(&{l}, &{r}).map_or(false, |o| !o.is_gt())"),
                BinOp::Gt => format!("painless_cmp(&{l}, &{r}).map_or(false, |o| o.is_gt())"),
                BinOp::Ge => format!("painless_cmp(&{l}, &{r}).map_or(false, |o| !o.is_lt())"),
                _ => unreachable!(),
            }
        }

        // Boolean operators
        Expr::BinaryOp {
            op: BinOp::And,
            left,
            right,
        } => {
            let l = emit_condition(left);
            let r = emit_condition(right);
            format!("({l}) && ({r})")
        }
        Expr::BinaryOp {
            op: BinOp::Or,
            left,
            right,
        } => {
            let l = emit_condition(left);
            let r = emit_condition(right);
            format!("({l}) || ({r})")
        }

        // instanceof is already boolean
        Expr::InstanceOf { .. } => emit_expr(expr),

        // Unary not
        Expr::UnaryOp {
            op: UnaryOp::Not,
            operand,
        } => {
            let inner = emit_condition(operand);
            format!("!({inner})")
        }

        // Everything else: wrap in painless_truthy
        other => {
            let e = emit_expr(other);
            format!("painless_truthy(&{e})")
        }
    }
}

/// Emit a method call on a receiver.
fn emit_method_call(receiver: &Expr, method: &str, args: &[Expr]) -> String {
    let recv = emit_expr(receiver);

    match method {
        // String methods
        "replace" if args.len() == 2 => {
            let a = emit_expr(&args[0]);
            let b = emit_expr(&args[1]);
            format!(
                "json!(({recv}).as_str().unwrap_or(\"\").replace(\
                    painless_to_string(&{a}).as_str(), \
                    painless_to_string(&{b}).as_str()))"
            )
        }
        "toLowerCase" => format!("json!(({recv}).as_str().unwrap_or(\"\").to_lowercase())"),
        "toUpperCase" => format!("json!(({recv}).as_str().unwrap_or(\"\").to_uppercase())"),
        "trim" => format!("json!(({recv}).as_str().unwrap_or(\"\").trim())"),
        "startsWith" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!(
                "json!(({recv}).as_str().unwrap_or(\"\").starts_with(painless_to_string(&{a}).as_str()))"
            )
        }
        "endsWith" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!(
                "json!(({recv}).as_str().unwrap_or(\"\").ends_with(painless_to_string(&{a}).as_str()))"
            )
        }
        "substring" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!(
                "{{ let s__ = ({recv}).as_str().unwrap_or(\"\"); \
                 let start__ = painless_to_i64(&{a}) as usize; \
                 json!(&s__[start__.min(s__.len())..]) }}"
            )
        }
        "substring" if args.len() == 2 => {
            let a = emit_expr(&args[0]);
            let b = emit_expr(&args[1]);
            format!(
                "{{ let s__ = ({recv}).as_str().unwrap_or(\"\"); \
                 let start__ = painless_to_i64(&{a}) as usize; \
                 let end__ = painless_to_i64(&{b}) as usize; \
                 json!(&s__[start__.min(s__.len())..end__.min(s__.len())]) }}"
            )
        }
        "splitOnToken" | "split" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!(
                "json!(({recv}).as_str().unwrap_or(\"\").split(\
                    painless_to_string(&{a}).as_str())\
                    .collect::<Vec<_>>())"
            )
        }
        "contains" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            // Works for both String.contains and List.contains
            format!(
                "json!(if ({recv}).is_string() {{ \
                    ({recv}).as_str().unwrap_or(\"\").contains(painless_to_string(&{a}).as_str()) \
                }} else if ({recv}).is_array() {{ \
                    ({recv}).as_array().map_or(false, |arr__| arr__.contains(&{a})) \
                }} else {{ false }})"
            )
        }
        "charAt" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!(
                "json!(({recv}).as_str().unwrap_or(\"\").chars()\
                    .nth(painless_to_i64(&{a}) as usize)\
                    .map_or(String::new(), |c__| c__.to_string()))"
            )
        }
        "isEmpty" => {
            format!("json!(({recv}).as_str().map_or(true, |s__| s__.is_empty()))")
        }
        "equals" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!("json!(painless_eq(&{recv}, &{a}))")
        }
        "toString" => {
            format!("json!(painless_to_string(&{recv}))")
        }
        "length" if args.is_empty() => {
            // Works for both String.length() and List.size()
            format!(
                "json!(if ({recv}).is_string() {{ \
                    ({recv}).as_str().unwrap_or(\"\").len() \
                }} else if ({recv}).is_array() {{ \
                    ({recv}).as_array().map_or(0, |a__| a__.len()) \
                }} else {{ 0 }})"
            )
        }

        // List methods
        "add" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!("({recv}).as_array_mut().map(|a__| a__.push({a}))")
        }
        "get" if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!(
                "if ({a}).is_number() {{ \
                    ({recv}).as_array().and_then(|a__| a__.get(painless_to_i64(&{a}) as usize)).cloned().unwrap_or(Value::Null) \
                }} else {{ \
                    ({recv}).as_object().and_then(|o__| o__.get(painless_to_string(&{a}).as_str())).cloned().unwrap_or(Value::Null) \
                }}"
            )
        }
        "set" if args.len() == 2 => {
            let idx = emit_expr(&args[0]);
            let val = emit_expr(&args[1]);
            format!(
                "({recv}).as_array_mut().map(|a__| {{ let i__ = painless_to_i64(&{idx}) as usize; \
                 if i__ < a__.len() {{ a__[i__] = {val}; }} }})"
            )
        }
        "size" => {
            format!(
                "json!(if ({recv}).is_array() {{ \
                    ({recv}).as_array().map_or(0, |a__| a__.len()) \
                }} else if ({recv}).is_object() {{ \
                    ({recv}).as_object().map_or(0, |o__| o__.len()) \
                }} else {{ 0 }})"
            )
        }
        "removeIf" if args.len() == 1 => {
            let pred = emit_expr(&args[0]);
            format!("({recv}).as_array_mut().map(|a__| a__.retain(|item__| !({pred})(item__)))")
        }

        // Map methods
        "put" if args.len() == 2 => {
            let k = emit_expr(&args[0]);
            let v = emit_expr(&args[1]);
            format!("({recv}).as_object_mut().map(|o__| o__.insert(painless_to_string(&{k}), {v}))")
        }
        "containsKey" if args.len() == 1 => {
            let k = emit_expr(&args[0]);
            format!(
                "json!(({recv}).as_object().map_or(false, |o__| o__.contains_key(painless_to_string(&{k}).as_str())))"
            )
        }
        "keySet" => {
            format!(
                "json!(({recv}).as_object().map_or(vec![], |o__| o__.keys().cloned().collect::<Vec<_>>()))"
            )
        }
        "values" => {
            format!(
                "Value::Array(({recv}).as_object().map_or(vec![], |o__| o__.values().cloned().collect()))"
            )
        }
        "entrySet" => {
            // entrySet is handled specially in iteration contexts
            // For now, return the object itself
            recv
        }
        "remove" if args.len() == 1 => {
            let k = emit_expr(&args[0]);
            format!(
                "({recv}).as_object_mut().and_then(|o__| o__.remove(painless_to_string(&{k}).as_str())).unwrap_or(Value::Null)"
            )
        }
        "clone" => format!("({recv}).clone()"),
        "getKey" => format!("json!(painless_to_string(&{recv}))"),
        "getValue" => format!("({recv}).clone()"),

        // Regex
        "matcher" if args.len() == 1 => {
            // Returns (regex, input) pair for chaining
            let a = emit_expr(&args[0]);
            format!("/* matcher({a}) on {recv} */({recv}, {a})")
        }
        "replaceAll" if args.len() == 1 => {
            let replacement = emit_expr(&args[0]);
            format!(
                "/* replaceAll */ json!(({recv}).replace(painless_to_string(&{replacement}).as_str(), \"\"))"
            )
        }

        // Fallback
        _ => {
            let arg_list: Vec<String> = args.iter().map(emit_expr).collect();
            format!("/* {method}({}) */ Value::Null", arg_list.join(", "))
        }
    }
}

/// Emit a static method call.
fn emit_static_call(class: &str, method: &str, args: &[Expr]) -> String {
    match (class, method) {
        ("Long", "parseLong") if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!("json!(painless_to_i64(&{a}))")
        }
        ("Integer", "parseInt") if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!("json!(painless_to_i64(&{a}))")
        }
        ("Double", "parseDouble") if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!("json!(painless_to_f64(&{a}))")
        }
        ("String", "valueOf") if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            format!("json!(painless_to_string(&{a}))")
        }
        ("Arrays", "asList") => {
            let elems: Vec<String> = args.iter().map(emit_expr).collect();
            format!("json!([{}])", elems.join(", "))
        }
        ("Pattern", "compile") if args.len() == 1 => {
            let a = emit_expr(&args[0]);
            emit_expr(&Expr::Regex {
                pattern: format!("\" + painless_to_string(&{a}) + \""),
            })
        }
        _ => {
            let arg_list: Vec<String> = args.iter().map(emit_expr).collect();
            format!(
                "/* {class}.{method}({}) */ Value::Null",
                arg_list.join(", ")
            )
        }
    }
}

/// Emit a JSON literal as Rust source.
fn emit_literal(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "Value::Null".to_string(),
        serde_json::Value::Bool(b) => format!("json!({b})"),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                format!("json!({i}_i64)")
            } else if let Some(f) = n.as_f64() {
                format!("json!({f}_f64)")
            } else {
                format!("json!({n})")
            }
        }
        serde_json::Value::String(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("json!(\"{escaped}\")")
        }
        serde_json::Value::Array(arr) => {
            let elems: Vec<String> = arr.iter().map(emit_literal).collect();
            format!("json!([{}])", elems.join(", "))
        }
        serde_json::Value::Object(map) => {
            let entries: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, emit_literal(v)))
                .collect();
            format!("json!({{{}}})", entries.join(", "))
        }
    }
}

/// Convert a fully-static path to a dotted string.
/// Returns `None` if any segment is dynamic.
fn static_path_to_dotted(path: &[PathSegment]) -> Option<String> {
    let mut parts = Vec::with_capacity(path.len());
    for seg in path {
        match seg {
            PathSegment::Static(s) => parts.push(s.as_str()),
            PathSegment::Dynamic(_) => return None,
        }
    }
    Some(parts.join("."))
}

/// Build a dynamic path string at runtime.
fn emit_dynamic_path(path: &[PathSegment]) -> String {
    let mut parts = Vec::new();
    for seg in path {
        match seg {
            PathSegment::Static(s) => parts.push(format!("\"{s}\"")),
            PathSegment::Dynamic(expr) => {
                let e = emit_expr(expr);
                parts.push(format!("painless_to_string(&{e}).as_str()"));
            }
        }
    }
    format!("[{}].join(\".\")", parts.join(", "))
}

/// Sanitize a Painless identifier to be a valid Rust identifier.
fn sanitize_ident(name: &str) -> String {
    let sanitized = name.replace('.', "_").replace('-', "_");
    // Avoid Rust keywords
    match sanitized.as_str() {
        "type" => "r#type".to_string(),
        "match" => "r#match".to_string(),
        "ref" => "r#ref".to_string(),
        "self" => "self_".to_string(),
        "super" => "super_".to_string(),
        "move" => "r#move".to_string(),
        "fn" => "fn_".to_string(),
        "let" => "let_".to_string(),
        "mut" => "mut_".to_string(),
        "use" => "r#use".to_string(),
        "mod" => "mod_".to_string(),
        "loop" => "r#loop".to_string(),
        "return" => "return_".to_string(),
        _ => sanitized,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::painless::ir::*;
    use serde_json::json;

    #[test]
    fn emit_ctx_access_static() {
        let expr = Expr::CtxAccess {
            path: vec![
                PathSegment::Static("event".into()),
                PathSegment::Static("duration".into()),
            ],
        };
        let result = emit_expr(&expr);
        assert!(result.contains("event.get(\"event.duration\")"));
    }

    #[test]
    fn emit_ctx_assign_static() {
        let expr = Expr::CtxAssign {
            path: vec![PathSegment::Static("event.kind".into())],
            value: Box::new(Expr::Literal {
                value: json!("event"),
            }),
        };
        let result = emit_expr(&expr);
        assert!(result.contains("event.set(\"event.kind\""));
    }

    #[test]
    fn emit_binary_add() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Literal { value: json!(2) }),
            op: BinOp::Add,
            right: Box::new(Expr::Literal { value: json!(3) }),
        };
        let result = emit_expr(&expr);
        assert!(result.contains("painless_add"));
    }

    #[test]
    fn emit_if_stmt() {
        let stmt = Stmt::If {
            cond: Expr::BinaryOp {
                left: Box::new(Expr::CtxAccess {
                    path: vec![PathSegment::Static("x".into())],
                }),
                op: BinOp::Ne,
                right: Box::new(Expr::Literal {
                    value: serde_json::Value::Null,
                }),
            },
            then_body: Box::new(Stmt::Expr(Expr::CtxAssign {
                path: vec![PathSegment::Static("y".into())],
                value: Box::new(Expr::Literal { value: json!(1) }),
            })),
            else_body: None,
        };
        let result = emit_stmt(&stmt, "    ");
        assert!(result.contains("if !"));
        assert!(result.contains("is_null()"));
    }

    #[test]
    fn emit_for_each() {
        let stmt = Stmt::ForEach {
            var: "item".into(),
            iter: Expr::CtxAccess {
                path: vec![PathSegment::Static("items".into())],
            },
            body: Box::new(Stmt::Empty),
        };
        let result = emit_stmt(&stmt, "");
        assert!(result.contains("for item_ref__"));
        assert!(result.contains("as_array()"));
    }

    #[test]
    fn sanitize_rust_keywords() {
        assert_eq!(sanitize_ident("type"), "r#type");
        assert_eq!(sanitize_ident("match"), "r#match");
        assert_eq!(sanitize_ident("normal"), "normal");
        assert_eq!(sanitize_ident("dotted.name"), "dotted_name");
    }
}
