use std::{fmt::Display, mem};

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
    IfElse {
        condition: ExprPtr,
        then_body: ExprVecPtr,
        else_body: ExprVecPtr,
    },
    WhileLoop {
        condition: ExprPtr,
        body: ExprVecPtr,
    },
    Call(&'a str, ExprVecPtr),
    Function {
        name: &'a str,
        parameters: ExprPtr,
        return_name: ExprPtr,
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

pub type ExprPtr = u32;
pub type ExprVecPtr = u32;
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

        match self.ast.get(self.node) {
            Expr::Statements(expr, next)
            | Expr::Parameters(expr, next)
            | Expr::ParametersDefinition(expr, next) => {
                self.node = next;
                Some(self.ast.get(expr))
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

    pub fn get(&self, index: ExprPtr) -> Expr<'c> {
        self.nodes[index as usize]
    }

    pub fn root(&self) -> Option<Expr<'c>> {
        self.nodes.last().copied()
    }

    pub fn iter_list(&'c self, node: ExprPtr) -> impl Iterator<Item = Expr<'c>> {
        AstList { ast: self, node }
    }

    pub fn iter_nodes<'r>(&'r self) -> impl Iterator<Item = Expr<'c>> + 'r {
        self.nodes.iter().copied()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.nodes.len()
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
            Expr::Function {
                name,
                parameters,
                return_name,
                body,
            } => {
                write!(f, "{prefix}function {name}(")?;
                self.print_ast(f, parameters, indent + 2, false)?;
                write!(f, ") -> ")?;
                self.print_ast(f, return_name, indent + 2, false)?;
                writeln!(f)?;
                self.print_ast(f, body, indent + 1, true)?;
            }
            Expr::IfElse {
                condition,
                then_body,
                else_body,
            } => {
                write!(f, "{prefix}if ")?;
                self.print_ast(f, condition, indent + 1, false)?;
                writeln!(f, "{ind}then")?;
                self.print_ast(f, then_body, indent + 1, true)?;
                if let Expr::IfElse { .. } = self.get(else_body) {
                    write!(f, "{ind}else ")?;
                    self.print_ast(f, else_body, indent, false)?;
                } else {
                    writeln!(f, "{ind}else")?;
                    self.print_ast(f, else_body, indent + 1, true)?;
                }
            }
            Expr::WhileLoop { condition, body } => {
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
            Expr::Literal(..) | Expr::Identifier(..) => {
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
        let root = (self.nodes.len() - 1) as ExprPtr;
        self.print_ast(f, root, 0, true)
    }
}
