use std::ops::Range;

use derive_where::derive_where;

use crate::stream::Stream;

pub trait Error<S: Stream>: Sized {
    type Cause: From<BuiltinCause<S>>;
    type Context;

    fn new(cause: Self::Cause, span: Range<S::SourceLoc>) -> Self;

    fn with_cause(self, cause: Self::Cause) -> Self;
    fn with_context(self, context: Self::Context, span: Range<S::SourceLoc>) -> Self;

    fn span(&self) -> Range<S::SourceLoc>;
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token)]
pub enum BuiltinCause<S: Stream> {
    ExpectedToken(S::Token),
    ExpectedMatch,
    ExpectedAny,
    ExpectedEnd,
    Unknown,
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token, S::SourceLoc)]
pub enum DefaultError<S: Stream> {
    Error {
        cause: DefaultErrorCause<S>,
        span: Range<S::SourceLoc>,
    },

    WithContext {
        context: Box<str>,
        span: Range<S::SourceLoc>,
        inner: Box<DefaultError<S>>,
    },
}

impl<S> Error<S> for DefaultError<S>
where
    S: Stream,
    S::SourceLoc: Clone,
{
    type Cause = DefaultErrorCause<S>;
    type Context = Box<str>;

    #[inline]
    fn new(kind: DefaultErrorCause<S>, span: Range<S::SourceLoc>) -> Self {
        Self::Error { cause: kind, span }
    }

    #[inline]
    fn with_cause(self, cause: Self::Cause) -> Self {
        match self {
            Self::Error { span, cause: _ } => Self::Error { cause, span },

            Self::WithContext {
                context,
                span,
                inner,
            } => Self::WithContext {
                context,
                span,
                inner: Box::new(inner.with_cause(cause)),
            },
        }
    }

    #[inline]
    fn with_context(self, context: Self::Context, span: Range<S::SourceLoc>) -> Self {
        Self::WithContext {
            context,
            span,
            inner: Box::new(self),
        }
    }

    #[inline]
    fn span(&self) -> Range<S::SourceLoc> {
        match self {
            Self::Error { span, .. } | Self::WithContext { span, .. } => span.clone(),
        }
    }
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token)]
pub enum DefaultErrorCause<S: Stream> {
    Builtin(BuiltinCause<S>),
    Custom(Box<str>),
}

impl<S: Stream> From<BuiltinCause<S>> for DefaultErrorCause<S> {
    #[inline]
    fn from(cause: BuiltinCause<S>) -> Self {
        DefaultErrorCause::Builtin(cause)
    }
}

impl<S: Stream, T: Into<Box<str>>> From<T> for DefaultErrorCause<S> {
    #[inline]
    fn from(message: T) -> Self {
        DefaultErrorCause::Custom(message.into())
    }
}

impl<S: Stream> DefaultErrorCause<S> {
    #[inline]
    pub fn custom(message: impl Into<Box<str>>) -> Self {
        Self::Custom(message.into())
    }
}
