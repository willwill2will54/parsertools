use std::collections::HashSet;

use non_empty_collections::NonEmptyIndexSet;

use crate::{results::{LeftRecursionCheck, ParseError, ParseOutput, PartialParseResult}, AstBounds, TokenBounds};

pub (super) trait ParserInner: Sync + Send {
    type Token: TokenBounds;
    type Ast: AstBounds;

    fn parse_front<'a>(&self, tokens: &'a [Self::Token]) -> ParseFrontOutput<'a, Self::Ast, Self::Token>;

    fn parse_unambiguous<'a>(
        &self,
        tokens: &'a [Self::Token],
    ) -> ParseOutput<Self::Ast, Self::Token> {
        let parsed = self.parse_front(tokens)?;
        let filtered: Vec<_> = parsed
            .iter()
            .filter(|p| p.remaining_tokens.is_empty())
            .map(|p| p.ast.clone())
            .collect();
        if filtered.is_empty() {
            let remaining_tokens = parsed
                .iter()
                .min_by_key(|x| x.remaining_tokens.len())
                .unwrap()
                .remaining_tokens
                .to_vec();
            Err(ParseError::UnhandledTokens(remaining_tokens))
        } else if filtered.len() == 1 {
            Ok(filtered.first().unwrap().clone())
        } else {
            Err(ParseError::AmbiguousGrammar(
                filtered.into_iter().map(|x| format!("{x:?}")).collect(),
            ))
        }
    }

    fn parse<'a>(
        &self,
        tokens: &'a [Self::Token],
    ) -> HashSet<Self::Ast> {
        let Ok(parsed) = self.parse_front(tokens) else { return HashSet::new() };
        parsed
            .iter()
            .filter(|p| p.remaining_tokens.is_empty())
            .map(|p| p.ast.clone())
            .collect()
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck;
}

pub type ParseFrontOutput<'a, Ast, Token> =
    Result<NonEmptyIndexSet<PartialParseResult<'a, Ast, Token>>, ParseError<Token>>;
