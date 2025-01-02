mod peg_parser;

use crate::ast::*;
use peg_parser::parser;

#[derive(Default)]
pub struct ParserContext {}

impl ParserContext {
    pub fn parse<'c>(
        &'c self,
        code: &'c str,
        arena: &'c Ast<'c>,
    ) -> Result<FunctionExpr<'c>, String> {
        parser::function(code, arena).map_err(|e| format!("Parsing error: {}", e))
    }
}
