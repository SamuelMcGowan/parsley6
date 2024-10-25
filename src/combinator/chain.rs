use std::marker::PhantomData;

use derive_where::derive_where;

use crate::parser::Parser;
use crate::stream::Stream;

macro_rules! impl_chained_parsers {
    ($($parser:ident $output:ident $n:tt),+) => {
        impl<'a, S, Error, $($parser, $output,)*>
        Parser<'a, S, ($($output,)*), Error> for ($($parser,)*)
        where
            S: Stream<'a>,
            $($parser: Parser<'a, S, $output, Error>,)*
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
pub fn prefixed<'a, A, B, S, AOutput, BOutput, Error>(
    prefix: A,
    parser: B,
) -> Prefixed<'a, A, B, AOutput, BOutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
{
    Prefixed {
        prefix,
        parser,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B)]
pub struct Prefixed<'a, A, B, AOutput, BOutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
{
    prefix: A,
    parser: B,
    _phantom: PhantomData<&'a (S, AOutput, BOutput, Error)>,
}

impl<'a, A, B, AOutput, BOutput, S, Error> Parser<'a, S, BOutput, Error>
    for Prefixed<'a, A, B, AOutput, BOutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
{
    fn parse(&mut self, stream: &mut S) -> Result<BOutput, Error> {
        let _ = self.prefix.parse(stream)?;
        self.parser.parse(stream)
    }
}

#[inline]
pub fn suffixed<'a, A, B, S, AOutput, BOutput, Error>(
    parser: A,
    suffix: B,
) -> Suffixed<'a, A, B, AOutput, BOutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
{
    Suffixed {
        parser,
        suffix,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B)]
pub struct Suffixed<'a, A, B, AOutput, BOutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
{
    parser: A,
    suffix: B,
    _phantom: PhantomData<&'a (S, AOutput, BOutput, Error)>,
}

impl<'a, A, B, AOutput, BOutput, S, Error> Parser<'a, S, AOutput, Error>
    for Suffixed<'a, A, B, AOutput, BOutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
{
    fn parse(&mut self, stream: &mut S) -> Result<AOutput, Error> {
        let output = self.parser.parse(stream)?;
        let _ = self.suffix.parse(stream)?;
        Ok(output)
    }
}

#[inline]
pub fn between<'a, A, B, C, S, AOutput, BOutput, COutput, Error>(
    prefix: A,
    parser: B,
    suffix: C,
) -> Between<'a, A, B, C, AOutput, BOutput, COutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
    C: Parser<'a, S, COutput, Error>,
{
    Between {
        prefix,
        parser,
        suffix,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B, C)]
pub struct Between<'a, A, B, C, AOutput, BOutput, COutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
    C: Parser<'a, S, COutput, Error>,
{
    prefix: A,
    parser: B,
    suffix: C,
    _phantom: PhantomData<&'a (S, AOutput, BOutput, COutput, Error)>,
}

impl<'a, A, B, C, AOutput, BOutput, COutput, S, Error> Parser<'a, S, BOutput, Error>
    for Between<'a, A, B, C, AOutput, BOutput, COutput, S, Error>
where
    S: Stream<'a>,
    A: Parser<'a, S, AOutput, Error>,
    B: Parser<'a, S, BOutput, Error>,
    C: Parser<'a, S, COutput, Error>,
{
    fn parse(&mut self, stream: &mut S) -> Result<BOutput, Error> {
        let _ = self.prefix.parse(stream)?;
        let output = self.parser.parse(stream)?;
        let _ = self.suffix.parse(stream)?;
        Ok(output)
    }
}
