use non_empty_collections::NonEmptyIndexSet;

use crate::LeftRecursionCheck;

use super::{AstBounds, ParseInnerOutput, ParserInner, ParseError, PartialParseResult, TokenBounds};

type TokenPredicate<'a, T, A> = Box<dyn Fn(&T) -> Option<A> + Sync + Send + 'a>;

pub struct TokenPredicateParser<'a, Token: TokenBounds, Ast: AstBounds> {
    predicate: TokenPredicate<'a, Token, Ast>,
}

impl<Token: TokenBounds, Ast: AstBounds> ParserInner for TokenPredicateParser<'_, Token, Ast> {
    type Token = Token;
    type Ast = Ast;

    fn parse_inner<'a>(&self, tokens: &'a [Token]) -> ParseInnerOutput<'a, Self::Ast, Self::Token> {
        if let Some(tok) = tokens.first() {
            if let Some(ast) = (self.predicate)(tok) {
                let remaining_tokens = &tokens[1..];
                Ok(NonEmptyIndexSet::new(PartialParseResult {
                    ast,
                    remaining_tokens,
                }))
            } else {
                Err(ParseError::UnexpectedTokenProperUnknown)
            }
        } else {
            Err(ParseError::UnexpectedEndOfInputProperUnknown)
        }
    }

    fn check_left_recursion(&self, _depth: usize) -> LeftRecursionCheck {
        LeftRecursionCheck::Ok
    }
}

pub fn pred<'a, T: TokenBounds, Ast: AstBounds>(
    predicate: impl Fn(&T) -> Option<Ast> + Sync + Send + 'a,
) -> TokenPredicateParser<'a, T, Ast> {
    TokenPredicateParser {
        predicate: Box::new(predicate),
    }
}
