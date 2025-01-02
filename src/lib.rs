pub mod ir;
pub mod jit;
pub mod parser;

use jit::*;
use parser::*;

pub struct MakoviJIT<In, Out> {
    parser: ParserContext,
    jit: JIT,
    fn_ptr: fn(In) -> Out,
}

impl<In, Out> Default for MakoviJIT<In, Out> {
    fn default() -> Self {
        MakoviJIT {
            jit: JIT::default(),
            parser: ParserContext::default(),
            fn_ptr: |_| panic!("Function not loaded!"),
        }
    }
}

impl<In, Out> MakoviJIT<In, Out> {
    pub fn load_function(&mut self, code: &str) -> Result<(), String> {
        let ast = Ast::default();
        self.parser.parse(code, &ast)?;
        let ptr = self.jit.compile(&ast)?;

        unsafe {
            self.fn_ptr = std::mem::transmute::<*const u8, fn(In) -> Out>(ptr);
        }
        Ok(())
    }

    pub fn run_code(&mut self, input: In) -> Out {
        (self.fn_ptr)(input)
    }
}
