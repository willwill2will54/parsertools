use crate::{AstBounds, ParseInnerOutput, ParserInner, TokenBounds,LeftRecursionCheck,Parser};

pub(super) struct LazyParser<'a, T: TokenBounds, A: AstBounds, F: Send + Sync + Fn() -> Parser<'a,T,A>> {
    pub(super) inner: F,
}

impl<'a, T: TokenBounds, A: AstBounds, F: Send + Sync + Fn() -> Parser<'a,T,A>> LazyParser<'a, T, A, F> {}

impl<'a, T: TokenBounds, A: AstBounds, F: Send + Sync + Fn() -> Parser<'a,T,A>> ParserInner for LazyParser<'a, T, A, F> {
    type Token = T;
    type Ast = A;

    fn parse_front<'b>(&self, tokens: &'b [Self::Token]) -> ParseInnerOutput<'b, Self::Ast, Self::Token> {
        (self.inner)().parse_front(tokens)
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            return LeftRecursionCheck::NotOk(vec![]);
        }
        (self.inner)().check_left_recursion(depth - 1)
    }
}

pub fn lazy<'a, T: 'a + TokenBounds, A: 'a + AstBounds, F: 'a + Send + Sync + Fn() -> Parser<'a,T,A>>(
    f: F,
) -> Parser<'a, T, A> {
    Parser::new(LazyParser { inner: f })
}
