use non_empty_collections::NonEmptyIndexSet;

use crate::{results::PartialParseResult, AstBounds, LeftRecursionCheck, ParseError, ParseFrontOutput, Parser, ParserInner, TokenBounds};

#[derive(Clone)]
pub(crate) struct FilterParser<
    'a,
    Token: TokenBounds + 'a,
    Ast: AstBounds + 'a,
    F: Fn(&Ast) -> bool + Sync + Send,
> {
    parser: Parser<'a, Token, Ast>,
    function: F,
    error: ParseError<Token>
}

impl<
        Token: TokenBounds,
        Ast: AstBounds,
        F: Fn(&Ast) -> bool + Sync + Send,
    > ParserInner for FilterParser<'_, Token, Ast, F>
{
    type Token = Token;
    type Ast = Ast;

    fn parse_front<'a>(&self, tokens: &'a [Token]) -> ParseFrontOutput<'a, Self::Ast, Self::Token> {
        match NonEmptyIndexSet::from_iterator(
            self.parser.parse_front(tokens)?.into_iter().filter(
                |PartialParseResult {
                    ast,
                    remaining_tokens: _,
                }| (self.function)(ast)
            )) {
                Ok(set) => Ok(set),
                Err(_) => Err(self.error.clone()),
        }
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            return LeftRecursionCheck::NotOk(vec![]);
        }
        self.parser.check_left_recursion(depth - 1)
    }
}

pub(crate) fn filter<'a,
    Token: 'a + TokenBounds,
    Ast: 'a + AstBounds,
    F: 'a + Fn(&Ast) -> bool + Sync + Send,
>(
    parser: Parser<'a, Token, Ast>,
    function: F,
    error: ParseError<Token>
) -> Parser<'a, Token, Ast> {
    Parser::new(FilterParser { parser, function, error })
}
