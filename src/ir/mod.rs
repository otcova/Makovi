mod control_flow;
mod translations;

use crate::parser::*;
use codegen::{write_function, Context};
use cranelift::prelude::*;
use cranelift_module::{FuncId, Linkage, Module};
use std::collections::HashMap;

pub struct CodeIR<M: Module> {
    pub module: M,
    pub ctx: Context,
    pub builder_context: FunctionBuilderContext,
}

pub struct FunctionTranslator<'a, 'b, M: Module> {
    pub int: types::Type,
    pub builder: FunctionBuilder<'b>,
    pub variables: HashMap<String, Variable>,
    pub module: &'b mut M,
    pub ast: &'a Ast<'a>,
}

type ExprValue = Option<Value>;

impl<M: Module> CodeIR<M> {
    pub fn write_ir(&self) -> String {
        let mut ir = String::new();
        write_function(&mut ir, &self.ctx.func).unwrap();
        ir
    }

    pub fn load<'a>(&mut self, ast: &'a Ast<'a>) -> Result<FuncId, String> {
        match ast.root().unwrap() {
            Expr::Function(name, params, return_name, body) => {
                self.load_function(ast, name, params, return_name, body)
            }
            _ => Err("Expected a single top function".to_owned()),
        }
    }

    fn load_function<'a>(
        &mut self,
        ast: &'a Ast<'a>,
        function_name: &str,
        params: u32,
        return_node: u32,
        function_body: u32,
    ) -> Result<FuncId, String> {
        let int = types::I64;

        for _ in ast.iter_list(params) {
            self.ctx
                .func
                .signature
                .params
                .push(cranelift::prelude::AbiParam::new(int));
        }

        // Our toy language currently only supports one return value, though
        // Cranelift is designed to support more.
        self.ctx.func.signature.returns.push(AbiParam::new(int));

        // Create the builder to build a function.
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        // Create the entry block, to start emitting code in.
        let entry_block = builder.create_block();

        // Since this is the entry block, add block parameters corresponding to
        // the function's parameters.
        //
        // TODO: Streamline the API here.
        builder.append_block_params_for_function_params(entry_block);

        // Tell the builder to emit code in this block.
        builder.switch_to_block(entry_block);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        builder.seal_block(entry_block);

        let params_names = ast.iter_list(params).map(|expr| match expr {
            Expr::IdentifierDefinition(name) => name,
            _ => unreachable!(),
        });

        let return_name = match ast.get(return_node) {
            Expr::IdentifierDefinition(name) => name,
            _ => unreachable!(),
        };

        // The toy language allows variables to be declared implicitly.
        // Walk the AST and declare all implicitly-declared variables.
        let variables = declare_variables(
            int,
            &mut builder,
            params_names,
            return_name,
            ast.nodes.borrow().iter().copied(),
            entry_block,
        );

        let mut trans = FunctionTranslator {
            int,
            builder,
            variables,
            module: &mut self.module,
            ast,
        };

        trans.translate(function_body);

        // Set up the return variable of the function. Above, we declared a
        // variable to hold the return value. Here, we just do a use of that
        // variable.
        let return_variable = trans.variables.get(return_name).unwrap();
        let return_value = trans.builder.use_var(*return_variable);

        // Emit the return instruction.
        trans.builder.ins().return_(&[return_value]);

        // Tell the builder we're done with this function.
        trans.builder.finalize();

        // Next, declare the function to jit. Functions must be declared
        // before they can be called, or defined.
        //
        // TODO: This may be an area where the API should be streamlined; should
        // we have a version of `declare_function` that automatically declares
        // the function?
        let id = self
            .module
            .declare_function(function_name, Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| format!("Compilation error: {}", e))?;

        Ok(id)
    }
}

fn declare_variables<'a>(
    int: types::Type,
    builder: &mut FunctionBuilder,
    params: impl Iterator<Item = &'a str>,
    the_return: &str,
    function_body: impl Iterator<Item = Expr<'a>> + 'a,
    entry_block: Block,
) -> HashMap<String, Variable> {
    let mut variables = HashMap::new();
    let mut index = 0;

    for (i, name) in params.enumerate() {
        // TODO: cranelift_frontend should really have an API to make it easy to set
        // up param variables.
        let val = builder.block_params(entry_block)[i];
        let var = declare_variable(int, builder, &mut variables, &mut index, name);
        builder.def_var(var, val);
    }
    let zero = builder.ins().iconst(int, 0);
    let return_variable = declare_variable(int, builder, &mut variables, &mut index, the_return);
    builder.def_var(return_variable, zero);

    for expr in function_body {
        if let Expr::Assign(name, _) = expr {
            declare_variable(int, builder, &mut variables, &mut index, name);
        }
    }

    variables
}

/// Declare a single variable declaration.
fn declare_variable(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    name: &str,
) -> Variable {
    let var = Variable::new(*index);
    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, int);
        *index += 1;
    }
    var
}
