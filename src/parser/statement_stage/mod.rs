mod checks;
mod expression_parser;

use super::*;
use crate::ast::*;
use crate::error::CompilationError;
use crate::lexer::*;
use Token::*;

impl<'a> Parser<'a> {
    pub fn next_satement(&mut self) -> Option<Statement<'a>> {
        loop {
            if let Ok(statement) = self.parse_statement() {
                return statement;
            }
        }
    }

    fn parse_statement(&mut self) -> ParseResult<Option<Statement<'a>>> {
        loop {
            let token = self.next_token()?;
            let token_nesting = token.nesting();
            let token_span = token.line_span();

            let class = match token.token {
                Some(Return) => self.parse_return()?,
                Some(If) => self.parse_if()?,
                Some(Else) => self.parse_else()?,
                Some(While) => self.parse_while()?,
                Some(Let) => self.parse_let()?,
                Some(Identifier) => self.parse_identifier(token)?,
                None => return Ok(None),

                Some(NewLine) => continue,
                Some(_) => self.unexpected_token("start of statement", token)?,
            };

            return Ok(Some(Statement {
                class,
                nesting: self.take_error(token_nesting)?,
                span: token_span,
            }));
        }
    }

    fn parse_let(&mut self) -> ParseResult<StatementClass<'a>> {
        let ident = self.expect_next(Identifier)?.slice;
        self.expect_next(Assign)?;

        let (expr, next) = self.parse_expr()?;
        self.end_statement(next)?;
        Ok(StatementClass::Declaration(ident, expr))
    }
    fn parse_while(&mut self) -> ParseResult<StatementClass<'a>> {
        let (expr, next) = self.parse_expr()?;
        self.end_statement(next)?;
        Ok(StatementClass::While(expr))
    }
    fn parse_return(&mut self) -> ParseResult<StatementClass<'a>> {
        let (expr, next) = self.parse_expr()?;
        self.end_statement(next)?;
        Ok(StatementClass::Return(expr))
    }
    fn parse_if(&mut self) -> ParseResult<StatementClass<'a>> {
        let (expr, next) = self.parse_expr()?;
        self.end_statement(next)?;
        Ok(StatementClass::If(expr))
    }
    fn parse_else(&mut self) -> ParseResult<StatementClass<'a>> {
        let token = self.next_token()?;
        if matches!(token.token, Some(Token::If)) {
            let (expr, next) = self.parse_expr()?;
            self.end_statement(next)?;
            Ok(StatementClass::ElseIf(expr))
        } else {
            self.end_statement(token)?;
            Ok(StatementClass::Else)
        }
    }

    fn parse_identifier(&mut self, token: TokenResult<'a>) -> ParseResult<StatementClass<'a>> {
        let slice = token.slice;
        let (atom, next) = self.parse_atom_identifyier(slice)?;
        match next.token {
            Some(AddAssign) => {
                let (expr, next) = self.parse_expr()?;
                self.end_statement(next)?;
                Ok(StatementClass::AssignOperation(slice, Operator::Add, expr))
            }
            Some(SubAssign) => {
                let (expr, next) = self.parse_expr()?;
                self.end_statement(next)?;
                Ok(StatementClass::AssignOperation(slice, Operator::Sub, expr))
            }
            Some(Assign) => {
                let (expr, next) = self.parse_expr()?;
                self.end_statement(next)?;
                Ok(StatementClass::Declaration(slice, expr))
            }
            Some(NewLine) | None => Ok(StatementClass::Atom(atom)),

            Some(_) => self.unexpected_token("an assignment", next)?,
        }
    }
}
