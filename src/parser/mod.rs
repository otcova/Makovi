mod peg_parser;

use crate::ast::*;
use peg_parser::parser;

#[derive(Default)]
pub struct ParserContext {}

impl ParserContext {
    pub fn parse<'c>(
        &'c self,
        code: &'c str,
        ast: &'c Ast<'c>,
    ) -> Result<FunctionExpr<'c>, String> {
        parser::function(code, ast).map_err(|e| format!("Parsing error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let code = include_str!("../../code_samples/smallest_factor.run");
        let expected = include_str!("../../code_samples/smallest_factor.ast.rs").trim();

        let ast = Ast::default();
        let ctx = ParserContext::default();

        ctx.parse(code, &ast).unwrap();

        assert_eq!(expected, &format!("{:?}", ast.nodes));
    }
}
