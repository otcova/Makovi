use crate::ir::*;
use crate::parser::*;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};
use std::slice;

/// The basic JIT class.
pub struct JIT {
    code: CodeIR<JITModule>,

    /// The data description, which is to data objects what `ctx` is to functions.
    data_description: DataDescription,
}

impl Default for JIT {
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

        let code = CodeIR {
            ctx: module.make_context(),
            builder_context: FunctionBuilderContext::new(),
            module,
        };

        Self {
            code,
            data_description: DataDescription::new(),
        }
    }
}

impl JIT {
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

    /// Create a zero-initialized data section.
    pub fn create_data(&mut self, name: &str, contents: Vec<u8>) -> Result<&[u8], String> {
        // The steps here are analogous to `compile`, except that data is much
        // simpler than functions.
        self.data_description.define(contents.into_boxed_slice());
        let id = self
            .code
            .module
            .declare_data(name, Linkage::Export, true, false)
            .map_err(|e| e.to_string())?;

        self.code
            .module
            .define_data(id, &self.data_description)
            .map_err(|e| e.to_string())?;
        self.data_description.clear();
        self.code.module.finalize_definitions().unwrap();
        let buffer = self.code.module.get_finalized_data(id);
        // TODO: Can we move the unsafe into cranelift?
        Ok(unsafe { slice::from_raw_parts(buffer.0, buffer.1) })
    }
}
