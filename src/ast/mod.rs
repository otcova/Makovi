use std::cell::RefCell;

#[derive(Debug, Clone)]
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

    // fn get(&self, index: ExprPtr) -> Expr {
    //     self.data.borrow()[index as usize].clone()
    // }
}
