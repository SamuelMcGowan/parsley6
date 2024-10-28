use std::marker::PhantomData;

use crate::error::Error;
use crate::parser::Parser;
use crate::stream::Stream;

pub struct ByRef<'a, P: ?Sized, S, O, E> {
    pub(crate) parser: &'a mut P,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<'a, P, S, O, E> Parser<S, O, E> for ByRef<'a, P, S, O, E>
where
    P: Parser<S, O, E>,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parser.parse(stream)
    }
}
