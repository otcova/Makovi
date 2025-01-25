//! Parses tokens into a syntax tree based on grammar rules.

mod block_stage;
mod statement_stage;

use crate::error::*;
use crate::{ast::*, lexer::Lexer};

#[derive(Default)]
pub struct ParserContext {
    ast_context: AstContext,
}

pub struct Parser<'a> {
    ast: Ast<'a>,
    lexer: Lexer<'a>,
    errors: CompilationErrorSet,

    // Current statement being parsed
    peek_statement: Option<Statement<'a>>,
}

impl ParserContext {
    pub fn new_parser<'c>(&'c mut self, code: &'c str) -> Parser<'c> {
        Parser {
            ast: self.ast_context.create_ast(),
            lexer: Lexer::new(code),
            errors: CompilationErrorSet::default(),
            peek_statement: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ParseError;
type ParseResult<T> = Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;
    use ::test::*;

    gen_tests!(generic_test(bench, code, test_name));

    fn generic_test(b: &mut Bencher, code: &str, test_name: &str) {
        let expected = &load_src(test_name, ".ast.run");
        let mut parser = ParserContext::default();

        let ast = &format!("{}", parser.new_parser(code).parse().unwrap());
        assert_source_eq(expected, ast);

        b.iter(|| {
            let ast = parser.new_parser(black_box(code)).parse().unwrap();
            black_box(ast).size()
        });

        eprintln!("Did you forget to clear the cache?");
        assert_source_eq(
            ast,
            &format!("{}", parser.new_parser(code).parse().unwrap()),
        );
    }
}
