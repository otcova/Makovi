use crate::ast::*;
use crate::ir::*;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};

/// The basic JIT class.
pub struct Jit {
    code: CodeIr<JITModule>,
}

impl Default for Jit {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        // TODO: flag_builder.set("opt_level", "speed_and_size").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        let module = JITModule::new(builder);

        Self {
            code: CodeIr::new(module),
        }
    }
}

impl Jit {
    /// Compile a string in the toy language into machine code.
    pub fn write_ir(&mut self, ast: &Ast) -> Result<String, String> {
        self.code.write_ir(ast)
    }

    /// Compile a string in the toy language into machine code.
    pub fn compile<'a>(&mut self, ast: &'a Ast<'a>) -> Result<*const u8, String> {
        let id = self.code.load(ast)?;

        // Finalize the functions which we just defined, which resolves any
        // outstanding relocations (patching in addresses, now that they're
        // available).
        self.code.module.finalize_definitions().unwrap();

        // We can now retrieve a pointer to the machine code.
        let code = self.code.module.get_finalized_function(id);

        Ok(code)
    }
}

#[cfg(test)]
mod run {
    use super::*;
    use crate::parser::*;
    use crate::utils::test_utils::*;
    use ::test::*;
    use std::fmt::Debug;

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
        let mut parser = ParserContext::default();
        let ast = parser.new_parser(code).parse().unwrap();

        let mut jit = Jit::default();
        let ptr = jit.compile(&ast).unwrap();

        let jit_fn = unsafe { std::mem::transmute::<*const u8, fn(In) -> Out>(ptr) };

        assert_eq!(expected_output, jit_fn(black_box(input.clone())));

        b.iter(|| jit_fn(black_box(input.clone())));

        assert_eq!(
            expected_output,
            jit_fn(black_box(input.clone())),
            "Did you forget to clear the cache?"
        );
    }
}
