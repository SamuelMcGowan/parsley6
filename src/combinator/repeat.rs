use std::{marker::PhantomData, num::NonZeroUsize};

use crate::{
    error::{BuiltinCause, Error},
    parser::Parser,
    stream::Stream,
};

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
//     #[inline]
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

pub struct NoCollection;

impl<T> FromIterator<T> for NoCollection {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        for _ in iter {}
        NoCollection
    }
}

pub struct RepeatUntil<P, F, Collection, S, O, E> {
    pub(crate) parser: P,
    pub(crate) f: F,

    pub(crate) min: usize,
    pub(crate) max: Option<NonZeroUsize>,

    pub(crate) _phantom: PhantomData<*const (Collection, S, O, E)>,
}

impl<P, F, Collection, S, O, E> RepeatUntil<P, F, Collection, S, O, E>
where
    P: Parser<S, O, E>,
    F: FnMut(&S::Token) -> bool,
    Collection: FromIterator<O>,
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
    pub fn collect<C: FromIterator<O>>(self) -> RepeatUntil<P, F, C, S, O, E> {
        RepeatUntil {
            parser: self.parser,
            f: self.f,
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
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<Collection, E> {
        debug_assert!(self.max.is_none_or(|m| m.get() >= self.min));

        let mut n = 0;

        std::iter::from_fn(|| {
            n += 1;

            if self.max.is_some_and(|max| n > max.get()) {
                return None;
            }

            match stream.peek_token() {
                Some(token) if !(self.f)(&token) => Some(self.parser.parse(stream)),
                _ if n < self.min => Some(Err(E::new(
                    BuiltinCause::Unknown.into(),
                    stream.peek_token_span(),
                ))),
                _ => None,
            }
        })
        .collect()
    }
}
