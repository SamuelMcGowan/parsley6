use std::{marker::PhantomData, num::NonZeroUsize};

use derive_where::derive_where;

use crate::{
    error::{Cause, Error},
    parser::Parser,
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

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; P, F)]
pub struct RepeatWhile<P, F, Collection, S, E> {
    pub(crate) parser: P,
    pub(crate) f: F,

    pub(crate) min: usize,
    pub(crate) max: Option<NonZeroUsize>,

    pub(crate) _phantom: PhantomData<*const (Collection, S, E)>,
}

impl<P, F, Collection, S, E> RepeatWhile<P, F, Collection, S, E>
where
    P: Parser<S, E>,
    F: Fn(&S::Token) -> bool,
    Collection: FromIterator<P::Output>,
    S: Stream,
    E: Error<S>,
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
    pub fn collect<C: FromIterator<P::Output>>(self) -> RepeatWhile<P, F, C, S, E> {
        RepeatWhile {
            parser: self.parser,
            f: self.f,
            min: self.min,
            max: self.max,
            _phantom: PhantomData,
        }
    }
}

impl<P, F, Collection, S, E> Parser<S, E> for RepeatWhile<P, F, Collection, S, E>
where
    P: Parser<S, E>,
    F: FnMut(&S::Token) -> bool,
    Collection: FromIterator<P::Output>,
    S: Stream,
    E: Error<S>,
{
    type Output = Collection;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        debug_assert!(self.max.is_none_or(|m| m.get() >= self.min));

        let mut n = 0;

        std::iter::from_fn(|| {
            n += 1;

            if self.max.is_some_and(|max| n > max.get()) {
                return None;
            }

            match stream.peek_token() {
                Some(token) if (self.f)(&token) => {
                    let start = stream.stream_position();
                    let result = self.parser.parse(stream);

                    if result.is_ok() && stream.stream_position() == start {
                        panic!("parser did not make progress");
                    }

                    Some(result)
                }
                _ if n < self.min => {
                    Some(Err(E::new(E::Cause::unknown(), stream.peek_token_span())))
                }
                _ => None,
            }
        })
        .collect()
    }
}
