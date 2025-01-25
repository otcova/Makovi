use super::*;
use std::fmt;

impl std::fmt::Display for Ast<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = (self.nodes.len() - 1) as ExprPtr;
        self.print(f, root)
    }
}

impl Ast<'_> {
    pub fn print<F: fmt::Write>(&self, f: &mut F, expr: ExprPtr) -> fmt::Result {
        self.print_node(f, expr, 0, true)
    }

    fn print_node<F: fmt::Write>(
        &self,
        f: &mut F,
        expr: ExprPtr,
        indent: usize,
        start_with_prefix: bool,
    ) -> std::fmt::Result {
        if expr == NULL_EXPR_PTR {
            return Ok(());
        }

        let ind = &"│ ".repeat(indent);
        let prefix = if start_with_prefix { ind } else { "" };

        match self[expr] {
            Expr::Function {
                name,
                parameters,
                body,
            } => {
                write!(f, "{prefix}function {name}(")?;
                self.print_node(f, parameters, indent + 2, false)?;
                writeln!(f, ")")?;
                self.print_node(f, body, indent + 1, true)?;
            }
            Expr::IfElse {
                condition,
                then_body,
                else_body,
            } => {
                write!(f, "{prefix}if ")?;
                self.print_node(f, condition, indent + 1, false)?;
                writeln!(f, "{ind}then")?;
                self.print_node(f, then_body, indent + 1, true)?;
                if else_body != NULL_EXPR_PTR {
                    if let Expr::IfElse { .. } = self[else_body] {
                        write!(f, "{ind}else ")?;
                        self.print_node(f, else_body, indent, false)?;
                    } else {
                        writeln!(f, "{ind}else")?;
                        self.print_node(f, else_body, indent + 1, true)?;
                    }
                }
            }
            Expr::WhileLoop { condition, body } => {
                write!(f, "{prefix}while ")?;
                self.print_node(f, condition, indent + 1, false)?;
                writeln!(f, "{ind}then")?;
                self.print_node(f, body, indent + 1, true)?;
            }
            Expr::ParametersDefinition(current, next) => {
                self.print_node(f, current, indent, start_with_prefix)?;
                if next != NULL_EXPR_PTR {
                    write!(f, ", ")?;
                    self.print_node(f, next, indent, false)?;
                }
            }
            Expr::Return(value) => {
                write!(f, "{prefix}return ")?;
                self.print_node(f, value, indent + 1, false)?;
            }
            Expr::Parameters(current, next) => {
                write!(f, "{prefix}(Parameter) ")?;
                self.print_node(f, current, indent, false)?;
                self.print_node(f, next, indent, true)?;
            }
            Expr::Statements(current, next) => {
                write!(f, "{prefix}(Statement) ")?;
                self.print_node(f, current, indent, false)?;
                self.print_node(f, next, indent, true)?;
            }
            Expr::Assign(name, expr) => {
                write!(f, "{prefix}{name} = ")?;
                self.print_node(f, expr, indent + 1, false)?;
            }
            Expr::Call(fn_name, args) => {
                writeln!(f, "{prefix}{fn_name}(...)")?;
                self.print_node(f, args, indent + 1, true)?;
            }
            Expr::VariableDefinition(name) => {
                write!(f, r#"{prefix}"{name}""#)?;
                if start_with_prefix {
                    writeln!(f)?;
                }
            }
            Expr::Bool(..) | Expr::Integer(..) | Expr::Variable(..) => {
                writeln!(f, "{prefix}{:?}", self[expr])?;
            }
            Expr::HeadOperation {
                lhs,
                operator,
                rhs,
                next,
            } => {
                if next == NULL_EXPR_PTR {
                    writeln!(f, "{prefix}lhs {operator} rhs")?;
                    write!(f, "{ind}(lhs) ")?;
                    self.print_node(f, lhs, indent + 1, false)?;
                    write!(f, "{ind}(rhs) ")?;
                    self.print_node(f, rhs, indent + 1, false)?;
                } else {
                    writeln!(f, "{prefix}lhs {operator} rhs next")?;
                    write!(f, "{ind}(lhs) ")?;
                    self.print_node(f, lhs, indent + 1, false)?;
                    write!(f, "{ind}(rhs) ")?;
                    self.print_node(f, rhs, indent + 1, false)?;
                    write!(f, "{ind}(next) ")?;
                    self.print_node(f, next, indent + 1, false)?;
                }
            }
            Expr::Operation {
                operator,
                rhs,
                next,
            } => {
                if next == NULL_EXPR_PTR {
                    writeln!(f, "{prefix}{operator} rhs")?;
                    write!(f, "{ind}(rhs) ")?;
                    self.print_node(f, rhs, indent + 1, false)?;
                } else {
                    writeln!(f, "{prefix}{operator} rhs next")?;
                    write!(f, "{ind}(rhs) ")?;
                    self.print_node(f, rhs, indent + 1, false)?;
                    write!(f, "{ind}(next) ")?;
                    self.print_node(f, next, indent + 1, false)?;
                }
            }
        };

        Ok(())
    }
}
