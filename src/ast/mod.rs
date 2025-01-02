use std::{cell::RefCell, fmt::Display};

#[derive(Debug, Copy, Clone)]
pub enum Expr<'a> {
    Literal(&'a str),
    Identifier(&'a str),
    IdentifierDefinition(&'a str),
    Assign(&'a str, ExprPtr),
    Eq(ExprPtr, ExprPtr),
    Ne(ExprPtr, ExprPtr),
    Lt(ExprPtr, ExprPtr),
    Le(ExprPtr, ExprPtr),
    Gt(ExprPtr, ExprPtr),
    Ge(ExprPtr, ExprPtr),
    Add(ExprPtr, ExprPtr),
    Sub(ExprPtr, ExprPtr),
    Mul(ExprPtr, ExprPtr),
    Div(ExprPtr, ExprPtr),
    Mod(ExprPtr, ExprPtr),
    /// (if_condition, if_body, else_body)
    IfElse(ExprPtr, ExprVecPtr, ExprVecPtr),
    /// (if_condition, if_body, if_else_expresion)
    IfElseIf(ExprPtr, ExprVecPtr, ExprPtr),
    WhileLoop(ExprPtr, ExprVecPtr),
    Call(&'a str, ExprVecPtr),
    GlobalDataAddr(&'a str),
    /// (function_name, parameters, return_name)
    Function(&'a str, ExprPtr, ExprPtr, ExprPtr),

    /// Linked lists: (line, next_line)
    Statements(ExprPtr, ExprPtr),
    Parameters(ExprPtr, ExprPtr),
    ParametersDefinition(ExprPtr, ExprPtr),
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub nodes: RefCell<Vec<Expr<'a>>>,
}

impl Default for Ast<'_> {
    fn default() -> Self {
        Self {
            nodes: RefCell::new(Vec::with_capacity(256)),
        }
    }
}

pub type ExprPtr = u32;
pub type ExprVecPtr = u32;
pub const NULL_EXPR_PTR: ExprPtr = ExprPtr::MAX;

impl<'a> Ast<'a> {
    pub fn push(&'a self, expr: Expr<'a>) -> ExprPtr {
        let mut data = self.nodes.borrow_mut();
        data.push(expr);
        (data.len() - 1) as ExprPtr
    }

    pub fn push_all(&self, exprs: &[Expr<'a>]) -> ExprVecPtr {
        self.nodes.borrow_mut().extend_from_slice(exprs);
        exprs.len() as ExprPtr
    }

    pub fn push_vec(&self, mut exprs: Vec<Expr<'a>>) -> ExprVecPtr {
        let len = exprs.len();

        self.nodes.borrow_mut().append(&mut exprs);
        len as ExprPtr
    }

    pub fn clear(&self) {
        self.nodes.borrow_mut().clear();
    }

    pub fn get(&self, index: ExprPtr) -> Expr<'a> {
        self.nodes.borrow()[index as usize]
    }

    fn print_ast(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        expr: ExprPtr,
        indent: usize,
        start_with_prefix: bool,
    ) -> std::fmt::Result {
        if expr == NULL_EXPR_PTR {
            return Ok(());
        }

        let ind = &"| ".repeat(indent);
        let prefix = if start_with_prefix { ind } else { "" };

        match self.get(expr) {
            Expr::Function(name, parameters, return_name, body) => {
                write!(f, "{prefix}function {name}(")?;
                self.print_ast(f, parameters, indent + 2, false)?;
                write!(f, ") -> ")?;
                self.print_ast(f, return_name, indent + 2, false)?;
                writeln!(f)?;
                self.print_ast(f, body, indent + 1, true)?;
            }
            Expr::IfElse(condition, body, else_body) => {
                write!(f, "{prefix}if ")?;
                self.print_ast(f, condition, indent + 1, false)?;
                writeln!(f, "{ind}then")?;
                self.print_ast(f, body, indent + 1, true)?;
                writeln!(f, "{ind}else")?;
                self.print_ast(f, else_body, indent + 1, true)?;
            }
            Expr::IfElseIf(condition, body, else_body) => {
                write!(f, "{prefix}if ")?;
                self.print_ast(f, condition, indent + 1, false)?;
                writeln!(f, "{ind}then")?;
                self.print_ast(f, body, indent + 1, true)?;
                write!(f, "{ind}else ")?;
                self.print_ast(f, else_body, indent, false)?;
            }
            Expr::WhileLoop(condition, body) => {
                write!(f, "{prefix}while ")?;
                self.print_ast(f, condition, indent + 1, false)?;
                writeln!(f, "{ind}then")?;
                self.print_ast(f, body, indent + 1, true)?;
            }
            Expr::ParametersDefinition(current, next) => {
                self.print_ast(f, current, indent, start_with_prefix)?;
                if next != NULL_EXPR_PTR {
                    write!(f, ", ")?;
                    self.print_ast(f, next, indent, false)?;
                }
            }
            Expr::Parameters(current, next) | Expr::Statements(current, next) => {
                self.print_ast(f, current, indent, true)?;
                self.print_ast(f, next, indent, true)?;
            }
            Expr::Assign(name, expr) => {
                write!(f, "{prefix}{name} = ")?;
                self.print_ast(f, expr, indent + 1, false)?;
            }
            Expr::Call(fn_name, args) => {
                writeln!(f, "{prefix}{fn_name}(...)")?;
                self.print_ast(f, args, indent + 1, true)?;
            }
            Expr::IdentifierDefinition(name) => {
                write!(f, r#"{prefix}"{name}""#)?;
                if start_with_prefix {
                    writeln!(f)?;
                }
            }
            Expr::Literal(..) | Expr::Identifier(..) | Expr::GlobalDataAddr(..) => {
                writeln!(f, "{prefix}{:?}", self.get(expr))?;
            }
            Expr::Eq(a, b)
            | Expr::Ne(a, b)
            | Expr::Lt(a, b)
            | Expr::Le(a, b)
            | Expr::Gt(a, b)
            | Expr::Ge(a, b)
            | Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Mod(a, b) => {
                let name = format!("{:?}", self.get(expr));
                writeln!(f, "{prefix}{}", name.split('(').next().unwrap())?;
                write!(f, "{ind}(Lhs) ")?;
                self.print_ast(f, a, indent + 1, false)?;
                write!(f, "{ind}(Rhs) ")?;
                self.print_ast(f, b, indent + 1, false)?;
            }
        };

        Ok(())
    }
}

impl Display for Ast<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last = (self.nodes.borrow().len() - 1) as ExprPtr;
        self.print_ast(f, last, 0, true)
    }
}
