#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

mod ir;
mod jit;
mod parser;

#[cfg(test)]
mod test_utils;

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
    pub fn gen_ir(&mut self, code: &str) -> Result<String, String> {
        let ast = Ast::default();
        self.parser.parse(code, &ast)?;
        self.jit.gen_ir(&ast)
    }

    pub fn load_code(&mut self, code: &str) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    // fn test_file<I, O>(b: &mut Bencher, code: &str, name: &str) {

    gen_tests! {
    fn(b, code, test_name, input: In, expected_output: Out) {
        b.iter(|| {
            let mut jit = MakoviJIT::<In, Out>::default();
            jit.load_code(code).unwrap();
            assert_eq!(expected_output, jit.run_code(input.clone()));
        });
    }}
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_utils::*;
//
//     gen_tests! {
//     fn(b, code, test_name, input: In, expected_output: Out) {
//         let mut jit = MakoviJIT::<In, Out>::default();
//         jit.load_code(code).unwrap();
//         assert_eq!(expected_output, jit.run_code(input));
//
//         b.iter(|| {
//             ast.clear();
//             parser.parse(code, &ast).unwrap()
//         });
//     }}
// }
