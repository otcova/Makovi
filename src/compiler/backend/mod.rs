use super::hir::{FnDefinitionStage, ParamsTypes};
use super::*;
use backend_value::*;
use compilation::*;
use cranelift::codegen::ir::FuncRef;
use cranelift::codegen::Context;
use cranelift::prelude::*;
use cranelift_module::{FuncId, Linkage, Module};
pub use executable::*;
use type_inference::TypeInferenceStage;

mod backend_value;
mod compilation;
mod executable;

impl<'compiler> TypeInferenceStage<'_, 'compiler> {
    pub fn backend_stage(
        self,
        compiler: &mut BackendCompiler,
    ) -> Result<Executable, &'compiler CompilationErrorSet> {
        if !self.errors.is_empty() {
            return Err(self.errors);
        }

        let Some(entry_point_id) = self
            .code
            .instances
            .get_id(self.entry_point.into(), ParamsTypes::default())
        else {
            unreachable!("Only code with an entry point should be provided");
        };
        let hir::FnInstanceStage::Ok(entry_point_instance) = &self.code.instances[entry_point_id]
        else {
            unreachable!("Only code with an entry point should be provided");
        };

        compiler.compile(&self.code);

        let func_id = compiler.func_ids[*entry_point_id];
        let return_type = entry_point_instance.return_type;
        Ok(Executable::new(func_id, return_type))
    }
}

/// In order to reduce memory reallocations when compiling multiple times,
/// [`BackendCompiler`] holds various data structures which are cleared between
/// functions, rather than dropped, preserving the underlying allocations.
pub struct BackendCompiler {
    module: JITModule,
    context: Context,
    builder_context: FunctionBuilderContext,

    variables: Vec<BackendVariable>,
    func_ids: Vec<FuncId>,
    func_refs: Vec<FuncRef>,
    /// Used to temporally create a slice of value. For example, to store the
    /// arguments of a function call while the instruction is beeing builded.
    values_buffer: Vec<Value>,
    name_buffer: String,
    if_scope_stack: Vec<IfScope>,
    loop_scope_stack: Vec<LoopScope>,
}

impl BackendCompiler {
    pub fn new(module: JITModule) -> Self {
        Self {
            context: module.make_context(),
            builder_context: FunctionBuilderContext::new(),
            module,

            variables: Default::default(),
            func_ids: Default::default(),
            func_refs: Default::default(),
            values_buffer: Default::default(),
            name_buffer: Default::default(),
            if_scope_stack: Default::default(),
            loop_scope_stack: Default::default(),
        }
    }

    // #[cfg(test)]
    // fn write_ir(&mut self, code: &hir::HirModule) -> String {
    //     use cranelift::codegen;
    //     use std::fmt::Write;
    //
    //     let mut ir = String::new();
    //
    //     for (id, declaration) in code.instances.iter_declarations() {
    //         let _ = writeln!(&mut ir, "; {declaration}");
    //
    //         let Some(definition) = code.definitions.get(declaration.name) else {
    //             unreachable!("Found undefined instance"); // The backend should only work with correct code
    //         };
    //         self.compile_function(code, definition, id);
    //         codegen::write_function(&mut ir, &self.context.func).unwrap();
    //     }
    //
    //     ir
    // }

    fn declare_functions(&mut self, code: &hir::CodeModule) {
        self.func_ids.clear();

        for (_, stage) in code.instances.iter() {
            let hir::FnInstanceStage::Ok(instance) = stage else {
                unreachable!("The backend should only work with correct code");
            };

            self.load_signature(instance);

            let signature = &mut self.context.func.signature;

            let linkage = match instance.external_ptr {
                Some(_) => Linkage::Import,
                None => Linkage::Export,
            };

            instance.name(&mut self.name_buffer);
            let id = self
                .module
                .declare_function(&self.name_buffer, linkage, signature)
                .expect("Invalid instance signature");

            self.func_ids.push(id);
        }
    }

    fn compile(&mut self, code: &hir::CodeModule) {
        self.declare_functions(code);

        for (instance, declaration) in code.instances.iter_local_declarations() {
            let definition = match &code.definitions[declaration.fn_id] {
                FnDefinitionStage::Defined(definition) => definition,
                FnDefinitionStage::ToDefine { name } => {
                    unreachable!("Found undefined function instance {name:?}");
                }
            };

            let id = self.compile_function(code, definition, instance);

            if self.module.define_function(id, &mut self.context).is_err() {
                unreachable!("The backend should only work with correct code");
            }
        }

        if self.module.finalize_definitions().is_err() {
            unreachable!("The backend should only work with correct code");
        }
    }

    fn compile_function(
        &mut self,
        code: &hir::CodeModule,
        definition: &hir::FnDefinition,
        instance_id: hir::FnInstanceId,
    ) -> FuncId {
        let hir::FnInstanceStage::Ok(instance) = &code.instances[instance_id] else {
            unreachable!("The backend should only work with correct code");
        };

        self.load_signature(instance);

        let builder = FunctionBuilder::new(&mut self.context.func, &mut self.builder_context);

        FnBackendCompiler {
            module: &mut self.module,
            builder,
            variables: &mut self.variables,
            instructions: definition.instructions.iter(),
            instance,
            definition,
            func_ids: &self.func_ids,
            func_refs: &mut self.func_refs,
            values_buffer: &mut self.values_buffer,
            instruction_flow: InstructionFlow::default(),
            if_scope_stack: &mut self.if_scope_stack,
            loop_scope_stack: &mut self.loop_scope_stack,
        }
        .compile();

        self.func_ids[*instance_id]
    }

    fn load_signature(&mut self, instance: &hir::FnInstance) {
        // We clear out context state before using it.
        self.module.clear_context(&mut self.context);

        for param in instance.parameters() {
            for param_type in BackendValueType::from(*param).types() {
                let abi = AbiParam::new(param_type);
                self.context.func.signature.params.push(abi);
            }
        }

        for return_type in BackendValueType::from(instance.return_type).types() {
            let abi = AbiParam::new(return_type);
            self.context.func.signature.returns.push(abi);
        }
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::compiler::parser::*;
//     use crate::utils::test_utils::*;
//     use ::test::*;
//     use cranelift_jit::{JITBuilder, JITModule};
//
//     gen_tests!(generic_test(bench, code, test_name));
//
//     fn generic_test(_b: &mut Bencher, code: &str, test_name: &str) {
//         let mut parser = ParserContext::default();
//         let ast = parser.new_parser(code).parse().unwrap();
//
//         let flags_builder = settings::builder();
//         // flags_builder.set("opt_level", "speed").unwrap();
//         let flags = settings::Flags::new(flags_builder);
//
//         let isa_builder = isa::lookup_by_name("x86_64").unwrap();
//         let isa = isa_builder.finish(flags).unwrap();
//
//         let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
//         let module = JITModule::new(builder);
//         let mut code = IrGenerator::new(module);
//
//         ////////////////////////////
//
//         let ir = &code.write_ir(&ast).unwrap();
//
//         let expected = &load_src(test_name, ".ir.run");
//         assert_source_eq(expected, ir);
//
//         // TODO: Bench compilation
//         // b.iter(|| code.load(black_box(&ast)).unwrap());
//     }
// }
