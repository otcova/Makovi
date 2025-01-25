use super::*;

impl<'a> Parser<'a> {
    pub(super) fn next_token(&mut self) -> ParseResult<TokenResult<'a>> {
        match self.lexer.next() {
            Ok(token) => Ok(token),
            Err(error) => self.statement_error(error)?,
        }
    }

    fn skip_statement(&mut self) -> ParseResult<!> {
        loop {
            match self.lexer.next().map(|t| t.token) {
                Ok(None) | Ok(Some(Token::NewLine)) => break,
                Ok(Some(_)) => {}
                Err(error) => self.errors.add(error),
            }
        }
        Err(ParseError)
    }

    pub(super) fn statement_error(&mut self, error: CompilationError) -> ParseResult<!> {
        self.errors.add(error);
        self.skip_statement()
    }

    pub(super) fn unexpected_token(
        &mut self,
        expected: &str,
        token: TokenResult<'a>,
    ) -> ParseResult<!> {
        self.statement_error(CompilationError {
            span: token.span,
            message: format!("Expected {expected}, but found '{}'", token.slice),
        })
    }

    pub(super) fn eof_error(&mut self, expected: &str, token: TokenResult<'a>) -> ParseResult<!> {
        self.statement_error(CompilationError {
            span: token.span,
            message: format!("Expected {expected}"),
        })
    }

    pub(super) fn end_statement(&mut self, token: TokenResult<'a>) -> ParseResult<()> {
        if !matches!(token.token, Some(NewLine) | None) {
            self.unexpected_token("a new line", token)?;
        } else {
            Ok(())
        }
    }

    pub(super) fn take_error<T>(&mut self, result: Result<T, CompilationError>) -> ParseResult<T> {
        match result {
            Ok(t) => Ok(t),
            Err(error) => self.statement_error(error)?,
        }
    }

    pub(super) fn expect(
        &mut self,
        expected_token: Token,
        actual_token: TokenResult<'a>,
    ) -> ParseResult<TokenResult<'a>> {
        match actual_token.token {
            Some(t) if t == expected_token => Ok(actual_token),
            Some(_) => self.statement_error(CompilationError {
                message: format!(
                    "Expected {expected_token:?} but found '{}'",
                    actual_token.slice
                ),
                span: actual_token.span,
            })?,
            None => self.statement_error(CompilationError {
                message: format!("Expected {expected_token:?} but reached end o file"),
                span: actual_token.span,
            })?,
        }
    }
    pub(super) fn expect_next(&mut self, token: Token) -> ParseResult<TokenResult<'a>> {
        let next = self.next_token()?;
        self.expect(token, next)
    }
}
