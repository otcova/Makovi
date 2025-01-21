use std::fmt::Display;

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operator {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Boolean
    And,
    Or,
    XOr,
}

impl Operator {
    pub fn priority(&self) -> u8 {
        use Operator::*;
        match self {
            Mul | Div | Mod => 4,
            Add | Sub => 3,
            Eq | Ne | Lt | Le | Gt | Ge => 2,
            And => 1,
            Or | XOr => 0,
        }
    }

    pub fn is_comparison(&self) -> bool {
        use Operator::*;
        matches!(self, Eq | Lt | Le | Gt | Ge)
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
            Operator::And => write!(f, "and"),
            Operator::Or => write!(f, "or"),
            Operator::XOr => write!(f, "xor"),
        }
    }
}
