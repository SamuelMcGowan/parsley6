#[macro_export]
macro_rules! select {
    ($($pat:pat $(if $cond:expr)? => $expr:expr),+ $(,)?) => {{
        |stream: &mut _| {
            match $crate::stream::Stream::peek_token(stream) {
                $(Some($pat) $(if $cond)? => $expr.parse(stream),)+

                _ => Err($crate::error::Error::new(
                    $crate::error::Cause::Unknown,
                    $crate::stream::Stream::peek_token_span(stream),
                )),
            }
        }
    }};
}
