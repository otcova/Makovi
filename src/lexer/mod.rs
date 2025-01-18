mod token_match_macros;
mod tokens;

use crate::error::*;
use logos::{Logos, Span};
pub(crate) use token_match_macros::*;
pub use tokens::*;

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
    token: TokenResult<'a>,
}

#[derive(Clone, Copy)]
pub struct TokenResult<'a> {
    pub token: Option<Result<Token, ()>>,
    pub slice: &'a str,
    pub span: LineSpan,
    indent: usize,
}

impl TokenResult<'_> {
    pub fn expect_token(self) -> Result<Token, CompilationError> {
        match self.token {
            Some(Ok(token)) => Ok(token),
            Some(Err(())) => Err(CompilationError {
                message: format!("Unknown token {}", self.slice),
                span: self.span,
            }),
            None => Err(CompilationError {
                message: "Expected a token but reached end of file".to_owned(),
                span: self.span,
            }),
        }
    }

    pub fn expect(self, token: Token) -> Result<Self, CompilationError> {
        if token == self.expect_token()? {
            Ok(self)
        } else {
            Err(CompilationError {
                message: format!("Expected token '{token:?}' but found '{}'", self.slice),
                span: self.span,
            })
        }
    }
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
            token: TokenResult {
                token: None,
                slice: "",
                span: LineSpan::default(),
                indent: 0,
            },
            lexer: Token::lexer_with_extras(trimed_source, context),
        };
        lexer.next();
        lexer
    }

    pub fn next(&mut self) -> TokenResult<'a> {
        let token = self.lexer.next();
        let line_span = self.lexer.extras.span(self.lexer.span());
        let slice = self.lexer.slice();

        if token == Some(Ok(Token::NewLine)) {
            self.lexer.extras.new_line(self.lexer.span());
        }

        std::mem::replace(
            &mut self.token,
            TokenResult {
                token,
                slice,
                span: line_span,
                indent: self.lexer.extras.indent,
            },
        )
    }

    pub fn peek(&self) -> TokenResult<'a> {
        self.token
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

impl TokenResult<'_> {
    pub fn nesting_span(&self) -> LineSpan {
        LineSpan {
            start: LineColumnNumber {
                line: self.span.start.line,
                column: 1,
            },
            end: LineColumnNumber {
                line: self.span.start.line,
                column: self.indent,
            },
        }
    }

    pub fn nesting(&self) -> Result<usize, CompilationError> {
        const INDENT_SIZE: usize = 4;
        if self.indent % INDENT_SIZE != 0 {
            return Err(CompilationError {
                message: format!(
                    "Invalid indentation of {} spaces. Expected an indentation multiple of {}",
                    self.indent, INDENT_SIZE
                ),
                span: self.nesting_span(),
            });
        }

        Ok(self.indent / INDENT_SIZE)
    }
}
