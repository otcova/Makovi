use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum ExprAst<'a> {
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

pub struct FunctionAst<'a> {
    pub name: &'a str,
    pub params_names: Vec<&'a str>,
    pub return_name: &'a str,
    pub statements: VecExpr<'a>,
}

pub struct ExprArena<'a> {
    data: RefCell<Vec<ExprAst<'a>>>,
}

impl Default for ExprArena<'_> {
    fn default() -> Self {
        Self {
            data: RefCell::new(Vec::with_capacity(256)),
        }
    }
}

impl ExprArena<'_> {
    pub fn clear(&mut self) {
        self.data.borrow_mut().clear();
    }
}

// pub type VecExpr<'a> = smallvec::SmallVec<ExprAst<'a>, 2>;
pub type VecExpr<'a> = Vec<ExprAst<'a>>;

/////////////////////////////////////////////

// type ExprPtr<'a> = Box<ExprAst<'a>>;
// type ExprVecPtr<'a> = Vec<ExprAst<'a>>;
//
// impl<'a> ExprArena<'a> {
//     pub fn push(&self, expr: ExprAst<'a>) -> ExprPtr<'a> {
//         Box::new(expr)
//     }
//
//     pub fn push_vec(&self, expr: Vec<ExprAst<'a>>) -> ExprVecPtr<'a> {
//         expr
//     }
// }

/////////////////////////////////////////////

// type ExprPtr<'a> = u32;
// type ExprVecPtr<'a> = Vec<ExprAst<'a>>;
//
// impl<'a> ExprArena<'a> {
//     pub fn push(&self, expr: ExprAst<'a>) -> ExprPtr<'a> {
//         let mut data = self.data.borrow_mut();
//         data.push(expr);
//         (data.len() - 1) as ExprPtr
//     }
//
//     pub fn push_vec(&self, expr: Vec<ExprAst<'a>>) -> ExprVecPtr<'a> {
//         expr
//     }
// }

/////////////////////////////////////////////

type ExprPtr<'a> = u32;
type ExprVecPtr<'a> = u32;

impl<'a> ExprArena<'a> {
    pub fn push(&self, expr: ExprAst<'a>) -> ExprPtr<'a> {
        let mut data = self.data.borrow_mut();
        data.push(expr);
        (data.len() - 1) as ExprPtr
    }

    pub fn push_all(&self, mut exprs: &[ExprAst<'a>]) -> ExprVecPtr<'a> {
        let len = exprs.len();

        self.data.borrow_mut().extend_from_slice(&mut exprs);
        len as ExprPtr
    }

    pub fn push_vec(&self, mut exprs: Vec<ExprAst<'a>>) -> ExprVecPtr<'a> {
        let len = exprs.len();

        self.data.borrow_mut().append(&mut exprs);
        len as ExprPtr
    }

    // fn get(&self, index: ExprPtr) -> ExprAst {
    //     self.data.borrow()[index as usize].clone()
    // }
}
