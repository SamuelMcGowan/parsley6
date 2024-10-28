use std::marker::PhantomData;

use crate::{
    error::Error,
    parser::Parser,
    stream::{merge_spans_right, Stream},
};

pub struct WithErrCause<P, MakeCause, Cause, S, O, E> {
    pub(crate) parser: P,
    pub(crate) make_cause: MakeCause,
    pub(crate) _phantom: PhantomData<*const (Cause, S, O, E)>,
}

impl<P, MakeCause, Cause, S, O, E> Parser<S, O, E> for WithErrCause<P, MakeCause, Cause, S, O, E>
where
    P: Parser<S, O, E>,
    MakeCause: FnMut() -> Cause,
    S: Stream,
    E: Error<S>,
    E::Cause: From<Cause>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parser
            .parse(stream)
            .map_err(|err| err.with_cause((self.make_cause)().into()))
    }
}

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
    E: Error<S>,
    E::Context: From<Context>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        let start_span = stream.peek_token_span();

        self.parser.parse(stream).map_err(|err| {
            let end_span = stream.prev_token_span();
            let span = merge_spans_right(start_span, end_span);

            err.with_context((self.make_context)().into(), span)
        })
    }
}
