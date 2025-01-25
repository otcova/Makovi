use super::statement_stage::*;
use super::*;
use cranelift::prelude::isa::x64::settings;

impl<'a> Parser<'a> {
    pub fn parse(mut self) -> Result<Ast<'a>, CompilationErrorSet> {
        self.peek_statement = self.next_satement();
        self.parse_code_block(0);

        if self.errors.is_empty() {
            Ok(self.ast)
        } else {
            Err(self.errors)
        }
    }

    fn parse_code_block(&mut self, block_nesting: usize) -> ExprPtr {
        let mut first_statement = NULL_EXPR_PTR;
        let mut previous_statement = NULL_EXPR_PTR;

        while let Some(statement) = self.peek_statement {
            // Has code block ended?
            if statement.nesting < block_nesting {
                break;
            }

            self.peek_statement = self.next_satement();

            // Is the code more nested than it should?
            if statement.nesting > block_nesting {
                self.errors.add(CompilationError {
                    span: statement.span,
                    message: format!(
                        "Invalid nesting. Expected {}, but found {}",
                        block_nesting, statement.nesting
                    ),
                });
                continue;
            }

            let Some(statement_expr) = self.parse_full_statement(statement) else {
                continue;
            };

            let statement = self
                .ast
                .push(Expr::Statements(statement_expr, NULL_EXPR_PTR));

            // Update the next pointer of the previous_statement
            if previous_statement == NULL_EXPR_PTR {
                first_statement = statement;
            } else {
                let Expr::Statements(_, next) = &mut self.ast[previous_statement] else {
                    // previous_statement is only assigned to NULL or statement
                    unreachable!();
                };
                *next = statement;
            }

            previous_statement = statement;
        }

        first_statement
    }

    fn parse_full_statement(&mut self, statement: Statement<'a>) -> Option<ExprPtr> {
        Some(match statement.class {
            StatementClass::Declaration(variable_name, value) => {
                assert!(value != NULL_EXPR_PTR);

                if matches!(self.ast[value], Expr::Function { .. }) {
                    let fn_body = self.parse_code_block(statement.nesting + 1);

                    // We need to get the pointer to the expr after the `parse_code_block`
                    // since the inner Vec of the ast could grow and reallocate.
                    let Expr::Function {
                        parameters: _,
                        body,
                        name,
                    } = &mut self.ast[value]
                    else {
                        // They type of an ExprPtr is never changed
                        unreachable!();
                    };

                    *body = fn_body;
                    *name = variable_name;
                    value
                } else {
                    self.ast.push(Expr::Assign(variable_name, value))
                }
            }
            StatementClass::AssignOperation(ident, operator, value) => {
                let var = self.ast.push(Expr::Variable(ident));
                let op = self.ast.push(Expr::HeadOperation {
                    lhs: var,
                    operator,
                    rhs: value,
                    next: NULL_EXPR_PTR,
                });
                self.ast.push(Expr::Assign(ident, op))
            }
            StatementClass::Return(value) => self.ast.push(Expr::Return(value)),
            StatementClass::If(condition) => {
                let then_body = self.parse_code_block(statement.nesting + 1);
                let else_body = self.parse_else_if_chain(statement.nesting);

                self.ast.push(Expr::IfElse {
                    condition,
                    then_body,
                    else_body,
                })
            }
            StatementClass::ElseIf(_) => {
                self.errors.add(CompilationError {
                    message: "Expected an if before the else if".to_owned(),
                    span: statement.span,
                });
                return None;
            }
            StatementClass::Else => {
                self.errors.add(CompilationError {
                    message: "Expected an if before the else".to_owned(),
                    span: statement.span,
                });
                return None;
            }
            StatementClass::While(condition) => {
                let body = self.parse_code_block(statement.nesting + 1);
                self.ast.push(Expr::WhileLoop { condition, body })
            }
            StatementClass::Atom(expr) => expr,
        })
    }

    fn parse_else_if_chain(&mut self, nesting: usize) -> ExprPtr {
        let Some(statement) = self.peek_statement else {
            return NULL_EXPR_PTR;
        };

        if statement.nesting != nesting {
            return NULL_EXPR_PTR;
        }

        match statement.class {
            StatementClass::ElseIf(condition) => {
                self.peek_statement = self.next_satement();
                let then_body = self.parse_code_block(statement.nesting + 1);
                let else_body = self.parse_else_if_chain(statement.nesting);
                self.ast.push(Expr::IfElse {
                    condition,
                    then_body,
                    else_body,
                })
            }
            StatementClass::Else => {
                self.peek_statement = self.next_satement();
                self.parse_code_block(statement.nesting + 1)
            }
            _ => NULL_EXPR_PTR,
        }
    }
}
