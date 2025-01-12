use crate::ast::*;
use crate::error::CompilationError;
use crate::lexer::Token::*;
use crate::lexer::*;

impl Token {
    fn get_operator(&self) -> Option<Operator> {
        match self {
            Mul => Some(Operator::Mul),
            Div => Some(Operator::Div),
            Plus => Some(Operator::Add),
            Minus => Some(Operator::Sub),
            Mod => Some(Operator::Mod),
            Eq => Some(Operator::Eq),
            Ne => Some(Operator::Ne),
            Lt => Some(Operator::Lt),
            Le => Some(Operator::Le),
            Gt => Some(Operator::Gt),
            Ge => Some(Operator::Ge),
            _ => None,
        }
    }
}

impl<'a> Ast<'a> {
    pub fn parse(&mut self, source: &'a str) -> Result<(), CompilationError> {
        assert!(self.root().is_none());

        let mut lexer = Lexer::new(source);

        self.function(&mut lexer)?;

        while let TokenResult {
            token: Some(token),
            span,
            ..
        } = lexer.next()
        {
            if token != Ok(NewLine) {
                return Err(CompilationError {
                    message: format!("Expected a single top level function, found {token:?}"),
                    span,
                });
            }
        }

        Ok(())
    }

    fn function<'l>(&mut self, lexer: &'l mut Lexer<'a>) -> Result<ExprPtr, CompilationError> {
        lexer.next().expect(Token::Function)?;

        let name = lexer.next().expect(Token::Identifier)?.slice;

        lexer.next().expect(Token::BracketOpen)?;
        let parameters = self.function_parameters(lexer)?;
        lexer.next().expect(Token::BracketClose)?;

        let body = self.statements_block(lexer)?;

        Ok(self.push(Expr::Function {
            name,
            parameters,
            body,
        }))
    }

    fn function_parameters(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, CompilationError> {
        Ok(match_token!(match lexer.peek() {
            Identifier => self.function_parameter_node(lexer)?,
            BracketClose => NULL_EXPR_PTR,
        }))
    }
    fn function_parameter_node(
        &mut self,
        lexer: &mut Lexer<'a>,
    ) -> Result<ExprPtr, CompilationError> {
        let name = lexer.next().expect(Token::Identifier)?.slice;
        let ident = self.push(Expr::VariableDefinition(name));

        let next_param = match_token!(match lexer.peek() {
            Comma => {
                lexer.next();
                self.function_parameters(lexer)?
            }
            BracketClose => NULL_EXPR_PTR,
        });

        Ok(self.push(Expr::ParametersDefinition(ident, next_param)))
    }

    fn statements_block(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, CompilationError> {
        lexer.next().expect(Token::CurlyOpen)?;
        let statements = self.statement_node(lexer)?;
        lexer.next().expect(Token::CurlyClose)?;

        Ok(statements)
    }

    fn statement_node(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, CompilationError> {
        Ok(match_token!(match lexer.peek() {
            Return => {
                let TokenResult {
                    span: return_span, ..
                } = lexer.next();
                let return_value = self.expr(lexer)?.ok_or_else(|| CompilationError {
                    message: "Expected a return value".to_owned(),
                    span: return_span.and(lexer.peek().span),
                })?;

                let return_statement = self.push(Expr::Return(return_value));

                // TODO: Consume/Skip dead code without pushing nodes
                self.statement_node(lexer)?;

                self.push(Expr::Statements(return_statement, NULL_EXPR_PTR))
            }
            If => {
                let if_statement = self.if_statement(lexer)?;

                let next_statement = self.statement_node(lexer)?;
                self.push(Expr::Statements(if_statement, next_statement))
            }
            While => {
                let TokenResult {
                    span: while_span, ..
                } = lexer.next();
                let condition = self.expr(lexer)?.ok_or_else(|| CompilationError {
                    message: "Expected the while condition".to_owned(),
                    span: while_span.and(lexer.peek().span),
                })?;
                let body = self.statements_block(lexer)?;
                let while_statement = self.push(Expr::WhileLoop { condition, body });

                let next_statement = self.statement_node(lexer)?;
                self.push(Expr::Statements(while_statement, next_statement))
            }
            Identifier(name) => {
                let TokenResult { span, .. } = lexer.next();
                match_token!(match lexer.next() {
                    Assign => {
                        let value = self.expr(lexer)?.ok_or_else(|| CompilationError {
                            message: format!("Expected an expression to assign to '{name}'"),
                            span: span.and(lexer.peek().span),
                        })?;

                        let assign = self.push(Expr::Assign(name, value));

                        let next_statement = self.statement_node(lexer)?;
                        self.push(Expr::Statements(assign, next_statement))
                    }
                    BracketOpen => {
                        let parameters = self.expr_list(lexer)?;
                        lexer.next().expect(Token::BracketClose)?;

                        let call = self.push(Expr::Call(name, parameters));

                        let next_statement = self.statement_node(lexer)?;
                        self.push(Expr::Statements(call, next_statement))
                    }
                })
            }
            NewLine => {
                lexer.next();
                self.statement_node(lexer)?
            }
            CurlyClose => NULL_EXPR_PTR,
        }))
    }

    fn if_statement(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, CompilationError> {
        let span = lexer.next().expect(Token::If)?.span;

        let condition = self.expr(lexer)?.ok_or_else(|| CompilationError {
            message: "Expected the if condition".to_owned(),
            span: span.and(lexer.peek().span),
        })?;
        let then_body = self.statements_block(lexer)?;

        let mut else_body = NULL_EXPR_PTR;

        if lexer.peek().token == Some(Ok(Else)) {
            lexer.next();

            else_body = match_token!(match lexer.peek() {
                If => self.if_statement(lexer),
                CurlyOpen => self.statements_block(lexer),
            })?;
        }

        Ok(self.push(Expr::IfElse {
            condition,
            then_body,
            else_body,
        }))
    }

    fn expr_list(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, CompilationError> {
        if let Some(expr) = self.expr(lexer)? {
            let next_expr = match_token!(match lexer.peek() {
                Comma => {
                    lexer.next();
                    self.expr_list(lexer)?
                }
                _any => NULL_EXPR_PTR,
            });
            Ok(self.push(Expr::Parameters(expr, next_expr)))
        } else {
            Ok(NULL_EXPR_PTR)
        }
    }

    fn expr(&mut self, lexer: &mut Lexer<'a>) -> Result<Option<ExprPtr>, CompilationError> {
        self.expr_node(lexer, 0)
    }

    fn expr_node(
        &mut self,
        lexer: &mut Lexer<'a>,
        min_priority: u8,
    ) -> Result<Option<ExprPtr>, CompilationError> {
        let Some(mut value) = self.expr_atom(lexer)? else {
            return Ok(None);
        };

        while let TokenResult {
            token: Some(Ok(token)),
            span,
            ..
        } = lexer.peek()
        {
            let Some(operator) = token.get_operator() else {
                break; // The expression has ended
            };
            let priority = operator.priority();

            if priority < min_priority {
                break; // The expression continues but with operators of lower priority
            }

            lexer.next();

            let next_expression =
                self.expr_node(lexer, priority + 1)?
                    .ok_or_else(|| CompilationError {
                        message: format!("Expected an expression after the operator {operator:?}"),
                        span,
                    })?;

            value = self.push(Expr::Operator(operator, value, next_expression));
        }

        Ok(Some(value))
    }

    fn expr_atom(&mut self, lexer: &mut Lexer<'a>) -> Result<Option<ExprPtr>, CompilationError> {
        Ok(match_token!(match lexer.peek() {
            Identifier(name) => {
                lexer.next();
                Some(match_token!(match lexer.peek() {
                    BracketOpen => {
                        lexer.next();
                        let parameters = self.expr_list(lexer)?;
                        lexer.next().expect(Token::BracketClose)?;

                        self.push(Expr::Call(name, parameters))
                    }
                    _any => self.push(Expr::Variable(name)),
                }))
            }
            Integer(value) => {
                lexer.next();
                Some(self.push(Expr::Integer(value)))
            }
            BracketOpen => {
                let TokenResult {
                    span: open_span, ..
                } = lexer.next();
                let expr = self.expr(lexer)?;
                let close_span = lexer.next().expect(Token::BracketClose)?.span;

                if expr.is_none() {
                    return Err(CompilationError {
                        message: "Expected an expression inside the '()'".to_owned(),
                        span: open_span.and(close_span),
                    });
                }

                expr
            }
            _any => None,
        }))
    }
}
