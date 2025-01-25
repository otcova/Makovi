use super::*;

impl<'a> Parser<'a> {
    fn parse_atom(&mut self, token: TokenResult<'a>) -> ParseResult<(ExprPtr, TokenResult<'a>)> {
        Ok(match token.token {
            Some(Identifier) => self.parse_atom_identifyier(token.slice)?,
            Some(Integer) => (
                self.ast.push(Expr::Integer(token.slice)),
                self.next_token()?,
            ),
            Some(True) => (self.ast.push(Expr::Bool(true)), self.next_token()?),
            Some(False) => (self.ast.push(Expr::Bool(false)), self.next_token()?),

            Some(BracketOpen) => {
                let (expr, next) = self.parse_expr()?;
                self.expect(BracketClose, next)?;
                (expr, self.next_token()?)
            }

            // Function Definition
            Some(Function) => {
                self.expect_next(BracketOpen)?;
                let parameters = self.parse_declaration_list(BracketClose)?;

                (
                    self.ast.push(Expr::Function {
                        name: "",
                        parameters,
                        body: NULL_EXPR_PTR,
                    }),
                    self.next_token()?,
                )
            }

            // Errors
            Some(_) => self.unexpected_token("a variable", token)?,
            None => self.eof_error("a variable", token)?,
        })
    }

    pub(super) fn parse_atom_identifyier(
        &mut self,
        ident: &'a str,
    ) -> ParseResult<(ExprPtr, TokenResult<'a>)> {
        let token = self.next_token()?;
        Ok(match token.token {
            Some(BracketOpen) => (self.parse_call(ident)?, self.next_token()?),
            _ => (self.ast.push(Expr::Variable(ident)), token),
        })
    }

    fn parse_call(&mut self, fn_name: &'a str) -> ParseResult<ExprPtr> {
        let params = self.parse_expr_list(BracketClose)?;
        Ok(self.ast.push(Expr::Call(fn_name, params)))
    }

    fn parse_declaration_list(&mut self, end_token: Token) -> ParseResult<ExprPtr> {
        let first_token = self.next_token()?;

        match first_token.token {
            Some(t) if t == end_token => return Ok(NULL_EXPR_PTR),
            Some(Token::Identifier) => {}

            // Errors
            Some(_) => self.unexpected_token(end_token.get_str(), first_token)?,
            None => self.eof_error(end_token.get_str(), first_token)?,
        }

        let variable = self.ast.push(Expr::VariableDefinition(first_token.slice));
        let first_item = self
            .ast
            .push(Expr::ParametersDefinition(variable, NULL_EXPR_PTR));

        let mut comma = self.next_token()?;
        let mut previous_item = first_item;

        while matches!(comma.token, Some(Comma)) {
            let token = self.expect_next(Identifier)?;
            comma = self.next_token()?;

            let variable = self.ast.push(Expr::VariableDefinition(token.slice));
            let item = self
                .ast
                .push(Expr::ParametersDefinition(variable, NULL_EXPR_PTR));

            let Expr::ParametersDefinition(_, next) = &mut self.ast[previous_item] else {
                unreachable!();
            };
            *next = item;
            previous_item = item;
        }

        self.expect(end_token, comma);
        Ok(first_item)
    }

    fn parse_expr_list(&mut self, end_token: Token) -> ParseResult<ExprPtr> {
        let first_token = self.next_token()?;

        match first_token.token {
            Some(t) if t == end_token => return Ok(NULL_EXPR_PTR),
            Some(_) => {}

            // Errors
            None => self.eof_error(end_token.get_str(), first_token)?,
        }

        let (mut expr, mut comma) = self.parse_expr_with_token(first_token)?;
        let first_item = self.ast.push(Expr::Parameters(expr, NULL_EXPR_PTR));

        let mut previous_item = first_item;

        while matches!(comma.token, Some(Comma)) {
            (expr, comma) = self.parse_expr()?;
            let item = self.ast.push(Expr::Parameters(expr, NULL_EXPR_PTR));

            let Expr::Parameters(_, next) = &mut self.ast[previous_item] else {
                unreachable!();
            };
            *next = item;
            previous_item = item;
        }

        self.expect(end_token, comma);
        Ok(first_item)
    }

    pub(super) fn parse_expr(&mut self) -> ParseResult<(ExprPtr, TokenResult<'a>)> {
        let token = self.next_token()?;
        self.parse_expr_with_token(token)
    }

    pub(super) fn parse_expr_with_token(
        &mut self,
        token: TokenResult<'a>,
    ) -> ParseResult<(ExprPtr, TokenResult<'a>)> {
        self.parse_operation(token, 0)
    }

    fn parse_operation(
        &mut self,
        token: TokenResult<'a>,
        min_priority: u8,
    ) -> ParseResult<(ExprPtr, TokenResult<'a>)> {
        let (lhs, operator) = self.parse_atom(token)?;
        self.parse_operation_with_lhs(lhs, operator, min_priority)
    }

    fn parse_operation_with_lhs(
        &mut self,
        lhs: ExprPtr,
        next: TokenResult<'a>,
        min_priority: u8,
    ) -> ParseResult<(ExprPtr, TokenResult<'a>)> {
        let Some(next_token) = next.token else {
            return Ok((lhs, next));
        };

        let Some(operator) = next_token.get_operator() else {
            return Ok((lhs, next));
        };

        let priority = operator.priority();

        if priority < min_priority {
            // The expression continues but with operators of lower priority
            return Ok((lhs, next));
        }

        // Now we know that the expression continues
        // with an operation of priority >= min_priority

        let rhs_token = self.next_token()?;
        let (rhs, next_operator) = self.parse_operation(rhs_token, priority + 1)?;

        let head_expr = self.ast.push(Expr::HeadOperation {
            lhs,
            operator,
            rhs,
            next: NULL_EXPR_PTR,
        });

        let mut lhs = head_expr;
        let mut operator_token = next_operator;

        // Loop for all operators of the same priority
        while let Some(operator) = operator_token.token {
            let Some(operator) = operator.get_operator() else {
                break; // The expression has ended
            };

            if operator.priority() != priority {
                // The expression continues but with lower priority than min
                if operator.priority() < min_priority {
                    break;
                }

                // The expression continues with lower priority, but >= than min
                return self.parse_operation_with_lhs(head_expr, operator_token, min_priority);
            }

            // Now we know that the expression continues
            // with an operation of the same priority

            let rhs_token = self.next_token()?;
            let (rhs, next_operator) = self.parse_operation(rhs_token, priority + 1)?;

            let operation = self.ast.push(Expr::Operation {
                operator,
                rhs,
                next: NULL_EXPR_PTR,
            });

            match &mut self.ast[lhs] {
                Expr::HeadOperation { next, .. } => *next = operation,
                Expr::Operation { next, .. } => *next = operation,
                _ => unreachable!(),
            }

            lhs = operation;
            operator_token = next_operator;
        }

        Ok((head_expr, operator_token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_expr() {
        const EXPR_STR: &str = "a + b - 3 + (hello * 123 + 2)";
        const EXPR_TREE: &str = r#"
lhs + rhs next
(lhs) Variable("a")
(rhs) Variable("b")
(next) - rhs next
│ (rhs) Integer("3")
│ (next) + rhs
│ │ (rhs) lhs + rhs
│ │ │ (lhs) lhs * rhs
│ │ │ │ (lhs) Variable("hello")
│ │ │ │ (rhs) Integer("123")
│ │ │ (rhs) Integer("2")
"#;

        let mut ctx = AstContext::default();
        let mut parser = Parser {
            ast: ctx.create_ast(),
            lexer: Lexer::new(EXPR_STR),
            errors: CompilationErrorSet::default(),
            peek_statement: None,
        };

        let Ok((expr, next)) = parser.parse_expr() else {
            panic!("{:?}", parser.errors);
        };
        assert_eq!(next.token, None);

        let tree = &mut String::new();
        parser.ast.print(tree, expr);

        assert_eq!(EXPR_TREE.trim(), tree.trim());
    }
}
