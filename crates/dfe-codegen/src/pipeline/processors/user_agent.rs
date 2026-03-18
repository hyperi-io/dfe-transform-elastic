use crate::pipeline::unsupported_fields;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::pipeline::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAgent {
    pub field: String,
    pub ignore_missing: Option<bool>,
    pub target_field: Option<String>,

    // Unsupported configuration options
    pub regex_file: Option<String>,
    pub properties: Option<Vec<String>>,
    pub extract_device_type: Option<bool>,
}

impl Validate for UserAgent {
    #[instrument(name = "UserAgent::validate", skip_all, err)]
    fn validate(&self) -> anyhow::Result<()> {
        unsupported_fields!(
            "user_agent",
            self,
            regex_file,
            properties,
            extract_device_type
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod parse {
        use crate::pipeline::{Pipeline, Processor};

        #[test]
        pub fn parse() {
            let configuration = r#"
              processors:
                - user_agent:
                    field: user_agent.original
                    ignore_missing: true
            "#;

            let pipeline = Pipeline::parse(configuration).unwrap();
            assert_eq!(pipeline.processors.len(), 1);

            let processor = &pipeline.processors[0];
            match processor {
                Processor::UserAgent(user_agent) => {
                    assert_eq!("user_agent.original", user_agent.field);
                    assert_eq!(Some(true), user_agent.ignore_missing);
                }
                _ => panic!("unexpected processor {:#?}", processor),
            }
        }

        #[test]
        pub fn unsupported_regex_file() {
            let configuration = r#"
              processors:
                - user_agent:
                    field: user_agent.original
                    regex_file: foo bar
            "#;

            let pipeline = Pipeline::parse(configuration);
            if pipeline.is_ok() {
                panic!("expected error for unsupported configuration")
            }
        }

        #[test]
        pub fn unsupported_properties() {
            let configuration = r#"
              processors:
                - user_agent:
                    field: user_agent.original
                    properties: ["name", "major"]
            "#;

            let pipeline = Pipeline::parse(configuration);
            if pipeline.is_ok() {
                panic!("expected error for unsupported configuration")
            }
        }

        #[test]
        pub fn unsupported_extract_device_type() {
            let configuration = r#"
              processors:
                - user_agent:
                    field: user_agent.original
                    extract_device_type: true
            "#;

            let pipeline = Pipeline::parse(configuration);
            if pipeline.is_ok() {
                panic!("expected error for unsupported configuration")
            }
        }
    }

    // TODO: Transpile tests commented out during VRL removal.
    // These will be rewritten as codegen tests.
    // mod transpile {
    //
    // use pretty_assertions::assert_eq;
    //
    // use crate::pipeline::Pipeline;
    //
    // #[test]
    // fn parse_user_agent() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - user_agent:
    // field: agent
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
    // }))?
    // .target;
    //
    // assert_eq!(
    // value!({
    // "agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
    // "user_agent": {
    // "name": "Chrome",
    // "original": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
    // "version": "51.0.2704.103",
    // "os": {
    // "name": "Mac OSX",
    // "version": "10.10.5",
    // "full": "Mac OSX 10.10.5"
    // },
    // "device" : {
    // "name" : "pc"
    // }
    // }
    // }),
    // target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn target_field() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - user_agent:
    // field: agent
    // target_field: example
    // "#;
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(value!({
    // agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
    // }))?
    // .target;
    //
    // assert_eq!(
    // value!({
    // "agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
    // "example": {
    // "name": "Chrome",
    // "original": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
    // "version": "51.0.2704.103",
    // "os": {
    // "name": "Mac OSX",
    // "version": "10.10.5",
    // "full": "Mac OSX 10.10.5"
    // },
    // "device" : {
    // "name" : "pc"
    // }
    // }
    // }),
    // target
    // );
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn missing_field_error() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - user_agent:
    // field: agent
    // "#;
    //
    // let target = Pipeline::parse(configuration)?.transpile()?.compile()?.run(value!({}));
    //
    // if target.is_ok() {
    // panic!("expected to fail with missing 'agent' field")
    // }
    //
    // Ok(())
    // }
    //
    // #[test]
    // fn ignore_missing() -> anyhow::Result<()> {
    // let configuration = r#"
    // processors:
    // - user_agent:
    // field: agent
    // ignore_missing: true
    // "#;
    //
    // let input = value!({foo: "bar"});
    //
    // let target = Pipeline::parse(configuration)?
    // .transpile()?
    // .compile()?
    // .run(input.clone())?
    // .target;
    //
    // assert_eq!(input, target);
    //
    // Ok(())
    // }
    // }
}
