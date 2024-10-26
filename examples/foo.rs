use parsley6::{
    error::BuiltinError,
    parser::Parser,
    stream::CharStream,
    token::{eat_match, text::Ascii},
};

fn main() {
    let mut stream = CharStream::new("hello world");
    let _ = dbg!(five_letters.parse(&mut stream));
}

fn five_letters<'a>(
    stream: &mut CharStream<'a>,
) -> Result<(char, char, char, char, char), BuiltinError<CharStream<'a>>> {
    let a = eat_match(Ascii::is_ascii_alphabetic);
    (a, a, a, a, a).parse(stream)
}
