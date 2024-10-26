use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::Error;
use crate::parser::Parser;
use crate::stream::Stream;

pub trait ChainedParsers<S, O, E>
where
    S: Stream,
    E: Error<S>,
{
    fn parse_chained(&mut self, stream: &mut S) -> Result<O, E>;
}

macro_rules! impl_chained_parsers {
    ($($parser:ident $output:ident $n:tt),+) => {
        impl<S, Err, $($parser, $output,)*>
        ChainedParsers<S, ($($output,)*), Err> for ($($parser,)*)
        where
            S: Stream,
            Err: Error<S>,
            $($parser: Parser<S, $output, Err>,)*
        {
            #[allow(non_snake_case)]
            #[inline]
            fn parse_chained(&mut self, stream: &mut S) -> Result<($($output,)*), Err> {
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
pub fn chain<S, O, E, Parsers>(parsers: Parsers) -> Chained<S, O, E, Parsers>
where
    S: Stream,
    E: Error<S>,
    Parsers: ChainedParsers<S, O, E>,
{
    Chained {
        parsers,
        _phantom: PhantomData,
    }
}

pub struct Chained<S, O, E, Parsers> {
    parsers: Parsers,
    _phantom: PhantomData<*const (S, O, E)>,
}

impl<S, O, E, Parsers> Parser<S, O, E> for Chained<S, O, E, Parsers>
where
    S: Stream,
    E: Error<S>,
    Parsers: ChainedParsers<S, O, E>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parsers.parse_chained(stream)
    }
}

#[inline]
pub fn prefixed<A, B, S, AOutput, BOutput, E>(
    prefix: A,
    parser: B,
) -> Prefixed<A, B, AOutput, BOutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
{
    Prefixed {
        prefix,
        parser,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B)]
pub struct Prefixed<A, B, AOutput, BOutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
{
    prefix: A,
    parser: B,
    _phantom: PhantomData<*const (S, AOutput, BOutput, E)>,
}

impl<A, B, AOutput, BOutput, S, E> Parser<S, BOutput, E> for Prefixed<A, B, AOutput, BOutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
{
    fn parse(&mut self, stream: &mut S) -> Result<BOutput, E> {
        let _ = self.prefix.parse(stream)?;
        self.parser.parse(stream)
    }
}

#[inline]
pub fn suffixed<A, B, S, AOutput, BOutput, E>(
    parser: A,
    suffix: B,
) -> Suffixed<A, B, AOutput, BOutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
{
    Suffixed {
        parser,
        suffix,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B)]
pub struct Suffixed<A, B, AOutput, BOutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
{
    parser: A,
    suffix: B,
    _phantom: PhantomData<*const (S, AOutput, BOutput, E)>,
}

impl<A, B, AOutput, BOutput, S, E> Parser<S, AOutput, E> for Suffixed<A, B, AOutput, BOutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
{
    fn parse(&mut self, stream: &mut S) -> Result<AOutput, E> {
        let output = self.parser.parse(stream)?;
        let _ = self.suffix.parse(stream)?;
        Ok(output)
    }
}

#[inline]
pub fn between<A, B, C, S, AOutput, BOutput, COutput, E>(
    prefix: A,
    parser: B,
    suffix: C,
) -> Between<A, B, C, AOutput, BOutput, COutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
    C: Parser<S, COutput, E>,
{
    Between {
        prefix,
        parser,
        suffix,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B, C)]
pub struct Between<A, B, C, AOutput, BOutput, COutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
    C: Parser<S, COutput, E>,
{
    prefix: A,
    parser: B,
    suffix: C,
    _phantom: PhantomData<*const (S, AOutput, BOutput, COutput, E)>,
}

impl<A, B, C, AOutput, BOutput, COutput, S, E> Parser<S, BOutput, E>
    for Between<A, B, C, AOutput, BOutput, COutput, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, AOutput, E>,
    B: Parser<S, BOutput, E>,
    C: Parser<S, COutput, E>,
{
    fn parse(&mut self, stream: &mut S) -> Result<BOutput, E> {
        let _ = self.prefix.parse(stream)?;
        let output = self.parser.parse(stream)?;
        let _ = self.suffix.parse(stream)?;
        Ok(output)
    }
}
