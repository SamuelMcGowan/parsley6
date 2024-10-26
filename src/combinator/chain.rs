use std::marker::PhantomData;

use derive_where::derive_where;

use crate::parser::Parser;
use crate::stream::Stream;

macro_rules! impl_chained_parsers {
    ($($parser:ident $output:ident $n:tt),+) => {
        impl<S, Error, $($parser, $output,)*>
        Parser<S, ($($output,)*), Error> for ($($parser,)*)
        where
            S: Stream,
            $($parser: Parser<S, $output, Error>,)*
        {
            #[allow(non_snake_case)]
            fn parse(&mut self, stream: &mut S) -> Result<($($output,)*), Error> {
                $(let $parser = self.$n.parse(stream)?;)*
                Ok(($($parser,)*))
            }
        }
    };
}

impl_chained_parsers! { A AO 0 }
impl_chained_parsers! { A AO 0, B BO 1 }
impl_chained_parsers! { A AO 0, B BO 1, C CO 2 }
impl_chained_parsers! { A AO 0, B BO 1, C CO 2, D DO 3 }
impl_chained_parsers! { A AO 0, B BO 1, C CO 2, D DO 3, E EO 4 }
impl_chained_parsers! { A AO 0, B BO 1, C CO 2, D DO 3, E EO 4, F FO 5 }
impl_chained_parsers! { A AO 0, B BO 1, C CO 2, D DO 3, E EO 4, F FO 5, G GO 6 }
impl_chained_parsers! { A AO 0, B BO 1, C CO 2, D DO 3, E EO 4, F FO 5, G GO 6, H HO 7 }

#[inline]
pub fn prefixed<A, B, S, AOutput, BOutput, Error>(
    prefix: A,
    parser: B,
) -> Prefixed<A, B, AOutput, BOutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
{
    Prefixed {
        prefix,
        parser,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B)]
pub struct Prefixed<A, B, AOutput, BOutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
{
    prefix: A,
    parser: B,
    _phantom: PhantomData<*const (S, AOutput, BOutput, Error)>,
}

impl<A, B, AOutput, BOutput, S, Error> Parser<S, BOutput, Error>
    for Prefixed<A, B, AOutput, BOutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
{
    fn parse(&mut self, stream: &mut S) -> Result<BOutput, Error> {
        let _ = self.prefix.parse(stream)?;
        self.parser.parse(stream)
    }
}

#[inline]
pub fn suffixed<A, B, S, AOutput, BOutput, Error>(
    parser: A,
    suffix: B,
) -> Suffixed<A, B, AOutput, BOutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
{
    Suffixed {
        parser,
        suffix,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B)]
pub struct Suffixed<A, B, AOutput, BOutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
{
    parser: A,
    suffix: B,
    _phantom: PhantomData<*const (S, AOutput, BOutput, Error)>,
}

impl<A, B, AOutput, BOutput, S, Error> Parser<S, AOutput, Error>
    for Suffixed<A, B, AOutput, BOutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
{
    fn parse(&mut self, stream: &mut S) -> Result<AOutput, Error> {
        let output = self.parser.parse(stream)?;
        let _ = self.suffix.parse(stream)?;
        Ok(output)
    }
}

#[inline]
pub fn between<A, B, C, S, AOutput, BOutput, COutput, Error>(
    prefix: A,
    parser: B,
    suffix: C,
) -> Between<A, B, C, AOutput, BOutput, COutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
    C: Parser<S, COutput, Error>,
{
    Between {
        prefix,
        parser,
        suffix,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B, C)]
pub struct Between<A, B, C, AOutput, BOutput, COutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
    C: Parser<S, COutput, Error>,
{
    prefix: A,
    parser: B,
    suffix: C,
    _phantom: PhantomData<*const (S, AOutput, BOutput, COutput, Error)>,
}

impl<A, B, C, AOutput, BOutput, COutput, S, Error> Parser<S, BOutput, Error>
    for Between<A, B, C, AOutput, BOutput, COutput, S, Error>
where
    S: Stream,
    A: Parser<S, AOutput, Error>,
    B: Parser<S, BOutput, Error>,
    C: Parser<S, COutput, Error>,
{
    fn parse(&mut self, stream: &mut S) -> Result<BOutput, Error> {
        let _ = self.prefix.parse(stream)?;
        let output = self.parser.parse(stream)?;
        let _ = self.suffix.parse(stream)?;
        Ok(output)
    }
}
