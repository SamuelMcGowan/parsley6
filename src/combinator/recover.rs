use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::{Error, Report};
use crate::parser::Parser;
use crate::stream::{BorrowState, Stream};

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, R)]
pub struct OrRecover<P, R, S, O, E> {
    pub(crate) parser: P,
    pub(crate) recover: R,
    pub(crate) _phantom: PhantomData<*const (S, O, E)>,
}

impl<P, R, S, O, E> Parser<S, O, E> for OrRecover<P, R, S, O, E>
where
    P: Parser<S, O, E>,
    R: Parser<S, O, E>,
    S: Stream + BorrowState<State: Report<E>>,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        match self.parser.parse(stream) {
            Ok(value) => Ok(value),
            Err(err) => {
                stream.borrow_state().report(err);

                // TODO: can we find a way to report that
                // this error occurred while recovering?
                self.recover.by_ref().parse(stream)
            }
        }
    }
}
