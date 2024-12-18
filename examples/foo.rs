use parsley6::prelude::*;

use parsley6::error::{DefaultCause, DefaultError};
use parsley6::stream::CharStream;

type Error<'a> = DefaultError<CharStream<'a>>;

fn main() {
    // let mut stream = CharStream::new("bar");
    // let _ = dbg!(bar_bat.parse(&mut stream));

    // let mut stream = CharStream::new("bat");
    // let _ = dbg!(bar_bat.parse(&mut stream));

    // let mut stream = CharStream::new("bart");
    // let _ = dbg!(bar_bat.parse(&mut stream));

    // let mut stream = CharStream::new("bap");
    // let _ = dbg!(bar_bat.parse(&mut stream));

    // let mut stream = CharStream::new("a0a 12");
    // let _ = dbg!(ident.parse(&mut stream));

    // let mut stream = CharStream::new("0a");
    // let _ = dbg!(ident.parse(&mut stream));

    println!("{:?}", test(list, "[bcbcbc]"));
    println!("{:?}", test(list, "[bcbcb]"));
    // println!("{:?}", test(12, "[bcbcb]"));

    println!("{:?}", test(seek_semicolon, "hello;world"));
    println!("{:?}", test(seek_semicolon, "no semicolon"));
}

fn test<'a, P: Parser<CharStream<'a>, Error<'a>>>(
    mut parser: P,
    input: &'a str,
) -> Result<P::Output, Error<'a>> {
    let mut stream = CharStream::new(input);
    parser.parse(&mut stream)
}

fn _bar_bat<'a>(stream: &mut CharStream<'a>) -> Result<String, Error<'a>> {
    chain!(
        eat('b'),
        eat('a'),
        eat_if(|ch| matches!(ch, 'r' | 't'))
            .with_err_cause(|| DefaultCause::custom("expected `r` or `t`"))
    )
    .to_slice()
    .map(|slice: &str| slice.to_ascii_uppercase())
    .then_drop(end())
    .with_err_context(|| "while parsing `bar` or `bat`")
    .parse(stream)
}

// fn ident<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
//     chain!(
//         eat_match(|ch: &char| ch.is_ascii_alphabetic() || *ch == '_')
//             .with_err_cause(|| "expected an alphabetic character or `_`"),
//         eat_match(|ch: &char| ch.is_ascii_alphanumeric() || *ch == '_').repeat()
//     )
//     .map_to_slice()
//     .parse(stream)
// }

fn _ident<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
    chain!(
        eat_if(|ch: &char| ch.is_ascii_alphabetic() || *ch == '_')
            .with_err_cause(|| DefaultCause::custom("expected an alphabetic character or `_`")),
        eat_while(|ch: &char| ch.is_ascii_alphanumeric() || *ch == '_')
    )
    .to_slice()
    .parse(stream)
}

fn seek_semicolon<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
    seek(|&ch| (ch == ';').then_some(true))
        .to_slice()
        .parse(stream)
}

fn list<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
    between(
        eat('['),
        chain!(eat('b'), eat('c'))
            .repeat_while(|ch: &char| *ch != ']')
            .to_slice(),
        eat(']'),
    )
    .parse(stream)
}

fn _abcdef<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
    select!(
        'a' => chain!(eat('a'), eat('b'), eat('c')),
        'd' => chain!(eat('d'), eat('e'), eat('f')),
    )
    .to_slice()
    .parse(stream)
}
