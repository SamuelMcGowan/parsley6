use crate::stream::Stream;

pub trait Parser<'a, S: Stream<'a>, Output, Error> {
    fn parse(&mut self, stream: &mut S) -> Result<Output, Error>;

    #[inline]
    fn opaque(self) -> impl Parser<'a, S, Output, Error>
    where
        Self: Sized,
    {
        self
    }
}
