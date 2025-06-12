use non_empty_collections::NonEmptyIndexSet;
use std::fmt;
use std::hash::Hash;
use std::sync::Arc;
use thiserror::Error;

pub trait TokenBounds: Eq + Hash + fmt::Debug + Clone + Sync + Send {}

impl<T: Eq + Hash + fmt::Debug + Clone + Sync + Send> TokenBounds for T {}

pub trait AstBounds: PartialEq + Hash + Clone + fmt::Debug {}

impl<T: PartialEq + Hash + Clone + fmt::Debug> AstBounds for T {}

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
    remaining_tokens: &'a [Token],
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

pub type ParseInnerOutput<'a, Ast, Token> =
    Result<NonEmptyIndexSet<PartialParseResult<'a, Ast, Token>>, ParseError<Token>>;

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

trait ParserInner: Sync + Send {
    type Token: TokenBounds;
    type Ast: AstBounds;

    fn parse_inner<'a>(&self, tokens: &'a [Self::Token]) -> ParseInnerOutput<'a, Self::Ast, Self::Token>;

    fn parse<'a>(
        &self,
        tokens: &'a [Self::Token],
    ) -> ParseOutput<'a, Self::Ast, Self::Token> {
        let parsed = self.parse_inner(tokens)?;
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

    fn parse_all<'a>(
        &self,
        tokens: &'a [Self::Token],
    ) -> Vec<Self::Ast> {
        let Ok(parsed) = self.parse_inner(tokens) else { return Vec::new() };
        parsed
            .iter()
            .filter(|p| p.remaining_tokens.is_empty())
            .map(|p| p.ast.clone())
            .collect()
    }

    fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck;
}

#[derive(Clone)]
pub struct Parser<'a, T: TokenBounds, A: AstBounds> {
    inner: Arc<dyn ParserInner<Token = T, Ast = A> + 'a>,
}

impl<'a, T: TokenBounds + 'a, A: AstBounds + 'a> Parser<'a, T, A> {
    fn new<P: ParserInner<Token = T, Ast = A> + 'a>(inner: P) -> Self {
        Parser {
            inner: Arc::new(inner),
        }
    }

    pub fn parse_inner<'b>(&self, tokens: &'b [T]) -> ParseInnerOutput<'b, A, T> {
        self.inner.parse_inner(tokens)
    }

    pub fn parse<'b>(&self, tokens: &'b [T]) -> ParseOutput<'b, A, T> {
        self.inner.parse(tokens)
    }

    pub fn parse_all<'b>(&self, tokens: &'b [T]) -> Vec<A> {
        self.inner.parse_all(tokens)
    }

    pub fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        self.inner.check_left_recursion(depth)
    }

    pub fn or(self, p2: Self) -> Self {
        Parser::new(alt_parser::alt(self, p2))
    }

    pub fn then<Ast2: AstBounds + 'a>(self, p2: Parser<'a, T, Ast2>) -> Parser<'a, T, (A, Ast2)> {
        Parser::new(seq_parser::seq(self, p2))
    }

    pub fn map<F: Fn(A) -> Ast + 'a + Sync + Send, Ast: AstBounds + 'a>(
        self,
        f: F,
    ) -> Parser<'a, T, Ast> {
        Parser::new(map_parser::map(self, f))
    }

    pub fn debug_msg(self, msg: impl ToString) -> Self
    where
        Self: Sized,
    {
        Parser::new(debug_parser::DebugParser {
            inner: self,
            msg: Some(msg.to_string()),
        })
    }

    pub fn debug(self) -> Self
    where
        Self: Sized,
    {
        Parser::new(debug_parser::DebugParser {
            inner: self,
            msg: None,
        })
    }
}

mod alt_parser;

mod seq_parser;

mod map_parser;

mod single_token_parser;

mod token_predicate_parser;

mod lazy_parser;

mod debug_parser;

pub fn lazy<'a, T: TokenBounds + 'a, A: AstBounds + 'a, F: 'a + Send + Sync + Fn() -> Parser<'a,T,A>>(
    f: F,
) -> Parser<'a, T, A> {
    Parser::new(lazy_parser::lazy(f))
}

pub fn tok<'a, T: TokenBounds + 'a>(token: T) -> Parser<'a, T, ()> {
    Parser::new(single_token_parser::tok(token))
}

pub fn pred<'a, T: TokenBounds + 'a, A: AstBounds + 'a>(
    predicate: impl Fn(&T) -> Option<A> + Sync + Send + 'a,
) -> Parser<'a, T, A> {
    Parser::new(token_predicate_parser::pred(predicate))
}
