use non_empty_collections::NonEmptyIndexSet;

use crate::{results::PartialParseResult, AstBounds, LeftRecursionCheck, ParseFrontOutput, Parser, ParserInner, TokenBounds};

#[derive(Clone)]
pub(crate) struct MapParser<
    'a,
    Token: TokenBounds + 'a,
    InAst: AstBounds + 'a,
    F: Fn(InAst) -> OutAst + Sync + Send,
    OutAst: AstBounds + 'a,
> {
    parser: Parser<'a, Token, InAst>,
    function: F,
}

impl<
        Token: TokenBounds,
        InAst: AstBounds,
        F: Fn(InAst) -> OutAst + Sync + Send,
        OutAst: AstBounds,
    > ParserInner for MapParser<'_, Token, InAst, F, OutAst>
{
    type Token = Token;
    type Ast = OutAst;

    fn parse_front<'a>(&self, tokens: &'a [Token]) -> ParseFrontOutput<'a, Self::Ast, Self::Token> {
        Ok(
            NonEmptyIndexSet::from_iterator(self.parser.parse_front(tokens)?.into_iter().map(
                |PartialParseResult {
                     ast,
                     remaining_tokens,
                 }| PartialParseResult {
                    ast: (self.function)(ast),
                    remaining_tokens,
                },
            ))
            .unwrap(), // safe because we know the iterator is non-empty
        )
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            return LeftRecursionCheck::NotOk(vec![]);
        }
        self.parser.check_left_recursion(depth - 1)
    }
}

pub(crate) fn map<
    Token: TokenBounds,
    InAst: AstBounds,
    OutAst: AstBounds,
    F: Fn(InAst) -> OutAst + Sync + Send,
>(
    parser: Parser<'_, Token, InAst>,
    function: F,
) -> MapParser<'_, Token, InAst, F, OutAst> {
    MapParser { parser, function }
}
