use std::{cmp::{max, min}, ops::Range};


pub mod sources;
pub mod traits;
pub mod errors;
pub mod span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    value: T,
    span: Range<usize>
}

impl<T> From<(T, Range<usize>)> for Spanned<T> {
    #[inline]
    fn from((value, span): (T, Range<usize>)) -> Self {
        Spanned::new(value, span)
    }
}

impl<T> Spanned<T> {
    #[inline]
    pub fn new(value: T, span: Range<usize>) -> Self {
        Self { value, span }
    }

    #[inline]
    pub fn get_value(&self) -> &T {
        &self.value
    }

    #[inline]
    pub fn get_span(&self) -> Range<usize> {
        self.span.clone()
    }

    #[inline]
    pub fn destruct(self) -> (T, Range<usize>) {
        (self.value, self.span)
    }

    pub fn concat(&self, rhs: &Self) -> Range<usize> {
        let a = self.get_span();
        let b = rhs.get_span();

        min(a.start, b.start)..max(a.end, b.end)
    }

    pub fn append_span(&self, rhs: &Range<usize>) -> Range<usize> {
        let a = self.get_span();
        let b = rhs;

        min(a.start, b.start)..max(a.end, b.end)
    }
}

