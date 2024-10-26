use std::ops::Range;

use derive_where::derive_where;

use crate::stream::Stream;

pub trait Error<S: Stream> {
    type Name;

    fn expected_token(token: S::Token, span: Range<S::SourceLoc>) -> Self;
    fn expected_match(span: Range<S::SourceLoc>) -> Self;
    fn expected_any(span: Range<S::SourceLoc>) -> Self;
    fn expected_end(span: Range<S::SourceLoc>) -> Self;
    fn expected_named(name: Self::Name, span: Range<S::SourceLoc>) -> Self;

    fn unknown(span: Range<S::SourceLoc>) -> Self;
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; S::Token, S::SourceLoc)]
pub enum DefaultError<S: Stream> {
    ExpectedToken {
        token: S::Token,
        span: Range<S::SourceLoc>,
    },
    ExpectedMatch {
        span: Range<S::SourceLoc>,
    },
    ExpectedAny {
        span: Range<S::SourceLoc>,
    },
    ExpectedEnd {
        span: Range<S::SourceLoc>,
    },
    ExpectedNamed {
        name: String,
        span: Range<S::SourceLoc>,
    },
    Unknown {
        span: Range<S::SourceLoc>,
    },
}

impl<S: Stream> Error<S> for DefaultError<S> {
    type Name = String;

    #[inline]
    fn expected_token(token: S::Token, span: Range<S::SourceLoc>) -> Self {
        Self::ExpectedToken { token, span }
    }

    #[inline]
    fn expected_match(span: Range<S::SourceLoc>) -> Self {
        Self::ExpectedMatch { span }
    }

    #[inline]
    fn expected_any(span: Range<S::SourceLoc>) -> Self {
        Self::ExpectedAny { span }
    }

    #[inline]
    fn expected_end(span: Range<S::SourceLoc>) -> Self {
        Self::ExpectedEnd { span }
    }

    #[inline]
    fn expected_named(name: Self::Name, span: Range<S::SourceLoc>) -> Self {
        Self::ExpectedNamed { name, span }
    }

    #[inline]
    fn unknown(span: Range<S::SourceLoc>) -> Self {
        Self::Unknown { span }
    }
}
