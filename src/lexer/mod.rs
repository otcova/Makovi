mod token_match_macros;
mod tokens;

use crate::error::*;
use logos::{Logos, Span};
pub(crate) use token_match_macros::*;
pub use tokens::*;

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token<'a>>,
    token: TokenResult<'a>,
}

pub type TokenResult<'a> = (Option<Result<Token<'a>, &'a str>>, LineSpan);

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Self {
            token: (None, LineSpan::default()),
            lexer: Token::lexer(source),
        };
        lexer.next();
        lexer
    }

    pub fn next(&mut self) -> TokenResult<'a> {
        let token = self.lexer.next();
        let span = self.lexer.extras.span(self.lexer.span());

        let next_token = match token {
            // Advance line
            Some(Ok(Token::NewLine)) => {
                self.lexer.extras.new_line(self.lexer.span());
                Some(Ok(Token::NewLine))
            }

            // Give the unrecognized token as the error type
            Some(Err(())) => Some(Err(self.lexer.slice())),

            Some(Ok(token)) => Some(Ok(token)),
            None => None,
        };
        std::mem::replace(&mut self.token, (next_token, span))
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
