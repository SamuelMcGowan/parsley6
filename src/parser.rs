use crate::stream::Stream;

pub trait Parser<S: Stream, Output, Error> {
    fn parse(&mut self, stream: &mut S) -> Result<Output, Error>;

    #[inline]
    fn opaque(self) -> impl Parser<S, Output, Error>
    where
        Self: Sized,
    {
        self
    }
}

impl<S, Output, Error, F> Parser<S, Output, Error> for F
where
    S: Stream,
    F: FnMut(&mut S) -> Result<Output, Error>,
{
    #[inline]
    fn parse(&mut self, stream: &mut S) -> Result<Output, Error> {
        self(stream)
    }
}
