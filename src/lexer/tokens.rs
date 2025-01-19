use super::*;
use crate::ast::Operator;
use logos::Logos;

macro_rules! tokens {
    ($($name:ident $(/$regex:literal/)? $($token:literal)?)*) => {
        #[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
        #[logos(skip r" +")]
        #[logos(extras = LexerContext)]
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

    BracketOpen "("
    BracketClose ")"

    If "if"
    Else "else"
    While "while"

    Assign "="
    AddAssign "+="
    SubAssign "-="

    Add "+"
    Sub "-"
    Mul "*"
    Div "/"
    Mod "mod"

    Eq "=="
    Ne "!="
    Lt "<"
    Le "<="
    Gt ">"
    Ge ">="

    And "and"
    Or "or"
    XOr "xor"

    Identifier /"[a-zA-Z_]+"/
    Integer /"[0-9]+"/
    True "true"
    False "false"

    NewLine /"\n *"/
}

macro_rules! into_operator {
    ($($operator:ident)*) => {
        impl Token {
            pub fn get_operator(&self) -> Option<Operator> {
                match self {
                    $(Token::$operator => Some(Operator::$operator),)*
                    _ => None,
                }
            }
        }
    };
}

into_operator! {
    Add Sub Mul Div Mod
    Eq Ne Lt Le Gt Ge
    And Or XOr
}
