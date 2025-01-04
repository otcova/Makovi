mod control_flow;
mod translations;

use crate::parser::*;
use codegen::{write_function, Context};
use cranelift::prelude::*;
use cranelift_module::{FuncId, Linkage, Module};
use std::collections::HashMap;

pub struct CodeIr<M: Module> {
    pub module: M,
    pub context: Context,
    pub builder_context: FunctionBuilderContext,
}

pub struct FunctionTranslator<'ast, 'build, M: Module> {
    module: &'build mut M,
    builder: FunctionBuilder<'build>,

    variables: HashMap<&'ast str, Variable>,
    ast: &'ast Ast<'ast>,
}

type ExprValue = Option<Value>;

impl<M: Module> CodeIr<M> {
    pub fn new(module: M) -> Self {
        Self {
            context: module.make_context(),
            builder_context: FunctionBuilderContext::new(),
            module,
        }
    }

    pub fn write_ir(&self) -> String {
        let mut ir = String::new();
        write_function(&mut ir, &self.context.func).unwrap();
        ir
    }

    pub fn load<'a>(&mut self, ast: &'a Ast<'a>) -> Result<FuncId, String> {
        // We clear out the context state before using it.
        self.module.clear_context(&mut self.context);

        let id = self.translate_function(ast, ast.root().unwrap())?;

        // Define the function to jit. This finishes compilation, although
        // there may be outstanding relocations to perform. Currently, jit
        // cannot finish relocations until all functions to be called are
        // defined. For this toy demo for now, we'll just finalize the
        // function below.
        self.module
            .define_function(id, &mut self.context)
            .map_err(|e| format!("Compilation error: {}", e))?;

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
            return_expr,
            body,
        } = function
        else {
            unreachable!("Expected a function");
        };

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
            Expr::IdentifierDefinition(name) => name,
            _ => unreachable!(),
        });

        let return_name = match ast.get(return_expr) {
            Expr::IdentifierDefinition(name) => name,
            _ => unreachable!(),
        };

        let mut trans = FunctionTranslator {
            module: &mut self.module,
            builder,

            variables: HashMap::new(),
            ast,
        };

        trans.function_declaration(params_names, return_name);
        trans.translate(body);

        let return_variable = trans.identifier(return_name);
        trans.function_return(return_variable);
        trans.seal();

        ////////////////

        let id = self
            .module
            .declare_function(name, Linkage::Export, &self.context.func.signature)
            .map_err(|e| format!("Compilation error: {}", e))?;

        Ok(id)
    }
}
