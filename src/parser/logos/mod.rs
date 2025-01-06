mod lexer;

use crate::ast::*;
use lexer::*;
use std::fmt::Debug;
use Token::*;

pub struct ParserError {
    pub message: String,
    pub span: LineColumnNumber,
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[Parser error] {} (line: {}, column: {})",
            self.message, self.span.line, self.span.column
        )
    }
}

macro_rules! match_next {
    (
        match $lexer:ident.$fn:ident() {
            $($pat:pat => $then:expr$(,)?)*
        }
    ) => {
        match $lexer.$fn() {
            $($pat => $then,)*
            #[allow(unreachable_patterns)]
                Some(Ok(found)) => {
                    return Err(ParserError {
                        message: format!( "Unexpected token '{found:?}'"),
                        span: LineColumnNumber { line: 0, column: 0 },
                    })
                }
            #[allow(unreachable_patterns)]
                Some(Err((token, span))) => {
                    return Err(ParserError {
                        message: format!("Unknown token '{}'", token),
                        span,
                    })
                }
            #[allow(unreachable_patterns)]
                None => {
                    return Err(ParserError {
                        message: "Unexpected end of file".to_owned(),
                        span: LineColumnNumber { line: 0, column: 0 },
                    })
                }
        }
    };
}

macro_rules! match_token {
    (
        match $lexer:ident.$fn:ident() {
            $($token:pat => $then:expr$(,)?)*
        }
    ) => {
        match_next!(match $lexer.$fn() {
            $(Some(Ok($token)) => $then,)*
        })
    };
}

macro_rules! expect_token {
    ($pat:pat, $token:expr) => {
        let token = $token;
        let Some(Ok($pat)) = token else {
            match token {
                Some(Ok(found)) => {
                    return Err(ParserError {
                        message: format!(
                            "Expected token '{}' but found '{found:?}'",
                            stringify!($pat)
                        ),
                        span: LineColumnNumber { line: 0, column: 0 },
                    })
                }
                Some(Err((token, span))) => {
                    return Err(ParserError {
                        message: format!("Unknown token {}", token),
                        span,
                    })
                }
                None => {
                    return Err(ParserError {
                        message: format!(
                            "Expected token '{}' but reached end of file",
                            stringify!($pat)
                        ),
                        span: LineColumnNumber { line: 0, column: 0 },
                    })
                }
            }
        };
    };
}

impl Token<'_> {
    fn operator_priority(&self) -> Option<u8> {
        match self {
            Mul => Some(3),
            Div => Some(3),
            Plus => Some(2),
            Minus => Some(2),
            Mod => Some(1),
            Eq => Some(0),
            Ne => Some(0),
            Lt => Some(0),
            Le => Some(0),
            Gt => Some(0),
            Ge => Some(0),
            _ => None,
        }
    }
}

impl<'a> Ast<'a> {
    pub fn parse(&mut self, source: &'a str) -> Result<(), ParserError> {
        assert!(self.root().is_none());

        let mut lexer = Lexer::new(source);

        self.function(&mut lexer)?;

        while let Some(token) = lexer.next() {
            if token != Ok(NewLine) {
                return Err(ParserError {
                    message: format!("Expected a single top level function, found {token:?}"),
                    span: LineColumnNumber { line: 0, column: 0 },
                });
            }
        }

        Ok(())
    }

    fn function<'l>(&mut self, lexer: &'l mut Lexer<'a>) -> Result<ExprPtr, ParserError> {
        expect_token!(Function, lexer.next());

        expect_token!(Identifier(name), lexer.next());

        expect_token!(BracketOpen, lexer.next());
        let parameters = self.function_parameters(lexer)?;
        expect_token!(BracketClose, lexer.next());

        let body = self.statements_block(lexer)?;

        Ok(self.push(Expr::Function {
            name,
            parameters,
            body,
        }))
    }

    fn function_parameters(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, ParserError> {
        Ok(match_token!(match lexer.peek() {
            Identifier(..) => self.function_parameter_node(lexer)?,
            BracketClose => NULL_EXPR_PTR,
        }))
    }
    fn function_parameter_node(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, ParserError> {
        expect_token!(Identifier(name), lexer.next());
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

    fn statements_block(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, ParserError> {
        expect_token!(CurlyOpen, lexer.next());
        let statements = self.statement_node(lexer)?;
        expect_token!(CurlyClose, lexer.next());

        Ok(statements)
    }

    fn statement_node(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, ParserError> {
        Ok(match_token!(match lexer.peek() {
            Return => {
                lexer.next();
                let return_value = self.expr(lexer)?.ok_or_else(|| ParserError {
                    message: "Expected a return value".to_owned(),
                    span: LineColumnNumber { line: 0, column: 0 },
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
                lexer.next();
                let condition = self.expr(lexer)?.ok_or_else(|| ParserError {
                    message: "Expected the while condition".to_owned(),
                    span: LineColumnNumber { line: 0, column: 0 },
                })?;
                let body = self.statements_block(lexer)?;
                let while_statement = self.push(Expr::WhileLoop { condition, body });

                let next_statement = self.statement_node(lexer)?;
                self.push(Expr::Statements(while_statement, next_statement))
            }
            Identifier(name) => {
                lexer.next();
                match_token!(match lexer.next() {
                    Assign => {
                        let value = self.expr(lexer)?.ok_or_else(|| ParserError {
                            message: format!("Expected an expression to assign to '{name}'"),
                            span: LineColumnNumber { line: 0, column: 0 },
                        })?;

                        let assign = self.push(Expr::Assign(name, value));

                        let next_statement = self.statement_node(lexer)?;
                        self.push(Expr::Statements(assign, next_statement))
                    }
                    BracketOpen => {
                        let parameters = self.expr_list(lexer)?;
                        expect_token!(BracketClose, lexer.next());

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

    fn if_statement(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, ParserError> {
        expect_token!(If, lexer.next());

        let condition = self.expr(lexer)?.ok_or_else(|| ParserError {
            message: "Expected the if condition".to_owned(),
            span: LineColumnNumber { line: 0, column: 0 },
        })?;
        let then_body = self.statements_block(lexer)?;

        expect_token!(Else, lexer.next());

        let else_body = match_token!(match lexer.peek() {
            If => self.if_statement(lexer),
            CurlyOpen => self.statements_block(lexer),
        })?;

        Ok(self.push(Expr::IfElse {
            condition,
            then_body,
            else_body,
        }))
    }

    fn expr_list(&mut self, lexer: &mut Lexer<'a>) -> Result<ExprPtr, ParserError> {
        if let Some(expr) = self.expr(lexer)? {
            let next_expr = match_token!(match lexer.peek() {
                Comma => {
                    lexer.next();
                    self.expr_list(lexer)?
                }
                _ => NULL_EXPR_PTR,
            });
            Ok(self.push(Expr::Parameters(expr, next_expr)))
        } else {
            Ok(NULL_EXPR_PTR)
        }
    }

    fn expr(&mut self, lexer: &mut Lexer<'a>) -> Result<Option<ExprPtr>, ParserError> {
        self.expr_node(lexer, 0)
    }

    fn expr_node(
        &mut self,
        lexer: &mut Lexer<'a>,
        min_priority: u8,
    ) -> Result<Option<ExprPtr>, ParserError> {
        let Some(mut value) = self.expr_atom(lexer)? else {
            return Ok(None);
        };

        while let Some(Ok(token)) = lexer.peek() {
            let Some(priority) = token.operator_priority() else {
                break; // The expression has ended
            };

            if priority < min_priority {
                break; // The expression continues but with operators of lower priority
            }

            let Some(Ok(operator)) = lexer.next() else {
                unreachable!("Only an operator should have priority");
            };

            let next_expression =
                self.expr_node(lexer, priority + 1)?
                    .ok_or_else(|| ParserError {
                        message: format!("Expected an expression after the operator {operator:?}"),
                        span: LineColumnNumber { line: 0, column: 0 },
                    })?;

            let expr = match operator {
                Plus => Expr::Add(value, next_expression),
                Minus => Expr::Sub(value, next_expression),
                Mul => Expr::Mul(value, next_expression),
                Div => Expr::Div(value, next_expression),
                Mod => Expr::Mod(value, next_expression),
                Eq => Expr::Eq(value, next_expression),
                Ne => Expr::Ne(value, next_expression),
                Lt => Expr::Lt(value, next_expression),
                Le => Expr::Le(value, next_expression),
                Gt => Expr::Gt(value, next_expression),
                Ge => Expr::Ge(value, next_expression),
                _ => unreachable!("All operators should have been matched"),
            };

            value = self.push(expr);
        }

        Ok(Some(value))
    }

    fn expr_atom(&mut self, lexer: &mut Lexer<'a>) -> Result<Option<ExprPtr>, ParserError> {
        Ok(match_token!(match lexer.peek() {
            Identifier(name) => {
                lexer.next();
                Some(match_token!(match lexer.peek() {
                    BracketOpen => {
                        lexer.next();
                        let parameters = self.expr_list(lexer)?;
                        expect_token!(BracketClose, lexer.next());

                        self.push(Expr::Call(name, parameters))
                    }
                    _ => self.push(Expr::Variable(name)),
                }))
            }
            Integer(value) => {
                lexer.next();
                Some(self.push(Expr::Integer(value)))
            }
            _ => None,
        }))
    }
}
