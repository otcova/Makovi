mod display;
mod operator;
mod statement;

pub use operator::*;
pub use statement::*;

use std::{
    mem,
    ops::{Index, IndexMut},
};

#[derive(Debug, Copy, Clone)]
pub enum Expr<'a> {
    Integer(&'a str),
    Bool(bool),
    Variable(&'a str),
    VariableDefinition(&'a str),
    Return(ExprPtr),
    Assign(&'a str, ExprPtr),

    /// # Example
    /// This expression
    /// `1 < 2 + 3 <= 4 + 5 + 6`
    /// Is represented with the following
    /// `1 < (2 + 3) -> <= (4 + 5 -> + 6)`
    /// Which is equivalent to
    /// `Head[1 < Head[2 + 3 -> null]  ->  Op(<= Head[4 + 5  ->  Op(+ 6 -> null)]  ->  null)]`
    HeadOperation {
        lhs: ExprPtr,
        operator: Operator,
        rhs: ExprPtr,
        next: ExprPtr,
    },
    /// See `HeadOperation` variant
    Operation {
        operator: Operator,
        rhs: ExprPtr,
        next: ExprPtr,
    },
    IfElse {
        condition: ExprPtr,
        then_body: ExprPtr,
        else_body: ExprPtr,
    },
    WhileLoop {
        condition: ExprPtr,
        body: ExprPtr,
    },
    Call(&'a str, ExprPtr),
    Function {
        name: &'a str,
        parameters: ExprPtr,
        body: ExprPtr,
    },

    /// Linked lists: (line, next_line)
    Statements(ExprPtr, ExprPtr),
    Parameters(ExprPtr, ExprPtr),
    ParametersDefinition(ExprPtr, ExprPtr),
}

pub struct AstContext {
    nodes_buffer: Vec<Expr<'static>>,
}

pub struct Ast<'c> {
    context: &'c mut AstContext,
    nodes: Vec<Expr<'c>>,
}

pub type ExprPtr = usize;
pub const NULL_EXPR_PTR: ExprPtr = ExprPtr::MAX;

struct AstList<'a> {
    ast: &'a Ast<'a>,
    node: ExprPtr,
}

impl<'a> Iterator for AstList<'a> {
    type Item = Expr<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.node == NULL_EXPR_PTR {
            return None;
        }

        match self.ast[self.node] {
            Expr::Statements(expr, next)
            | Expr::Parameters(expr, next)
            | Expr::ParametersDefinition(expr, next) => {
                self.node = next;
                Some(self.ast[expr])
            }
            _ => None,
        }
    }
}

impl Default for AstContext {
    fn default() -> Self {
        Self {
            nodes_buffer: Vec::with_capacity(256),
        }
    }
}

impl AstContext {
    pub fn create_ast(&mut self) -> Ast {
        Ast {
            nodes: mem::take(&mut self.nodes_buffer),
            context: self,
        }
    }
}

impl Drop for Ast<'_> {
    fn drop(&mut self) {
        self.context.nodes_buffer = {
            self.nodes.clear();
            let (ptr, _, cap) = mem::take(&mut self.nodes).into_raw_parts();

            // The cast is necessary to change the lifetime
            #[allow(clippy::unnecessary_cast)]
            let ptr = ptr as *mut Expr<'static>;

            // SAFETY:
            // - `cap` is valid because comes from `Vec::into_parts_with_alloc`
            // - `ptr` is valid because comes from `Vec::<U, Global>::into_parts_with_alloc` where
            // U has the same type with diferent lifetime. And because the vector has been cleared, the lifetime does not matter.
            // - `length` is valid size it's zero after the `vec.clear`
            unsafe { Vec::from_raw_parts(ptr, 0, cap) }
        }
    }
}

impl<'c> Ast<'c> {
    pub fn push(&mut self, expr: Expr<'c>) -> ExprPtr {
        self.nodes.push(expr);
        (self.nodes.len() - 1) as ExprPtr
    }

    pub fn root(&self) -> Option<Expr<'c>> {
        self.nodes.last().copied()
    }

    pub fn iter_list(&'c self, node: ExprPtr) -> impl Iterator<Item = Expr<'c>> {
        AstList { ast: self, node }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}

impl<'c> Index<ExprPtr> for Ast<'c> {
    type Output = Expr<'c>;
    fn index(&self, index: ExprPtr) -> &Self::Output {
        #[allow(clippy::unnecessary_cast)]
        &self.nodes[index as usize]
    }
}

impl IndexMut<ExprPtr> for Ast<'_> {
    fn index_mut(&mut self, index: ExprPtr) -> &mut Self::Output {
        #[allow(clippy::unnecessary_cast)]
        &mut self.nodes[index as usize]
    }
}
