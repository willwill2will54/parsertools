use non_empty_collections::NonEmptyIndexSet;

use crate::LeftRecursionCheck;

use super::{ParseOutput, ParserInner, ParsingError, PartialParseResult, TokenBounds};

pub(crate) struct SingleTokenParser<T: TokenBounds> {
    pub(crate) token: T,
}

impl<T: TokenBounds> ParserInner for SingleTokenParser<T> {
    type Token = T;
    type Ast = ();

    fn parse<'a>(&self, tokens: &'a [T]) -> ParseOutput<'a, Self::Ast, Self::Token> {
        match tokens.first() {
            Some(t) if t == &self.token => {
                let new_tokens = &tokens[1..];

                Ok(NonEmptyIndexSet::new(PartialParseResult {
                    ast: (),
                    remaining_tokens: new_tokens,
                }))
            }
            Some(t) => Err(ParsingError::UnexpectedTokenProperKnown {
                expected: self.token.clone(),
                found: t.clone(),
            }),
            None => Err(ParsingError::UnexpectedEndOfInputProperKnown {
                expected: self.token.clone(),
            }),
        }
    }

    fn check_left_recursion(&self, _depth: usize) -> LeftRecursionCheck {
        LeftRecursionCheck::Ok
    }
}

pub(crate) fn tok<T: TokenBounds>(token: T) -> SingleTokenParser<T> {
    SingleTokenParser { token }
}
