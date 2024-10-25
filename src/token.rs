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

impl<'a, S> Parser<'a, S, S::Token, BuiltinError<'a, S>> for Peek<S::Token>
where
    S: Stream<'a>,
{
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<S::Token, BuiltinError<'a, S>> {
        match input.peek_token() {
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

impl<'a, S> Parser<'a, S, S::Token, BuiltinError<'a, S>> for Eat<S::Token>
where
    S: Stream<'a>,
{
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<S::Token, BuiltinError<'a, S>> {
        match input.peek_token() {
            Some(token) if token == self.0 => {
                input.advance();
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

impl<'a, S> Parser<'a, S, S::Token, BuiltinError<'a, S>> for PeekAny
where
    S: Stream<'a>,
{
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<S::Token, BuiltinError<'a, S>> {
        input.peek_token().ok_or_else(|| BuiltinError::ExpectedAny)
    }
}

#[inline]
pub fn peek_match<F>(f: F) -> PeekMatch<F> {
    PeekMatch(f)
}

pub struct PeekMatch<F>(F);

impl<'a, S, F> Parser<'a, S, S::Token, BuiltinError<'a, S>> for PeekMatch<F>
where
    S: Stream<'a>,
    F: FnMut(&S::Token) -> bool,
{
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<S::Token, BuiltinError<'a, S>> {
        match input.peek_token() {
            Some(token) if (self.0)(&token) => Ok(token),
            _ => Err(BuiltinError::ExpectedMatch),
        }
    }
}

#[inline]
pub fn eat_match<F>(f: F) -> EatMatch<F> {
    EatMatch(f)
}

pub struct EatMatch<F>(F);

impl<'a, S, F> Parser<'a, S, S::Token, BuiltinError<'a, S>> for EatMatch<F>
where
    S: Stream<'a>,
    F: FnMut(&S::Token) -> bool,
{
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<S::Token, BuiltinError<'a, S>> {
        match input.peek_token() {
            Some(token) if (self.0)(&token) => {
                input.advance();
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

impl<'a, S> Parser<'a, S, S::Token, BuiltinError<'a, S>> for EatAny
where
    S: Stream<'a>,
{
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<S::Token, BuiltinError<'a, S>> {
        input.next_token().ok_or_else(|| BuiltinError::ExpectedAny)
    }
}

#[inline]
pub fn end() -> End {
    End {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct End {}

impl<'a, S> Parser<'a, S, (), BuiltinError<'a, S>> for End
where
    S: Stream<'a>,
{
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<(), BuiltinError<'a, S>> {
        if input.at_end() {
            Ok(())
        } else {
            Err(BuiltinError::ExpectedEnd)
        }
    }
}
