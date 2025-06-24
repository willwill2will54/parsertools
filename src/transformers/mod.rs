mod filter;
mod map;
mod split;

pub(crate) use filter::filter;
pub(crate) use map::map;
pub(crate) use split::split_map;

use crate::{helpers::lazy, tokens::pred, AstBounds, Parser, TokenBounds};

pub fn alternating<'a, T: 'a + TokenBounds, A: 'a + AstBounds> (left: Parser<'a,T,A>, right: Parser<'a,T,A>) -> Parser<'a, T,Vec<A>> {
    alternating_vecs(vecify(left), vecify(right))
}
pub fn alternating_vecs<'a, T: 'a + TokenBounds, A: 'a + AstBounds> (a: Parser<'a,T,Vec<A>>, b: Parser<'a,T,Vec<A>>) -> Parser<'a, T,Vec<A>> {
    let abx = alternating_inner(a.clone(),b.clone());
    let bax = alternating_inner(b.clone(),a.clone());
    let a_bax = concat_vecs(a.clone(),bax.clone());
    let b_abx = concat_vecs(b.clone(),abx.clone());
    disjunction([a,b,abx,bax,a_bax,b_abx])
}
fn alternating_inner<'a, T: 'a + TokenBounds, A: 'a + AstBounds> (left: Parser<'a,T,Vec<A>>, right: Parser<'a,T,Vec<A>>) -> Parser<'a, T,Vec<A>> {
    series_vecs(concat_vecs(left,right))
}

pub fn repeat_multiple_of_n_times<'a,T: 'a + TokenBounds, A: 'a + AstBounds>(parser: Parser<'a,T,A>, n: usize) -> Parser<'a,T,Vec<A>> {
    repeat_multiple_of_n_times_vecs(vecify(parser),n)
}
pub fn repeat_multiple_of_n_times_vecs<'a,T: 'a + TokenBounds, A: 'a + AstBounds>(parser: Parser<'a,T,Vec<A>>, n: usize) -> Parser<'a,T,Vec<A>> {
    series_vecs(repeat_n_times_vecs(parser,n))
}

pub fn repeat_n_times<'a,T: 'a + TokenBounds, A: 'a + AstBounds>(parser: Parser<'a,T,A>, n: usize) -> Parser<'a,T,Vec<A>> {
    repeat_n_times_vecs(vecify(parser), n)
}
pub fn repeat_n_times_vecs<'a,T: 'a + TokenBounds, A: 'a + AstBounds>(parser: Parser<'a,T,Vec<A>>, n: usize) -> Parser<'a,T,Vec<A>> {
    if n == 0 { panic!("Attempted to repeat parser 0 times") }
    else if n == 1 { parser }
    else { concat_vecs(repeat_n_times_vecs(parser.clone(), n-1),parser) }
}

pub fn series<'a, T: 'a + TokenBounds, A: 'a + AstBounds>(parser: Parser<'a, T,A>) -> Parser<'a, T,Vec<A>> {
    series_vecs(vecify(parser))
}
pub fn series_vecs<'a, T: 'a + TokenBounds, A: 'a + AstBounds>(parser: Parser<'a, T,Vec<A>>) -> Parser<'a, T,Vec<A>> {
    parser.clone().or(concat_vecs(parser.clone(),lazy(move || series_vecs(parser.clone()))))
}

pub fn conjoin<'a, T: 'a + TokenBounds, A: 'a + AstBounds, I: IntoIterator<Item=Parser<'a, T,A>>> (parsers: I) -> Parser<'a, T,Vec<A>> {
    conjoin_vecs(parsers.into_iter().map(|parser| vecify(parser)))
}
pub fn conjoin_vecs<'a, T: 'a + TokenBounds, A: 'a + AstBounds, I: IntoIterator<Item=Parser<'a, T,Vec<A>>>> (parsers: I) -> Parser<'a, T,Vec<A>> {
    parsers.into_iter()
        .reduce(|acc,next| concat_vecs(acc, next))
        .unwrap_or(pred(|_| Some(vec![])))
}

pub fn disjunction<'a,T:'a + TokenBounds,A: 'a + AstBounds, I: IntoIterator<Item = Parser<'a,T,A>>>(parsers: I) -> Parser<'a,T,A> {
    parsers.into_iter()
        .reduce(|acc,next| acc.or(next))
        .unwrap_or(pred(|_| None))
}

pub fn concat<'a, T: 'a + TokenBounds, A: 'a + AstBounds> (left: Parser<'a,T,A>, right: Parser<'a,T,A>) -> Parser<'a, T,Vec<A>> {
    concat_vecs(vecify(left),vecify(right))
}
fn concat_vecs<'a, T: 'a + TokenBounds, A: 'a + AstBounds> (left: Parser<'a,T,Vec<A>>, right: Parser<'a,T,Vec<A>>) -> Parser<'a, T,Vec<A>> {
    left.then(right).map(|(l,r)| [l,r].concat())
}

pub fn vecify<'a, T: 'a + TokenBounds, A: 'a + AstBounds> (parser: Parser<'a, T,A>) -> Parser<'a, T,Vec<A>> {
    parser.map(|token| vec![token])
}
