pub mod text;

use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::{Cause, Error};
use crate::parser::Parser;
use crate::prelude::TokenSet;
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
pub fn peek_in<T, S, E>(token_set: T) -> PeekIn<T, S, E>
where
    T: TokenSet<S::Token>,
    S: Stream,
    E: Error<S>,
{
    PeekIn {
        token_set,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; T)]
pub struct PeekIn<T, S, E> {
    token_set: T,
    _phantom: PhantomData<*const (S, E)>,
}

impl<T, S, E> Parser<S, S::Token, E> for PeekIn<T, S, E>
where
    T: TokenSet<S::Token>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if self.token_set.contains(&token) => Ok(token),
            _ => Err(E::new(
                E::Cause::expected_in_set(),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn eat_in<T, S, E>(token_set: T) -> EatIn<T, S, E>
where
    T: TokenSet<S::Token>,
    S: Stream,
    E: Error<S>,
{
    EatIn {
        token_set,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; T)]
pub struct EatIn<T, S, E> {
    token_set: T,
    _phantom: PhantomData<*const (S, E)>,
}

impl<T, S, E> Parser<S, S::Token, E> for EatIn<T, S, E>
where
    T: TokenSet<S::Token>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if self.token_set.contains(&token) => {
                stream.next_token();
                Ok(token)
            }
            _ => Err(E::new(
                E::Cause::expected_in_set(),
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
pub fn eat_while_in<T, S, E>(token_set: T) -> EatWhileIn<T, S, E>
where
    T: TokenSet<S::Token>,
    S: Stream,
    E: Error<S>,
{
    EatWhileIn {
        token_set,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; T)]
pub struct EatWhileIn<T, S, E> {
    token_set: T,
    _phantom: PhantomData<*const (S, E)>,
}

impl<T, S, E> Parser<S, S::SliceRef, E> for EatWhileIn<T, S, E>
where
    T: TokenSet<S::Token>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        let start = stream.stream_position();
        while stream
            .peek_token()
            .is_some_and(|t| self.token_set.contains(&t))
        {
            stream.next_token();
        }
        let end = stream.stream_position();
        Ok(stream.slice(start, end))
    }
}
