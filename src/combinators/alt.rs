use crate::{AstBounds, ParseInnerOutput, ParserInner, TokenBounds,LeftRecursionCheck,Parser};

#[derive(Clone)]
pub(crate) struct AltParser<'a, Token: TokenBounds, Ast: AstBounds> {
    pub(super) p1: Parser<'a, Token, Ast>,
    pub(super) p2: Parser<'a, Token, Ast>,
}

impl<Token: TokenBounds, Ast: AstBounds> ParserInner for AltParser<'_, Token, Ast> {
    type Token = Token;
    type Ast = Ast;

    fn parse_inner<'a>(&self, tokens: &'a [Token]) -> ParseInnerOutput<'a, Self::Ast, Self::Token> {
        // p1 success and p2 success: return both
        // p1 success and p2 fail: return p1
        // p1 fail and p2 success: return p2
        // p1 fail and p2 fail: return p1
        let tokens_remaining = tokens.len();
        match self.p1.parse_inner(tokens) {
            Ok(mut p1_res) => match self.p2.parse_inner(tokens) {
                Ok(p2_res) => {
                    p1_res.extend(p2_res);
                    let max_len = p1_res
                        .iter()
                        .map(|x| x.remaining_tokens.len())
                        .max()
                        .unwrap_or(0);
                    assert!(tokens_remaining > max_len);
                    Ok(p1_res)
                }
                Err(_) => Ok(p1_res),
            },
            Err(err) => self.p2.parse_inner(tokens).map_err(|_| err),
        }
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            return LeftRecursionCheck::Ok;
        }
        self.p1
            .check_left_recursion(depth - 1)
            .not_ok_or_else(|| self.p2.check_left_recursion(depth - 1))
    }
}

pub (crate) fn alt<'a, Token: TokenBounds, Ast: AstBounds>(
    p1: Parser<'a, Token, Ast>,
    p2: Parser<'a, Token, Ast>,
) -> AltParser<'a, Token, Ast> {
    AltParser { p1, p2 }
}
