use super::*;
use logos::Logos;

macro_rules! tokens {
    ($($name:ident $(/$regex:literal/)? $($token:literal)?)*) => {
        #[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
        #[logos(skip r"[ \t\f]+")]
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
