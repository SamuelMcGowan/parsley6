use std::{marker::PhantomData, num::NonZeroUsize};

use derive_where::derive_where;

use crate::{
    error::{Cause, Error},
    parser::Parser,
    prelude::TokenSet,
    stream::Stream,
};

// #[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P)]
// pub struct Repeat<P, Collection, S, O, E> {
//     pub(crate) parser: P,
//     pub(crate) min: usize,
//     pub(crate) max: Option<usize>,
//     pub(crate) _phantom: PhantomData<*const (Collection, S, O, E)>,
// }

// impl<P, Collection, S, O, E> Repeat<P, Collection, S, O, E>
// where
//     P: Parser<S, O, E>,
//     Collection: FromIterator<O>,
//     S: Stream,
//     E: Error<S>,
// {
//     #[inline]
//     pub fn min(mut self, min: usize) -> Self {
//         self.min = min;
//         self
//     }

//     #[inline]
//     pub fn max(mut self, max: usize) -> Self {
//         self.max = Some(max);
//         self
//     }

//     #[inline]
//     pub fn collect<C: FromIterator<O>>(self) -> Repeat<P, C, S, O, E> {
//         Repeat {
//             parser: self.parser,
//             min: self.min,
//             max: self.max,
//             _phantom: PhantomData,
//         }
//     }
// }

// impl<P, Collection, S, O, E> Parser<S, Collection, E> for Repeat<P, Collection, S, O, E>
// where
//     P: Parser<S, O, E>,
//     Collection: FromIterator<O>,
//     S: Stream,
//     E: Error<S>,
// {
//     fn parse(&mut self, stream: &mut S) -> Result<Collection, E> {
//         let mut n = 0;

//         let max = self.max.map(|m| m.max(self.min));

//         std::iter::from_fn(|| {
//             if max.is_some_and(|m| n >= m) {
//                 None
//             } else {
//                 n += 1;

//                 match self.parser.parse(stream) {
//                     Ok(output) => Some(Ok(output)),

//                     // Not enough repetitions.
//                     Err(err) if n <= self.min => Some(Err(err)),

//                     Err(_) => None,
//                 }
//             }
//         })
//         .collect()
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoCollection;

impl<T> FromIterator<T> for NoCollection {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        for _ in iter {}
        NoCollection
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, T)]
pub struct RepeatUntil<P, T, Collection, S, O, E> {
    pub(crate) parser: P,
    pub(crate) token_set: T,

    pub(crate) min: usize,
    pub(crate) max: Option<NonZeroUsize>,

    pub(crate) _phantom: PhantomData<*const (Collection, S, O, E)>,
}

impl<P, T, Collection, S, O, E> RepeatUntil<P, T, Collection, S, O, E>
where
    P: Parser<S, O, E>,
    T: TokenSet<S::Token>,
    Collection: FromIterator<O>,
    S: Stream,
    E: Error<Stream = S>,
{
    #[inline]
    pub fn min(mut self, min: usize) -> Self {
        self.min = min;
        self
    }

    #[inline]
    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(NonZeroUsize::new(max).expect("`max` must be non-zero"));
        self
    }

    #[inline]
    pub fn collect<C: FromIterator<O>>(self) -> RepeatUntil<P, T, C, S, O, E> {
        RepeatUntil {
            parser: self.parser,
            token_set: self.token_set,
            min: self.min,
            max: self.max,
            _phantom: PhantomData,
        }
    }
}

impl<P, F, Collection, S, O, E> Parser<S, Collection, E> for RepeatUntil<P, F, Collection, S, O, E>
where
    P: Parser<S, O, E>,
    F: FnMut(&S::Token) -> bool,
    Collection: FromIterator<O>,
    S: Stream,
    E: Error<Stream = S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<Collection, E> {
        debug_assert!(self.max.is_none_or(|m| m.get() >= self.min));

        let mut n = 0;

        std::iter::from_fn(|| {
            n += 1;

            if self.max.is_some_and(|max| n > max.get()) {
                return None;
            }

            match stream.peek_token() {
                Some(token) if !(self.token_set)(&token) => Some(self.parser.parse(stream)),
                _ if n < self.min => {
                    Some(Err(E::new(E::Cause::unknown(), stream.peek_token_span())))
                }
                _ => None,
            }
        })
        .collect()
    }
}
