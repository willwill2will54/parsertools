use non_empty_collections::NonEmptyIndexSet;

use crate::{LeftRecursionCheck, Parser};

use super::{AstBounds, ParseOutput, ParserInner, ParsingError, PartialParseResult, TokenBounds};

use std::collections::HashSet;

pub struct SeqParser<'a, Token: TokenBounds, Ast1: AstBounds, Ast2: AstBounds> {
    p1: Parser<'a, Token, Ast1>,
    p2: Parser<'a, Token, Ast2>,
}

impl<Token: TokenBounds, Ast1: AstBounds, Ast2: AstBounds> ParserInner
    for SeqParser<'_, Token, Ast1, Ast2>
{
    type Token = Token;
    type Ast = (Ast1, Ast2);

    fn parse<'a>(&self, tokens: &'a [Token]) -> ParseOutput<'a, Self::Ast, Self::Token> {
        // Parse the first part, then with each result, parse the second part
        // if the first part fails, return the error
        // if every result from the first part causes the second part to fail, return the first error
        let p1_res = self.p1.parse(tokens)?;
        let mut error: Option<ParsingError<Self::Token>> = None;
        let mut results = HashSet::new();
        for r1 in p1_res {
            match self.p2.parse(r1.remaining_tokens) {
                Ok(p2_res) => {
                    results.extend(p2_res.into_iter().map(|r2| PartialParseResult {
                        ast: (r1.ast.clone(), r2.ast),
                        remaining_tokens: r2.remaining_tokens,
                    }));
                }
                Err(e) => {
                    if error.is_none() {
                        error = Some(e);
                    }
                }
            }
        }
        NonEmptyIndexSet::from_iterator(results).map_err(|_| error.unwrap())
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        if depth == 0 {
            return LeftRecursionCheck::NotOk(vec![]);
        }
        self.p1.check_left_recursion(depth - 1)
    }
}

pub fn seq<'a, Token: TokenBounds, Ast1: AstBounds, Ast2: AstBounds>(
    p1: Parser<'a, Token, Ast1>,
    p2: Parser<'a, Token, Ast2>,
) -> SeqParser<'a, Token, Ast1, Ast2> {
    SeqParser { p1, p2 }
}
