mod control_flow;
mod translations;

use crate::ast::*;
use codegen::Context;
use cranelift::prelude::*;
use cranelift_module::{FuncId, Linkage, Module};
use std::collections::HashMap;

pub struct CodeIr<M: Module> {
    pub module: M,
    pub context: Context,
    pub builder_context: FunctionBuilderContext,
}

struct FunctionTranslator<'ast, 'build, M: Module> {
    module: &'build mut M,
    builder: FunctionBuilder<'build>,

    variables: HashMap<&'ast str, Variable>,
    ast: &'ast Ast<'ast>,
}

#[derive(Debug, Clone, PartialEq)]
enum ExprValue {
    I64(Value),
    Bool(Value),
    Null,
    Unreachable,
}

impl ExprValue {
    pub fn expect_int(self) -> Value {
        match &self {
            Self::I64(value) => *value,
            _ => panic!("Expected int, found: {:?}", &self),
        }
    }

    pub fn expect_bool(self) -> Value {
        match &self {
            Self::Bool(value) => *value,
            _ => panic!("Expected Primitive Value, found: {:?}", &self),
        }
    }
}

impl From<Option<Value>> for ExprValue {
    fn from(value: Option<Value>) -> Self {
        match value {
            Some(v) => Self::I64(v),
            None => Self::Null,
        }
    }
}

impl<M: Module> CodeIr<M> {
    pub fn new(module: M) -> Self {
        Self {
            context: module.make_context(),
            builder_context: FunctionBuilderContext::new(),
            module,
        }
    }

    pub fn write_ir(&mut self, ast: &Ast) -> Result<String, String> {
        self.translate_function(ast, ast.root().unwrap())?;

        let mut ir = String::new();
        codegen::write_function(&mut ir, &self.context.func).unwrap();
        Ok(ir)
    }

    pub fn load<'a>(&mut self, ast: &'a Ast<'a>) -> Result<FuncId, String> {
        let id = self.translate_function(ast, ast.root().unwrap())?;

        self.module
            .define_function(id, &mut self.context)
            .map_err(|e| format!("{e}"))?;

        Ok(id)
    }

    fn translate_function<'a>(
        &mut self,
        ast: &Ast<'a>,
        function: Expr<'a>,
    ) -> Result<FuncId, String> {
        let int = types::I64;
        let Expr::Function {
            name,
            parameters,
            body,
        } = function
        else {
            unreachable!("Expected a function");
        };

        // We clear out the context state before using it.
        self.module.clear_context(&mut self.context);

        for _ in ast.iter_list(parameters) {
            self.context
                .func
                .signature
                .params
                .push(cranelift::prelude::AbiParam::new(int));
        }

        self.context.func.signature.returns.push(AbiParam::new(int));

        ////////////////

        let builder = FunctionBuilder::new(&mut self.context.func, &mut self.builder_context);

        let params_names = ast.iter_list(parameters).map(|expr| match expr {
            Expr::VariableDefinition(name) => name,
            _ => unreachable!(),
        });

        let mut trans = FunctionTranslator {
            module: &mut self.module,
            builder,

            variables: HashMap::new(),
            ast,
        };

        trans.function_declaration(params_names);
        if ExprValue::Unreachable != trans.translate(body) {
            trans.function_return(ExprValue::Null);
        }
        trans.finish_translation();

        ////////////////

        // TODO:
        // self.context
        //     .optimize(self.module.isa(), &mut control::ControlPlane::default())
        //     .unwrap();

        ////////////////

        let id = self
            .module
            .declare_function(name, Linkage::Export, &self.context.func.signature)
            .map_err(|e| format!("Compilation error: {}", e))?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use crate::utils::test_utils::*;
    use ::test::*;
    use cranelift_jit::{JITBuilder, JITModule};

    gen_tests!(generic_test(bench, code, test_name));

    fn generic_test(_b: &mut Bencher, code: &str, test_name: &str) {
        let mut parser = Parser::default();
        let ast = parser.parse(code).unwrap();

        let flags_builder = settings::builder();
        // flags_builder.set("opt_level", "speed").unwrap();
        let flags = settings::Flags::new(flags_builder);

        let isa_builder = isa::lookup_by_name("x86_64").unwrap();
        let isa = isa_builder.finish(flags).unwrap();

        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        let mut code = CodeIr::new(module);

        ////////////////////////////

        let ir = &code.write_ir(&ast).unwrap();

        let expected = &load_src(test_name, ".ir.run");
        assert_source_eq(expected, ir);

        // TODO: b.iter(|| code.load(black_box(&ast)).unwrap());
    }
}
