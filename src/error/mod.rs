mod line_span;

pub use line_span::*;
use std::fmt::Debug;

pub struct CompilationError {
    pub message: String,
    pub span: LineSpan,
}

impl From<CompilationError> for String {
    fn from(error: CompilationError) -> Self {
        format!("{:?}", error)
    }
}

impl Debug for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[Compilation error] {} (from {}:{} to {}:{})",
            self.message,
            self.span.start.line,
            self.span.start.column,
            self.span.end.line,
            self.span.end.column,
        )
    }
}
