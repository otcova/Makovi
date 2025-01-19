#![feature(vec_into_raw_parts)]
#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

mod ast;
mod error;
mod ir;
mod jit;
mod lexer;
mod parser;
mod utils;

use jit::*;
use parser::*;

pub struct MakoviJit<In, Out> {
    parser: Parser,
    jit: Jit,
    fn_ptr: fn(In) -> Out,
}

impl<In, Out> Default for MakoviJit<In, Out> {
    fn default() -> Self {
        MakoviJit {
            jit: Jit::default(),
            parser: Parser::default(),
            fn_ptr: |_| panic!("Function not loaded!"),
        }
    }
}

impl<In, Out> MakoviJit<In, Out> {
    pub fn write_ast(&mut self, code: &str) -> Result<String, String> {
        Ok(format!("{}", self.parser.parse(code)?))
    }

    pub fn write_ir(&mut self, code: &str) -> Result<String, String> {
        let ast = self.parser.parse(code)?;
        self.jit.write_ir(&ast)
    }

    pub fn load_code(&mut self, code: &str) -> Result<(), String> {
        let ast = self.parser.parse(code)?;
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

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use test::Bencher;

    use super::*;
    use crate::utils::test_utils::*;

    gen_tests!(generic_test(bench, code, test_name, input, output));

    fn generic_test<In, Out>(
        b: &mut Bencher,
        code: &str,
        _test_name: &str,
        input: In,
        expected_output: Out,
    ) where
        In: Clone,
        Out: Debug + PartialEq,
    {
        b.iter(|| {
            let mut jit = MakoviJit::<In, Out>::default();
            jit.load_code(code).unwrap();
            assert_eq!(expected_output, jit.run_code(input.clone()));
        });
    }
}
