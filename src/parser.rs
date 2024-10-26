use crate::{
    error::Error,
    prelude::{
        chain::{Prefixed, Suffixed},
        prefixed, suffixed,
    },
    stream::Stream,
};

pub trait Parser<S, O, E>
where
    S: Stream,
    E: Error<S>,
{
    /// Run the parser on a stream.
    fn parse(&mut self, stream: &mut S) -> Result<O, E>;

    /// Creates a parser that runs this parser followed by another, discarding the first parser's output.
    #[inline]
    fn drop_then<P, POutput>(self, parser: P) -> Prefixed<Self, P, O, POutput, S, E>
    where
        Self: Sized,
        P: Parser<S, POutput, E>,
    {
        prefixed(self, parser)
    }

    /// Creates a parser that runs this parser followed by another, discarding the second parser's output.
    #[inline]
    fn then_drop<P, POutput>(self, parser: P) -> Suffixed<Self, P, O, POutput, S, E>
    where
        Self: Sized,
        P: Parser<S, POutput, E>,
    {
        suffixed(self, parser)
    }

    /// Hide the type of the parser.
    ///
    /// This is useful for debugging as it can simplify type errors,
    /// but it will hide the traits that the parser implements such as [`Clone`], which
    /// can cause additional type errors.
    #[inline]
    fn opaque(self) -> impl Parser<S, O, E>
    where
        Self: Sized,
    {
        self
    }
}

impl<S, O, E, F> Parser<S, O, E> for F
where
    S: Stream,
    E: Error<S>,
    F: FnMut(&mut S) -> Result<O, E>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self(stream)
    }
}
