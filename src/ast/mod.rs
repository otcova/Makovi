use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Literal(&'a str),
    Identifier(&'a str),
    Assign(&'a str, ExprPtr<'a>),
    Eq(ExprPtr<'a>, ExprPtr<'a>),
    Ne(ExprPtr<'a>, ExprPtr<'a>),
    Lt(ExprPtr<'a>, ExprPtr<'a>),
    Le(ExprPtr<'a>, ExprPtr<'a>),
    Gt(ExprPtr<'a>, ExprPtr<'a>),
    Ge(ExprPtr<'a>, ExprPtr<'a>),
    Add(ExprPtr<'a>, ExprPtr<'a>),
    Sub(ExprPtr<'a>, ExprPtr<'a>),
    Mul(ExprPtr<'a>, ExprPtr<'a>),
    Div(ExprPtr<'a>, ExprPtr<'a>),
    Mod(ExprPtr<'a>, ExprPtr<'a>),
    /// (if_condition, if_body, else_body)
    IfElse(ExprPtr<'a>, ExprVecPtr<'a>, ExprVecPtr<'a>),
    /// (if_condition, if_body, if_else_expresion)
    IfElseIf(ExprPtr<'a>, ExprVecPtr<'a>, ExprPtr<'a>),
    WhileLoop(ExprPtr<'a>, ExprVecPtr<'a>),
    Call(&'a str, ExprVecPtr<'a>),
    GlobalDataAddr(&'a str),
}

pub struct FunctionExpr<'a> {
    pub name: &'a str,
    pub params_names: Vec<&'a str>,
    pub return_name: &'a str,
    pub statements: &'a Ast<'a>,
}

pub struct Ast<'a> {
    nodes: RefCell<Vec<Expr<'a>>>,
}

impl Default for Ast<'_> {
    fn default() -> Self {
        Self {
            nodes: RefCell::new(Vec::with_capacity(256)),
        }
    }
}

impl Ast<'_> {
    pub fn clear(&mut self) {
        self.nodes.borrow_mut().clear();
    }
}

// pub type VecExpr<'a> = smallvec::SmallVec<Expr<'a>, 2>;
pub type VecExpr<'a> = Vec<Expr<'a>>;

type ExprPtr<'a> = u32;
type ExprVecPtr<'a> = u32;

impl<'a> Ast<'a> {
    pub fn push(&self, expr: Expr<'a>) -> ExprPtr<'a> {
        let mut data = self.nodes.borrow_mut();
        data.push(expr);
        (data.len() - 1) as ExprPtr
    }

    pub fn push_all(&self, exprs: &[Expr<'a>]) -> ExprVecPtr<'a> {
        self.nodes.borrow_mut().extend_from_slice(exprs);
        exprs.len() as ExprPtr
    }

    pub fn push_vec(&self, mut exprs: Vec<Expr<'a>>) -> ExprVecPtr<'a> {
        let len = exprs.len();

        self.nodes.borrow_mut().append(&mut exprs);
        len as ExprPtr
    }

    // fn get(&self, index: ExprPtr) -> Expr {
    //     self.data.borrow()[index as usize].clone()
    // }
}
