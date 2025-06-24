use non_empty_collections::NonEmptyIndexSet;

use crate::{results::PartialParseResult, AstBounds, LeftRecursionCheck, ParseInnerOutput, Parser, ParserInner, TokenBounds};

#[derive(Clone)]
pub(crate) struct SplitParser<
    'a,
    Token: TokenBounds + 'a,
    InAst: AstBounds + 'a,
    It: IntoIterator<Item = OutAst>,
    F: Fn(InAst) -> It + Sync + Send,
    OutAst: AstBounds + 'a,
> {
    parser: Parser<'a, Token, InAst>,
    function: F,
}

impl<
        Token: TokenBounds,
        InAst: AstBounds,
        F: Fn(InAst) -> It + Sync + Send,
        It: IntoIterator<Item = OutAst>,
        OutAst: AstBounds,
    > ParserInner for SplitParser<'_, Token, InAst, It, F, OutAst>
{
    type Token = Token;
    type Ast = OutAst;

    fn parse_front<'a>(&self, tokens: &'a [Token]) -> ParseInnerOutput<'a, Self::Ast, Self::Token> {
        Ok(NonEmptyIndexSet::from_iterator(
            self.parser.parse_front(tokens)?.into_iter()
                .map(|PartialParseResult {
                        ast,
                        remaining_tokens,
                    }| (self.function)(ast).into_iter().map(|ast|
                    PartialParseResult {
                        ast,
                        remaining_tokens,
                    }).collect(),
                ).flat_map(|it: Vec<PartialParseResult<'_, OutAst, Token>>| it.clone())
            ).unwrap(), // safe because we know the iterator is non-empty
        )
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            return LeftRecursionCheck::NotOk(vec![]);
        }
        self.parser.check_left_recursion(depth - 1)
    }
}

pub(crate) fn split_map<
    Token: TokenBounds,
    InAst: AstBounds,
    OutAst: AstBounds,
    It: IntoIterator<Item = OutAst>,
    F: Fn(InAst) -> It + Sync + Send,
>(
    parser: Parser<'_, Token, InAst>,
    function: F,
) -> SplitParser<'_, Token, InAst, It, F, OutAst> {
    SplitParser { parser, function }
}
