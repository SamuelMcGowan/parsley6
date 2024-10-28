use std::ops::{Range, RangeFull, RangeInclusive};

pub trait TokenSet<Token> {
    fn contains(&self, token: &Token) -> bool;

    #[inline]
    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not(self)
    }
}

impl<F: Fn(&Token) -> bool, Token> TokenSet<Token> for F {
    #[inline]
    fn contains(&self, token: &Token) -> bool {
        (self)(token)
    }
}

impl<Token: PartialEq> TokenSet<Token> for [Token] {
    #[inline]
    fn contains(&self, token: &Token) -> bool {
        self.contains(token)
    }
}

impl<const N: usize, Token: PartialEq> TokenSet<Token> for [Token; N] {
    #[inline]
    fn contains(&self, token: &Token) -> bool {
        self.as_slice().contains(token)
    }
}

impl<T: PartialOrd> TokenSet<T> for Range<T> {
    #[inline]
    fn contains(&self, token: &T) -> bool {
        self.contains(token)
    }
}

impl<T: PartialOrd> TokenSet<T> for RangeInclusive<T> {
    #[inline]
    fn contains(&self, token: &T) -> bool {
        self.contains(token)
    }
}

impl<T> TokenSet<T> for RangeFull {
    #[inline]
    fn contains(&self, _: &T) -> bool {
        true
    }
}

// TODO: more range types?

macro_rules! impl_token_set {
    ($($name:ident),*) => {
        impl<$($name: TokenSet<Token>,)* Token> TokenSet<Token> for ($($name,)*) {
            #[inline]
            #[allow(non_snake_case)]
            fn contains(&self, token: &Token) -> bool {
                let ($($name,)*) = self;
                $($name.contains(token))||*
            }
        }
    };
}

impl_token_set! { A }
impl_token_set! { A, B }
impl_token_set! { A, B, C }
impl_token_set! { A, B, C, D }
impl_token_set! { A, B, C, D, E }
impl_token_set! { A, B, C, D, E, F }
impl_token_set! { A, B, C, D, E, F, G }
impl_token_set! { A, B, C, D, E, F, G, H }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Not<T>(T);

impl<T: TokenSet<Token>, Token> TokenSet<Token> for Not<T> {
    #[inline]
    fn contains(&self, token: &Token) -> bool {
        !self.0.contains(token)
    }
}
