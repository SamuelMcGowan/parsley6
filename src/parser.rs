use std::{marker::PhantomData, ops::Range};

use crate::{
    combinator::{
        chain::{Prefixed, Suffixed},
        errors::{WithErrCause, WithErrContext},
        map::{
            Map, MapErr, MapErrTo, MapErrWithSlice, MapErrWithSpan, MapErrWithState, MapTo,
            MapToSlice, MapWithSlice, MapWithSpan, MapWithState,
        },
    },
    error::Error,
    prelude::{prefixed, suffixed},
    stream::{BorrowState, Stream},
};

pub trait Parser<S, O, E>
where
    S: Stream,
    E: Error<S>,
{
    /// Run the parser on a stream.
    fn parse(&mut self, stream: &mut S) -> Result<O, E>;

    /// Map the output of this parser to another value.
    #[inline]
    fn map<F, OB>(self, f: F) -> Map<Self, O, OB, F, S, E>
    where
        Self: Sized,
        F: FnMut(O) -> OB,
    {
        Map {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to another value, with access to the stream's state.
    ///
    /// This is for use with the [`StreamWithState`](crate::stream::StreamWithState) stream
    /// type, which as such is the only stream type that implements [`BorrowState`].
    #[inline]
    fn map_with_state<F, OB>(self, f: F) -> MapWithState<Self, O, OB, F, S, E>
    where
        Self: Sized,
        S: BorrowState,
        F: FnMut(O, &mut S::State) -> OB,
    {
        MapWithState {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to another value, with access to the parsed
    /// value's source span.
    #[inline]
    fn map_with_span<F, OB>(self, f: F) -> MapWithSpan<Self, O, OB, F, S, E>
    where
        Self: Sized,
        F: FnMut(O, Range<S::SourceLoc>) -> OB,
    {
        MapWithSpan {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to another value, with access to the parsed
    /// value's source slice.
    #[inline]
    fn map_with_slice<F, OB>(self, f: F) -> MapWithSlice<Self, O, OB, F, S, E>
    where
        Self: Sized,
        F: FnMut(O, S::SliceRef) -> OB,
    {
        MapWithSlice {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to a value without requiring a callback.
    #[inline]
    fn map_to<OB>(self, value: OB) -> MapTo<Self, O, OB, S, E>
    where
        Self: Sized,
        OB: Clone,
    {
        MapTo {
            parser: self,
            value,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to the parsed value's source slice.
    #[inline]
    fn map_to_slice(self) -> MapToSlice<Self, S, O, E>
    where
        Self: Sized,
    {
        MapToSlice {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Map the error of this parser to another error.
    #[inline]
    fn map_err<F>(self, f: F) -> MapErr<Self, F, S, O, E>
    where
        Self: Sized,
        F: FnMut(E) -> E,
    {
        MapErr {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the error of this parser to another error, with access to the stream's state.
    #[inline]
    fn map_err_with_state<F>(self, f: F) -> MapErrWithState<Self, F, S, O, E>
    where
        Self: Sized,
        S: BorrowState,
        F: FnMut(E, &mut S::State) -> E,
    {
        MapErrWithState {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the error of this parser to another error, with access to the parsed
    /// value's source span.
    #[inline]
    fn map_err_with_span<F>(self, f: F) -> MapErrWithSpan<Self, F, S, O, E>
    where
        Self: Sized,
        F: FnMut(E, Range<S::SourceLoc>) -> E,
    {
        MapErrWithSpan {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the error of this parser to another error, with access to the parsed
    /// value's source slice.
    #[inline]
    fn map_err_with_slice<F>(self, f: F) -> MapErrWithSlice<Self, F, S, O, E>
    where
        Self: Sized,
        F: FnMut(E, S::SliceRef) -> E,
    {
        MapErrWithSlice {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the error of this parser to another error without requiring a callback.
    #[inline]
    fn map_err_to(self, error: E) -> MapErrTo<Self, S, O, E>
    where
        Self: Sized,
        E: Clone,
    {
        MapErrTo {
            parser: self,
            error,
            _phantom: PhantomData,
        }
    }

    /// Creates a parser that runs this parser followed by another, discarding the first parser's output.
    #[inline]
    fn drop_then<P, POutput>(self, parser: P) -> Prefixed<Self, P, O, POutput, S, E>
    where
        Self: Sized,
        P: Parser<S, POutput, E>,
    {
        prefixed(self, parser)
    }

    /// Creates a parser that runs this parser followed by another, discarding the second parser's output.
    #[inline]
    fn then_drop<P, POutput>(self, parser: P) -> Suffixed<Self, P, O, POutput, S, E>
    where
        Self: Sized,
        P: Parser<S, POutput, E>,
    {
        suffixed(self, parser)
    }

    /// Creates a parser with a custom cause for error messages.
    #[inline]
    fn with_err_cause<F, Cause>(self, make_cause: F) -> WithErrCause<Self, F, Cause, S, O, E>
    where
        Self: Sized,
        F: FnMut() -> Cause,
        E::Cause: From<Cause>,
    {
        WithErrCause {
            parser: self,
            make_cause,
            _phantom: PhantomData,
        }
    }

    /// Creates a parser with additional context for error messages.
    fn with_err_context<F, Context>(
        self,
        make_context: F,
    ) -> WithErrContext<Self, F, Context, S, O, E>
    where
        Self: Sized,
        F: FnMut() -> Context,
        E::Context: From<Context>,
    {
        WithErrContext {
            parser: self,
            make_context,
            _phantom: PhantomData,
        }
    }

    /// Hide the type of the parser.
    ///
    /// This is useful for debugging as it can simplify type errors,
    /// but it will hide the traits that the parser implements such as [`Clone`], which
    /// can cause additional type errors.
    #[inline]
    fn opaque(self) -> impl Parser<S, O, E>
    where
        Self: Sized,
    {
        self
    }
}

impl<S, O, E, F> Parser<S, O, E> for F
where
    S: Stream,
    E: Error<S>,
    F: FnMut(&mut S) -> Result<O, E>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self(stream)
    }
}
