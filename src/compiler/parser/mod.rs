use super::error::CompilationError;
use super::hir::*;
use super::lexer::{Lexer, LexerStage, Token};
use super::CompilationErrorSet;
use scope::FunctionScope;

mod expressions;
mod parameters;
mod scope;
mod tokens;

pub struct ParserStage<'code, 'compiler> {
    pub errors: &'compiler mut CompilationErrorSet,
    pub definitions: ModuleDefinitions<'code>,
    pub entry_point: FnDefinitionId,
}

impl<'code> LexerStage<'code> {
    pub fn parser_stage<'compiler>(
        mut self,
        errors: &'compiler mut CompilationErrorSet,
        external_definitions: ExternalDefinitions,
    ) -> ParserStage<'code, 'compiler> {
        while self.lexer.token() == Token::NewLine {
            self.lexer.next();
        }

        let mut module = ModuleDefinitions::new(external_definitions);
        let mut parser = FunctionParser {
            lexer: &mut self.lexer,
            function: FnDefinition::new("", 0),
            module: &mut module,
            errors,
            scope: FunctionScope::default(),
        };

        parser.parse_block(0);

        while parser.expect_token(Token::EndOfFile).is_err() {
            parser.parse_block(0);
        }

        let entry_point = parser
            .module
            .define(parser.function)
            .expect("Only the entry point function should have the empty string as it's name.");

        ParserStage {
            errors,
            definitions: module,
            entry_point,
        }
    }
}

struct FunctionParser<'r, 'code> {
    lexer: &'r mut Lexer<'code>,
    module: &'r mut ModuleDefinitions<'code>,
    errors: &'r mut CompilationErrorSet,
    function: FnDefinition<'code>,
    scope: FunctionScope<'code>,
}

impl FunctionParser<'_, '_> {
    fn parse_block(&mut self, block_nesting: usize) {
        let mut nesting = self.nesting(block_nesting);

        while nesting == block_nesting && self.lexer.token() != Token::EndOfFile {
            let _ = self.parse_statement(nesting);
            nesting = self.nesting(block_nesting);
        }

        self.scope.set_nesting(nesting);
    }

    fn parse_statement(&mut self, nesting: usize) -> Result<(), ()> {
        match self.lexer.token() {
            Token::Fn => {
                self.lexer.next(); // fn

                let name = if self.expect_token(Token::Identifier).is_ok() {
                    Some(self.lexer.slice())
                } else {
                    None
                };
                let name_span = self.lexer.span();
                self.lexer.next(); // Identifier

                let mut scope = FunctionScope::default();

                let parameters = self.parse_parameters(&mut scope).ok();
                self.end_statement();

                let mut parser = FunctionParser {
                    lexer: self.lexer,
                    module: self.module,
                    errors: self.errors,
                    function: FnDefinition::new(
                        name.unwrap_or_default(),
                        parameters.unwrap_or_default(),
                    ),
                    scope,
                };
                parser.parse_block(nesting + 1);

                if name.is_some() && parser.module.define(parser.function).is_err() {
                    self.errors.push(CompilationError {
                        message: format!("Function {name:?} already defined"),
                        span: name_span,
                    });
                }
            }
            Token::Return => {
                self.lexer.next(); // return

                let value = if !matches!(self.lexer.token(), Token::NewLine | Token::EndOfFile) {
                    self.parse_expression()
                } else {
                    Variable::Const(Value::Null)
                };

                self.end_statement(); // return <value> \n
                self.function.push(Instruction::Return(value));
            }
            Token::Let => {
                self.lexer.next(); // let

                self.expect_token(Token::Identifier)
                    .map_err(|_| self.skip_statement())?;
                let name = self.lexer.slice();
                self.lexer.next(); // var_name

                self.expect_token(Token::Assign)
                    .map_err(|_| self.skip_statement())?;
                self.lexer.next(); // =

                let value = self.parse_expression();
                self.end_statement(); // let name = <expr> \n

                let variable = if let Variable::Id(variable) = value {
                    variable
                } else {
                    let variable = self.function.new_variable();
                    self.function.push(Instruction::Assign { variable, value });
                    variable
                };

                self.scope.define_variable(nesting, name, variable);
            }
            Token::Identifier => self.parse_statement_expression(),
            Token::If => {
                let mut if_chain_size = 0;

                loop {
                    if_chain_size += 1;
                    self.lexer.next(); // if

                    // if condition
                    let condition = self.parse_expression();
                    self.end_statement(); // if <cond> \n
                    self.function
                        .push(Instruction::IfStart(condition, RunIf::True));

                    // if body
                    self.parse_block(nesting + 1);

                    // else / else if
                    if self.lexer.token() == Token::Else {
                        self.lexer.next(); // else
                        self.function.push(Instruction::Else);

                        if self.lexer.token() == Token::If {
                            continue; // else if
                        }

                        // else body
                        self.end_statement(); // else \n
                        self.parse_block(nesting + 1);
                    }

                    break;
                }

                self.function.push(Instruction::IfEnd(if_chain_size));
            }
            Token::Else => {
                self.unexpected_token("if before else");
                self.skip_statement();
            }
            Token::While => {
                self.lexer.next(); // while

                self.function.push(Instruction::LoopStart);

                // While condition
                let condition = self.parse_expression();
                self.end_statement(); // while <cond> \n

                self.function
                    .push(Instruction::IfStart(condition, RunIf::False));
                self.function.push(Instruction::Break);
                self.function.push(Instruction::IfEnd(1));

                // Execute the main body
                self.parse_block(nesting + 1);

                self.function.push(Instruction::LoopEnd);
            }
            _ => {
                self.unexpected_token("statement");
                self.skip_statement();
            }
        };

        Ok(())
    }

    /// Validates for correct indentation and returns the
    /// current nesting. In case of error, an estimation will be provided.
    fn nesting(&mut self, max_nesting: usize) -> usize {
        const INDENT_SIZE: usize = 4;

        let indent = self.lexer.indent();
        let nesting = indent / INDENT_SIZE;

        if indent % INDENT_SIZE != 0 {
            self.errors.push(CompilationError {
                message: format!(
                    "Invalid indentation of {} spaces. Expected an indentation multiple of {}",
                    indent, INDENT_SIZE
                ),
                span: self.lexer.indent_span(),
            });

            return max_nesting.min(nesting + 1);
        }

        if nesting > max_nesting {
            self.errors.push(CompilationError {
                message: format!("Expected nesting of {}, but found {}", max_nesting, nesting),
                span: self.lexer.indent_span(),
            });
            return max_nesting;
        }

        nesting
    }
}
