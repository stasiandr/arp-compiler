use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Span {
    start: usize,
    end: usize,
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Span { start: value.start, end: value.end, }
    }
}

impl From<Span> for Range<usize> {
    fn from(val: Span) -> Self {
        Self { start: val.start, end: val.end, }
    }
}
