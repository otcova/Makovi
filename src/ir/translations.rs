use super::*;
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};

impl<'a, M: Module> FunctionTranslator<'a, '_, M> {
    const VAR_TYPE: types::Type = types::I64;

    pub fn literal(&mut self, literal: &str) -> ExprValue {
        let imm: i64 = literal.parse().unwrap();
        Some(self.builder.ins().iconst(Self::VAR_TYPE, imm))
    }
    pub fn identifier(&mut self, name: &str) -> ExprValue {
        // `use_var` is used to read the value of a variable.
        let variable = self.variables.get(name).expect("variable not defined");
        Some(self.builder.use_var(*variable))
    }
    pub fn assign(&mut self, name: &'a str, value: ExprValue) -> ExprValue {
        let value = value.expect("Expected a value");
        let variable = self.get_variable(name);
        self.builder.def_var(variable, value);
        Some(value)
    }
    pub fn eq(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().icmp(IntCC::Equal, lhs, rhs))
    }
    pub fn ne(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs))
    }
    pub fn lt(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs))
    }
    pub fn le(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(
            self.builder
                .ins()
                .icmp(IntCC::SignedLessThanOrEqual, lhs, rhs),
        )
    }
    pub fn gt(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs))
    }
    pub fn ge(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(
            self.builder
                .ins()
                .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs),
        )
    }
    pub fn add(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().iadd(lhs, rhs))
    }
    pub fn sub(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().isub(lhs, rhs))
    }
    pub fn mul(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().imul(lhs, rhs))
    }
    pub fn div(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().udiv(lhs, rhs))
    }
    pub fn module(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect("Expected an value");
        let rhs = rhs.expect("Expected an value");
        Some(self.builder.ins().urem(lhs, rhs))
    }
    pub fn if_else(
        &mut self,
        condition: ExprValue,
        if_body: impl FnOnce(&mut Self) -> ExprValue,
        else_body: impl FnOnce(&mut Self) -> ExprValue,
    ) -> ExprValue {
        let condition = condition.expect("Expected a value");

        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let merge_block = self.builder.create_block();

        self.builder.append_block_param(merge_block, Self::VAR_TYPE);

        // If
        self.builder
            .ins()
            .brif(condition, then_block, &[], else_block, &[]);

        // Then
        self.builder.switch_to_block(then_block);
        self.builder.seal_block(then_block);
        let then_return =
            if_body(self).unwrap_or_else(|| self.builder.ins().iconst(Self::VAR_TYPE, 0));
        self.builder.ins().jump(merge_block, &[then_return]);

        // Else
        self.builder.switch_to_block(else_block);
        self.builder.seal_block(else_block);
        let else_return =
            else_body(self).unwrap_or_else(|| self.builder.ins().iconst(Self::VAR_TYPE, 0));
        self.builder.ins().jump(merge_block, &[else_return]);

        // Finally
        self.builder.switch_to_block(merge_block);
        self.builder.seal_block(merge_block);

        Some(self.builder.block_params(merge_block)[0])
    }

    pub fn while_loop<C, B>(&mut self, condition: C, body: B)
    where
        C: FnOnce(&mut Self) -> ExprValue,
        B: FnOnce(&mut Self) -> ExprValue,
    {
        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();

        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(header_block);

        let condition_result = condition(self).expect("Expected a value");

        self.builder
            .ins()
            .brif(condition_result, body_block, &[], exit_block, &[]);

        self.builder.switch_to_block(body_block);
        self.builder.seal_block(body_block);

        body(self);

        self.builder.ins().jump(header_block, &[]);

        self.builder.switch_to_block(exit_block);

        // We've reached the bottom of the loop, so there will be no
        // more backedges to the header to exits to the bottom.
        self.builder.seal_block(header_block);
        self.builder.seal_block(exit_block);

        // Just return 0 for now.
        self.builder.ins().iconst(Self::VAR_TYPE, 0);
    }
    pub fn prepare_parameters(parameters: impl Iterator<Item = ExprValue>) -> Vec<Value> {
        parameters.map(|v| v.expect("Expected a value")).collect()
    }
    pub fn call(&mut self, name: &str, parameters: &[Value]) -> ExprValue {
        // Create Signature
        let mut sig = self.module.make_signature();
        sig.params = vec![AbiParam::new(Self::VAR_TYPE); parameters.len()];
        sig.returns.push(AbiParam::new(Self::VAR_TYPE));

        // TODO: Streamline the API here?
        let callee = self
            .module
            .declare_function(name, Linkage::Import, &sig)
            .expect("problem declaring function");
        let local_callee = self.module.declare_func_in_func(callee, self.builder.func);

        let call = self.builder.ins().call(local_callee, parameters);
        self.builder.inst_results(call).first().copied()
    }

    pub fn function_return(&mut self, return_value: ExprValue) {
        let return_value = return_value.expect("Expected an value");
        self.builder.ins().return_(&[return_value]);
    }

    pub fn function_declaration(
        &mut self,
        params: impl Iterator<Item = &'a str>,
        the_return: &'a str,
    ) {
        // Create the entry block, to start emitting code in.
        let entry_block = self.builder.create_block();

        // Since this is the entry block, add block parameters corresponding to
        // the function's parameters.
        //
        // TODO: Streamline the API here.
        self.builder
            .append_block_params_for_function_params(entry_block);

        // Tell the builder to emit code in this block.
        self.builder.switch_to_block(entry_block);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        self.builder.seal_block(entry_block);

        // Define the parameters
        for (i, name) in params.enumerate() {
            // TODO: cranelift_frontend should really have an API to make it easy to set
            // up param variables.
            let val = self.builder.block_params(entry_block)[i];
            let var = self.get_variable(name);
            self.builder.def_var(var, val);
        }

        // Define the return variable
        let zero = self.builder.ins().iconst(Self::VAR_TYPE, 0);
        let return_variable = self.get_variable(the_return);
        self.builder.def_var(return_variable, zero);
    }

    pub fn seal(self) {
        // TODO: Check if this code ends up reachable
        self.builder.finalize();
    }

    ///////////////////////////////////////////////////////////////

    fn get_variable(&mut self, name: &'a str) -> Variable {
        self.variables.get(name).copied().unwrap_or_else(|| {
            let var_index = self.variables.len();
            let var = Variable::new(var_index);
            self.variables.insert(name, var);
            self.builder.declare_var(var, Self::VAR_TYPE);
            var
        })
    }
}
