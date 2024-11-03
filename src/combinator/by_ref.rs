use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::Error;
use crate::parser::Parser;
use crate::stream::Stream;

#[derive_where(Debug, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
pub struct ByRef<'a, P, S, E> {
    pub(crate) parser: &'a mut P,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<'a, P, S, E> Parser<S, E> for ByRef<'a, P, S, E>
where
    P: Parser<S, E>,
    S: Stream,
    E: Error<S>,
{
    type Output = P::Output;

    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parser.parse(stream)
    }
}
