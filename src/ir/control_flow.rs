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
            Expr::Integer(literal) => self.integer(literal),
            Expr::Bool(bool) => self.bool(bool),
            Expr::Variable(name) => self.variable(name),
            Expr::Assign(name, value) => {
                let value = self.translate(value);
                self.assign(name, value)
            }
            Expr::HeadOperation {
                lhs,
                operator,
                rhs,
                next,
            } => {
                let lhs = self.translate(lhs);
                let rhs = self.translate(rhs);
                let mut prev_result = self.operator(operator, lhs, rhs);

                let is_comparison = operator.is_comparison();

                let mut lhs = rhs;
                let mut next_operation = next;

                while next_operation != NULL_EXPR_PTR {
                    let Expr::Operation {
                        operator,
                        rhs,
                        next,
                    } = self.ast[next_operation]
                    else {
                        unreachable!("HeadOperation.next should only point to Operation or NULL");
                    };

                    let rhs = self.translate(rhs);

                    if is_comparison {
                        let this_comparison = self.operator(operator, lhs, rhs);
                        prev_result = self.operator(Operator::And, prev_result, this_comparison);
                    } else {
                        prev_result = self.operator(operator, prev_result, rhs);
                    }

                    lhs = rhs;
                    next_operation = next;
                }

                prev_result
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
            Expr::Operation { .. }
            | Expr::VariableDefinition(..)
            | Expr::Parameters(..)
            | Expr::ParametersDefinition(..) => {
                unreachable!()
            }
        }
    }
}
