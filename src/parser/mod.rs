mod ast;
mod peg_parser;

use std::cell::RefCell;

pub use ast::*;
use peg_parser::parser;

#[derive(Default)]
pub struct ParserContext {
    ast_context: AstContext,
}

impl ParserContext {
    pub fn parse<'c>(&'c mut self, code: &'c str) -> Result<Ast<'c>, String> {
        let mut ast = self.ast_context.create_ast();
        parser::function(code, &RefCell::new(&mut ast))
            .map_err(|e| format!("Parsing error: {}", e))?;
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
        let mut parser = ParserContext::default();

        {
            // We parse twice to test that the context cache is cleared
            parser.parse(code).unwrap();
            let ast = parser.parse(code).unwrap();

            let parsed = format!("{}", ast);

            let expected = &load_src(test_name, ".ast");
            if expected.trim() != parsed.trim() {
                println!("Expected:");
                println!("{expected}");
                println!();
                println!("But was:");
                println!("{parsed}");
                panic!();
            }
        }

        b.iter(|| {
            let ast = parser.parse(code).unwrap();
            black_box(ast).size()
        });
    }
}
