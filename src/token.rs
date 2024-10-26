// TODO: add more bounds to builder functions.

pub mod text;

use crate::error::BuiltinError;
use crate::parser::Parser;
use crate::stream::Stream;

#[inline]
pub fn peek<T>(token: T) -> Peek<T> {
    Peek(token)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Peek<T>(T);

impl<S> Parser<S, S::Token, BuiltinError<S>> for Peek<S::Token>
where
    S: Stream,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, BuiltinError<S>> {
        match stream.peek_token() {
            Some(token) if token == self.0 => Ok(token),
            _ => Err(BuiltinError::ExpectedToken(self.0.clone())),
        }
    }
}

#[inline]
pub fn eat<T>(token: T) -> Eat<T> {
    Eat(token)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Eat<T>(T);

impl<S> Parser<S, S::Token, BuiltinError<S>> for Eat<S::Token>
where
    S: Stream,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, BuiltinError<S>> {
        match stream.peek_token() {
            Some(token) if token == self.0 => {
                stream.advance();
                Ok(token)
            }
            _ => Err(BuiltinError::ExpectedToken(self.0.clone())),
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

impl<S> Parser<S, S::Token, BuiltinError<S>> for PeekAny
where
    S: Stream,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, BuiltinError<S>> {
        stream.peek_token().ok_or_else(|| BuiltinError::ExpectedAny)
    }
}

#[inline]
pub fn peek_match<F>(f: F) -> PeekMatch<F> {
    PeekMatch(f)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeekMatch<F>(F);

impl<S, F> Parser<S, S::Token, BuiltinError<S>> for PeekMatch<F>
where
    S: Stream,
    F: FnMut(&S::Token) -> bool,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, BuiltinError<S>> {
        match stream.peek_token() {
            Some(token) if (self.0)(&token) => Ok(token),
            _ => Err(BuiltinError::ExpectedMatch),
        }
    }
}

#[inline]
pub fn eat_match<F>(f: F) -> EatMatch<F> {
    EatMatch(f)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EatMatch<F>(F);

impl<S, F> Parser<S, S::Token, BuiltinError<S>> for EatMatch<F>
where
    S: Stream,
    F: FnMut(&S::Token) -> bool,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, BuiltinError<S>> {
        match stream.peek_token() {
            Some(token) if (self.0)(&token) => {
                stream.advance();
                Ok(token)
            }
            _ => Err(BuiltinError::ExpectedMatch),
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

impl<S> Parser<S, S::Token, BuiltinError<S>> for EatAny
where
    S: Stream,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<S::Token, BuiltinError<S>> {
        stream.next_token().ok_or_else(|| BuiltinError::ExpectedAny)
    }
}

#[inline]
pub fn end() -> End {
    End {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct End {}

impl<S> Parser<S, (), BuiltinError<S>> for End
where
    S: Stream,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<(), BuiltinError<S>> {
        if stream.at_end() {
            Ok(())
        } else {
            Err(BuiltinError::ExpectedEnd)
        }
    }
}
