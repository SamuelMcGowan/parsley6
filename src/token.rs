pub mod text;

use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::{Cause, Error};
use crate::parser::Parser;
use crate::stream::Stream;

#[inline]
pub fn peek<S, E>(token: S::Token) -> Peek<S, E>
where
    S: Stream<Token: Clone>,
    E: Error<S>,
{
    Peek {
        token,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; S::Token)]
pub struct Peek<S: Stream, E> {
    token: S::Token,
    _phantom: PhantomData<*const E>,
}

impl<S, E> Parser<S, S::Token, E> for Peek<S, E>
where
    S: Stream<Token: Clone>,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if self.token == token => Ok(token),
            _ => Err(E::new(
                E::Cause::expected_token(self.token.clone()),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn eat<S, E>(token: S::Token) -> Eat<S, E>
where
    S: Stream<Token: Clone>,
    E: Error<S>,
{
    Eat {
        token,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; S::Token)]
pub struct Eat<S: Stream, E> {
    token: S::Token,
    _phantom: PhantomData<*const (S, E)>,
}

impl<S, E> Parser<S, S::Token, E> for Eat<S, E>
where
    S: Stream<Token: Clone>,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if self.token == token => {
                stream.next_token();
                Ok(token)
            }
            _ => Err(E::new(
                E::Cause::expected_token(self.token.clone()),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn peek_if<F, S, E>(f: F) -> PeekIf<F, S, E>
where
    F: Fn(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    PeekIf {
        f,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; F)]
pub struct PeekIf<F, S, E> {
    f: F,
    _phantom: PhantomData<*const (S, E)>,
}

impl<F, S, E> Parser<S, S::Token, E> for PeekIf<F, S, E>
where
    F: Fn(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if (self.f)(&token) => Ok(token),
            _ => Err(E::new(
                E::Cause::expected_predicate(),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn eat_if<F, S, E>(f: F) -> EatIf<F, S, E>
where
    F: Fn(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    EatIf {
        f,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; F)]
pub struct EatIf<F, S, E> {
    f: F,
    _phantom: PhantomData<*const (S, E)>,
}

impl<F, S, E> Parser<S, S::Token, E> for EatIf<F, S, E>
where
    F: Fn(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if (self.f)(&token) => {
                stream.next_token();
                Ok(token)
            }
            _ => Err(E::new(
                E::Cause::expected_predicate(),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn peek_slice<S, E>(slice: &'static S::Slice) -> PeekSlice<S, E>
where
    S: Stream<Slice: PartialEq>,
    E: Error<S>,
{
    PeekSlice {
        slice,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; &'static S::Slice)]
pub struct PeekSlice<S: Stream, E> {
    slice: &'static S::Slice,
    _phantom: PhantomData<*const E>,
}

impl<S, E> Parser<S, S::SliceRef, E> for PeekSlice<S, E>
where
    S: Stream<Slice: PartialEq>,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        stream.peek_slice(self.slice).ok_or_else(|| {
            E::new(
                E::Cause::expected_slice(self.slice),
                stream.peek_token_span(),
            )
        })
    }
}

#[inline]
pub fn eat_slice<S, E>(slice: &'static S::Slice) -> EatSlice<S, E>
where
    S: Stream<Slice: PartialEq>,
    E: Error<S>,
{
    EatSlice {
        slice,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; &'static S::Slice)]
pub struct EatSlice<S: Stream, E> {
    slice: &'static S::Slice,
    _phantom: PhantomData<*const E>,
}

impl<S, E> Parser<S, S::SliceRef, E> for EatSlice<S, E>
where
    S: Stream<Slice: PartialEq>,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        stream.eat_slice(self.slice).ok_or_else(|| {
            E::new(
                E::Cause::expected_slice(self.slice),
                stream.peek_token_span(),
            )
        })
    }
}

#[inline]
pub fn end<S, E>() -> End<S, E>
where
    S: Stream,
    E: Error<S>,
{
    End {
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct End<S, E> {
    _phantom: PhantomData<*const (S, E)>,
}

impl<S, E> Parser<S, (), E> for End<S, E>
where
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<(), E> {
        if stream.at_end() {
            Ok(())
        } else {
            Err(E::new(E::Cause::expected_end(), stream.peek_token_span()))
        }
    }
}

#[inline]
pub fn eat_while<F, S, E>(f: F) -> EatWhile<F, S, E>
where
    F: Fn(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    EatWhile {
        f,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; F)]
pub struct EatWhile<F, S, E> {
    f: F,
    _phantom: PhantomData<*const (S, E)>,
}

impl<F, S, E> Parser<S, S::SliceRef, E> for EatWhile<F, S, E>
where
    F: Fn(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        let start = stream.stream_position();
        while stream.peek_token().is_some_and(|t| (self.f)(&t)) {
            stream.next_token();
        }
        let end = stream.stream_position();
        Ok(stream.slice(start, end))
    }
}
