use derive_where::derive_where;

use crate::stream::Stream;

/// An error that can be used with builtin parsers and combinators.
#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; S::Token)]
pub enum BuiltinError<'a, S: Stream<'a>> {
    ExpectedToken(S::Token),
    ExpectedAny,
    ExpectedEnd,
}
