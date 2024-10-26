use parsley6::{
    combinator::chain,
    error::DefaultError,
    parser::Parser,
    stream::CharStream,
    token::{eat, eat_match, text::Ascii},
};

fn main() {
    let mut stream = CharStream::new("hello world");
    let _ = dbg!(five_letters.parse(&mut stream));
}

fn five_letters<'a>(
    stream: &mut CharStream<'a>,
) -> Result<(char, char, char, char, char), DefaultError<CharStream<'a>>> {
    let a = eat_match(Ascii::is_ascii_alphabetic);
    chain((a, a, a, a, eat('o'))).parse(stream)
}
