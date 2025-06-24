mod filter;
mod map;
mod split;
pub mod vecs;

pub(crate) use filter::filter;
pub(crate) use map::map;
pub(crate) use split::split_map;
pub use vecs::*;

use crate::{tokens::pred, AstBounds, Parser, TokenBounds};

pub fn disjunction<'a,T:'a + TokenBounds,A: 'a + AstBounds>(parsers: impl IntoIterator<Item = Parser<'a,T,A>>) -> Parser<'a,T,A> {
    parsers.into_iter()
        .reduce(|acc,next| acc.or(next))
        .unwrap_or(pred(|_| None))
}
