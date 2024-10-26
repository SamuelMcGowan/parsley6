// TODO: add more bounds to builder functions.

pub mod text;

use crate::error::Error;
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
            _ => Err(E::expected_token(self.0.clone(), stream.source_span())),
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
                stream.advance();
                Ok(token)
            }
            _ => Err(E::expected_token(self.0.clone(), stream.source_span())),
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
            .ok_or_else(|| E::expected_end(stream.source_span()))
    }
}

#[inline]
pub fn peek_match<F>(f: F) -> PeekMatch<F> {
    PeekMatch(f)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeekMatch<F>(F);

impl<S, E, F> Parser<S, S::Token, E> for PeekMatch<F>
where
    S: Stream,
    E: Error<S>,
    F: FnMut(&S::Token) -> bool,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if (self.0)(&token) => Ok(token),
            _ => Err(E::expected_match(stream.source_span())),
        }
    }
}

#[inline]
pub fn eat_match<F>(f: F) -> EatMatch<F> {
    EatMatch(f)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EatMatch<F>(F);

impl<S, E, F> Parser<S, S::Token, E> for EatMatch<F>
where
    S: Stream,
    E: Error<S>,
    F: FnMut(&S::Token) -> bool,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, E> {
        match stream.peek_token() {
            Some(token) if (self.0)(&token) => {
                stream.advance();
                Ok(token)
            }
            _ => Err(E::expected_match(stream.source_span())),
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
            .ok_or_else(|| E::expected_any(stream.source_span()))
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
            Err(E::expected_end(stream.source_span()))
        }
    }
}
