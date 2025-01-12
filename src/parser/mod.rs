mod parse;

use crate::ast::*;
use crate::error::*;

#[derive(Default)]
pub struct Parser {
    ast_context: AstContext,
}

impl Parser {
    pub fn parse<'c>(&'c mut self, code: &'c str) -> Result<Ast<'c>, CompilationError> {
        let mut ast = self.ast_context.create_ast();
        ast.parse(code)?;
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;
    use ::test::*;

    gen_tests!(generic_test(bench, code, test_name));

    fn generic_test(b: &mut Bencher, code: &str, test_name: &str) {
        let expected = &load_src(test_name, ".ast");
        let mut parser = Parser::default();

        let ast = &format!("{}", parser.parse(code).unwrap());
        assert_source_eq(expected, ast);

        b.iter(|| {
            let ast = parser.parse(black_box(code)).unwrap();
            black_box(ast).size()
        });

        eprintln!("Did you forget to clear the cache?");
        assert_source_eq(ast, &format!("{}", parser.parse(code).unwrap()));
    }
}
