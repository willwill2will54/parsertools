use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::sync::Arc;

use crate::parsers::{inner::{ParseInnerOutput, ParserInner}, results::{LeftRecursionCheck, ParseError, ParseOutput}};

mod inner;
pub mod results;

pub mod tokens;
pub mod combinators;
pub mod transformers;
pub mod helpers;

pub trait TokenBounds: Eq + Hash + fmt::Debug + Clone + Sync + Send {}
impl<T: Eq + Hash + fmt::Debug + Clone + Sync + Send> TokenBounds for T {}

pub trait AstBounds: PartialEq + Eq + Hash + Clone + fmt::Debug {}
impl<T: PartialEq + Eq + Hash + Clone + fmt::Debug> AstBounds for T {}

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

    pub fn parse<'b,I: IntoIterator<Item = T>>(&self, tokens: I) -> ParseOutput<'b, A, T> {
        let tokens: Vec<T> = tokens.into_iter().collect();
        self.inner.parse(tokens.as_slice())
    }

    pub fn parse_all<'b,I: IntoIterator<Item = T>>(&self, tokens: I) -> HashSet<A> {
        let tokens: Vec<T> = tokens.into_iter().collect();
        self.inner.parse_all(tokens.as_slice())
    }

    pub fn check_left_recursion(&self, depth: usize) -> LeftRecursionCheck {
        self.inner.check_left_recursion(depth)
    }

    pub fn or(self, p2: Self) -> Self {
        Parser::new(combinators::alt(self, p2))
    }

    pub fn then<Ast2: AstBounds + 'a>(self, p2: Parser<'a, T, Ast2>) -> Parser<'a, T, (A, Ast2)> {
        Parser::new(combinators::seq(self, p2))
    }

    pub fn map<F: Fn(A) -> Ast + 'a + Sync + Send, Ast: AstBounds + 'a>
        (self,f: F) -> Parser<'a, T, Ast> {
        Parser::new(transformers::map(self, f))
    }

    pub fn filter<F: Fn(&A) -> bool + 'a + Sync + Send>
        (self, f: F, e: ParseError<T>) -> Parser<'a, T, A> {
        transformers::filter(self, f, e)
    }

    pub fn split_map<It: 'a + IntoIterator<Item=Ast>, F: Fn(A) -> It + 'a + Sync + Send, Ast: AstBounds + 'a>
        (self,f: F) -> Parser<'a, T, Ast> {
        Parser::new(transformers::split_map(self, f))
    }

    pub fn debug_msg(self, msg: impl ToString) -> Self where Self: Sized, {
        Parser::new(helpers::DebugParser {
            inner: self,
            msg: Some(msg.to_string()),
        })
    }

    pub fn debug(self) -> Self where Self: Sized, {
        Parser::new(helpers::DebugParser {
            inner: self,
            msg: None,
        })
    }
}
