use tracing::{span, trace, Level};

use crate::{AstBounds, ParseFrontOutput, ParserInner, TokenBounds,LeftRecursionCheck,Parser};

pub(crate) struct DebugParser<'a, T: TokenBounds, A: AstBounds> {
    pub(crate) inner: Parser<'a, T, A>,
    pub(crate) msg: Option<String>,
}

impl<T: TokenBounds, A: AstBounds> ParserInner for DebugParser<'_, T, A>
where
    T: std::fmt::Debug,
    A: std::fmt::Debug,
{
    type Token = T;
    type Ast = A;

    fn parse_front<'a>(&self, tokens: &'a [Self::Token]) -> ParseFrontOutput<'a, Self::Ast, Self::Token> {
        let span = span!(
            Level::INFO,
            "parsing",
            remaining_tokens = tokens.len(),
            label = self.msg.as_deref()
        );
        let _enter = span.enter();
        let result = self.inner.parse_front(tokens);
        match &result {
            Ok(results) => {
                trace!("Parse results: {:?}", results);
            }
            Err(e) => {
                trace!("Error: {}", e);
            }
        }
        result
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            let mut v = vec![];
            if let Some(msg) = self.msg.as_ref() {
                v.push(msg.clone())
            }
            return LeftRecursionCheck::NotOk(v);
        }
        if let LeftRecursionCheck::NotOk(mut v) = self.inner.check_left_recursion(depth - 1) {
            if let Some(msg) = self.msg.as_ref() {
                v.push(msg.clone())
            }
            LeftRecursionCheck::NotOk(v)
        } else {
            LeftRecursionCheck::Ok
        }
    }
}
