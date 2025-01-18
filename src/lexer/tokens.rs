use super::*;
use crate::ast::Operator;
use logos::Logos;

macro_rules! tokens {
    ($($name:ident $(/$regex:literal/)? $($token:literal)?)*) => {
        #[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
        #[logos(skip r" ")]
        #[logos(extras = LexerLineContext)]
        pub enum Token {
            $(
                $(#[regex($regex)])?
                $(#[token($token)])?
                $name,
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

    Identifier /"[a-zA-Z_]+"/
    Integer /"[0-9]+"/

    NewLine "\n"
}

impl Token {
    pub fn get_operator(&self) -> Option<Operator> {
        match self {
            Token::Mul => Some(Operator::Mul),
            Token::Div => Some(Operator::Div),
            Token::Plus => Some(Operator::Add),
            Token::Minus => Some(Operator::Sub),
            Token::Mod => Some(Operator::Mod),
            Token::Eq => Some(Operator::Eq),
            Token::Ne => Some(Operator::Ne),
            Token::Lt => Some(Operator::Lt),
            Token::Le => Some(Operator::Le),
            Token::Gt => Some(Operator::Gt),
            Token::Ge => Some(Operator::Ge),
            _ => None,
        }
    }
}
