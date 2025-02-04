//! Breaks the source code into tokens (keywords, identifiers, literals).

mod tokens;

use super::CompilationPipeline;
use crate::compiler::error::*;
use logos::{Logos, Span};
pub use tokens::*;

pub struct LexerStage<'a> {
    pub lexer: Lexer<'a>,
}

impl CompilationPipeline {
    pub fn lexer_stage(self, source_code: &str) -> LexerStage {
        LexerStage {
            lexer: Lexer::new(source_code),
        }
    }
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
    token: Token,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let trimed_source = source.trim_start_matches(' ');
        let context = LexerContext {
            line_number: 1,
            line_char_index: 0,
            indent: source.len() - trimed_source.len(),
        };

        let mut lexer = Self {
            lexer: Token::lexer_with_extras(trimed_source, context),
            token: Token::Invalid,
        };
        lexer.next();
        lexer
    }

    pub fn remainder(&self) -> &'a str {
        self.lexer.remainder()
    }

    pub fn next(&mut self) -> Token {
        // TODO: Make a token for indentation
        if self.token == Token::NewLine {
            self.lexer.extras.new_line(self.lexer.span());
        }

        let token = self.lexer.next();

        self.token = match token {
            Some(Ok(token)) => token,
            Some(Err(())) => Token::Invalid,
            None => Token::EndOfFile,
        };

        self.token
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn slice(&self) -> &'a str {
        self.lexer.slice()
    }

    pub fn span(&self) -> LineSpan {
        self.lexer.extras.span(self.lexer.span())
    }

    pub fn indent(&self) -> usize {
        self.lexer.extras.indent
    }

    pub fn line_span(&self) -> LineSpan {
        let span = self.span();
        LineSpan {
            start: LineColumnNumber {
                line: span.start.line,
                column: 1,
            },
            end: LineColumnNumber {
                line: span.end.line,
                // TODO: span.end is not the end of the line
                column: span.end.column,
            },
        }
    }

    pub fn indent_span(&self) -> LineSpan {
        let span = self.span();
        LineSpan {
            start: LineColumnNumber {
                line: span.start.line,
                column: 1,
            },
            end: LineColumnNumber {
                line: span.start.line,
                column: self.lexer.extras.indent,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct LexerContext {
    line_number: usize,
    line_char_index: usize,
    indent: usize,
}

impl LexerContext {
    fn new_line(&mut self, new_line_char_span: Span) {
        self.line_number += 1;
        self.line_char_index = new_line_char_span.start + 1;
        self.indent = new_line_char_span.len() - 1;
    }

    fn span(&self, span: Span) -> LineSpan {
        LineSpan {
            start: LineColumnNumber {
                line: self.line_number,
                column: span.start - self.line_char_index + 1,
            },
            end: LineColumnNumber {
                line: self.line_number,
                column: span.end - self.line_char_index,
            },
        }
    }
}
