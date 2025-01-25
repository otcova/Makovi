mod line_span;

pub use line_span::*;
use std::fmt::Debug;

pub struct CompilationError {
    pub message: String,
    pub span: LineSpan,
}

#[derive(Default)]
pub struct CompilationErrorSet {
    errors: Vec<CompilationError>,
}

impl From<CompilationErrorSet> for String {
    fn from(errors: CompilationErrorSet) -> Self {
        format!("{:?}", errors)
    }
}

impl CompilationErrorSet {
    pub fn add(&mut self, error: CompilationError) {
        self.errors.push(error);
    }
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Debug for CompilationErrorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.errors.is_empty() {
            return Ok(());
        }

        writeln!(f);

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
