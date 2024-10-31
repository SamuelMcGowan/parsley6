use std::ops::{Deref, Range};
use std::slice::Iter;
use std::str::Chars;

pub trait Stream {
    type Token: Clone + PartialEq;

    type Slice: PartialEq + ?Sized;
    type SliceRef: Deref<Target = Self::Slice> + Copy;

    type Span: Span + Default + Clone;

    fn peek_token(&self) -> Option<Self::Token>;
    fn next_token(&mut self) -> Option<Self::Token>;

    fn try_slice(&self, start: usize, end: usize) -> Option<Self::SliceRef>;

    fn peek_token_span(&self) -> Self::Span;
    fn prev_token_span(&self) -> Self::Span;

    fn stream_position(&self) -> usize;

    #[inline]
    fn at_end(&self) -> bool {
        self.peek_token().is_none()
    }

    #[inline]
    fn slice(&self, start: usize, end: usize) -> Self::SliceRef {
        self.try_slice(start, end).expect("slice out of bounds")
    }
}

pub trait StreamEatSlice<Slice: ?Sized>: Stream {
    fn peek_slice(&self, slice: &Slice) -> Option<Self::SliceRef>;
    fn eat_slice(&mut self, slice: &Slice) -> Option<Self::SliceRef>;
}

pub trait Span {
    fn merge(self, other: Self) -> Self;
    fn merge_right(self, other: Self) -> Self;
}

impl<T: Ord> Span for Range<T> {
    #[inline]
    fn merge(self, other: Self) -> Self {
        self.start.min(other.start)..self.end.max(other.end)
    }

    #[inline]
    fn merge_right(self, other: Self) -> Self {
        self.start..self.end.max(other.end)
    }
}

#[derive(Debug, Clone)]
pub struct CharStream<'a> {
    all: &'a str,
    chars: Chars<'a>,
}

impl<'a> CharStream<'a> {
    #[inline]
    pub fn new(s: &'a str) -> Self {
        Self {
            all: s,
            chars: s.chars(),
        }
    }
}

impl<'a> Stream for CharStream<'a> {
    type Token = char;

    type Slice = str;
    type SliceRef = &'a str;

    type Span = Range<usize>;

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.chars.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.chars.next()
    }

    #[inline]
    fn try_slice(&self, start: usize, end: usize) -> Option<Self::SliceRef> {
        self.all.get(start..end)
    }

    #[inline]
    fn peek_token_span(&self) -> Range<usize> {
        let pos = self.stream_position();
        let ch_len = self.peek_token().map(char::len_utf8).unwrap_or_default();
        pos..(pos + ch_len)
    }

    #[inline]
    fn prev_token_span(&self) -> Range<usize> {
        let pos = self.stream_position();
        let ch_len = self.all[..pos]
            .chars()
            .next_back()
            .map(char::len_utf8)
            .unwrap_or_default();
        (pos - ch_len)..pos
    }

    #[inline]
    fn stream_position(&self) -> usize {
        self.all.len() - self.chars.as_str().len()
    }
}

impl<'a> StreamEatSlice<str> for CharStream<'a> {
    #[inline]
    fn peek_slice(&self, slice: &str) -> Option<Self::SliceRef> {
        match self.chars.as_str().split_at_checked(slice.len()) {
            Some((prefix, _)) if prefix == slice => Some(prefix),
            _ => None,
        }
    }

    #[inline]
    fn eat_slice(&mut self, slice: &str) -> Option<Self::SliceRef> {
        match self.chars.as_str().split_at_checked(slice.len()) {
            Some((prefix, rest)) if prefix == slice => {
                self.chars = rest.chars();
                Some(prefix)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SliceStream<'a, T: SourceSpanned> {
    all: &'a [T],
    iter: Iter<'a, T>,
    end: T::Span,
}

impl<'a, T: SourceSpanned + Clone + PartialEq> SliceStream<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T], end: T::Span) -> Self {
        Self {
            all: slice,
            iter: slice.iter(),
            end,
        }
    }
}

impl<'a, T: SourceSpanned + Clone + PartialEq> Stream for SliceStream<'a, T> {
    type Token = &'a T;

    type Slice = [T];
    type SliceRef = &'a [T];

    type Span = T::Span;

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.iter.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.iter.next()
    }

    #[inline]
    fn try_slice(&self, start: usize, end: usize) -> Option<Self::SliceRef> {
        self.all.get(start..end)
    }

    #[inline]
    fn peek_token_span(&self) -> Self::Span {
        self.peek_token()
            .map(|t| t.source_span())
            .unwrap_or_else(|| self.end.clone())
    }

    #[inline]
    fn prev_token_span(&self) -> Self::Span {
        self.all[..self.stream_position()]
            .last()
            .map(|t| t.source_span())
            .unwrap_or_default()
    }

    #[inline]
    fn stream_position(&self) -> usize {
        self.all.len() - self.iter.as_slice().len()
    }
}

impl<'a, T: SourceSpanned + Clone + PartialEq> StreamEatSlice<[T]> for SliceStream<'a, T> {
    #[inline]
    fn peek_slice(&self, slice: &[T]) -> Option<Self::SliceRef> {
        match self.iter.as_slice().split_at_checked(slice.len()) {
            Some((prefix, _)) if prefix == slice => Some(prefix),
            _ => None,
        }
    }

    #[inline]
    fn eat_slice(&mut self, slice: &[T]) -> Option<Self::SliceRef> {
        match self.iter.as_slice().split_at_checked(slice.len()) {
            Some((prefix, rest)) if prefix == slice => {
                self.iter = rest.iter();
                Some(prefix)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamWithState<S: Stream, State> {
    pub stream: S,
    pub state: State,
}

impl<S: Stream, State> StreamWithState<S, State> {
    #[inline]
    pub fn new(stream: S, state: State) -> Self {
        Self { stream, state }
    }
}

impl<S: Stream, State> Stream for StreamWithState<S, State> {
    type Token = S::Token;

    type Slice = S::Slice;
    type SliceRef = S::SliceRef;

    type Span = S::Span;

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.stream.peek_token()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.stream.next_token()
    }

    #[inline]
    fn try_slice(&self, start: usize, end: usize) -> Option<Self::SliceRef> {
        self.stream.try_slice(start, end)
    }

    #[inline]
    fn peek_token_span(&self) -> Self::Span {
        self.stream.peek_token_span()
    }

    #[inline]
    fn prev_token_span(&self) -> Self::Span {
        self.stream.prev_token_span()
    }

    #[inline]
    fn stream_position(&self) -> usize {
        self.stream.stream_position()
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.stream.at_end()
    }
}

impl<S: Stream + StreamEatSlice<Slice>, State, Slice> StreamEatSlice<Slice>
    for StreamWithState<S, State>
{
    #[inline]
    fn peek_slice(&self, slice: &Slice) -> Option<S::SliceRef> {
        self.stream.peek_slice(slice)
    }

    #[inline]
    fn eat_slice(&mut self, slice: &Slice) -> Option<S::SliceRef> {
        self.stream.eat_slice(slice)
    }
}

pub trait BorrowState: crate::sealed::Sealed {
    type State;

    // TODO: do we need both of these?
    fn borrow_state(&self) -> &Self::State;
    fn borrow_state_mut(&mut self) -> &mut Self::State;
}

impl<S: Stream, State> crate::sealed::Sealed for StreamWithState<S, State> {}

impl<S: Stream, State> BorrowState for StreamWithState<S, State> {
    type State = State;

    #[inline]
    fn borrow_state(&self) -> &Self::State {
        &self.state
    }

    #[inline]
    fn borrow_state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }
}

pub trait SourceSpanned {
    type Span: Span + Default + Clone;

    fn source_span(&self) -> Self::Span;
}
