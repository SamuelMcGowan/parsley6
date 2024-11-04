use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::{Error, Report};
use crate::parser::Parser;
use crate::stream::{BorrowState, Stream};
// use crate::token::{eat_if, seek, ShouldConsume};

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, R)]
pub struct OrRecover<P, R, S, E> {
    pub(crate) parser: P,
    pub(crate) recover: R,
    pub(crate) _phantom: PhantomData<*const (S, E)>,
}

impl<P, R, S, E> Parser<S, E> for OrRecover<P, R, S, E>
where
    P: Parser<S, E>,
    R: Parser<S, E, Output = P::Output>,
    S: Stream + BorrowState<State: Report<E>>,
    E: Error<S>,
{
    type Output = P::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
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

// #[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, S::Token, D)]
// pub struct Terminated<P, D, S: Stream, E> {
//     pub(crate) parser: P,
//     pub(crate) token: S::Token,
//     pub(crate) default: D,
//     pub(crate) _phantom: PhantomData<*const (S, E)>,
// }

// impl<P, D, S, E> Parser<S, E> for Terminated<P, D, S, E>
// where
//     P: Parser<S, E>,
//     D: FnMut() -> P::Output,
//     S: Stream + BorrowState<State: Report<E>>,
//     E: Error<S>,
// {
//     type Output = P::Output;

//     fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
//         self.parser
//             .by_ref()
//             .then_drop(eat_if(|t| t == &self.token))
//             .or_recover(
//                 seek(|t| (t == &self.token).then_some(ShouldConsume::Yes))
//                     .map(|_| (self.default)()),
//             )
//             .parse(stream)
//     }
// }
