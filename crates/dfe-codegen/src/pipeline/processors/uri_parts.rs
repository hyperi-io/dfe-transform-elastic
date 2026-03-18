use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::{Validate, conditional::Conditional, unsupported_fields};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UriParts {
    pub field: String,
    pub target_field: Option<String>,
    pub ignore_failure: Option<bool>,
    pub keep_original: Option<bool>,
    pub remove_if_successful: Option<bool>,

    #[serde(alias = "if")]
    pub conditional: Option<Conditional>,

    // Unsupported fields
    pub ignore_missing: Option<bool>,
}

impl Validate for UriParts {
    #[instrument(name = "UriParts::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!("uri_parts", self, ignore_missing);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::{
            Pipeline, Processor, conditional::Conditional, unsupported_fields_tests,
        };

        #[test]
        fn parse() {
            let configuration = r#"
                processors:
                    - uri_parts:
                        field: url
            "#;

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::UriParts(uri_parts)] => {
                    assert_eq!(uri_parts.field, "url");
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn parse_target_field() {
            let configuration = r#"
                processors:
                    - uri_parts:
                        field: url
                        target_field: uri_parts
            "#;

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::UriParts(uri_parts)] => {
                    assert_eq!(uri_parts.field, "url");
                    assert_eq!(uri_parts.target_field, Some("uri_parts".into()));
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn parse_ignore_failure() {
            let configuration = r#"
                processors:
                    - uri_parts:
                        field: url
                        ignore_failure: true
            "#;

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::UriParts(uri_parts)] => {
                    assert_eq!(uri_parts.field, "url");
                    assert_eq!(uri_parts.ignore_failure, Some(true));
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        #[test]
        fn parse_conditional() {
            let configuration = r#"
                processors:
                    - uri_parts:
                        field: url.original
                        if: ctx?.url?.original != null
            "#;

            match &Pipeline::parse(configuration).unwrap().processors[..] {
                [Processor::UriParts(uri_parts)] => {
                    assert_eq!(uri_parts.field, "url.original");
                    assert_eq!(
                        uri_parts.conditional,
                        Some(Conditional("ctx?.url?.original != null".into()))
                    );
                }
                _ => panic!("unexpected pipeline structure"),
            }
        }

        unsupported_fields_tests!(
            "uri_parts",
            r#"
                processors:
                    - uri_parts:
                        field: url
                        {}: {}
            "#,
            ignore_missing => "true"
        );
    }
    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    // use crate::pipeline::Pipeline;
    // use crate::test_utils::{self, assert_eq};
    //
    // #[test]
    // fn example() {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // url: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment"
    // }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // url: {
    // path: "/foo.gif",
    // fragment: "fragment",
    // extension: "gif",
    // password: "mypassword",
    // original: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment",
    // scheme: "http",
    // port: 80,
    // user_info: "myusername:mypassword",
    // domain: "www.example.com",
    // query: "key1=val1&key2=val2",
    // username: "myusername"
    // }
    // })
    // );
    // }
    //
    // #[test]
    // fn no_file_extension() {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // url: "http://myusername:mypassword@www.example.com:80/foo?key1=val1&key2=val2#fragment"
    // }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // url: {
    // path: "/foo",
    // fragment: "fragment",
    // password: "mypassword",
    // original: "http://myusername:mypassword@www.example.com:80/foo?key1=val1&key2=val2#fragment",
    // scheme: "http",
    // port: 80,
    // user_info: "myusername:mypassword",
    // domain: "www.example.com",
    // query: "key1=val1&key2=val2",
    // username: "myusername"
    // }
    // })
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_true() {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // url: "/foo/bar?query=baz"
    // }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // url: "/foo/bar?query=baz"
    // })
    // );
    // }
    //
    // #[test]
    // fn ignore_failure_false() {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url
    // ignore_failure: false
    // "#;
    //
    // assert!(Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // url: "/foo/bar?query=baz"
    // }))
    // .unwrap_err()
    // .root_cause()
    // .to_string()
    // .contains("unable to parse url: relative URL without a base"));
    // }
    //
    // #[test]
    // fn target_field() {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url
    // target_field: uri_parts
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // url: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment"
    // }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // url: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment",
    // uri_parts: {
    // path: "/foo.gif",
    // fragment: "fragment",
    // extension: "gif",
    // password: "mypassword",
    // original: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment",
    // scheme: "http",
    // port: 80,
    // user_info: "myusername:mypassword",
    // domain: "www.example.com",
    // query: "key1=val1&key2=val2",
    // username: "myusername"
    // }
    // })
    // );
    // }
    //
    // #[test]
    // fn conditional_positive() {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url.original
    // target_field: uri_parts
    // if: ctx?.url?.original != null
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // url: { original: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment" }
    // }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // url: { original: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment" },
    // uri_parts: {
    // path: "/foo.gif",
    // fragment: "fragment",
    // extension: "gif",
    // password: "mypassword",
    // original: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment",
    // scheme: "http",
    // port: 80,
    // user_info: "myusername:mypassword",
    // domain: "www.example.com",
    // query: "key1=val1&key2=val2",
    // username: "myusername"
    // }
    // })
    // );
    // }
    //
    // #[test]
    // fn conditional_negative() {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url.original
    // if: ctx?.url?.original != null
    // "#;
    //
    // assert_eq!(
    // Pipeline::parse(configuration)
    // .unwrap()
    // .transpile()
    // .unwrap()
    // .compile()
    // .unwrap()
    // .run(vrl::value!({
    // url: { original: null }
    // }))
    // .unwrap()
    // .target,
    // vrl::value!({
    // url: { original: null }
    // })
    // );
    // }
    //
    // #[test]
    // fn keep_original_false() -> anyhow::Result<()> {
    // test_utils::init();
    //
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: url
    // keep_original: false
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // url: {
    // path: "/foo.gif",
    // fragment: "fragment",
    // extension: "gif",
    // password: "mypassword",
    // scheme: "http",
    // port: 80,
    // user_info: "myusername:mypassword",
    // domain: "www.example.com",
    // query: "key1=val1&key2=val2",
    // username: "myusername"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // url: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn remove_if_successful() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: original_url
    // target_field: parsed_url
    // remove_if_successful: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // parsed_url: {
    // path: "/foo.gif",
    // fragment: "fragment",
    // extension: "gif",
    // password: "mypassword",
    // original: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment",
    // scheme: "http",
    // port: 80,
    // user_info: "myusername:mypassword",
    // domain: "www.example.com",
    // query: "key1=val1&key2=val2",
    // username: "myusername"
    // }
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // original_url: "http://myusername:mypassword@www.example.com:80/foo.gif?key1=val1&key2=val2#fragment"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn remove_if_successful_failure_case() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - uri_parts:
    // field: original_url
    // target_field: parsed_url
    // remove_if_successful: true
    // ignore_failure: true
    // "#;
    //
    // assert_eq!(
    // vrl::value!({
    // original_url: "Hello, World"
    // }),
    // Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(vrl::value!({
    // original_url: "Hello, World"
    // }))?
    // .target
    // );
    //
    // Ok(())
    // }
    // }
}
