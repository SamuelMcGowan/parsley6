use parsley6::prelude::*;

use parsley6::error::{Cause, DefaultError, Error};
use parsley6::stream::CharStream;

type ParseError<'a> = DefaultError<CharStream<'a>, ParseErrorCause>;

#[derive(Debug, Clone)]
pub enum ParseErrorCause {
    Expected(String),

    ExpectedChar(char),
    ExpectedSlice(&'static str),

    ExpectedInSet,
    ExpectedEnd,
    Unknown,

    IntError(std::num::ParseIntError),
}

impl<'a> Cause<CharStream<'a>> for ParseErrorCause {
    fn expected_token(token: char) -> Self {
        ParseErrorCause::ExpectedChar(token)
    }

    fn expected_slice(slice: &'static str) -> Self {
        ParseErrorCause::ExpectedSlice(slice)
    }

    fn expected_in_set() -> Self {
        Self::ExpectedInSet
    }

    fn expected_end() -> Self {
        Self::ExpectedEnd
    }

    fn unknown() -> Self {
        Self::Unknown
    }
}

impl From<String> for ParseErrorCause {
    fn from(s: String) -> Self {
        ParseErrorCause::Expected(s)
    }
}

impl From<&str> for ParseErrorCause {
    fn from(s: &str) -> Self {
        ParseErrorCause::Expected(s.to_owned())
    }
}

impl From<std::num::ParseIntError> for ParseErrorCause {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseErrorCause::IntError(err)
    }
}

fn main() {
    let mut stream = CharStream::new("true");
    let _ = dbg!(parse_value(&mut stream));

    let mut stream = CharStream::new("12");
    let _ = dbg!(parse_value(&mut stream));

    let mut stream = CharStream::new("foo");
    let _ = dbg!(parse_value(&mut stream));

    let mut stream = CharStream::new("1000000000000000000000000000000");
    let _ = dbg!(parse_value(&mut stream));
}

#[derive(Debug, Clone)]
enum Value {
    Number(i32),
    Bool(bool),
    Null,
}

fn parse_value<'a>(stream: &mut CharStream<'a>) -> Result<Value, ParseError<'a>> {
    select!(
        't' => eat_slice("true").map_to(Value::Bool(true)).with_err_cause(|| "expected a value".into()),
        'f' => eat_slice("false").map_to(Value::Bool(false)).with_err_cause(|| "expected a value".into()),
        'n' => eat_slice("null").map_to(Value::Null).with_err_cause(|| "expected a value".into()),
        ch if ch.is_ascii_digit() => parse_number.map(Value::Number).with_err_context(|| "while parsing a number"),
    )
    .parse(stream)
}

fn parse_number<'a>(stream: &mut CharStream<'a>) -> Result<i32, ParseError<'a>> {
    consume(Ascii::is_ascii_digit)
        .with_span()
        .and_then(|(s, span): (&str, _)| {
            s.parse::<i32>()
                .map_err(|err| DefaultError::new(err.into(), span))
        })
        .parse(stream)
}
