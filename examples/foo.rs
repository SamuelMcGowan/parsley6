use parsley6::prelude::*;

use parsley6::error::DefaultError;
use parsley6::stream::CharStream;

// type Error =

fn main() {
    let mut stream = CharStream::new("bar");
    let _ = dbg!(bar_bat.parse(&mut stream));

    let mut stream = CharStream::new("bat");
    let _ = dbg!(bar_bat.parse(&mut stream));

    let mut stream = CharStream::new("bart");
    let _ = dbg!(bar_bat.parse(&mut stream));

    let mut stream = CharStream::new("bap");
    let _ = dbg!(bar_bat.parse(&mut stream));

    let mut stream = CharStream::new("a0a 12");
    let _ = dbg!(ident.parse(&mut stream));

    let mut stream = CharStream::new("0a");
    let _ = dbg!(ident.parse(&mut stream));
}

fn bar_bat<'a>(stream: &mut CharStream<'a>) -> Result<String, DefaultError<CharStream<'a>>> {
    chain!(
        eat('b'),
        eat('a'),
        alt!(eat('r'), eat('t')).with_err_cause(|| "expected `r` or `t`")
    )
    .map_with_slice(|_, slice: &str| slice.to_ascii_uppercase())
    .then_drop(end())
    .with_err_context(|| "while parsing `bar` or `bat`")
    .parse(stream)
}

fn ident<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, DefaultError<CharStream<'a>>> {
    chain!(
        eat_match(|ch: &char| ch.is_ascii_alphabetic() || *ch == '_')
            .with_err_cause(|| "expected an alphabetic character or `_`"),
        eat_match(|ch: &char| ch.is_ascii_alphanumeric() || *ch == '_').repeat()
    )
    .map_to_slice()
    .parse(stream)
}
