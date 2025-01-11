use super::*;
use crate::ast::Operator;
use logos::Logos;

macro_rules! tokens {
    ($($name:ident$((slice) $regex:literal)? $($token:literal)?)*) => {
    #[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
    #[logos(skip r"[ \t\f]+")]
    #[logos(extras = LexerLineContext)]
    pub enum Token<'s> {

        #[token("+", |_| Operator::Add)]
        #[token("-", |_| Operator::Sub)]
        #[token("*", |_| Operator::Mul)]
        #[token("/", |_| Operator::Div)]
        #[token("mod", |_| Operator::Mod)]
        #[token("==", |_| Operator::Eq)]
        #[token("!=", |_| Operator::Ne)]
        #[token("<", |_| Operator::Lt)]
        #[token("<=", |_| Operator::Le)]
        #[token(">", |_| Operator::Gt)]
        #[token(">=", |_| Operator::Ge)]
        Operator(Operator),

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

    Identifier(slice) "[a-zA-Z_]+"
    Integer(slice) "[0-9]+"

    NewLine "\n"
}
