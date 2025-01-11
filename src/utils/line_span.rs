#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LineSpan {
    pub start: LineColumnNumber,
    pub end: LineColumnNumber,
}
impl LineSpan {
    pub fn and(self, other: LineSpan) -> LineSpan {
        LineSpan {
            start: self.start.min(other.start),
            end: other.end.max(other.end),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineColumnNumber {
    pub line: usize,
    pub column: usize,
}

impl Default for LineColumnNumber {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}
