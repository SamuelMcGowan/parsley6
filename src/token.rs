use crate::{error::BuiltinError, parser::Parser, stream::Stream};

#[inline]
pub fn peek<T>(token: T) -> Peek<T> {
    Peek(token)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Peek<T>(T);

impl<'a, S: Stream<'a>> Parser<'a, S, S::Token, BuiltinError<'a, S>> for Peek<S::Token> {
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

impl<'a, S: Stream<'a>> Parser<'a, S, S::Token, BuiltinError<'a, S>> for Eat<S::Token> {
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

impl<'a, S: Stream<'a>> Parser<'a, S, S::Token, BuiltinError<'a, S>> for PeekAny {
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<S::Token, BuiltinError<'a, S>> {
        input.peek_token().ok_or_else(|| BuiltinError::ExpectedAny)
    }
}

#[inline]
pub fn eat_any() -> EatAny {
    EatAny {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct EatAny {}

impl<'a, S: Stream<'a>> Parser<'a, S, S::Token, BuiltinError<'a, S>> for EatAny {
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

impl<'a, S: Stream<'a>> Parser<'a, S, (), BuiltinError<'a, S>> for End {
    #[inline]
    fn parse(&mut self, input: &mut S) -> Result<(), BuiltinError<'a, S>> {
        if input.at_end() {
            Ok(())
        } else {
            Err(BuiltinError::ExpectedEnd)
        }
    }
}
