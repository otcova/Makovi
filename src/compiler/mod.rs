use crate::*;
use backend::{BackendCompiler, Executable};
use cranelift_jit::{JITBuilder, JITModule};
pub use error::CompilationErrorSet;

mod backend;
mod error;
mod hir;
mod lexer;
mod parser;
pub mod symbols;
mod type_inference;

pub struct Compiler<R: RuntimeModule> {
    errors: CompilationErrorSet,
    backend: BackendCompiler,
    runtime: R,
}

pub struct CompilationPipeline;

impl<R: RuntimeModule> Compiler<R> {
    pub fn new(runtime: R) -> Self {
        let flags = &[
            ("opt_level", "speed"), // "none" / "speed_and_size"
        ];
        let builder =
            JITBuilder::with_flags(flags, cranelift_module::default_libcall_names()).unwrap();
        // builder.symbo();

        let module = JITModule::new(builder);

        Self {
            backend: BackendCompiler::new(module),
            errors: CompilationErrorSet::default(),
            runtime,
        }
    }

    pub fn compile(&mut self, source_code: &str) -> Result<Executable, &CompilationErrorSet> {
        self.errors.clear();

        CompilationPipeline
            .lexer_stage(source_code)
            .parser_stage(&mut self.errors)
            .type_inference_stage()
            .backend_stage(&mut self.backend)
    }

    pub fn run(&mut self, executable: &Executable) -> hir::Value {
        self.backend.run(executable)
    }

    pub fn runtime(&self) -> &R {
        &self.runtime
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::test_utils::{assert_source_eq, load_source};

    #[bench]
    pub fn stages(_: &mut test::Bencher) {
        let runtime = BaseModule::default();
        let mut compiler = Compiler::new(runtime);
        let pipeline = CompilationPipeline;
        let source_code = &load_source("example.rb");

        let pipeline = pipeline.lexer_stage(source_code);

        let pipeline = pipeline.parser_stage(&mut compiler.errors);
        print!("{:?}", pipeline.errors);
        assert_source_eq("example.parsed.run", &pipeline.definitions);

        let pipeline = pipeline.type_inference_stage();
        print!("{:?}", pipeline.errors);
        assert_source_eq("example.instanced.run", &pipeline.code.instances);

        let executable = pipeline
            .backend_stage(&mut compiler.backend)
            .expect("Backend Stage");

        let result = compiler.run(&executable);
        assert!(matches!(result, hir::Value::Int(123)));

        assert!(
            compiler.errors.is_empty(),
            "The compiler ended with errors but did not fail. {:?}",
            compiler.errors
        );
    }
}
