use std::marker::PhantomData;

use derive_where::derive_where;

use crate::{
    error::Error,
    parser::Parser,
    stream::{BorrowState, Span, Stream},
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

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct MapErr<P, F, S, O, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, F, S, O, E> Parser<S, O, E> for MapErr<P, F, S, O, E>
where
    P: Parser<S, O, E>,
    F: FnMut(E) -> E,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parser.parse(stream).map_err(&mut self.f)
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct MapErrWithState<P, F, S, O, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, F, S, O, E> Parser<S, O, E> for MapErrWithState<P, F, S, O, E>
where
    P: Parser<S, O, E>,
    S: BorrowState,
    F: FnMut(E, &mut S::State) -> E,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parser
            .parse(stream)
            .map_err(|err| (self.f)(err, stream.borrow_state_mut()))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, E)]
pub struct MapErrTo<P, S, O, E> {
    pub(crate) parser: P,
    pub(crate) error: E,
    pub(crate) _phantom: PhantomData<*const (S, O)>,
}

impl<P, S, O, E> Parser<S, O, E> for MapErrTo<P, S, O, E>
where
    P: Parser<S, O, E>,
    S: Stream,
    E: Error<S> + Clone,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parser.parse(stream).map_err(|_| self.error.clone())
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct AndThen<P, F, OA, OB, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, OA, OB, E)>,
}

impl<P, F, OA, OB, S, E> Parser<S, OB, E> for AndThen<P, F, OA, OB, S, E>
where
    P: Parser<S, OA, E>,
    F: FnMut(OA) -> Result<OB, E>,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<OB, E> {
        self.parser.parse(stream).and_then(&mut self.f)
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct OrElse<P, F, S, O, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, F, S, O, E> Parser<S, O, E> for OrElse<P, F, S, O, E>
where
    P: Parser<S, O, E>,
    F: FnMut(E) -> Result<O, E>,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parser.parse(stream).or_else(&mut self.f)
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct ToSlice<P, S, O, E> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, S, O, E> Parser<S, S::SliceRef, E> for ToSlice<P, S, O, E>
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

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct WithSlice<P, S, O, E> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, S, O, E> Parser<S, (O, S::SliceRef), E> for WithSlice<P, S, O, E>
where
    P: Parser<S, O, E>,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<(O, S::SliceRef), E> {
        let start = stream.stream_position();
        let output = self.parser.parse(stream)?;
        let end = stream.stream_position();
        Ok((output, stream.slice(start, end)))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct WithSpan<P, S, O, E> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, S, O, E> Parser<S, (O, S::Span), E> for WithSpan<P, S, O, E>
where
    P: Parser<S, O, E>,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<(O, S::Span), E> {
        let start_span = stream.peek_token_span();
        let output = self.parser.parse(stream)?;
        let end_span = stream.prev_token_span();
        let span = start_span.merge_right(end_span);
        Ok((output, span))
    }
}
