use std::fmt::Debug;

pub struct CompilationError {
    pub message: String,
    pub span: LineSpan,
}

#[derive(Default)]
pub struct CompilationErrorSet {
    errors: Vec<CompilationError>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LineSpan {
    pub start: LineColumnNumber,
    pub end: LineColumnNumber,
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

impl From<CompilationErrorSet> for String {
    fn from(errors: CompilationErrorSet) -> Self {
        format!("{:?}", errors)
    }
}

impl CompilationErrorSet {
    pub fn push(&mut self, error: CompilationError) {
        self.errors.push(error);
    }
    pub fn clear(&mut self) {
        self.errors.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl LineSpan {
    pub fn and(self, other: LineSpan) -> LineSpan {
        LineSpan {
            start: self.start.min(other.start),
            end: other.end.max(other.end),
        }
    }
}

impl Debug for CompilationErrorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.errors.is_empty() {
            return Ok(());
        }

        writeln!(f)?;

        for error in &self.errors {
            writeln!(
                f,
                "[Compilation error] {} (from {}:{} to {}:{})",
                error.message,
                error.span.start.line,
                error.span.start.column,
                error.span.end.line,
                error.span.end.column,
            )?;
        }
        Ok(())
    }
}
