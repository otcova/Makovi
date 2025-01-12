use super::*;

impl std::fmt::Display for Ast<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = (self.nodes.len() - 1) as ExprPtr;
        self.print_ast(f, root, 0, true)
    }
}

impl Ast<'_> {
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

        let ind = &"│ ".repeat(indent);
        let prefix = if start_with_prefix { ind } else { "" };

        match self[expr] {
            Expr::Function {
                name,
                parameters,
                body,
            } => {
                write!(f, "{prefix}function {name}(")?;
                self.print_ast(f, parameters, indent + 2, false)?;
                writeln!(f, ")")?;
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
                if else_body != NULL_EXPR_PTR {
                    if let Expr::IfElse { .. } = self[else_body] {
                        write!(f, "{ind}else ")?;
                        self.print_ast(f, else_body, indent, false)?;
                    } else {
                        writeln!(f, "{ind}else")?;
                        self.print_ast(f, else_body, indent + 1, true)?;
                    }
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
            Expr::Return(value) => {
                write!(f, "{prefix}return ")?;
                self.print_ast(f, value, indent + 1, false)?;
            }
            Expr::Parameters(current, next) => {
                write!(f, "{prefix}(Parameter) ")?;
                self.print_ast(f, current, indent, false)?;
                self.print_ast(f, next, indent, true)?;
            }
            Expr::Statements(current, next) => {
                write!(f, "{prefix}(Statement) ")?;
                self.print_ast(f, current, indent, false)?;
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
            Expr::VariableDefinition(name) => {
                write!(f, r#"{prefix}"{name}""#)?;
                if start_with_prefix {
                    writeln!(f)?;
                }
            }
            Expr::Integer(..) | Expr::Variable(..) => {
                writeln!(f, "{prefix}{:?}", self[expr])?;
            }
            Expr::Operator(operator, lhs, rhs) => {
                writeln!(f, "{prefix}lhs {operator} rhs")?;
                write!(f, "{ind}(lhs) ")?;
                self.print_ast(f, lhs, indent + 1, false)?;
                write!(f, "{ind}(rhs) ")?;
                self.print_ast(f, rhs, indent + 1, false)?;
            }
        };

        Ok(())
    }
}
