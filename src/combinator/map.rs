use std::marker::PhantomData;

use derive_where::derive_where;

use crate::{
    error::Error,
    parser::Parser,
    stream::{BorrowState, Span, Stream},
};

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct Map<P, O, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, O, F, S, E> Parser<S, E> for Map<P, O, F, S, E>
where
    P: Parser<S, E>,
    F: FnMut(P::Output) -> O,
    S: Stream,
    E: Error<S>,
{
    type Output = O;

    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser.parse(stream).map(&mut self.f)
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct MapWithState<P, O, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, O, F, S, E> Parser<S, E> for MapWithState<P, O, F, S, E>
where
    P: Parser<S, E>,
    F: FnMut(P::Output, &mut S::State) -> O,
    S: Stream + BorrowState,
    E: Error<S>,
{
    type Output = O;

    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser
            .parse(stream)
            .map(|output| (self.f)(output, stream.borrow_state()))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, O)]
pub struct MapTo<P, O, S, E> {
    pub(crate) parser: P,
    pub(crate) value: O,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, O, S, E> Parser<S, E> for MapTo<P, O, S, E>
where
    P: Parser<S, E>,
    O: Clone,
    S: Stream,
    E: Error<S>,
{
    type Output = O;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser.parse(stream).map(|_| self.value.clone())
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct MapErr<P, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, F, S, E> Parser<S, E> for MapErr<P, F, S, E>
where
    P: Parser<S, E>,
    F: FnMut(E) -> E,
    S: Stream,
    E: Error<S>,
{
    type Output = P::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser.parse(stream).map_err(&mut self.f)
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct MapErrWithState<P, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, F, S, E> Parser<S, E> for MapErrWithState<P, F, S, E>
where
    P: Parser<S, E>,
    S: BorrowState,
    F: FnMut(E, &mut S::State) -> E,
    S: Stream,
    E: Error<S>,
{
    type Output = P::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser
            .parse(stream)
            .map_err(|err| (self.f)(err, stream.borrow_state()))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, E)]
pub struct MapErrTo<P, S, E> {
    pub(crate) parser: P,
    pub(crate) error: E,
    pub(crate) _phantom: PhantomData<*const S>,
}

impl<P, S, E> Parser<S, E> for MapErrTo<P, S, E>
where
    P: Parser<S, E>,
    S: Stream,
    E: Error<S> + Clone,
{
    type Output = P::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser.parse(stream).map_err(|_| self.error.clone())
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct AndThen<P, F, O, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, F, O, S, E> Parser<S, E> for AndThen<P, F, O, S, E>
where
    P: Parser<S, E>,
    F: FnMut(P::Output) -> Result<O, E>,
    S: Stream,
    E: Error<S>,
{
    type Output = O;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser.parse(stream).and_then(&mut self.f)
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct OrElse<P, F, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, F, S, E> Parser<S, E> for OrElse<P, F, S, E>
where
    P: Parser<S, E>,
    F: FnMut(E) -> Result<P::Output, E>,
    S: Stream,
    E: Error<S>,
{
    type Output = P::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser.parse(stream).or_else(&mut self.f)
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct ToSlice<P, S, E> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, S, E> Parser<S, E> for ToSlice<P, S, E>
where
    P: Parser<S, E>,
    S: Stream,
    E: Error<S>,
{
    type Output = S::SliceRef;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        let start = stream.stream_position();
        let _ = self.parser.parse(stream)?;
        let end = stream.stream_position();

        Ok(stream.slice(start, end))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct WithSlice<P, S, E> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, S, E> Parser<S, E> for WithSlice<P, S, E>
where
    P: Parser<S, E>,
    S: Stream,
    E: Error<S>,
{
    type Output = (P::Output, S::SliceRef);

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        let start = stream.stream_position();
        let output = self.parser.parse(stream)?;
        let end = stream.stream_position();
        Ok((output, stream.slice(start, end)))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct WithSpan<P, S, E> {
    pub(crate) parser: P,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, S, E> Parser<S, E> for WithSpan<P, S, E>
where
    P: Parser<S, E>,
    S: Stream,
    E: Error<S>,
{
    type Output = (P::Output, S::Span);

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        let start_span = stream.peek_token_span();
        let output = self.parser.parse(stream)?;
        let end_span = stream.prev_token_span();
        let span = start_span.merge_right(end_span);
        Ok((output, span))
    }
}
