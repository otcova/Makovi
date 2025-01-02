use std::{cell::RefCell, fmt::Display, ops::Index};

#[derive(Debug, Copy, Clone)]
pub enum Expr<'a> {
    Literal(&'a str),
    Identifier(&'a str),
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

    /// Linked list: (line, next_line)
    Statements(ExprPtr, ExprPtr),
}

#[derive(Debug)]
pub struct FunctionExpr<'a> {
    pub name: &'a str,
    pub params_names: Vec<&'a str>,
    pub return_name: &'a str,
    pub statements: &'a Ast<'a>,
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

// pub type VecExpr<'a> = smallvec::SmallVec<Expr<'a>, 2>;
pub type VecExpr<'a> = Vec<Expr<'a>>;

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

    fn get(&self, index: ExprPtr) -> Expr<'a> {
        self.nodes.borrow()[index as usize]
    }

    fn print_ast(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        expr: ExprPtr,
        indent: usize,
    ) -> std::fmt::Result {
        if expr == NULL_EXPR_PTR {
            return Ok(());
        }

        let i = "| ".repeat(indent);

        match self.get(expr) {
            Expr::IfElse(condition, body, else_body) => {
                writeln!(f, "{i}if")?;
                self.print_ast(f, condition, indent + 1)?;
                writeln!(f, "{i}then")?;
                self.print_ast(f, body, indent + 1)?;
                writeln!(f, "{i}else")?;
                self.print_ast(f, else_body, indent + 1)?;
            }
            Expr::IfElseIf(condition, body, else_body) => {
                writeln!(f, "{i}if")?;
                self.print_ast(f, condition, indent + 1)?;
                writeln!(f, "{i}then")?;
                self.print_ast(f, body, indent + 1)?;
                writeln!(f, "{i}else")?;
                self.print_ast(f, else_body, indent)?;
            }
            // Code Block
            Expr::Statements(current, next) => {
                self.print_ast(f, current, indent)?;
                self.print_ast(f, next, indent)?;
            }
            // 0 childs
            Expr::Literal(..) | Expr::Identifier(..) | Expr::GlobalDataAddr(..) => {
                writeln!(f, "{i}{:?}", self.get(expr))?;
            }
            // 1 child
            Expr::Assign(name, expr) => {
                writeln!(f, "{i}{name} = ")?;
                self.print_ast(f, expr, indent + 1)?;
            }
            // 1 vec child
            Expr::Call(fn_name, args) => {
                writeln!(f, "{i}{fn_name}(...)")?;
            }
            // 2 childs
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
            | Expr::Mod(a, b)
            | Expr::WhileLoop(a, b) => {
                writeln!(f, "{i}{:?}", self.get(expr))?;
                writeln!(f, "{i}(A)")?;
                self.print_ast(f, a, indent + 1)?;
                writeln!(f, "{i}(B)")?;
                self.print_ast(f, b, indent + 1)?;
            }
        };

        Ok(())
    }
}

impl Display for Ast<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last = (self.nodes.borrow().len() - 1) as ExprPtr;
        self.print_ast(f, last, 0)
    }
}
