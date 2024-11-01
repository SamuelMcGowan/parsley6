use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::Error;
use crate::parser::Parser;
use crate::stream::Stream;

#[derive_where(Debug, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct ByRef<'a, P: ?Sized, S, O, E> {
    pub(crate) parser: &'a mut P,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<'a, P, S, O, E> Parser<S, O, E> for ByRef<'a, P, S, O, E>
where
    P: Parser<S, O, E>,
    S: Stream,
    E: Error<Stream = S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parser.parse(stream)
    }
}
