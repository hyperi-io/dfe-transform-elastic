use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::{Validate, conditional::Conditional};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Drop {
    #[serde(alias = "if")]
    pub condition: Conditional,
}

impl Validate for Drop {
    #[instrument(name = "Drop::validate", skip_all)]
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::pipeline::Pipeline;
    // use crate::test_utils::assert_eq;
    //
    // #[test]
    // pub fn test() {
    // let configuration = r#"
    // processors:
    // - drop:
    // if: ctx?.event?.severity > 7
    // "#;
    //
    // for (input, output) in [
    // (vrl::value!({ event: { severity: 6 }}), false),
    // (vrl::value!({ event: { severity: 8 }}), true),
    // ] {
    // assert_eq!(
    // output,
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(input)
    // .unwrap()
    // .dropped
    // );
    // }
    // }
    // }
}
