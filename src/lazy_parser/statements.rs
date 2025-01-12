use super::*;
use crate::error::CompilationError;
use crate::lexer::*;

pub enum Statement<'a> {
    Assign(&'a str),
    If,
    Else,
    While,
    Function(&'a str),
    Return,
}

impl<'a> Parser<'a> {
    pub fn parse_statement(
        &mut self,
        lexer: &mut Lexer<'a>,
    ) -> Result<Statement<'a>, CompilationError> {
        Ok(match_token!(match lexer.next() {
            Return => {
                self.parse_expression(lexer)?;
                Statement::Return
            }
            If => {
                self.parse_expression(lexer)?;
                Statement::If
            }
            While => {
                self.parse_expression(lexer)?;
                Statement::While
            }
            Identifier(name) => {
                expect_token!(Assign, lexer.next());
                self.parse_expression(lexer)?;

                Statement::Assign(name)
            }
            NewLine => {
                self.parse_statement(lexer)?
            }
        }))
    }
}
