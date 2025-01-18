use super::*;
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};

impl<'a, M: Module> FunctionTranslator<'a, '_, M> {
    const VAR_TYPE: types::Type = types::I64;

    pub fn integer(&mut self, literal: &str) -> ExprValue {
        let imm: i64 = literal.parse().unwrap();
        ExprValue::Primitive(self.builder.ins().iconst(Self::VAR_TYPE, imm))
    }
    pub fn variable(&mut self, name: &str) -> ExprValue {
        let variable = self
            .variables
            .get(name)
            .unwrap_or_else(|| panic!("Variable {name} not defined"));
        ExprValue::Primitive(self.builder.use_var(*variable))
    }
    pub fn assign(&mut self, name: &'a str, value: ExprValue) -> ExprValue {
        let value = value.expect_primitive();
        let variable = self.get_variable(name);
        self.builder.def_var(variable, value);
        ExprValue::Primitive(value)
    }
    pub fn eq(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().icmp(IntCC::Equal, lhs, rhs))
    }
    pub fn ne(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs))
    }
    pub fn lt(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs))
    }
    pub fn le(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(
            self.builder
                .ins()
                .icmp(IntCC::SignedLessThanOrEqual, lhs, rhs),
        )
    }
    pub fn gt(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs))
    }
    pub fn ge(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(
            self.builder
                .ins()
                .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs),
        )
    }
    pub fn add(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().iadd(lhs, rhs))
    }
    pub fn sub(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().isub(lhs, rhs))
    }
    pub fn mul(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().imul(lhs, rhs))
    }
    pub fn div(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().udiv(lhs, rhs))
    }
    pub fn module(&mut self, lhs: ExprValue, rhs: ExprValue) -> ExprValue {
        let lhs = lhs.expect_primitive();
        let rhs = rhs.expect_primitive();
        ExprValue::Primitive(self.builder.ins().urem(lhs, rhs))
    }
    pub fn if_statement(
        &mut self,
        condition: ExprValue,
        then_body: impl FnOnce(&mut Self) -> ExprValue,
    ) -> ExprValue {
        let condition = condition.expect_primitive();

        let then_block = self.builder.create_block();
        let finally_block = self.builder.create_block();

        // If
        self.builder
            .ins()
            .brif(condition, then_block, &[], finally_block, &[]);

        // Then
        self.builder.switch_to_block(then_block);
        self.builder.seal_block(then_block);
        let then_result = then_body(self);

        if ExprValue::Unreachable != then_result {
            self.builder.ins().jump(finally_block, &[]);
        }

        // Finally
        self.builder.switch_to_block(finally_block);
        self.builder.seal_block(finally_block);

        ExprValue::Null
    }

    pub fn if_else(
        &mut self,
        condition: ExprValue,
        then_body: impl FnOnce(&mut Self) -> ExprValue,
        else_body: impl FnOnce(&mut Self) -> ExprValue,
    ) -> ExprValue {
        let condition = condition.expect_primitive();

        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let finally_block = self.builder.create_block();

        let mut then_unreachable = false;
        let mut else_unreachable = false;

        // If
        self.builder
            .ins()
            .brif(condition, then_block, &[], else_block, &[]);

        // Then
        self.builder.switch_to_block(then_block);
        self.builder.seal_block(then_block);
        if ExprValue::Unreachable == then_body(self) {
            then_unreachable = true;
        } else {
            self.builder.ins().jump(finally_block, &[]);
        }

        // Else
        self.builder.switch_to_block(else_block);
        self.builder.seal_block(else_block);
        if ExprValue::Unreachable == else_body(self) {
            else_unreachable = true;
        } else {
            self.builder.ins().jump(finally_block, &[]);
        }

        // Finally
        self.builder.switch_to_block(finally_block);
        self.builder.seal_block(finally_block);

        if then_unreachable && else_unreachable {
            ExprValue::Unreachable
        } else {
            ExprValue::Null
        }
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

        let condition_result = condition(self).expect_primitive();

        self.builder
            .ins()
            .brif(condition_result, body_block, &[], exit_block, &[]);

        self.builder.switch_to_block(body_block);
        self.builder.seal_block(body_block);

        if ExprValue::Unreachable != body(self) {
            self.builder.ins().jump(header_block, &[]);
        }

        self.builder.switch_to_block(exit_block);

        // We've reached the bottom of the loop, so there will be no
        // more backedges to the header to exits to the bottom.
        self.builder.seal_block(header_block);
        self.builder.seal_block(exit_block);

        // Just return 0 for now.
        self.builder.ins().iconst(Self::VAR_TYPE, 0);
    }
    pub fn prepare_parameters(parameters: impl Iterator<Item = ExprValue>) -> Vec<Value> {
        parameters.map(|v| v.expect_primitive()).collect()
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
        self.builder.inst_results(call).first().copied().into()
    }

    pub fn function_return(&mut self, return_value: ExprValue) {
        let return_value = return_value.expect_primitive();
        self.builder.ins().return_(&[return_value]);
    }

    pub fn function_declaration(&mut self, params: impl Iterator<Item = &'a str>) {
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
    }

    pub fn finish_translation(self) {
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
