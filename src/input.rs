use std::{slice::Iter, str::Chars};

pub trait Input<'a> {
    type Token: Clone + PartialEq;
    type Slice: Slice + PartialEq + ?Sized;

    fn peek_token(&self) -> Option<Self::Token>;
    fn next_token(&mut self) -> Option<Self::Token>;

    fn as_slice(&self) -> &'a Self::Slice;

    #[inline]
    fn advance(&mut self) {
        let _ = self.next_token();
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.peek_token().is_none()
    }
}

/// A slice, namely `[T]` or [`str`].
pub trait Slice: crate::sealed::Sealed {
    fn slice_prefix(&self, len: usize) -> Option<&Self>;
}

impl crate::sealed::Sealed for str {}

impl Slice for str {
    #[inline]
    fn slice_prefix(&self, len: usize) -> Option<&Self> {
        self.get(..len)
    }
}

impl<T> crate::sealed::Sealed for [T] {}

impl<T> Slice for [T] {
    #[inline]
    fn slice_prefix(&self, len: usize) -> Option<&Self> {
        self.get(..len)
    }
}

impl<'a> Input<'a> for Chars<'a> {
    type Token = char;
    type Slice = str;

    #[inline]
    fn as_slice(&self) -> &'a Self::Slice {
        self.as_str()
    }

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.next()
    }
}

impl<'a, T: Clone + PartialEq> Input<'a> for Iter<'a, T> {
    type Token = &'a T;
    type Slice = [T];

    #[inline]
    fn as_slice(&self) -> &'a Self::Slice {
        self.as_slice()
    }

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.next()
    }
}
