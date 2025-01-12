use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operator {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Boolean
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Operator {
    pub fn priority(&self) -> u8 {
        use Operator::*;
        match self {
            Mul => 3,
            Div => 3,
            Add => 2,
            Sub => 2,
            Mod => 1,
            Eq => 0,
            Ne => 0,
            Lt => 0,
            Le => 0,
            Gt => 0,
            Ge => 0,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Mod => write!(f, "mod"),
            Operator::Eq => write!(f, "=="),
            Operator::Ne => write!(f, "!="),
            Operator::Lt => write!(f, "<"),
            Operator::Le => write!(f, "<="),
            Operator::Gt => write!(f, ">"),
            Operator::Ge => write!(f, ">="),
        }
    }
}
