use non_empty_collections::NonEmptyIndexSet;

use crate::{results::PartialParseResult, LeftRecursionCheck, ParseError, ParseInnerOutput, Parser, ParserInner, TokenBounds};

pub(crate) struct SingleTokenParser<T: TokenBounds> {
    pub(crate) token: T,
}

impl<T: TokenBounds> ParserInner for SingleTokenParser<T> {
    type Token = T;
    type Ast = T;

    fn parse_front<'a>(&self, tokens: &'a [T]) -> ParseInnerOutput<'a, Self::Ast, Self::Token> {
        match tokens.first() {
            Some(t) if t == &self.token => {
                let new_tokens = &tokens[1..];

                Ok(NonEmptyIndexSet::new(PartialParseResult {
                    ast: t.clone(),
                    remaining_tokens: new_tokens,
                }))
            }
            Some(t) => Err(ParseError::UnexpectedTokenProperKnown {
                expected: self.token.clone(),
                found: t.clone(),
            }),
            None => Err(ParseError::UnexpectedEndOfInputProperKnown {
                expected: self.token.clone(),
            }),
        }
    }

    fn check_left_recursion(&self, _depth: usize) -> LeftRecursionCheck {
        LeftRecursionCheck::Ok
    }
}

pub fn tok<'a,T: TokenBounds + 'a>(token: T) -> Parser<'a,T,T> {
    Parser::new(SingleTokenParser { token })
}
