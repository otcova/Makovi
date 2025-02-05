use super::*;
use crate::compiler::lexer::OperatorPriority;
use smallvec::SmallVec;

impl FunctionParser<'_, '_> {
    pub(super) fn parse_statement_expression(&mut self) {
        const IS_STATEMENT: bool = true;
        let lhs = self.parse_atom::<IS_STATEMENT>();
        self.parse_expression_rhs::<IS_STATEMENT>(lhs, 0);
        self.end_statement();
    }

    pub(super) fn parse_expression(&mut self) -> Variable {
        const IS_STATEMENT: bool = false;
        let lhs = self.parse_atom::<IS_STATEMENT>();
        self.parse_expression_rhs::<IS_STATEMENT>(lhs, 0)
    }

    /// This functions expects: expr, ..., expr)
    fn parse_expression_list(&mut self) -> SmallVec<Variable, 8> {
        let mut list = SmallVec::new();
        if self.lexer.token() == Token::BracketClose {
            return list;
        }

        loop {
            list.push(self.parse_expression());

            match self.lexer.token() {
                Token::Comma => {
                    self.lexer.next(); // ...<expr>,
                    continue;
                }
                Token::BracketClose => {
                    self.lexer.next(); // ...<expr>)
                    return list;
                }
                _ => {
                    self.unexpected_token(")"); // ...<expr> ?
                    return list;
                }
            }
        }
    }

    /// Parses an expression composed by operators of priority >= to `min_priority`
    /// The current token should be the operator in between lhs and rhs.
    fn parse_expression_rhs<const IS_STATEMENT: bool>(
        &mut self,
        lhs: Variable,
        min_priority: OperatorPriority,
    ) -> Variable {
        let operator = self.lexer.token();
        let operator_priority = operator.operator_priority();
        let Some(priority) = operator_priority.filter(|p| *p >= min_priority) else {
            // The expression has ended, or continues but with operators of lower priority
            return lhs;
        };

        let operator_name = self.lexer.slice();
        self.lexer.next();

        let rhs = self.parse_atom::<false>();
        let rhs = self.parse_expression_rhs::<false>(rhs, priority + 1);

        // TODO: Bench: Operators should have a pre assigned id.
        let definition_id = self.module.get_fn_id(operator_name);
        let result = self.function.new_variable();
        self.function
            .push_fn_call(definition_id, [lhs, rhs], Some(result));

        self.parse_expression_rhs::<IS_STATEMENT>(Variable::Id(result), min_priority)
    }

    fn parse_atom<const IS_STATEMENT: bool>(&mut self) -> Variable {
        match self.lexer.token() {
            Token::Identifier => {
                let name = self.lexer.slice();
                // TODO: get variable here. functions are variables
                self.lexer.next(); // skip Identifier

                match self.lexer.token() {
                    // Function Call
                    Token::BracketOpen => {
                        self.lexer.next(); // skip BracketOpen

                        let arguments = self.parse_expression_list();

                        let result = match IS_STATEMENT {
                            true => None,
                            false => Some(self.function.new_variable()),
                        };

                        let definition_id = self.module.get_fn_id(name);
                        self.function.push_fn_call(definition_id, arguments, result);

                        result.into()
                    }
                    // Assignment
                    Token::Assign if IS_STATEMENT => {
                        let variable = self.parse_variable(name);

                        self.lexer.next(); // skip Assign

                        let value = self.parse_expression();

                        if let Variable::Id(variable) = variable {
                            self.function.push(Instruction::Assign { variable, value });
                        }
                        Variable::Unknown
                    }
                    // Variable
                    _ => self.parse_variable(name),
                }
            }
            Token::Integer => {
                let Ok(value) = self.lexer.slice().parse::<i64>() else {
                    self.unexpected_token("i64 number");
                    self.lexer.next(); // skip Integer
                    return Variable::Unknown;
                };

                self.lexer.next(); // skip Integer
                Variable::Const(Value::Int(value))
            }
            Token::True => {
                self.lexer.next(); // skip True
                Variable::Const(Value::Bool(true))
            }
            Token::False => {
                self.lexer.next(); // skip False
                Variable::Const(Value::Bool(false))
            }

            Token::BracketOpen => {
                self.lexer.next(); // skip BracketOpen
                let value = self.parse_expression();
                if self.expect_token(Token::BracketClose).is_ok() {
                    self.lexer.next(); // skip BracketClose
                }
                value
            }

            // Lambda function Definition
            Token::Fn => todo!("Lambda's are not yet implemented"),

            // Errors
            _ => {
                self.unexpected_token("a variable");
                Variable::Unknown
            }
        }
    }

    pub fn parse_variable(&mut self, name: &str) -> Variable {
        let variable = self.get_variable(name);
        if variable.is_none() {
            self.errors.push(CompilationError {
                message: format!("Undefined variable {name:?}"),
                span: self.lexer.span(),
            });
        }
        variable.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_diff;

    #[test]
    pub fn parse_expression_with_operators() {
        const RESULT: &str = r#"
f0 = fn +(...) <not defined>
f1 = fn -(...) <not defined>
f2 = fn *(...) <not defined>
f3 = fn test(v0, v1, v2)
    v3 = local_f0(v0, v1)
    v4 = local_f1(v3, 3)
    v5 = local_f2(v2, 123)
    v6 = local_f0(v5, 2)
    v7 = local_f0(v4, v6)
"#;
        test_parse_expression("a + b - 3 + (hey * 123 + 2)", &["a", "b", "hey"], RESULT);
    }

    #[ignore = "Feature not implemented"]
    #[test]
    pub fn parse_expression_with_comparisons() {
        const RESULT: &str = r#"
f0 = fn +(...) <not defined>
f1 = fn <(...) <not defined>
f2 = fn >=(...) <not defined>
f3 = fn and(...) <not defined>
f4 = fn test(v0)
    v1 = local_f0(3, 9)
    v2 = local_f1(v0, v1)
    v3 = local_f2(v1, 1)
    v4 = local_f3(v2, v3)
"#;
        test_parse_expression("a < 3 + 9 >= 1", &["a"], RESULT);
    }

    fn test_parse_expression(expr: &str, variables: &[&str], expect: &str) {
        let mut lexer = Lexer::new(expr);
        let mut module = ModuleDefinitions::default();
        let mut errors = CompilationErrorSet::default();

        let mut parser = FunctionParser {
            lexer: &mut lexer,
            function: FnDefinition::new("test", variables.len()),
            module: &mut module,
            errors: &mut errors,
            scope: FunctionScope::default(),
        };

        for (id, name) in variables.iter().enumerate() {
            parser.scope.define_variable(0, name, id.into());
        }

        parser.parse_expression();
        let _ = parser.expect_token(Token::EndOfFile);
        parser.module.define(parser.function).unwrap();

        println!("{:?}", parser.errors);
        assert_diff(expect, format!("{}", parser.module));
        assert!(parser.errors.is_empty());
    }
}
