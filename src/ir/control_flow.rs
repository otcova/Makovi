use super::*;

impl<M: Module> FunctionTranslator<'_, '_, M> {
    pub fn translate(&mut self, expr: ExprPtr) -> ExprValue {
        if expr == NULL_EXPR_PTR {
            None
        } else {
            self.translate_expr(self.ast.get(expr))
        }
    }

    fn translate_expr(&mut self, expr: Expr) -> ExprValue {
        match expr {
            Expr::Literal(literal) => self.literal(literal),
            Expr::Identifier(name) => self.identifier(name),
            Expr::IdentifierDefinition(_) => unreachable!(),
            Expr::Assign(name, value) => self.eval1(value, |s, v| s.assign(name, v)),
            Expr::Eq(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.eq(l, r)),
            Expr::Ne(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.ne(l, r)),
            Expr::Lt(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.lt(l, r)),
            Expr::Le(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.le(l, r)),
            Expr::Gt(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.gt(l, r)),
            Expr::Ge(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.ge(l, r)),
            Expr::Add(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.add(l, r)),
            Expr::Sub(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.sub(l, r)),
            Expr::Mul(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.mul(l, r)),
            Expr::Div(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.div(l, r)),
            Expr::Mod(lhs, rhs) => self.eval2(lhs, rhs, |s, l, r| s.module(l, r)),
            Expr::IfElse(condition, if_block, else_block) => {
                let condition = self.translate(condition);
                self.if_else(
                    condition,
                    |s| s.translate(if_block),
                    |s| s.translate(else_block),
                )
            }
            Expr::WhileLoop(condition, body) => {
                self.while_loop(|s| s.translate(condition), |s| s.translate(body));
                None
            }
            Expr::Call(name, parameters) => {
                let parameters = self
                    .ast
                    .iter_list(parameters)
                    .map(|p| self.translate_expr(p));
                let parameters = Self::prepare_parameters(parameters);
                self.call(name, &parameters)
            }
            Expr::GlobalDataAddr(_) => todo!(),
            Expr::Function(name, parameters, return_def, body) => unreachable!(),
            Expr::Statements(expr, next) => {
                if next == NULL_EXPR_PTR {
                    self.translate(expr)
                } else {
                    self.translate(expr);
                    self.translate(next)
                }
            }
            Expr::Parameters(parameter, next) => unreachable!(),
            Expr::ParametersDefinition(parameter, next) => unreachable!(),
        }
    }

    fn eval1<F>(&mut self, a: ExprPtr, translation: F) -> ExprValue
    where
        F: Fn(&mut Self, ExprValue) -> ExprValue,
    {
        let a = self.translate(a);
        translation(self, a)
    }

    fn eval2<F>(&mut self, a: ExprPtr, b: ExprPtr, translation: F) -> ExprValue
    where
        F: Fn(&mut Self, ExprValue, ExprValue) -> ExprValue,
    {
        let a = self.translate(a);
        let b = self.translate(b);
        translation(self, a, b)
    }
}
