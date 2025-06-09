use std::sync::LazyLock;

use crate::AstBounds;
use crate::LeftRecursionCheck;
use crate::Parser;
use crate::TokenBounds;

use super::ParseOutput;

use super::ParserInner;

pub(super) struct LazyParser<'a, T: TokenBounds, A: AstBounds> {
    pub(super) inner: LazyLock<Parser<'a, T, A>>,
}

impl<T: TokenBounds, A: AstBounds> LazyParser<'_, T, A> {}

impl<T: TokenBounds, A: AstBounds> ParserInner for LazyParser<'_, T, A> {
    type Token = T;
    type Ast = A;

    fn parse<'a>(&self, tokens: &'a [Self::Token]) -> ParseOutput<'a, Self::Ast, Self::Token> {
        self.inner.parse(tokens)
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            return LeftRecursionCheck::NotOk(vec![]);
        }
        self.inner.check_left_recursion(depth - 1)
    }
}

pub(super) fn lazy<'a, T: TokenBounds, A: AstBounds>(
    f: fn() -> Parser<'a, T, A>,
) -> LazyParser<'a, T, A> {
    LazyParser {
        inner: LazyLock::new(f),
    }
}
