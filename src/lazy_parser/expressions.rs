use crate::ast::Operator;
use crate::error::CompilationError;
use crate::lexer::*;

pub enum Expr<'a> {
    Variable(&'a str),
    Integer(&'a str),
    Call(&'a str, ExprList),
    Operator(Operator, ExprOffset<1>, ExprOffset<2>),
}

type ExprPtr = u32;
pub struct ExprOffset<const OFFSET: u32>;
pub struct ExprList(u32);

#[derive(Default)]
pub struct ExprContext<'a> {
    buffer: Vec<Expr<'a>>,
}

impl<'a> ExprContext<'a> {
    pub fn parse(&mut self, lexer: &mut Lexer<'a>) -> Result<(), CompilationError> {
        self.buffer.clear();
        self.expr(lexer, 0)
    }

    fn expr(&mut self, lexer: &mut Lexer<'a>, min_priority: u8) -> Result<(), CompilationError> {
        self.expr_atom(lexer)?;

        while let (Some(Ok(Token::Operator(operator))), _) = lexer.peek() {
            let priority = operator.priority();

            if priority < min_priority {
                break; // The expression continues but with operators of lower priority
            }

            lexer.next();

            // Parse right hand side expression
            self.expr(lexer, priority + 1)?;

            self.buffer
                .push(Expr::Operator(operator, ExprOffset, ExprOffset));
        }
        Ok(())
    }

    fn expr_list(
        &mut self,
        lexer: &mut Lexer<'a>,
        termination: Token,
    ) -> Result<ExprList, CompilationError> {
        expect_token!(next_token, lexer.peek());

        if next_token == termination {
            return Ok(ExprList(0));
        }

        self.expr(lexer, 0)?;
        let list = self.expr_list_next(lexer, termination)?;
        Ok(ExprList(list.0 + 1))
    }

    fn expr_list_next(
        &mut self,
        lexer: &mut Lexer<'a>,
        termination: Token,
    ) -> Result<ExprList, CompilationError> {
        expect_token!(next_token, span, lexer.next());

        if next_token == termination {
            Ok(ExprList(0))
        } else if next_token == Token::Comma {
            lexer.next();
            self.expr(lexer, 0)?;
            let list = self.expr_list_next(lexer, termination)?;
            Ok(ExprList(list.0 + 1))
        } else {
            Err(CompilationError {
                message: format!("Expected a {termination:?}"),
                span,
            })
        }
    }

    fn expr_atom(&mut self, lexer: &mut Lexer<'a>) -> Result<(), CompilationError> {
        match_token!(match lexer.next() {
            Identifier(name) => {
                match_token!(match lexer.peek() {
                    BracketOpen => {
                        lexer.next();
                        let parameters = self.expr_list_next(lexer, Token::BracketClose)?;

                        self.buffer.push(Expr::Call(name, parameters));
                    }
                    _ => {
                        self.buffer.push(Expr::Variable(name));
                    }
                });
            }
            Integer(value) => {
                self.buffer.push(Expr::Integer(value));
            }
            BracketOpen => {
                self.expr(lexer, 0)?;
            }
        });
        Ok(())
    }
}
