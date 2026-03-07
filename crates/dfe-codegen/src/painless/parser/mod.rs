use antlr_rust::{
    common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory,
    DefaultErrorStrategy, InputStream,
};

use self::{
    painlesslexer::PainlessLexer,
    painlessparser::{PainlessParser, PainlessParserContextType},
};

#[allow(clippy::all)]
pub mod painlesslexer;
#[allow(clippy::all)]
pub mod painlessparser;
#[allow(clippy::all)]
pub mod painlessparserlistener;
#[allow(clippy::all)]
pub mod painlessparservisitor;

pub fn parse(
    input: &str,
) -> PainlessParser<
    '_,
    CommonTokenStream<'_, PainlessLexer<'_, InputStream<&'_ str>>>,
    DefaultErrorStrategy<'_, PainlessParserContextType>,
> {
    let lexer: PainlessLexer<InputStream<&str>> =
        PainlessLexer::new_with_token_factory(InputStream::new(input), &CommonTokenFactory);

    let parser = PainlessParser::new(CommonTokenStream::new(lexer));

    parser
}
