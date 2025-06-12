use crate::{AstBounds, LeftRecursionCheck, ParseInnerOutput, Parser, ParserInner, TokenBounds};

pub(super) struct LazyParser<'a, T: TokenBounds, A: AstBounds, F: Send + Sync + Fn() -> Parser<'a,T,A>> {
    pub(super) inner: F,
}

impl<'a, T: TokenBounds, A: AstBounds, F: Send + Sync + Fn() -> Parser<'a,T,A>> LazyParser<'a, T, A, F> {}

impl<'a, T: TokenBounds, A: AstBounds, F: Send + Sync + Fn() -> Parser<'a,T,A>> ParserInner for LazyParser<'a, T, A, F> {
    type Token = T;
    type Ast = A;

    fn parse_inner<'b>(&self, tokens: &'b [Self::Token]) -> ParseInnerOutput<'b, Self::Ast, Self::Token> {
        (self.inner)().parse_inner(tokens)
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            return LeftRecursionCheck::NotOk(vec![]);
        }
        (self.inner)().check_left_recursion(depth - 1)
    }
}

pub(super) fn lazy<'a, T: TokenBounds, A: AstBounds, F: Send + Sync + Fn() -> Parser<'a,T,A>>(
    f: F,
) -> LazyParser<'a, T, A, F> {
    LazyParser {
        inner: f,
    }
}
