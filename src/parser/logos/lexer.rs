use logos::{Logos, Span};

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token<'a>>,
    token: Option<TokenResult<'a>>,
}

pub type TokenResult<'a> = Result<Token<'a>, (&'a str, LineColumnNumber)>;

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Self {
            token: None,
            lexer: Token::lexer(source),
        };
        lexer.next();
        lexer
    }

    pub fn next(&mut self) -> Option<TokenResult<'a>> {
        let next_token = match self.lexer.next() {
            Some(Ok(token)) => Some(Ok(token)),
            Some(Err(())) => Some(Err((
                self.lexer.slice(),
                self.lexer.extras.line_column_number(self.lexer.span()),
            ))),
            None => None,
        };
        std::mem::replace(&mut self.token, next_token)
    }

    pub fn peek(&self) -> Option<TokenResult<'a>> {
        self.token
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineColumnNumber {
    pub line: usize,
    pub column: usize,
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
        self.line_char_index += new_line_char_span.end;
    }
    fn line_column_number(&self, span: Span) -> LineColumnNumber {
        LineColumnNumber {
            line: self.line_number,
            column: span.start - self.line_char_index,
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
            #[token("\n", |lexer| lexer.extras.new_line(lexer.span()))]
            NewLine,
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

    Identifier(slice) "[a-zA-Z_]+"
    Integer(slice) "[1-9]+[0-9]*"
}
