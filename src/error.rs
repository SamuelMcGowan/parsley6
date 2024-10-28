use std::ops::Range;

use derive_where::derive_where;

use crate::stream::Stream;

pub trait Error<S: Stream>: Sized {
    type Context;
    type CustomCause;

    fn new(cause: Cause<S, Self::CustomCause>, span: Range<S::SourceLoc>) -> Self;

    fn with_cause(self, cause: Cause<S, Self::CustomCause>) -> Self;
    fn with_context(self, context: Self::Context, span: Range<S::SourceLoc>) -> Self;

    fn span(&self) -> Range<S::SourceLoc>;
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token, Custom)]
pub enum Cause<S: Stream, Custom> {
    Custom(Custom),
    Unknown,

    ExpectedToken(S::Token),
    ExpectedInSet,
    ExpectedEnd,
}

impl<S: Stream, Custom> Cause<S, Custom> {
    #[inline]
    pub fn custom(custom: impl Into<Custom>) -> Self {
        Self::Custom(custom.into())
    }
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token, S::SourceLoc, CustomCause)]
pub enum DefaultError<S: Stream, CustomCause = Box<str>> {
    Error {
        cause: Cause<S, CustomCause>,
        span: Range<S::SourceLoc>,
    },

    WithContext {
        context: Box<str>,
        span: Range<S::SourceLoc>,
        inner: Box<DefaultError<S, CustomCause>>,
    },
}

impl<S, CustomCause> Error<S> for DefaultError<S, CustomCause>
where
    S: Stream,
    S::SourceLoc: Clone,
{
    type Context = Box<str>;
    type CustomCause = CustomCause;

    #[inline]
    fn new(kind: Cause<S, Self::CustomCause>, span: Range<S::SourceLoc>) -> Self {
        Self::Error { cause: kind, span }
    }

    #[inline]
    fn with_cause(self, cause: Cause<S, Self::CustomCause>) -> Self {
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
