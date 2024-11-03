use std::marker::PhantomData;

use derive_where::derive_where;

use crate::{
    error::{Error, ErrorWithContext},
    parser::Parser,
    stream::{Span, Stream},
};

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, MakeCause)]
pub struct WithErrCause<P, MakeCause, S, E> {
    pub(crate) parser: P,
    pub(crate) make_cause: MakeCause,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, MakeCause, S, E> Parser<S, E> for WithErrCause<P, MakeCause, S, E>
where
    P: Parser<S, E>,
    MakeCause: FnMut() -> E::Cause,
    S: Stream,
    E: Error<S>,
{
    type Output = P::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser.parse(stream).map_err(|mut err| {
            err.set_cause((self.make_cause)());
            err
        })
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, MakeContext)]
pub struct WithErrContext<P, MakeContext, Context, S, E> {
    pub(crate) parser: P,
    pub(crate) make_context: MakeContext,
    pub(crate) _phantom: PhantomData<*const (Context, S, E)>,
}

impl<P, MakeContext, Context, S, E> Parser<S, E> for WithErrContext<P, MakeContext, Context, S, E>
where
    P: Parser<S, E>,
    MakeContext: FnMut() -> Context,
    S: Stream,
    E: ErrorWithContext<S, Context: From<Context>>,
{
    type Output = P::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        let start_span = stream.peek_token_span();

        self.parser.parse(stream).map_err(|err| {
            let end_span = stream.prev_token_span();
            let span = start_span.merge_right(end_span);

            err.with_context((self.make_context)().into(), span)
        })
    }
}
