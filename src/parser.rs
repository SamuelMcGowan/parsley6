use std::marker::PhantomData;

use crate::{
    combinator::*,
    error::{Error, ErrorWithContext, Report},
    prelude::{prefixed, suffixed},
    stream::{BorrowState, Stream},
};

#[diagnostic::on_unimplemented(
    message = "Not a parser (for these stream and error types).",
    label = "Doesn't implement `Parser<{S}, {E}>`."
)]
#[must_use = "Parsers do nothing unless `parse` is called."]
pub trait Parser<S, E>
where
    S: Stream,
    E: Error<S>,
{
    type Output;

    /// Run the parser on a stream.
    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E>;

    /// Map the output of this parser to another value.
    #[inline]
    fn map<F, O>(self, f: F) -> Map<Self, O, F, S, E>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> O,
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
    fn map_with_state<F, O>(self, f: F) -> MapWithState<Self, O, F, S, E>
    where
        Self: Sized,
        S: BorrowState,
        F: FnMut(Self::Output, &mut S::State) -> O,
    {
        MapWithState {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to a value without requiring a callback.
    #[inline]
    fn map_to<O>(self, value: O) -> MapTo<Self, O, S, E>
    where
        Self: Sized,
        O: Clone,
    {
        MapTo {
            parser: self,
            value,
            _phantom: PhantomData,
        }
    }

    /// Map the error of this parser to another error.
    #[inline]
    fn map_err<F>(self, f: F) -> MapErr<Self, F, S, E>
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
    fn map_err_with_state<F>(self, f: F) -> MapErrWithState<Self, F, S, E>
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

    /// Map the error of this parser to another error without requiring a callback.
    #[inline]
    fn map_err_to(self, error: E) -> MapErrTo<Self, S, E>
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

    /// Map the output of this parser to a result value.
    #[inline]
    fn and_then<F, O>(self, f: F) -> AndThen<Self, F, O, S, E>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Result<O, E>,
    {
        AndThen {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to a result value, with access to the stream's state.
    #[inline]
    fn and_then_with_state<F, O>(self, f: F) -> AndThenWithState<Self, F, O, S, E>
    where
        Self: Sized,
        F: FnMut(Self::Output, &mut S::State) -> Result<O, E>,
        S: BorrowState,
    {
        AndThenWithState {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the error of this parser to a result value.
    #[inline]
    fn or_else<F>(self, f: F) -> OrElse<Self, F, S, E>
    where
        Self: Sized,
        F: FnMut(E) -> Result<Self::Output, E>,
    {
        OrElse {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the error of this parser to a result value, with access to the stream's state.
    #[inline]
    fn or_else_with_state<F>(self, f: F) -> OrElseWithState<Self, F, S, E>
    where
        Self: Sized,
        F: FnMut(E, &mut S::State) -> Result<Self::Output, E>,
        S: BorrowState,
    {
        OrElseWithState {
            parser: self,
            f,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to the parsed value's source slice.
    #[inline]
    fn to_slice(self) -> ToSlice<Self, S, E>
    where
        Self: Sized,
    {
        ToSlice {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to `(output, slice)`.
    #[inline]
    fn with_slice(self) -> WithSlice<Self, S, E>
    where
        Self: Sized,
    {
        WithSlice {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Map the output of this parser to `(output, span)`.
    #[inline]
    fn with_span(self) -> WithSpan<Self, S, E>
    where
        Self: Sized,
    {
        WithSpan {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Call [`Into::into`] on the output of this parser.
    #[inline]
    fn map_into<O>(self) -> MapInto<Self, O, S, E>
    where
        Self: Sized,
        O: From<Self::Output>,
    {
        MapInto {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Discard the output of this parser (and output `()` instead).
    #[inline]
    fn drop(self) -> MapTo<Self, (), S, E>
    where
        Self: Sized,
    {
        self.map_to(())
    }

    /// Run this parser followed by another, discarding the first parser's output.
    #[inline]
    fn drop_then<P>(self, parser: P) -> Prefixed<Self, P, S, E>
    where
        Self: Sized,
        P: Parser<S, E>,
    {
        prefixed(self, parser)
    }

    /// Run this parser followed by another, discarding the second parser's output.
    #[inline]
    fn then_drop<P>(self, parser: P) -> Suffixed<Self, P, S, E>
    where
        Self: Sized,
        P: Parser<S, E>,
    {
        suffixed(self, parser)
    }

    // #[inline]
    // fn repeat(self) -> Repeat<Self, NoCollection, S, O, E>
    // where
    //     Self: Sized,
    // {
    //     Repeat {
    //         parser: self,
    //         min: 0,
    //         max: None,
    //         _phantom: PhantomData,
    //     }
    // }

    /// Repeat this parser while the next token matches the predicate.
    ///
    /// Does not consume tokens matched by the predicate.
    ///
    /// # Panics
    ///
    /// Panics if an iteration succeeds without making progress.
    #[inline]
    fn repeat_while<F>(self, f: F) -> RepeatWhile<Self, F, NoCollection, S, E>
    where
        Self: Sized,
        F: Fn(&S::Token) -> bool,
    {
        RepeatWhile {
            parser: self,
            f,
            min: 0,
            max: None,
            _phantom: PhantomData,
        }
    }

    /// If this parser fails, report the error and then recover by running another parser.
    #[inline]
    fn or_recover<R>(self, recover: R) -> OrRecover<Self, R, S, E>
    where
        Self: Sized,
        R: Parser<S, E, Output = Self::Output>,
        S: BorrowState<State: Report<E>>,
    {
        OrRecover {
            parser: self,
            recover,
            _phantom: PhantomData,
        }
    }

    // /// Create a parser terminated by a token.
    // /// If the parser fails, reports the error and seeks until the token is found
    // /// or the stream ends.
    // #[inline]
    // fn terminated<D>(self, token: S::Token, default: D) -> Terminated<Self, D, S, E>
    // where
    //     Self: Sized,
    //     D: FnMut() -> Self::Output,
    //     S: BorrowState<State: Report<E>>,
    // {
    //     Terminated {
    //         parser: self,
    //         token,
    //         default,
    //         _phantom: PhantomData,
    //     }
    // }

    /// Set the cause for errors produced by this parser.
    #[inline]
    fn with_err_cause<F>(self, make_cause: F) -> WithErrCause<Self, F, S, E>
    where
        Self: Sized,
        F: FnMut() -> E::Cause,
    {
        WithErrCause {
            parser: self,
            make_cause,
            _phantom: PhantomData,
        }
    }

    /// Add context to errors produced by this parser.
    #[inline]
    fn with_err_context<F, Context>(self, make_context: F) -> WithErrContext<Self, F, Context, S, E>
    where
        Self: Sized,
        F: FnMut() -> Context,
        E: ErrorWithContext<S, Context: From<Context>>,
    {
        WithErrContext {
            parser: self,
            make_context,
            _phantom: PhantomData,
        }
    }

    /// Take this parser by reference.
    #[inline]
    fn by_ref(&mut self) -> ByRef<Self, S, E>
    where
        Self: Sized,
    {
        ByRef {
            parser: self,
            _phantom: PhantomData,
        }
    }

    /// Hide the type of the parser.
    ///
    /// This is useful for debugging as it can simplify type errors,
    /// but it will hide the traits that the parser implements such as [`Clone`], which
    /// can cause additional type errors.
    #[inline]
    fn opaque(self) -> impl Parser<S, E, Output = Self::Output>
    where
        Self: Sized,
    {
        self
    }
}

impl<S, O, E, F> Parser<S, E> for F
where
    S: Stream,
    E: Error<S>,
    F: FnMut(&mut S) -> Result<O, E>,
{
    type Output = O;

    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self(stream)
    }
}
