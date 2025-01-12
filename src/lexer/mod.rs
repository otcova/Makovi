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
        let mut lexer = Self {
            token: TokenResult {
                token: None,
                slice: "",
                span: LineSpan::default(),
            },
            lexer: Token::lexer(source),
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
            },
        )
    }

    pub fn peek(&self) -> TokenResult<'a> {
        self.token
    }
}

#[derive(Clone, Copy)]
pub struct LexerLineContext {
    line_number: usize,
    line_char_index: usize,
}

impl Default for LexerLineContext {
    fn default() -> Self {
        Self {
            line_number: 1,
            line_char_index: 0,
        }
    }
}

impl LexerLineContext {
    fn new_line(&mut self, new_line_char_span: Span) {
        self.line_number += 1;
        self.line_char_index = new_line_char_span.end;
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
