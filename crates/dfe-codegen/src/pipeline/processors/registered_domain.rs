use anyhow::Result;
use serde::Deserialize;
use tracing::instrument;

use crate::pipeline::{
    Validate, conditional::Conditional, on_failure::OnFailure, unsupported_fields,
};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RegisteredDomain {
    pub field: String,
    pub target_field: Option<String>,
    pub ignore_missing: Option<bool>,
    pub tag: Option<String>,
    pub ignore_failure: Option<bool>,

    // Unsupported Fields
    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,
    pub on_failure: Option<OnFailure>,
}

impl Validate for RegisteredDomain {
    #[instrument(name = "RegisteredDomain::validate", skip(self), err)]
    fn validate(&self) -> Result<()> {
        unsupported_fields!("registered_domain", self, conditional, on_failure);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod validate {
        use crate::pipeline::unsupported_fields_tests;

        unsupported_fields_tests!(
            "registered_domain",
            r#"
                processors:
                    - registered_domain:
                        field: "foo"
                        {}: {}
            "#,
            conditional => "true && false",
            on_failure => "[]"
        );
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::pipeline::Pipeline;
    // use crate::test_utils::{self, assert_eq};
    //
    // #[test]
    // fn happy_path() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - registered_domain:
    // field: message
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "www.example.ac.uk",
    // subdomain: "www",
    // registered_domain: "example.ac.uk",
    // top_level_domain: "ac.uk",
    // domain: "www.example.ac.uk",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "www.example.ac.uk" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn target_field() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - registered_domain:
    // field: message
    // target_field: url
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "www.example.ac.uk",
    // url: {
    // subdomain: "www",
    // registered_domain: "example.ac.uk",
    // top_level_domain: "ac.uk",
    // domain: "www.example.ac.uk",
    // }
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "www.example.ac.uk" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_missing_true() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - registered_domain:
    // field: other
    // target_field: url
    // ignore_missing: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: "www.example.ac.uk",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "www.example.ac.uk" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_missing_false() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - registered_domain:
    // field: other
    // target_field: url
    // ignore_missing: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: "www.example.ac.uk" }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains(".other must not be nullish"));
    // }
    //
    // // Elastic is wrong here, servicebus.windows.net appears in the mozilla effective TLD list
    // // https://publicsuffix.org/list/public_suffix_list.dat
    // // Which means as weird as it sounds, test.servicebus.windows.net is the domain
    // // and there is no subdomain
    // #[test]
    // fn windows_servicebus() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - registered_domain:
    // field: message
    // target_field: url
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // "message": "test.servicebus.windows.net",
    // "url": {
    // "registered_domain": "test.servicebus.windows.net",
    // "top_level_domain": "servicebus.windows.net",
    // "domain": "test.servicebus.windows.net"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({ message: "test.servicebus.windows.net" }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_failure_true_invalid() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - registered_domain:
    // field: message
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // message: 42,
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ message: 42 }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_true_missing() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - registered_domain:
    // field: message
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // other: "i am a banana",
    // }),
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ other: "i am a banana" }))
    // .unwrap()
    // .target
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_false() {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - registered_domain:
    // field: message
    // ignore_failure: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({ other: "i am a banana" }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains(".message must not be nullish"));
    // }
    // }
}
