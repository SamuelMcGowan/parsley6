use derive_where::derive_where;

use crate::stream::Stream;

pub trait Error<S: Stream> {
    type Cause: Cause;
    type Context;

    fn new(cause: Self::Cause, span: S::Span) -> Self;

    fn with_cause(self, cause: Self::Cause) -> Self;
    fn with_context(self, context: Self::Context, span: S::Span) -> Self;

    fn span(&self) -> S::Span;
}

pub trait Cause {
    fn expected_in_set() -> Self;
    fn expected_end() -> Self;

    fn unknown() -> Self;
}

pub trait CauseFromToken<Token>: Cause {
    fn expected_token(token: Token) -> Self;
}

pub trait CauseFromSlice<Slice: ?Sized>: Cause {
    fn expected_slice(slice: &'static Slice) -> Self;
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token, &'static S::Slice)]
pub enum DefaultCause<S: Stream> {
    Custom(Box<str>),

    ExpectedToken(S::Token),
    ExpectedSlice(&'static S::Slice),

    ExpectedInSet,
    ExpectedEnd,

    Unknown,
}

impl<S: Stream> DefaultCause<S> {
    #[inline]
    pub fn custom(custom: impl Into<Box<str>>) -> Self {
        Self::Custom(custom.into())
    }
}

impl<S: Stream> Cause for DefaultCause<S> {
    #[inline]
    fn expected_in_set() -> Self {
        Self::ExpectedInSet
    }

    #[inline]
    fn expected_end() -> Self {
        Self::ExpectedEnd
    }

    #[inline]
    fn unknown() -> Self {
        Self::Unknown
    }
}

impl<S: Stream> CauseFromToken<S::Token> for DefaultCause<S> {
    #[inline]
    fn expected_token(token: S::Token) -> Self {
        Self::ExpectedToken(token)
    }
}

impl<S: Stream> CauseFromSlice<S::Slice> for DefaultCause<S> {
    #[inline]
    fn expected_slice(slice: &'static S::Slice) -> Self {
        Self::ExpectedSlice(slice)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultError<S: Stream, C: Cause = DefaultCause<S>, Context = Box<str>> {
    Error {
        cause: C,
        span: S::Span,
    },

    WithContext {
        context: Context,
        span: S::Span,
        inner: Box<Self>,
    },
}

impl<S, C, Context> Error<S> for DefaultError<S, C, Context>
where
    S: Stream,
    C: Cause,
{
    type Cause = C;
    type Context = Context;

    #[inline]
    fn new(cause: Self::Cause, span: S::Span) -> Self {
        Self::Error { cause, span }
    }

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
                inner: Box::new(Error::with_cause(*inner, cause)),
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
            Self::Error { span, cause: _ } => span.clone(),
            Self::WithContext { span, .. } => span.clone(),
        }
    }
}
