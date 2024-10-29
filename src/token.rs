// TODO: add more bounds to builder functions.

pub mod text;

use crate::error::{Cause, Error};
use crate::parser::Parser;
use crate::prelude::TokenSet;
use crate::stream::{Stream, StreamEatSlice};

#[inline]
pub fn peek<T>(token: T) -> Peek<T> {
    Peek(token)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Peek<T>(T);

impl<S, E> Parser<S, S::Token, E> for Peek<S::Token>
where
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if token == self.0 => Ok(token),
            _ => Err(E::new(
                Cause::ExpectedToken(self.0.clone()),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn eat<T>(token: T) -> Eat<T> {
    Eat(token)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Eat<T>(T);

impl<S, E> Parser<S, S::Token, E> for Eat<S::Token>
where
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if token == self.0 => {
                stream.next_token();
                Ok(token)
            }
            _ => Err(E::new(
                Cause::ExpectedToken(self.0.clone()),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn peek_in<T>(token_set: T) -> PeekIn<T> {
    PeekIn(token_set)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeekIn<T>(T);

impl<T, S, E> Parser<S, S::Token, E> for PeekIn<T>
where
    T: TokenSet<S::Token>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if self.0.contains(&token) => Ok(token),
            _ => Err(E::new(Cause::ExpectedInSet, stream.peek_token_span())),
        }
    }
}

#[inline]
pub fn eat_in<T>(token_set: T) -> EatIn<T> {
    EatIn(token_set)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EatIn<T>(T);

impl<T, S, E> Parser<S, S::Token, E> for EatIn<T>
where
    T: TokenSet<S::Token>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if self.0.contains(&token) => {
                stream.next_token();
                Ok(token)
            }
            _ => Err(E::new(Cause::ExpectedInSet, stream.peek_token_span())),
        }
    }
}

#[inline]
pub fn peek_slice<Slice: ?Sized>(slice: &Slice) -> PeekSlice<Slice> {
    PeekSlice(slice)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeekSlice<'a, Slice: ?Sized>(&'a Slice);

impl<'a, Slice, S, E> Parser<S, S::SliceRef, E> for PeekSlice<'a, Slice>
where
    Slice: ?Sized,
    S: StreamEatSlice<Slice>,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        stream
            .peek_slice(self.0)
            .ok_or_else(|| E::new(Cause::ExpectedSlice, stream.peek_token_span()))
    }
}

#[inline]
pub fn eat_slice<Slice: ?Sized>(slice: &Slice) -> EatSlice<Slice> {
    EatSlice(slice)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EatSlice<'a, Slice: ?Sized>(&'a Slice);

impl<'a, Slice, S, E> Parser<S, S::SliceRef, E> for EatSlice<'a, Slice>
where
    Slice: ?Sized,
    S: StreamEatSlice<Slice>,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        stream
            .eat_slice(self.0)
            .ok_or_else(|| E::new(Cause::ExpectedSlice, stream.peek_token_span()))
    }
}

#[inline]
pub fn end() -> End {
    End {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct End {}

impl<S, E> Parser<S, (), E> for End
where
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<(), E> {
        if stream.at_end() {
            Ok(())
        } else {
            Err(E::new(Cause::ExpectedEnd, stream.peek_token_span()))
        }
    }
}

#[inline]
pub fn eat_while_in<T>(token_set: T) -> EatWhileIn<T> {
    EatWhileIn { token_set }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EatWhileIn<T> {
    pub(crate) token_set: T,
}

impl<T, S, E> Parser<S, S::SliceRef, E> for EatWhileIn<T>
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
