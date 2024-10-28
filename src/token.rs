// TODO: add more bounds to builder functions.

pub mod text;

use crate::error::{BuiltinCause, Error};
use crate::parser::Parser;
use crate::stream::Stream;

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
                BuiltinCause::ExpectedToken(self.0.clone()).into(),
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
                BuiltinCause::ExpectedToken(self.0.clone()).into(),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn peek_any() -> PeekAny {
    PeekAny {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct PeekAny {}

impl<S, E> Parser<S, S::Token, E> for PeekAny
where
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        stream
            .peek_token()
            .ok_or_else(|| E::new(BuiltinCause::ExpectedEnd.into(), stream.peek_token_span()))
    }
}

#[inline]
pub fn peek_match<F>(f: F) -> PeekMatch<F> {
    PeekMatch(f)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeekMatch<F>(F);

impl<F, S, E> Parser<S, S::Token, E> for PeekMatch<F>
where
    F: FnMut(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if (self.0)(&token) => Ok(token),
            _ => Err(E::new(
                BuiltinCause::ExpectedMatch.into(),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn eat_match<F>(f: F) -> EatMatch<F> {
    EatMatch(f)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EatMatch<F>(F);

impl<F, S, E> Parser<S, S::Token, E> for EatMatch<F>
where
    F: FnMut(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if (self.0)(&token) => {
                stream.next_token();
                Ok(token)
            }
            _ => Err(E::new(
                BuiltinCause::ExpectedMatch.into(),
                stream.peek_token_span(),
            )),
        }
    }
}

#[inline]
pub fn eat_any() -> EatAny {
    EatAny {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct EatAny {}

impl<S, E> Parser<S, S::Token, E> for EatAny
where
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        stream
            .next_token()
            .ok_or_else(|| E::new(BuiltinCause::ExpectedAny.into(), stream.peek_token_span()))
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
            Err(E::new(
                BuiltinCause::ExpectedEnd.into(),
                stream.peek_token_span(),
            ))
        }
    }
}

#[inline]
pub fn eat_while<F>(f: F) -> EatWhile<F> {
    EatWhile { f }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EatWhile<F> {
    pub(crate) f: F,
}

impl<F, S, E> Parser<S, S::SliceRef, E> for EatWhile<F>
where
    F: FnMut(&S::Token) -> bool,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        let start = stream.stream_position();
        while stream.peek_token().is_some_and(|t| (self.f)(&t)) {
            stream.next_token();
        }
        let end = stream.stream_position();
        Ok(stream.slice(start, end))
    }
}
