// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Go-template based script rendering for pipeline processors.
//!
//! Uses `gtmpl` to render Go-style templates with custom helper functions.
//! The rendered output is a string that was previously parsed as VRL; now it
//! is kept as a rendered template string for future codegen passes.

use anyhow::anyhow;
use gtmpl::{Context, FuncError, Value};
use tracing::instrument;

/// A rendered script template string.
#[derive(Debug, Clone)]
pub struct ScriptTemplate(pub String);

fn is_array(values: &[Value]) -> Result<Value, FuncError> {
    match values {
        [Value::Array(_)] => Ok(true.into()),
        [_] => Ok(false.into()),
        _ => Err(FuncError::ExactlyXArgs("is_array".into(), 1)),
    }
}

fn quote_strings(values: &[Value]) -> Result<Value, FuncError> {
    Ok(match values {
        [Value::Array(arr)] => arr
            .iter()
            .map(|item| match item {
                Value::String(str) => Ok(format!("\"{str}\"").into()),
                Value::Number(n) => Ok(format!("{}", n).into()),
                Value::Nil => Ok("null".into()),
                other => Err(FuncError::Generic(format!(
                    "quote_strings only supports operating on arrays of string, found: {other:?}"
                ))),
            })
            .collect::<Result<Vec<Value>, _>>()?
            .into(),
        [Value::String(str)] => format!("\"{str}\"").into(),
        _ => Err(FuncError::ExactlyXArgs("print_array".into(), 1))?,
    })
}

fn print_array(values: &[Value]) -> Result<Value, FuncError> {
    match values {
        [Value::Array(arr)] => Ok(format!(
            "[ {} ]",
            arr.iter()
                .map(|value| match value {
                    Value::String(str) => Ok(str.clone()),
                    _ => Err(FuncError::Generic(
                        "print_array only supports formatting arrays of strings".into(),
                    )),
                })
                .collect::<Result<Vec<_>, _>>()?
                .join(",")
        )
        .into()),
        [value] => Ok(value.clone()),
        _ => Err(FuncError::ExactlyXArgs("print_array".into(), 1)),
    }
}

fn raw_strings(values: &[Value]) -> Result<Value, FuncError> {
    Ok(match values {
        [Value::Array(arr)] => arr
            .iter()
            .map(|item| match item {
                Value::String(str) => Ok(format!("s'{str}'", str = str.replace('\'', "\\'")).into()),
                other => Err(FuncError::Generic(format!(
                    "raw_strings only supports operating on arrays of string, found: {other:?}"
                ))),
            })
            .collect::<Result<Vec<Value>, _>>()?
            .into(),
        [Value::String(str)] => format!("s'{str}'", str = str.replace('\'', "\\'")).into(),
        _ => Err(FuncError::ExactlyXArgs("print_array".into(), 1))?,
    })
}

fn print_map(values: &[Value]) -> Result<Value, FuncError> {
    match values {
        [Value::Map(obj)] => {
            let mut entries = obj.iter().collect::<Vec<_>>();
            entries.sort_by(|(a, _), (b, _)| a.cmp(b));

            Ok(format!(
                "{{ {} }}",
                entries
                    .into_iter()
                    .map(|(key, value)| {
                        let value = match value {
                            Value::String(value) => format!("s'{value}'"),
                            Value::Number(n) if n.as_f64().is_some() => {
                                format!("{}", n.as_f64().unwrap())
                            }
                            Value::Number(n) if n.as_i64().is_some() => {
                                format!("{}", n.as_i64().unwrap())
                            }
                            Value::Number(n) if n.as_u64().is_some() => {
                                format!("{}", n.as_u64().unwrap())
                            }
                            map @ Value::Map(_) => {
                                if let Value::String(str) = print_map(std::slice::from_ref(map))? {
                                    str
                                } else {
                                    unreachable!()
                                }
                            }
                            arr @ Value::Array(_) => {
                                if let Value::String(str) = print_array(&[quote_strings(std::slice::from_ref(arr))?])? {
                                    str
                                } else {
                                    unreachable!()
                                }
                            }
                            value => {
                                return Err(FuncError::Generic(format!(
                                    "print_map only supports formatting values of strings, got {value:?}"
                                )))
                            }
                        };

                        Ok(format!("\"{key}\": {value}",))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ")
            )
            .into())
        }
        [other] => Err(FuncError::Generic(format!(
            "print_map only supports formatting objects, got {other:?}"
        ))),
        _ => Err(FuncError::ExactlyXArgs("print_array".into(), 1)),
    }
}

fn into_array(values: &[Value]) -> Result<Value, FuncError> {
    match values {
        [arr @ Value::Array(_)] => Ok(arr.clone()),
        [other] => Ok(Value::Array(vec![other.clone()])),
        _ => Err(FuncError::ExactlyXArgs("format_array".into(), 1)),
    }
}

fn contains(values: &[Value]) -> Result<Value, FuncError> {
    match values {
        [Value::Array(query), Value::Array(target)] => Ok(Value::Bool(query.iter().all(|item| target.contains(item)))),
        [value, Value::Array(arr)] => Ok(Value::Bool(arr.iter().any(|item| item == value))),
        [_, _] => Err(FuncError::Generic("contains second argument must be an array".into())),
        [..] => Err(FuncError::ExactlyXArgs("contains takes only two arguments".into(), 2)),
    }
}

fn list(values: &[Value]) -> Result<Value, FuncError> {
    Ok(Value::Array(Vec::from(values)))
}

fn unquote(values: &[Value]) -> Result<Value, FuncError> {
    match values {
        [Value::String(str)] => Ok(Value::String(str.replace('"', ""))),
        [_] => Err(FuncError::UnableToConvertFromValue),
        _ => Err(FuncError::ExactlyXArgs("escape takes a single argument".into(), 1)),
    }
}

fn explode_path(values: &[Value]) -> Result<Value, FuncError> {
    match values {
        [Value::String(str)] => Ok(str.split('.').collect::<Vec<_>>().into()),
        [_] => Err(FuncError::UnableToConvertFromValue),
        _ => Err(FuncError::ExactlyXArgs(
            "explode_path takes a single argument".into(),
            1,
        )),
    }
}

fn error(values: &[Value]) -> Result<Value, FuncError> {
    Err(FuncError::Generic(
        values
            .iter()
            .map(|value| match value {
                Value::String(str) => Ok(str.as_str()),
                other => Err(anyhow!("error only takes strings as arguments: {other}")),
            })
            .collect::<Result<Vec<_>, _>>()?
            .join(""),
    ))
}

fn replace(values: &[Value]) -> Result<Value, FuncError> {
    match values {
        [Value::String(find), Value::String(replace), Value::String(str)] => {
            Ok(Value::String(str.replace(find, replace)))
        }
        [_, _, _] => Err(FuncError::UnableToConvertFromValue),
        _ => Err(FuncError::ExactlyXArgs("replace takes three arguments".into(), 3)),
    }
}

impl ScriptTemplate {
    /// Render a Go-template with the given context, returning the rendered string.
    #[instrument(name = "ScriptTemplate::render", skip_all, fields(context), err)]
    pub fn render(text: impl Into<String>, context: impl Into<Value>) -> anyhow::Result<Self> {
        let mut template = gtmpl::Template::default();
        template.add_funcs(&[
            ("is_array", is_array as fn(&[Value]) -> Result<Value, FuncError>),
            ("print_array", print_array),
            ("quote_string", quote_strings),
            ("raw_string", raw_strings),
            ("print_map", print_map),
            ("into_array", into_array),
            ("contains", contains),
            ("list", list),
            ("unquote", unquote),
            ("explode_path", explode_path),
            ("error", error),
            ("replace", replace),
        ]);

        template.parse(text)?;

        Ok(ScriptTemplate(template.render(&Context::from(context.into()))?))
    }

    /// Return the rendered template string.
    pub fn rendered(&self) -> &str {
        &self.0
    }

    /// Print the rendered template to stdout (debug aid).
    pub fn inspect(self) -> Self {
        println!("{}", self.0);
        self
    }
}
