mod buffers;
mod peg_parser;

use crate::ast::*;
use buffers::*;
use peg_parser::parser;

#[derive(Default)]
pub struct ParserContext<'a> {
    buffers: ReusablePool<VecExpr<'a>>,
}

impl<'a> ParserContext<'a> {
    pub fn parse<'c: 'a>(
        &'c self,
        code: &'c str,
        arena: &ExprArena<'c>,
    ) -> Result<FunctionAst<'c>, String> {
        parser::function(code, arena, &self.buffers).map_err(|e| format!("Parsing error: {}", e))
    }
}
