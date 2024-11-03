use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::Error;
use crate::parser::Parser;
use crate::stream::Stream;

pub trait ChainedParsers<S, E>
where
    S: Stream,
    E: Error<S>,
{
    type Output;

    fn parse_chained(&mut self, stream: &mut S) -> Result<Self::Output, E>;
}

macro_rules! impl_chained_parsers {
    ($($parser:ident $output:ident $n:tt),+) => {
        impl<S, Err, $($parser, $output,)*>
        ChainedParsers<S, Err> for ($($parser,)*)
        where
            S: Stream,
            Err: Error<S>,
            $($parser: Parser<S, Err, Output = $output>,)*
        {
            type Output = ($($output,)*);

            #[allow(non_snake_case)]
            #[inline]
            fn parse_chained(&mut self, stream: &mut S) -> Result<Self::Output, Err> {
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

#[macro_export]
macro_rules! chain {
    ($($e:expr),+ $(,)?) => {
        $crate::combinator::chain_inner(($($e),*))
    };
}

#[inline]
#[doc(hidden)]
pub fn chain_inner<S, E, Parsers>(parsers: Parsers) -> Chained<S, E, Parsers>
where
    S: Stream,
    E: Error<S>,
    Parsers: ChainedParsers<S, E>,
{
    Chained {
        parsers,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; Parsers)]
pub struct Chained<S, E, Parsers> {
    parsers: Parsers,
    _phantom: PhantomData<*const (S, E)>,
}

impl<S, E, Parsers> Parser<S, E> for Chained<S, E, Parsers>
where
    S: Stream,
    E: Error<S>,
    Parsers: ChainedParsers<S, E>,
{
    type Output = Parsers::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        self.parsers.parse_chained(stream)
    }
}

#[inline]
pub fn prefixed<A, B, S, E>(prefix: A, parser: B) -> Prefixed<A, B, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
{
    Prefixed {
        prefix,
        parser,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B)]
pub struct Prefixed<A, B, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
{
    prefix: A,
    parser: B,
    _phantom: PhantomData<*const (S, E)>,
}

impl<A, B, S, E> Parser<S, E> for Prefixed<A, B, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
{
    type Output = B::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        let _ = self.prefix.parse(stream)?;
        self.parser.parse(stream)
    }
}

#[inline]
pub fn suffixed<A, B, S, E>(parser: A, suffix: B) -> Suffixed<A, B, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
{
    Suffixed {
        parser,
        suffix,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B)]
pub struct Suffixed<A, B, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
{
    parser: A,
    suffix: B,
    _phantom: PhantomData<*const (S, E)>,
}

impl<A, B, S, E> Parser<S, E> for Suffixed<A, B, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
{
    type Output = A::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        let output = self.parser.parse(stream)?;
        let _ = self.suffix.parse(stream)?;
        Ok(output)
    }
}

#[inline]
pub fn between<A, B, C, S, E>(prefix: A, parser: B, suffix: C) -> Between<A, B, C, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
    C: Parser<S, E>,
{
    Between {
        prefix,
        parser,
        suffix,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; A, B, C)]
pub struct Between<A, B, C, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
    C: Parser<S, E>,
{
    prefix: A,
    parser: B,
    suffix: C,
    _phantom: PhantomData<*const (S, E)>,
}

impl<A, B, C, S, E> Parser<S, E> for Between<A, B, C, S, E>
where
    S: Stream,
    E: Error<S>,
    A: Parser<S, E>,
    B: Parser<S, E>,
    C: Parser<S, E>,
{
    type Output = B::Output;

    fn parse(&mut self, stream: &mut S) -> Result<Self::Output, E> {
        let _ = self.prefix.parse(stream)?;
        let output = self.parser.parse(stream)?;
        let _ = self.suffix.parse(stream)?;
        Ok(output)
    }
}
