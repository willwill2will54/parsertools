use non_empty_collections::NonEmptyIndexSet;

use crate::LeftRecursionCheck;

use super::{AstBounds, ParseOutput, ParserInner, ParsingError, PartialParseResult, TokenBounds};

type TokenPredicate<'a, T, A> = Box<dyn Fn(&T) -> Option<A> + Sync + Send + 'a>;

pub struct TokenPredicateParser<'a, Token: TokenBounds, Ast: AstBounds> {
    predicate: TokenPredicate<'a, Token, Ast>,
}

impl<Token: TokenBounds, Ast: AstBounds> ParserInner for TokenPredicateParser<'_, Token, Ast> {
    type Token = Token;
    type Ast = Ast;

    fn parse<'a>(&self, tokens: &'a [Token]) -> ParseOutput<'a, Self::Ast, Self::Token> {
        if let Some(tok) = tokens.first() {
            if let Some(ast) = (self.predicate)(tok) {
                let remaining_tokens = &tokens[1..];
                Ok(NonEmptyIndexSet::new(PartialParseResult {
                    ast,
                    remaining_tokens,
                }))
            } else {
                Err(ParsingError::UnexpectedTokenProperUnknown)
            }
        } else {
            Err(ParsingError::UnexpectedEndOfInputProperUnknown)
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
