use thiserror::Error;
use crate::parsers::{AstBounds, TokenBounds};
use std::hash::Hash;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseError<T: TokenBounds> {
    #[error("Grammar permits multiple interpretations: {0:?}")]
    AmbiguousGrammar(Vec<String>),
    #[error("Unexpected token")]
    UnexpectedTokenProperUnknown,
    #[error("Unexpected token, expected: {expected:?}")]
    UnexpectedTokenProperKnown { expected: T, found: T },
    #[error("Unexpected end of input")]
    UnexpectedEndOfInputProperUnknown,
    #[error("Unexpected end of input, expected: {expected:?}")]
    UnexpectedEndOfInputProperKnown { expected: T },
    #[error("Unhandled tokens: {0:?}")]
    UnhandledTokens(Vec<T>),
}

#[derive(Debug, Clone)]
pub struct PartialParseResult<'a, Ast: AstBounds, Token: TokenBounds> {
    pub ast: Ast,
    pub (super) remaining_tokens: &'a [Token],
}

impl<Ast: AstBounds, Token: TokenBounds> PartialEq for PartialParseResult<'_, Ast, Token> {
    fn eq(&self, other: &Self) -> bool {
        self.ast == other.ast && self.remaining_tokens == other.remaining_tokens
    }
}

impl<Ast: AstBounds, Token: TokenBounds> Eq for PartialParseResult<'_, Ast, Token> {}

impl<Ast: AstBounds, Token: TokenBounds> Hash for PartialParseResult<'_, Ast, Token> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ast.hash(state);
        self.remaining_tokens.hash(state);
    }
}

pub type ParseOutput<'a, Ast, Token> = Result<Ast, ParseError<Token>>;

pub enum LeftRecursionCheck {
    Ok,
    NotOk(Vec<String>),
}

impl LeftRecursionCheck {
    pub fn is_ok(&self) -> bool {
        matches!(self, LeftRecursionCheck::Ok)
    }

    pub fn is_not_ok(&self) -> bool {
        !self.is_ok()
    }

    pub fn not_ok_or_else<F: FnOnce() -> LeftRecursionCheck>(self, f: F) -> LeftRecursionCheck {
        if self.is_not_ok() {
            self
        } else {
            f()
        }
    }
}
