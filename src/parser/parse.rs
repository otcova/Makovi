use super::*;
use crate::ast::*;
use crate::error::CompilationError;
use crate::lexer::Token::*;
use crate::lexer::*;
use std::cmp::Ordering;

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

        let body = self.statements_block(1)?;

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

    fn statements_block(&mut self, nesting: usize) -> Result<ExprPtr, CompilationError> {
        let mut node = self.statement_node(nesting)?;

        // Empty block
        if node == NULL_EXPR_PTR {
            return Ok(NULL_EXPR_PTR);
        }

        let first_statement = self.ast.push(Expr::Statements(node, NULL_EXPR_PTR));
        let mut previous_statement = first_statement;

        loop {
            node = self.statement_node(nesting)?;

            // Block ended
            if node == NULL_EXPR_PTR {
                break;
            }

            let statement = self.ast.push(Expr::Statements(node, NULL_EXPR_PTR));

            match &mut self.ast[previous_statement] {
                Expr::Statements(_, next) => *next = statement,
                _ => unreachable!(),
            }

            previous_statement = statement;
        }

        Ok(first_statement)
    }

    fn statement_node(&mut self, block_nesting: usize) -> Result<ExprPtr, CompilationError> {
        let mut token = self.lexer.peek();

        while token.token == Some(Ok(NewLine)) {
            self.lexer.next();
            token = self.lexer.peek();
        }

        match token.nesting()?.cmp(&block_nesting) {
            Ordering::Equal => {}
            Ordering::Less => return Ok(NULL_EXPR_PTR),
            Ordering::Greater => {
                return Err(CompilationError {
                    message: "Unexpected high nesting".to_owned(),
                    span: token.nesting_span(),
                })
            }
        }

        Ok(match_token!(match self.lexer.peek() {
            Return => self.return_statement()?,
            If => self.if_statement(block_nesting)?,
            While => self.while_statement(block_nesting)?,
            Identifier => {
                self.lexer.next();
                match_token!(match self.lexer.next() {
                    Assign => {
                        let value = self.expr()?;
                        self.ast.push(Expr::Assign(token.slice, value))
                    }
                    AddAssign => {
                        let value = self.expr()?;
                        let var = self.ast.push(Expr::Variable(token.slice));
                        let result = self.ast.push(Expr::HeadOperation {
                            lhs: var,
                            operator: Operator::Add,
                            rhs: value,
                            next: NULL_EXPR_PTR,
                        });
                        self.ast.push(Expr::Assign(token.slice, result))
                    }
                    SubAssign => {
                        let value = self.expr()?;
                        let var = self.ast.push(Expr::Variable(token.slice));
                        let result = self.ast.push(Expr::HeadOperation {
                            lhs: var,
                            operator: Operator::Sub,
                            rhs: value,
                            next: NULL_EXPR_PTR,
                        });
                        self.ast.push(Expr::Assign(token.slice, result))
                    }
                    BracketOpen => {
                        let parameters = self.expr_list(Token::BracketClose)?;

                        self.ast.push(Expr::Call(token.slice, parameters))
                    }
                })
            }
        }))
    }

    fn return_statement(&mut self) -> Result<ExprPtr, CompilationError> {
        self.lexer.next().expect(Token::Return)?;
        let return_value = self.expr()?;

        Ok(self.ast.push(Expr::Return(return_value)))
    }

    fn while_statement(&mut self, while_nesting: usize) -> Result<ExprPtr, CompilationError> {
        self.lexer.next().expect(Token::While)?;

        let condition = self.expr()?;

        let body = self.statements_block(while_nesting + 1)?;
        Ok(self.ast.push(Expr::WhileLoop { condition, body }))
    }

    fn if_statement(&mut self, if_nesting: usize) -> Result<ExprPtr, CompilationError> {
        self.lexer.next().expect(Token::If)?;

        let condition = self.expr()?;
        let then_body = self.statements_block(if_nesting + 1)?;

        let mut else_body = NULL_EXPR_PTR;

        if self.lexer.peek().token == Some(Ok(Else)) {
            self.lexer.next();

            else_body = match_token!(match self.lexer.peek() {
                If => self.if_statement(if_nesting),              // else if
                NewLine => self.statements_block(if_nesting + 1), // else
            })?;
        }

        Ok(self.ast.push(Expr::IfElse {
            condition,
            then_body,
            else_body,
        }))
    }

    fn expr_list(&mut self, closed_with: Token) -> Result<ExprPtr, CompilationError> {
        loop {
            match self.lexer.peek() {
                whitespace if whitespace.token == Some(Ok(Token::NewLine)) => {
                    self.lexer.next();
                    continue;
                }
                end if end.token == Some(Ok(closed_with)) => {
                    self.lexer.next();
                    return Ok(NULL_EXPR_PTR);
                }
                _ => break,
            }
        }

        let param_value = self.expr()?;
        let first_param = self.ast.push(Expr::Parameters(param_value, NULL_EXPR_PTR));

        let mut previous_param = first_param;

        loop {
            match_token!(match self.lexer.next() {
                Comma => {},
                #if closed_with => return Ok(first_param),
            });

            let param_value = self.expr()?;
            let param = self.ast.push(Expr::Parameters(param_value, NULL_EXPR_PTR));

            let Expr::Parameters(_, next) = &mut self.ast[previous_param] else {
                // SAFETY: `previous_param` is only assigned to Expr::Parameters
                unreachable!();
            };

            *next = param;

            previous_param = param;
        }
    }

    fn expr(&mut self) -> Result<ExprPtr, CompilationError> {
        self.expr_priority(0)
    }

    fn expr_priority(&mut self, min_priority: u8) -> Result<ExprPtr, CompilationError> {
        let lhs = self.expr_atom()?;
        Ok(self.expr_operation(lhs, min_priority)?.unwrap_or(lhs))
    }

    fn expr_operation(
        &mut self,
        lhs: ExprPtr,
        min_priority: u8,
    ) -> Result<Option<ExprPtr>, CompilationError> {
        // Check: lhs <token>
        let TokenResult {
            token: Some(Ok(token)),
            ..
        } = self.lexer.peek()
        else {
            return Ok(Some(lhs));
        };

        // Check: lhs <operator>
        let Some(operator) = token.get_operator() else {
            return Ok(Some(lhs));
        };

        let priority = operator.priority();

        if priority < min_priority {
            return Ok(Some(lhs)); // The expression continues but with operators of lower priority
        }

        // Now we know that the expression continues
        // with an operation of priority >= min_priority
        self.lexer.next();

        let rhs = self.expr_priority(priority + 1)?;

        let head_expr = self.ast.push(Expr::HeadOperation {
            lhs,
            operator,
            rhs,
            next: NULL_EXPR_PTR,
        });

        let mut lhs = head_expr;

        // Loop for all operators of the same priority
        while let TokenResult {
            token: Some(Ok(token)),
            ..
        } = self.lexer.peek()
        {
            // Check: lhs <operator>
            let Some(operator) = token.get_operator() else {
                break; // The expression has ended
            };

            if operator.priority() != priority {
                // The expression continues but with lower priority than min
                if operator.priority() < min_priority {
                    return Ok(Some(head_expr));
                }

                // The expression continues with lower priority, but >= than min
                return self.expr_operation(head_expr, min_priority);
            }

            // Now we know that the expression continues
            // with an operation of the same priority
            self.lexer.next();

            let rhs = self.expr_priority(priority + 1)?;

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
        }

        Ok(Some(head_expr))
    }

    fn expr_atom(&mut self) -> Result<ExprPtr, CompilationError> {
        let token = self.lexer.peek();
        Ok(match_token!(match token {
            Identifier => {
                self.lexer.next();
                match_token!(match self.lexer.peek() {
                    BracketOpen => {
                        self.lexer.next();
                        let parameters = self.expr_list(Token::BracketClose)?;

                        self.ast.push(Expr::Call(token.slice, parameters))
                    }
                    _ => self.ast.push(Expr::Variable(token.slice)),
                })
            }
            Integer => {
                self.lexer.next();
                self.ast.push(Expr::Integer(token.slice))
            }
            True => {
                self.lexer.next();
                self.ast.push(Expr::Bool(true))
            }
            False => {
                self.lexer.next();
                self.ast.push(Expr::Bool(false))
            }
            BracketOpen => {
                self.lexer.next();
                let value = self.expr()?;
                self.lexer.next().expect(Token::BracketClose)?;
                value
            }
        }))
    }
}
