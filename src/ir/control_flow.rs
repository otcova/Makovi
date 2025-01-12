use super::*;

impl<'a, M: Module> FunctionTranslator<'a, '_, M> {
    pub fn translate(&mut self, expr: ExprPtr) -> ExprValue {
        if expr == NULL_EXPR_PTR {
            ExprValue::Null
        } else {
            self.translate_expr(self.ast[expr])
        }
    }

    fn translate_expr(&mut self, expr: Expr<'a>) -> ExprValue {
        match expr {
            Expr::Integer(literal) => self.literal(literal),
            Expr::Variable(name) => self.identifier(name),
            Expr::Assign(name, value) => {
                let value = self.translate(value);
                self.assign(name, value)
            }
            Expr::Operator(operator, lhs, rhs) => {
                let lhs = self.translate(lhs);
                let rhs = self.translate(rhs);
                match operator {
                    Operator::Eq => self.eq(lhs, rhs),
                    Operator::Ne => self.ne(lhs, rhs),
                    Operator::Lt => self.lt(lhs, rhs),
                    Operator::Le => self.le(lhs, rhs),
                    Operator::Gt => self.gt(lhs, rhs),
                    Operator::Ge => self.ge(lhs, rhs),
                    Operator::Add => self.add(lhs, rhs),
                    Operator::Sub => self.sub(lhs, rhs),
                    Operator::Mul => self.mul(lhs, rhs),
                    Operator::Div => self.div(lhs, rhs),
                    Operator::Mod => self.module(lhs, rhs),
                }
            }
            Expr::IfElse {
                condition,
                then_body,
                else_body,
            } => {
                let condition = self.translate(condition);
                if else_body == NULL_EXPR_PTR {
                    self.if_statement(condition, |s| s.translate(then_body))
                } else {
                    self.if_else(
                        condition,
                        |s| s.translate(then_body),
                        |s| s.translate(else_body),
                    )
                }
            }
            Expr::WhileLoop { condition, body } => {
                self.while_loop(|s| s.translate(condition), |s| s.translate(body));
                ExprValue::Null
            }
            Expr::Call(name, parameters) => {
                let parameters = self
                    .ast
                    .iter_list(parameters)
                    .map(|p| self.translate_expr(p));
                let parameters = Self::prepare_parameters(parameters);
                self.call(name, &parameters)
            }
            Expr::Return(value) => {
                let value = self.translate(value);
                self.function_return(value);
                ExprValue::Unreachable
            }
            Expr::Statements(expr, next) => {
                if next == NULL_EXPR_PTR {
                    self.translate(expr)
                } else {
                    self.translate(expr);
                    self.translate(next)
                }
            }
            Expr::Function { .. } => {
                todo!()
            }
            Expr::VariableDefinition(..)
            | Expr::Parameters(..)
            | Expr::ParametersDefinition(..) => {
                unreachable!()
            }
        }
    }
}
