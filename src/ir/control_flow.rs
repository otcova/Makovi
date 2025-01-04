use super::*;

impl<'a, M: Module> FunctionTranslator<'a, '_, M> {
    pub fn translate(&mut self, expr: ExprPtr) -> ExprValue {
        if expr == NULL_EXPR_PTR {
            ExprValue::Null
        } else {
            self.translate_expr(self.ast.get(expr))
        }
    }

    fn translate_expr(&mut self, expr: Expr<'a>) -> ExprValue {
        match expr {
            Expr::Literal(literal) => self.literal(literal),
            Expr::Identifier(name) => self.identifier(name),
            Expr::Assign(name, value) => {
                let value = self.translate(value);
                self.assign(name, value)
            }
            Expr::Eq(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.eq(l, r)),
            Expr::Ne(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.ne(l, r)),
            Expr::Lt(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.lt(l, r)),
            Expr::Le(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.le(l, r)),
            Expr::Gt(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.gt(l, r)),
            Expr::Ge(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.ge(l, r)),
            Expr::Add(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.add(l, r)),
            Expr::Sub(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.sub(l, r)),
            Expr::Mul(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.mul(l, r)),
            Expr::Div(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.div(l, r)),
            Expr::Mod(lhs, rhs) => self.operator(lhs, rhs, |s, l, r| s.module(l, r)),
            Expr::IfElse {
                condition,
                then_body,
                else_body,
            } => {
                let condition = self.translate(condition);
                self.if_else(
                    condition,
                    |s| s.translate(then_body),
                    |s| s.translate(else_body),
                )
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
            Expr::IdentifierDefinition(..)
            | Expr::Parameters(..)
            | Expr::ParametersDefinition(..) => {
                unreachable!()
            }
        }
    }

    fn operator<F>(&mut self, a: ExprPtr, b: ExprPtr, translation: F) -> ExprValue
    where
        F: FnOnce(&mut Self, ExprValue, ExprValue) -> ExprValue,
    {
        let a = self.translate(a);
        let b = self.translate(b);
        translation(self, a, b)
    }
}
