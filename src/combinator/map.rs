use std::{marker::PhantomData, ops::Range};

use derive_where::derive_where;

use crate::{
    error::Error,
    parser::Parser,
    stream::{merge_spans_right, BorrowState, Stream},
};

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct Map<P, OA, OB, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, OA, OB, E)>,
}

impl<P, OA, OB, F, S, E> Parser<S, OB, E> for Map<P, OA, OB, F, S, E>
where
    P: Parser<S, OA, E>,
    F: FnMut(OA) -> OB,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<OB, E> {
        self.parser.parse(stream).map(&mut self.f)
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct MapWithState<P, OA, OB, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, OA, OB, E)>,
}

impl<P, OA, OB, F, S, E> Parser<S, OB, E> for MapWithState<P, OA, OB, F, S, E>
where
    P: Parser<S, OA, E>,
    F: FnMut(OA, &mut S::State) -> OB,
    S: Stream + BorrowState,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<OB, E> {
        self.parser
            .parse(stream)
            .map(|output| (self.f)(output, stream.borrow_state_mut()))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct MapWithSpan<P, OA, OB, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, OA, OB, E)>,
}

impl<P, OA, OB, F, S, E> Parser<S, OB, E> for MapWithSpan<P, OA, OB, F, S, E>
where
    P: Parser<S, OA, E>,
    F: FnMut(OA, Range<S::SourceLoc>) -> OB,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<OB, E> {
        let start_span = stream.peek_token_span();

        let output = self.parser.parse(stream)?;

        let end_span = stream.prev_token_span();
        let span = merge_spans_right(start_span, end_span);

        Ok((self.f)(output, span))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct MapWithSlice<P, OA, OB, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, OA, OB, E)>,
}

impl<P, OA, OB, F, S, E> Parser<S, OB, E> for MapWithSlice<P, OA, OB, F, S, E>
where
    P: Parser<S, OA, E>,
    F: FnMut(OA, S::SliceRef) -> OB,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<OB, E> {
        let start = stream.stream_position();
        let output = self.parser.parse(stream)?;
        let end = stream.stream_position();

        let slice = stream.slice(start, end);

        Ok((self.f)(output, slice))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, OB)]
pub struct MapTo<P, OA, OB, S, E> {
    pub(crate) parser: P,
    pub(crate) value: OB,
    pub(crate) _phantom: PhantomData<*const (S, OA, E)>,
}

impl<P, OA, OB, S, E> Parser<S, OB, E> for MapTo<P, OA, OB, S, E>
where
    P: Parser<S, OA, E>,
    OB: Clone,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<OB, E> {
        self.parser.parse(stream).map(|_| self.value.clone())
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct MapToSlice<P, S, O, E> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, S, O, E> Parser<S, S::SliceRef, E> for MapToSlice<P, S, O, E>
where
    P: Parser<S, O, E>,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<S::SliceRef, E> {
        let start = stream.stream_position();
        let _ = self.parser.parse(stream)?;
        let end = stream.stream_position();

        Ok(stream.slice(start, end))
    }
}
