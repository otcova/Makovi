use super::*;

impl<'code> FunctionParser<'_, 'code> {
    pub(super) fn parse_parameters(
        &mut self,
        scope: &mut FunctionScope<'code>,
    ) -> Result<usize, ()> {
        self.expect_token(Token::BracketOpen)?;
        self.lexer.next();

        let mut arity = 0;

        if self.lexer.token() == Token::BracketClose {
            return Ok(arity);
        }

        loop {
            self.expect_token(Token::Identifier)?;
            scope.define_variable(0, self.lexer.slice(), arity.into());
            arity += 1;

            match self.lexer.next() {
                Token::Comma => self.lexer.next(),
                Token::BracketClose => {
                    self.lexer.next();
                    return Ok(arity);
                }
                _ => {
                    self.unexpected_token("')'");
                    self.lexer.next();
                    // TODO: Maybe skip parameters
                    return Err(());
                }
            };
        }
    }
}
