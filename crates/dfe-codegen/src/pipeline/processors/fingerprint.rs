use anyhow::bail;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    Validate, conditional::Conditional, on_failure::OnFailure, unsupported_fields,
};

/*

  - fingerprint:
      fields:
        - '@timestamp'
        - crowdstrike.event.SessionId
        - crowdstrike.event.DetectId
        - crowdstrike.metadata.eventType
        - crowdstrike.metadata.customerIDString
      target_field: _id
      tag: fingerprint
      ignore_missing: true
*/

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Fingerprint {
    pub fields: Vec<String>,
    pub target_field: Option<String>,
    pub ignore_missing: Option<bool>,
    pub tag: Option<String>,

    // Unsupported fields
    pub salt: Option<String>,
    pub method: Option<String>,
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub on_failure: Option<OnFailure>,
    pub ignore_failure: Option<bool>,
}

impl Validate for Fingerprint {
    #[instrument(name = "Fingerprint::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!(
            "fingerprint",
            self,
            salt,
            method,
            conditional,
            ignore_failure,
            on_failure
        );

        if self.fields.is_empty() {
            bail!("fields must not be empty");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod validate {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "fingerprint",
            r#"
                processors:
                    - fingerprint:
                        fields:
                            - '@timestamp'
                        {}: {}
            "#,
            salt => "12345",
            method => "sha256",
            conditional => "true",
            ignore_failure => true,
            on_failure => "[]"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::{
    // pipeline::Pipeline,
    // test_utils::{self, assert_eq},
    // };
    //
    // #[test]
    // fn does_nothing() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - fingerprint:
    // fields: ["user"]
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // user: {
    // last_name: "Smith",
    // first_name: "John",
    // date_of_birth: "1980-01-15",
    // is_active: true
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // user: {
    // last_name: "Smith",
    // first_name: "John",
    // date_of_birth: "1980-01-15",
    // is_active: true
    // }
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    // }
}
