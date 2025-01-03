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
    use std::iter;

    gen_tests!(generic_test(bench, code, test_name));

    fn generic_test(b: &mut Bencher, code: &str, test_name: &str) {
        let mut parser = ParserContext::default();

        let ast = format!("{}", parser.parse(code).unwrap());

        // We parse twice to test that the context cache is cleared
        let ast2 = format!("{}", parser.parse(code).unwrap());
        assert_eq!(ast, ast2);

        let expected = &load_src(test_name, ".ast");
        if expected.trim() != ast.trim() {
            println!("Expected:");
            println!("{expected}");
            println!();
            println!("But was:");
            println!("{}", compare_and_mark(expected, &ast));
            panic!();
        }

        b.iter(|| {
            let ast = parser.parse(black_box(code)).unwrap();
            black_box(ast).size()
        });
    }

    fn compare_and_mark(expected: &str, parsed: &str) -> String {
        let expected = expected.trim().lines();
        let parsed = parsed.trim().lines();

        let mut result = Vec::new();
        for (l1, l2) in parsed.zip(expected.chain(iter::repeat(""))) {
            if l1 == l2 {
                result.push(l1.to_string());
            } else {
                result.push(format!("\x1b[0;31m{l1}\x1b[0m"));
            }
        }

        result.join("\n")
    }
}
