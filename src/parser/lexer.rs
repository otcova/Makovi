use logos::{Logos, Span};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineColumnNumber {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LineSpan {
    pub start: LineColumnNumber,
    pub end: LineColumnNumber,
}

impl Default for LineColumnNumber {
    fn default() -> Self {
        Self { line: 1, column: 1 }
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

impl LineSpan {
    pub fn and(self, other: LineSpan) -> LineSpan {
        LineSpan {
            start: self.start.min(other.start),
            end: other.end.max(other.end),
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

macro_rules! tokens {
    ($($name:ident$((slice) $regex:literal)? $($token:literal)?)*) => {
        #[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
        #[logos(skip r"[ \t\f]+")]
        #[logos(extras = LexerLineContext)]
        pub enum Token<'s> {
            $(
                $(
                #[regex($regex, |lexer| lexer.slice())]
                $name(&'s str),
                )?

                $(
                #[token($token)]
                $name,
                )?
            )*
        }
    };
}

tokens! {
    Function "function"
    Return "return"

    Comma ","

    CurlyOpen "{"
    CurlyClose "}"

    BracketOpen "("
    BracketClose ")"

    If "if"
    Else "else"
    While "while"

    Assign "="
    Eq "=="
    Ne "!="
    Lt "<"
    Le "<="
    Gt ">"
    Ge ">="

    Plus "+"
    Minus "-"
    Mul "*"
    Div "/"
    Mod "mod"

    Identifier(slice) "[a-zA-Z_]+"
    Integer(slice) "[0-9]+"

    NewLine "\n"
}
