mod peg_parser;

use crate::ast::*;
use peg_parser::parser;

#[derive(Default)]
pub struct ParserContext {}

impl ParserContext {
    pub fn parse<'c>(&'c self, code: &'c str, ast: &'c Ast<'c>) -> Result<ExprPtr, String> {
        parser::function(code, ast).map_err(|e| format!("Parsing error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    macro_rules! gen_test {
        ($($name:ident)*) => {$(
            #[test]
            fn $name() {
                parse(stringify!($name));
            }
        )*};
    }

    gen_test! {
        old_mult
        // smallest_factor
    }

    fn parse(name: &str) {
        let code = &fs::read_to_string(format!("code_samples/{name}.run")).unwrap();
        let expected =
            fs::read_to_string(format!("code_samples/{name}.ast.run")).unwrap_or_default();
        let expected = expected.trim();

        let ast = Ast::default();
        let ctx = ParserContext::default();

        ctx.parse(code, &ast).unwrap();
        let parsed = format!("{}", ast);
        let parsed = parsed.trim();

        if expected != parsed {
            println!("Expected:");
            println!("{expected}");
            println!();
            println!("But was:");
            println!("{parsed}");
            panic!();
        }
    }
}
