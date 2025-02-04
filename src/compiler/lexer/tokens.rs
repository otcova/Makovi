use super::*;
use logos::Logos;

macro_rules! tokens {
    ($($name:ident $(/$regex:literal/)? $($token:literal)?)*) => {
        #[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
        #[logos(skip r" +")]
        #[logos(extras = LexerContext)]
        #[repr(usize)]
        pub enum Token {
            $(
                $(#[regex($regex)])?
                $(#[token($token)])?
                $name,
            )*

            /// The found slice doesn't match any restriction.
            Invalid,
            /// All sources end with this token.
            EndOfFile,
        }

        impl Token {
            pub fn get_str(&self) -> &'static str {
                match self {
                    $(
                        Token::$name =>
                            $(if true { stringify!($name) } else { $regex },)?
                            $($token,)?
                    )*

                    Token::Invalid => "<invalid token>",
                    Token::EndOfFile => "end of file",
                }
            }
        }
    };
}

tokens! {
    Fn "fn"
    Return "return"

    Comma ","

    BracketOpen "("
    BracketClose ")"

    If "if"
    Else "else"
    While "while"

    Let "let"

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

// macro_rules! into_operator {
//     ($($operator:ident)*) => {
//         impl Token {
//             pub fn get_operator(&self) -> Option<Operator> {
//                 match self {
//                     $(Token::$operator => Some(Operator::$operator),)*
//                     _ => None,
//                 }
//             }
//         }
//     };
// }
//
// into_operator! {
//     Add Sub Mul Div Mod
//     Eq Ne Lt Le Gt Ge
//     And Or XOr
// }

pub type OperatorPriority = usize;

impl Token {
    pub fn operator_priority(&self) -> Option<OperatorPriority> {
        use Token::*;
        match self {
            Mul | Div | Mod => Some(4),
            Add | Sub => Some(3),
            Eq | Ne | Lt | Le | Gt | Ge => Some(2),
            And => Some(1),
            Or | XOr => Some(0),
            _ => None,
        }
    }

    pub fn is_comparison(&self) -> bool {
        use Token::*;
        matches!(self, Eq | Lt | Le | Gt | Ge)
    }
}
