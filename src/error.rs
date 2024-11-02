use derive_where::derive_where;

use crate::stream::Stream;

pub trait Report<Error> {
    fn report(&mut self, error: Error);
}

pub trait Error<S: Stream> {
    type Cause: Cause<S>;

    fn new(cause: Self::Cause, span: S::Span) -> Self;
    fn set_cause(&mut self, cause: Self::Cause);
}

pub trait ErrorWithContext<S: Stream>: Error<S> {
    type Context;

    fn with_context(self, context: Self::Context, span: S::Span) -> Self;
}

pub trait Cause<S: Stream> {
    fn expected_token(token: S::Token) -> Self;
    fn expected_slice(slice: &'static S::Slice) -> Self;

    fn expected_in_set() -> Self;
    fn expected_end() -> Self;

    fn unknown() -> Self;
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

impl<S: Stream> Cause<S> for DefaultCause<S> {
    #[inline]
    fn expected_token(token: S::Token) -> Self {
        Self::ExpectedToken(token)
    }

    #[inline]
    fn expected_slice(slice: &'static S::Slice) -> Self {
        Self::ExpectedSlice(slice)
    }

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultError<S: Stream, C: Cause<S> = DefaultCause<S>, Context = Box<str>> {
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

impl<S, C, Context> DefaultError<S, C, Context>
where
    S: Stream,
    C: Cause<S>,
{
    #[inline]
    pub fn span(&self) -> &S::Span {
        match self {
            Self::Error { span, cause: _ } => span,
            Self::WithContext { span, .. } => span,
        }
    }
}

impl<S, C, Context> Error<S> for DefaultError<S, C, Context>
where
    S: Stream,
    C: Cause<S>,
{
    type Cause = C;

    #[inline]
    fn new(cause: Self::Cause, span: S::Span) -> Self {
        Self::Error { cause, span }
    }

    #[inline]
    fn set_cause(&mut self, cause: Self::Cause) {
        match self {
            Self::Error {
                cause: prev_cause, ..
            } => *prev_cause = cause,

            Self::WithContext { inner, .. } => inner.set_cause(cause),
        }
    }
}

impl<S, C, Context> ErrorWithContext<S> for DefaultError<S, C, Context>
where
    S: Stream,
    C: Cause<S>,
{
    type Context = Context;

    #[inline]
    fn with_context(self, context: Self::Context, span: S::Span) -> Self {
        Self::WithContext {
            context,
            span,
            inner: Box::new(self),
        }
    }
}
