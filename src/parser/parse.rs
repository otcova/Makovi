use super::*;
use crate::ast::*;
use crate::error::CompilationError;
use crate::lexer::Token::*;
use crate::lexer::*;

impl<'a> AstParser<'a> {
    pub fn parse(mut self) -> Result<Ast<'a>, CompilationError> {
        self.function()?;

        while let TokenResult {
            token: Some(token),
            span,
            ..
        } = self.lexer.next()
        {
            if token != Ok(NewLine) {
                return Err(CompilationError {
                    message: format!("Expected a single top level function, found {token:?}"),
                    span,
                });
            }
        }

        Ok(self.ast)
    }

    fn function(&mut self) -> Result<ExprPtr, CompilationError> {
        self.lexer.next().expect(Token::Function)?;

        let name = self.lexer.next().expect(Token::Identifier)?.slice;

        self.lexer.next().expect(Token::BracketOpen)?;
        let parameters = self.function_parameters()?;
        self.lexer.next().expect(Token::BracketClose)?;

        let body = self.statements_block()?;

        Ok(self.ast.push(Expr::Function {
            name,
            parameters,
            body,
        }))
    }

    fn function_parameters(&mut self) -> Result<ExprPtr, CompilationError> {
        Ok(match_token!(match self.lexer.peek() {
            Identifier => self.function_parameter_node()?,
            BracketClose => NULL_EXPR_PTR,
        }))
    }
    fn function_parameter_node(&mut self) -> Result<ExprPtr, CompilationError> {
        let name = self.lexer.next().expect(Token::Identifier)?.slice;
        let ident = self.ast.push(Expr::VariableDefinition(name));

        let next_param = match_token!(match self.lexer.peek() {
            Comma => {
                self.lexer.next();
                self.function_parameters()?
            }
            BracketClose => NULL_EXPR_PTR,
        });

        Ok(self.ast.push(Expr::ParametersDefinition(ident, next_param)))
    }

    fn statements_block(&mut self) -> Result<ExprPtr, CompilationError> {
        self.lexer.next().expect(Token::CurlyOpen)?;
        let statements = self.statement_node()?;
        self.lexer.next().expect(Token::CurlyClose)?;

        Ok(statements)
    }

    fn statement_node(&mut self) -> Result<ExprPtr, CompilationError> {
        let token = self.lexer.peek();
        Ok(match_token!(match self.lexer.peek() {
            Return => {
                self.lexer.next();
                let return_value = self.expr()?.ok_or_else(|| CompilationError {
                    message: "Expected a return value".to_owned(),
                    span: token.span.and(self.lexer.peek().span),
                })?;

                let return_statement = self.ast.push(Expr::Return(return_value));

                // TODO: Consume/Skip dead code without pushing nodes
                self.statement_node()?;

                self.ast
                    .push(Expr::Statements(return_statement, NULL_EXPR_PTR))
            }
            If => {
                let if_statement = self.if_statement()?;

                let next_statement = self.statement_node()?;
                self.ast
                    .push(Expr::Statements(if_statement, next_statement))
            }
            While => {
                self.lexer.next();
                let condition = self.expr()?.ok_or_else(|| CompilationError {
                    message: "Expected the while condition".to_owned(),
                    span: token.span.and(self.lexer.peek().span),
                })?;
                let body = self.statements_block()?;
                let while_statement = self.ast.push(Expr::WhileLoop { condition, body });

                let next_statement = self.statement_node()?;
                self.ast
                    .push(Expr::Statements(while_statement, next_statement))
            }
            Identifier => {
                self.lexer.next();
                match_token!(match self.lexer.next() {
                    Assign => {
                        let value = self.expr()?.ok_or_else(|| CompilationError {
                            message: format!(
                                "Expected an expression to assign to '{}'",
                                token.slice
                            ),
                            span: token.span.and(self.lexer.peek().span),
                        })?;

                        let assign = self.ast.push(Expr::Assign(token.slice, value));

                        let next_statement = self.statement_node()?;
                        self.ast.push(Expr::Statements(assign, next_statement))
                    }
                    BracketOpen => {
                        let parameters = self.expr_list()?;
                        self.lexer.next().expect(Token::BracketClose)?;

                        let call = self.ast.push(Expr::Call(token.slice, parameters));

                        let next_statement = self.statement_node()?;
                        self.ast.push(Expr::Statements(call, next_statement))
                    }
                })
            }
            NewLine => {
                self.lexer.next();
                self.statement_node()?
            }
            CurlyClose => NULL_EXPR_PTR,
        }))
    }

    fn if_statement(&mut self) -> Result<ExprPtr, CompilationError> {
        let span = self.lexer.next().expect(Token::If)?.span;

        let condition = self.expr()?.ok_or_else(|| CompilationError {
            message: "Expected the if condition".to_owned(),
            span: span.and(self.lexer.peek().span),
        })?;
        let then_body = self.statements_block()?;

        let mut else_body = NULL_EXPR_PTR;

        if self.lexer.peek().token == Some(Ok(Else)) {
            self.lexer.next();

            else_body = match_token!(match self.lexer.peek() {
                If => self.if_statement(),
                CurlyOpen => self.statements_block(),
            })?;
        }

        Ok(self.ast.push(Expr::IfElse {
            condition,
            then_body,
            else_body,
        }))
    }

    fn expr_list(&mut self) -> Result<ExprPtr, CompilationError> {
        if let Some(expr) = self.expr()? {
            let next_expr = match_token!(match self.lexer.peek() {
                Comma => {
                    self.lexer.next();
                    self.expr_list()?
                }
                _ => NULL_EXPR_PTR,
            });
            Ok(self.ast.push(Expr::Parameters(expr, next_expr)))
        } else {
            Ok(NULL_EXPR_PTR)
        }
    }

    fn expr(&mut self) -> Result<Option<ExprPtr>, CompilationError> {
        self.expr_node(0)
    }

    fn expr_node(&mut self, min_priority: u8) -> Result<Option<ExprPtr>, CompilationError> {
        let Some(mut value) = self.expr_atom()? else {
            return Ok(None);
        };

        while let TokenResult {
            token: Some(Ok(token)),
            span,
            ..
        } = self.lexer.peek()
        {
            let Some(operator) = token.get_operator() else {
                break; // The expression has ended
            };
            let priority = operator.priority();

            if priority < min_priority {
                break; // The expression continues but with operators of lower priority
            }

            self.lexer.next();

            let next_expression =
                self.expr_node(priority + 1)?
                    .ok_or_else(|| CompilationError {
                        message: format!("Expected an expression after the operator {operator:?}"),
                        span,
                    })?;

            value = self
                .ast
                .push(Expr::Operator(operator, value, next_expression));
        }

        Ok(Some(value))
    }

    fn expr_atom(&mut self) -> Result<Option<ExprPtr>, CompilationError> {
        let token = self.lexer.peek();
        Ok(match_token!(match token {
            Identifier => {
                self.lexer.next();
                Some(match_token!(match self.lexer.peek() {
                    BracketOpen => {
                        self.lexer.next();
                        let parameters = self.expr_list()?;
                        self.lexer.next().expect(Token::BracketClose)?;

                        self.ast.push(Expr::Call(token.slice, parameters))
                    }
                    _ => self.ast.push(Expr::Variable(token.slice)),
                }))
            }
            Integer => {
                self.lexer.next();
                Some(self.ast.push(Expr::Integer(token.slice)))
            }
            BracketOpen => {
                self.lexer.next();
                let expr = self.expr()?;
                let close_span = self.lexer.next().expect(Token::BracketClose)?.span;

                if expr.is_none() {
                    return Err(CompilationError {
                        message: "Expected an expression inside the '()'".to_owned(),
                        span: token.span.and(close_span),
                    });
                }

                expr
            }
            _ => None,
        }))
    }
}
