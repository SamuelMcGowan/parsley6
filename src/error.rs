use derive_where::derive_where;

use crate::stream::Stream;

pub trait Error<S: Stream>: Sized {
    type Context;
    type CustomCause;

    fn new(cause: Cause<S, Self::CustomCause>, span: S::Span) -> Self;

    fn with_cause(self, cause: Cause<S, Self::CustomCause>) -> Self;
    fn with_context(self, context: Self::Context, span: S::Span) -> Self;

    fn span(&self) -> S::Span;
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token, Custom)]
pub enum Cause<S: Stream, Custom> {
    Custom(Custom),
    Unknown,

    ExpectedToken(S::Token),
    ExpectedSlice,
    ExpectedInSet,
    ExpectedEnd,
}

impl<S: Stream, Custom> Cause<S, Custom> {
    #[inline]
    pub fn custom(custom: impl Into<Custom>) -> Self {
        Self::Custom(custom.into())
    }
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token, S::Span, CustomCause)]
pub enum DefaultError<S: Stream, CustomCause = Box<str>> {
    Error {
        cause: Cause<S, CustomCause>,
        span: S::Span,
    },

    WithContext {
        context: Box<str>,
        span: S::Span,
        inner: Box<DefaultError<S, CustomCause>>,
    },
}

impl<S, CustomCause> Error<S> for DefaultError<S, CustomCause>
where
    S: Stream,
{
    type Context = Box<str>;
    type CustomCause = CustomCause;

    #[inline]
    fn new(kind: Cause<S, Self::CustomCause>, span: S::Span) -> Self {
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
    fn with_context(self, context: Self::Context, span: S::Span) -> Self {
        Self::WithContext {
            context,
            span,
            inner: Box::new(self),
        }
    }

    #[inline]
    fn span(&self) -> S::Span {
        match self {
            Self::Error { span, .. } | Self::WithContext { span, .. } => span.clone(),
        }
    }
}
