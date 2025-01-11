use super::*;

enum Statement<'a> {
    Assing(&'a str, Expr<'a>),
    Expr(Expr<'a>),
    If(Expr<'a>),
    Else,
    While(Expr<'a>),
    Function(&'a str, u8),
}
