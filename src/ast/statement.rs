use super::*;
use crate::error::LineSpan;

#[derive(Debug, Clone, Copy)]
pub enum StatementClass<'a> {
    Declaration(&'a str, ExprPtr),
    AssignOperation(&'a str, Operator, ExprPtr),
    Return(ExprPtr),
    If(ExprPtr),
    ElseIf(ExprPtr),
    Else,
    While(ExprPtr),
    Atom(ExprPtr),
}

#[derive(Debug, Clone, Copy)]
pub struct Statement<'a> {
    pub class: StatementClass<'a>,
    pub nesting: usize,
    pub span: LineSpan,
}
