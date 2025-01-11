use super::*;
use logos::Logos;

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
