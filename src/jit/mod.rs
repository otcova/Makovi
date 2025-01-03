use crate::ir::*;
use crate::parser::*;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;

/// The basic JIT class.
pub struct Jit {
    code: CodeIr<JITModule>,
}

impl Default for Jit {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        let module = JITModule::new(builder);

        let code = CodeIr {
            ctx: module.make_context(),
            builder_context: FunctionBuilderContext::new(),
            module,
        };

        Self { code }
    }
}

impl Jit {
    pub fn gen_ir<'a>(&mut self, ast: &'a Ast<'a>) -> Result<String, String> {
        self.code.load(ast)?;
        Ok(self.code.write_ir())
    }

    /// Compile a string in the toy language into machine code.
    pub fn compile<'a>(&mut self, ast: &'a Ast<'a>) -> Result<*const u8, String> {
        let id = self.code.load(ast)?;

        // Define the function to jit. This finishes compilation, although
        // there may be outstanding relocations to perform. Currently, jit
        // cannot finish relocations until all functions to be called are
        // defined. For this toy demo for now, we'll just finalize the
        // function below.
        self.code
            .module
            .define_function(id, &mut self.code.ctx)
            .map_err(|e| format!("Compilation error: {}", e))?;

        // Now that compilation is finished, we can clear out the context state.
        self.code.module.clear_context(&mut self.code.ctx);

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
        let ast = parser.parse(code).unwrap();

        let mut jit = Jit::default();
        let ptr = jit.compile(&ast).unwrap();

        let jit_fn = unsafe { std::mem::transmute::<*const u8, fn(In) -> Out>(ptr) };

        assert_eq!(expected_output, jit_fn(black_box(input.clone())));

        b.iter(|| jit_fn(black_box(input.clone())));

        // Check for wierd inner state
        assert_eq!(expected_output, jit_fn(black_box(input.clone())));
    }
}
