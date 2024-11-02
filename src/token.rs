pub mod text;

use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::{Cause, Error};
use crate::parser::Parser;
use crate::stream::Stream;

#[inline]
pub fn peek<Pat, S, E>(pattern: Pat) -> Peek<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    Peek {
        pattern,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; Pat)]
pub struct Peek<Pat, S, E> {
    pattern: Pat,
    _phantom: PhantomData<*const (S, E)>,
}

impl<Pat, S, E> Parser<S, S::Token, E> for Peek<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if self.pattern.is_match(&token) => Ok(token),
            _ => Err(E::new(self.pattern.error_cause(), stream.peek_token_span())),
        }
    }
}

#[inline]
pub fn eat<Pat, S, E>(pattern: Pat) -> Eat<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    Eat {
        pattern,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; Pat)]
pub struct Eat<Pat, S, E> {
    pattern: Pat,
    _phantom: PhantomData<*const (S, E)>,
}

impl<Pat, S, E> Parser<S, S::Token, E> for Eat<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if self.pattern.is_match(&token) => {
                stream.next_token();
                Ok(token)
            }
            _ => Err(E::new(self.pattern.error_cause(), stream.peek_token_span())),
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
pub fn seek_until<Pat, S, E>(pattern: Pat) -> SeekUntil<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    SeekUntil {
        pattern,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; Pat)]
pub struct SeekUntil<Pat, S, E> {
    pattern: Pat,
    _phantom: PhantomData<*const (S, E)>,
}

impl<Pat, S, E> Parser<S, S::Token, E> for SeekUntil<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        while let Some(token) = stream.peek_token() {
            if self.pattern.is_match(&token) {
                return Ok(token);
            }

            stream.next_token();
        }

        Err(E::new(self.pattern.error_cause(), stream.peek_token_span()))
    }
}

#[inline]
pub fn seek_past<Pat, S, E>(pattern: Pat) -> SeekPast<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    SeekPast {
        pattern,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; Pat)]
pub struct SeekPast<Pat, S, E> {
    pattern: Pat,
    _phantom: PhantomData<*const (S, E)>,
}

impl<Pat, S, E> Parser<S, S::Token, E> for SeekPast<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        while let Some(token) = stream.next_token() {
            if self.pattern.is_match(&token) {
                return Ok(token);
            }
        }

        Err(E::new(self.pattern.error_cause(), stream.peek_token_span()))
    }
}

#[inline]
pub fn consume<Pat, S, E>(pattern: Pat) -> Consume<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    Consume {
        pattern,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; Pat)]
pub struct Consume<Pat, S, E> {
    pattern: Pat,
    _phantom: PhantomData<*const (S, E)>,
}

impl<Pat, S, E> Parser<S, S::SliceRef, E> for Consume<Pat, S, E>
where
    Pat: Pattern<S::Token, E::Cause>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        let start = stream.stream_position();
        while stream
            .peek_token()
            .is_some_and(|t| self.pattern.is_match(&t))
        {
            stream.next_token();
        }
        let end = stream.stream_position();
        Ok(stream.slice(start, end))
    }
}

pub trait Pattern<Token, C: Cause> {
    fn is_match(&self, token: &Token) -> bool;
    fn error_cause(&self) -> C;
}

impl<T, F, C> Pattern<T, C> for F
where
    F: Fn(&T) -> bool,
    C: Cause,
{
    #[inline]
    fn is_match(&self, token: &T) -> bool {
        (*self)(token)
    }

    #[inline]
    fn error_cause(&self) -> C {
        C::expected_matching_fn()
    }
}

impl<C> Pattern<char, C> for char
where
    C: Cause<Token = char>,
{
    #[inline]
    fn is_match(&self, token: &char) -> bool {
        *token == *self
    }

    #[inline]
    fn error_cause(&self) -> C {
        C::expected_token(*self)
    }
}

impl<C> Pattern<u8, C> for u8
where
    C: Cause<Token = u8>,
{
    #[inline]
    fn is_match(&self, token: &u8) -> bool {
        *token == *self
    }

    #[inline]
    fn error_cause(&self) -> C {
        C::expected_token(*self)
    }
}
