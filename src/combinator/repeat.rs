use std::marker::PhantomData;

use crate::{error::Error, parser::Parser, stream::Stream};

pub struct Repeat<P, Collection, S, O, E> {
    pub(crate) parser: P,
    pub(crate) min: usize,
    pub(crate) max: Option<usize>,
    pub(crate) _phantom: PhantomData<*const (Collection, S, O, E)>,
}

impl<P, Collection, S, O, E> Repeat<P, Collection, S, O, E>
where
    P: Parser<S, O, E>,
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
        self.max = Some(max);
        self
    }

    #[inline]
    pub fn collect<C: FromIterator<O>>(self) -> Repeat<P, C, S, O, E> {
        Repeat {
            parser: self.parser,
            min: self.min,
            max: self.max,
            _phantom: PhantomData,
        }
    }
}

impl<P, Collection, S, O, E> Parser<S, Collection, E> for Repeat<P, Collection, S, O, E>
where
    P: Parser<S, O, E>,
    Collection: FromIterator<O>,
    S: Stream,
    E: Error<S>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<Collection, E> {
        let mut n = 0;

        let max = self.max.map(|m| m.max(self.min));

        std::iter::from_fn(|| {
            if max.is_some_and(|m| n >= m) {
                None
            } else {
                n += 1;

                match self.parser.parse(stream) {
                    Ok(output) => Some(Ok(output)),

                    // Not enough repetitions.
                    Err(err) if n <= self.min => Some(Err(err)),

                    Err(_) => None,
                }
            }
        })
        .collect()
    }
}

pub struct NoCollection;

impl<T> FromIterator<T> for NoCollection {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().for_each(|_| {});
        NoCollection
    }
}
