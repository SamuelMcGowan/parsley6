use std::marker::PhantomData;

use crate::{error::Error, parser::Parser, stream::Stream};

pub struct Named<P, Name, S, O, E> {
    pub(crate) parser: P,
    pub(crate) name: Name,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, Name, S, O, E> Parser<S, O, E> for Named<P, Name, S, O, E>
where
    P: Parser<S, O, E>,
    Name: Clone + Into<E::Name>,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        // FIXME: get full span!
        let start_span = stream.source_span();

        self.parser
            .parse(stream)
            .map_err(|_| E::expected_named(self.name.clone().into(), start_span))
    }
}
