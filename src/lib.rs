// #![feature(libstd_sys_internals, rt)]

pub mod ast;
pub mod parser;
//pub mod ir;
//pub mod jit;
/*
use ast::FunctionAST;
use jit::*;

pub struct MakoviJIT<In, Out> {
    jit: JIT,
    fn_ptr: fn(In) -> Out,
}

impl<In, Out> Default for MakoviJIT<In, Out> {
    fn default() -> Self {
        MakoviJIT {
            jit: JIT::default(),
            fn_ptr: |_| panic!("Function not loaded!"),
        }
    }
}

impl<In, Out> MakoviJIT<In, Out> {
    pub fn load_function(&mut self, code: &str) -> Result<(), String> {
        let function_ast = FunctionAST::parse(code)?;

        let ptr = self.jit.compile_function(function_ast)?;

        unsafe {
            self.fn_ptr = std::mem::transmute::<*const u8, fn(In) -> Out>(ptr);
        }
        Ok(())
    }

    pub fn run_code(&mut self, input: In) -> Out {
        (self.fn_ptr)(input)
    }
}
*/
