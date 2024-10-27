use std::ops::Range;

use derive_where::derive_where;

use crate::stream::Stream;

pub trait Error<S: Stream> {
    type Cause: ErrorCause<S>;
    type Context;

    fn new(cause: Self::Cause, span: Range<S::SourceLoc>) -> Self;

    fn set_cause(&mut self, cause: Self::Cause);
    fn add_context(&mut self, context: Self::Context, span: Range<S::SourceLoc>);

    fn span(&self) -> Range<S::SourceLoc>;
}

pub trait ErrorCause<S: Stream> {
    fn expected_token(token: S::Token) -> Self;
    fn expected_match() -> Self;
    fn expected_any() -> Self;
    fn expected_end() -> Self;

    fn unknown() -> Self;
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token, S::SourceLoc)]
pub struct DefaultError<S: Stream> {
    pub cause: DefaultErrorCause<S>,
    pub span: Range<S::SourceLoc>,

    pub context: Vec<(String, Range<S::SourceLoc>)>,
}

impl<S: Stream> Error<S> for DefaultError<S> {
    type Cause = DefaultErrorCause<S>;
    type Context = String;

    #[inline]
    fn new(kind: DefaultErrorCause<S>, span: Range<S::SourceLoc>) -> Self {
        Self {
            cause: kind,
            span,
            context: vec![],
        }
    }

    #[inline]
    fn set_cause(&mut self, cause: Self::Cause) {
        self.cause = cause;
    }

    #[inline]
    fn add_context(&mut self, context: Self::Context, span: Range<S::SourceLoc>) {
        self.context.push((context, span));
    }

    #[inline]
    fn span(&self) -> Range<S::SourceLoc> {
        self.span.clone()
    }
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token)]
pub enum DefaultErrorCause<S: Stream> {
    Token(S::Token),
    Match,
    Any,
    End,

    Unknown,

    Custom(String),
}

impl<S: Stream> From<String> for DefaultErrorCause<S> {
    #[inline]
    fn from(message: String) -> Self {
        DefaultErrorCause::Custom(message)
    }
}

impl<S: Stream> From<&str> for DefaultErrorCause<S> {
    #[inline]
    fn from(message: &str) -> Self {
        DefaultErrorCause::Custom(message.into())
    }
}

impl<S: Stream> DefaultErrorCause<S> {
    #[inline]
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }
}

impl<S: Stream> ErrorCause<S> for DefaultErrorCause<S> {
    #[inline]
    fn expected_token(token: S::Token) -> Self {
        DefaultErrorCause::Token(token)
    }

    #[inline]
    fn expected_match() -> Self {
        DefaultErrorCause::Match
    }

    #[inline]
    fn expected_any() -> Self {
        DefaultErrorCause::Any
    }

    #[inline]
    fn expected_end() -> Self {
        DefaultErrorCause::End
    }

    #[inline]
    fn unknown() -> Self {
        DefaultErrorCause::Unknown
    }
}
