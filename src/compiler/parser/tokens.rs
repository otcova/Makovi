use super::*;

impl FunctionParser<'_, '_> {
    pub fn expect_token(&mut self, token: Token) -> Result<(), ()> {
        if self.lexer.token() != token {
            self.unexpected_token(token.get_str());
            return Err(());
        }
        Ok(())
    }

    /// Use to skip the tokens of a statement after a syntax error
    pub fn skip_statement(&mut self) {
        // Consume remaining statement tokens
        while !matches!(self.lexer.token(), Token::EndOfFile | Token::NewLine) {
            self.consume_invalid_tokens();
            self.lexer.next();
        }

        // Consume white space
        while self.lexer.token() == Token::NewLine {
            self.lexer.next();
        }
    }

    pub fn consume_invalid_tokens(&mut self) {
        while let Token::Invalid = self.lexer.token() {
            self.errors.push(CompilationError {
                span: self.lexer.span(),
                message: format!("Invalid token '{:?}'", self.lexer.slice()),
            });
            self.lexer.next();
        }
    }

    pub fn unexpected_token(&mut self, expected: &str) {
        if self.lexer.token() == Token::EndOfFile {
            self.errors.push(CompilationError {
                span: self.lexer.span(),
                message: format!("Expected {expected}, but reached end of file"),
            });
        } else {
            self.errors.push(CompilationError {
                span: self.lexer.span(),
                message: format!("Expected {expected}, but found '{:?}'", self.lexer.slice()),
            });
        }
    }

    /// Makes sure that the statement has ended. Aka, makes sure that the line has ended.
    /// If not, it will do `Self::skip_statement`.
    pub fn end_statement(&mut self) {
        if !matches!(self.lexer.token(), Token::NewLine | Token::EndOfFile) {
            self.unexpected_token("a new line");
        }
        self.skip_statement();
    }
}
