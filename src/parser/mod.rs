mod ast;
mod peg_parser;

pub use ast::*;
use peg_parser::parser;

#[derive(Default)]
pub struct ParserContext {}

impl ParserContext {
    pub fn parse<'c>(&'c self, code: &'c str, ast: &'c Ast<'c>) -> Result<(), String> {
        parser::function(code, ast).map_err(|e| format!("Parsing error: {}", e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    gen_tests! {
    fn(b, code, test_name) {
        let ast = Ast::default();
        let parser = ParserContext::default();

        parser.parse(code, &ast).unwrap();
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

        b.iter(|| {
            ast.clear();
            parser.parse(code, &ast).unwrap()
        });
    }}
}
