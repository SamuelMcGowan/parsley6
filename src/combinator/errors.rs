use std::marker::PhantomData;

use derive_where::derive_where;

use crate::{
    error::Error,
    parser::Parser,
    stream::{Span, Stream},
};

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, MakeCause)]
pub struct WithErrCause<P, MakeCause, S, O, E> {
    pub(crate) parser: P,
    pub(crate) make_cause: MakeCause,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, MakeCause, S, O, E> Parser<S, O, E> for WithErrCause<P, MakeCause, S, O, E>
where
    P: Parser<S, O, E>,
    MakeCause: FnMut() -> E::Cause,
    S: Stream,
    E: Error<Stream = S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parser
            .parse(stream)
            .map_err(|err| err.with_cause((self.make_cause)()))
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, MakeContext)]
pub struct WithErrContext<P, MakeContext, Context, S, O, E> {
    pub(crate) parser: P,
    pub(crate) make_context: MakeContext,
    pub(crate) _phantom: PhantomData<*const (Context, S, O, E)>,
}

impl<P, MakeContext, Context, S, O, E> Parser<S, O, E>
    for WithErrContext<P, MakeContext, Context, S, O, E>
where
    P: Parser<S, O, E>,
    MakeContext: FnMut() -> Context,
    S: Stream,
    E: Error<Stream = S>,
    E::Context: From<Context>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        let start_span = stream.peek_token_span();

        self.parser.parse(stream).map_err(|err| {
            let end_span = stream.prev_token_span();
            let span = start_span.merge_right(end_span);

            err.with_context((self.make_context)().into(), span)
        })
    }
}
