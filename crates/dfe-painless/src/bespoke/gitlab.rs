// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `gitlab`'s GraphQL variable blocks and its sidekiq job duration,
//! transcribed.
//!
//! A GraphQL query's variables are user-supplied and arbitrarily keyed, so the
//! vendor pipeline stores the block as JSON TEXT rather than letting an unknown
//! object reach the mapping.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{painless_to_f64, painless_to_string};

/// The request parameters the production log carries.
const PARAMS: &str = "gitlab.production.params";

/// The GraphQL block on the production log.
const GRAPHQL: &str = "gitlab.production.graphql";

/// `convert_params_graphql_variables` in `gitlab/production`: the variables
/// under `params.graphql` rendered as JSON text.
fn dump_params_graphql_variables(event: &mut Event, _params: &Value) {
    let path = "gitlab.production.params.graphql.variables";
    let Some(dumped) = event.get(path).map(Value::to_string) else {
        return;
    };
    let _ = event.update(path, dumped);
}

/// `kv_params` in `gitlab/production`: the `{key, value}` pairs the log lists
/// folded into one object, with a `variables` block rendered as JSON text.
fn fold_params_pairs(event: &mut Event, _params: &Value) {
    let rebuilt = {
        let Some(pairs) = event.get_array(PARAMS) else {
            return;
        };
        let mut rebuilt = Map::with_capacity(pairs.len());
        for pair in pairs {
            let key = painless_to_string(pair.get("key").unwrap_or(&Value::Null));
            let value = pair.get("value").cloned().unwrap_or(Value::Null);
            if key == "variables" && value.is_object() {
                rebuilt.insert(key, Value::String(value.to_string()));
            } else {
                rebuilt.insert(key, value);
            }
        }
        rebuilt
    };
    let _ = event.update(PARAMS, Value::Object(rebuilt));
}

/// `convert_graphql_variables` in `gitlab/production`: the variables inside the
/// GraphQL block rendered as JSON text.
///
/// The block is a LIST where one request carried several operations and a map
/// where it carried one, and the vendor script reads both.
fn dump_graphql_variables(event: &mut Event, _params: &Value) {
    if let Some(mut operations) = event.take_array(GRAPHQL) {
        for operation in &mut operations {
            let Some(dumped) = operation
                .get("variables")
                .filter(|variables| variables.is_object())
                .map(Value::to_string)
            else {
                continue;
            };
            if let Some(slot) = operation.get_mut("variables") {
                *slot = Value::String(dumped);
            }
        }
        let _ = event.update(GRAPHQL, Value::Array(operations));
        return;
    }
    let Some(dumped) = event
        .get(GRAPHQL)
        .filter(|block| block.is_object())
        .and_then(|block| block.get("variables"))
        .filter(|variables| variables.is_object())
        .map(Value::to_string)
    else {
        return;
    };
    let _ = event.update("gitlab.production.graphql.variables", dumped);
}

/// `convert_duration_to_nanoseconds` in `gitlab/sidekiq`: the job duration
/// taken from fractional seconds to whole nanoseconds.
fn duration_seconds_to_nanoseconds(event: &mut Event, _params: &Value) {
    let Some(seconds) = event.get("event.duration").map(painless_to_f64) else {
        return;
    };
    let _ = event.update("event.duration", (seconds * 1e9) as i64);
}

/// Every gitlab script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "79066a30f5f42ddac23bfb370b9f7ae7537d730902c210a2e6945305abbed995",
        source: "gitlab",
        name: "dump_params_graphql_variables",
        run: dump_params_graphql_variables,
    },
    Entry {
        hash: "8e0fa49584feb9d2120ef0cd949f1f1c53d8bb11570b34b4e7ee4bb2f43ebe9e",
        source: "gitlab",
        name: "fold_params_pairs",
        run: fold_params_pairs,
    },
    Entry {
        hash: "86906957b229d963fbdd60b0a8146820ac49c05ac87a5e35ad912cfd601ad1a5",
        source: "gitlab",
        name: "dump_graphql_variables",
        run: dump_graphql_variables,
    },
    Entry {
        hash: "bab4a4c2fa99f137884cecefcf7fe6b15753d20d7c061f4c4c8dfecd29b7591f",
        source: "gitlab",
        name: "duration_seconds_to_nanoseconds",
        run: duration_seconds_to_nanoseconds,
    },
];
