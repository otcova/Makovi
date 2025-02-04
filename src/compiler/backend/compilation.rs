use super::*;
use cranelift::codegen::ir::{FuncRef, Inst};
use cranelift::codegen::packed_option::PackedOption;
use cranelift::frontend::FuncInstBuilder;

pub struct FnBackendCompiler<'r, 'code> {
    pub module: &'r mut JITModule,
    pub builder: FunctionBuilder<'r>,
    pub variables: &'r mut Vec<BackendVariable>,
    pub instructions: std::slice::Iter<'r, hir::Instruction>,
    pub instance: &'r hir::FnInstance,
    pub definition: &'r hir::FnDefinition<'code>,
    pub func_ids: &'r [FuncId],
    pub func_refs: &'r mut Vec<FuncRef>,
    /// Used to temporally create a slice of value. For example, to store the
    /// arguments of a function call while the instruction is beeing builded.
    pub values_buffer: &'r mut Vec<Value>,

    pub instruction_flow: InstructionFlow,

    pub if_scope_stack: &'r mut Vec<IfScope>,
    pub loop_scope_stack: &'r mut Vec<LoopScope>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum InstructionFlow {
    /// There's no active open block to place the code in.
    #[default]
    Terminated,

    /// The current active block is terminated, but the current flow
    /// will start with the provided successor block.
    ///
    /// The block will not be used in the next instructions.
    /// This means that the block will be sealed immediatly when any instruction is added.
    CleanStart(Block),

    /// The instruction is not the first one of the active block.
    Dirty,
}

pub struct LoopScope {
    /// Jump to `start_label` to repeat the loop (the continue instruction)
    start_label: Block,

    /// Jump to `end_label` to break the loop
    end_label: PackedOption<Block>,
}

pub struct IfScope(Block);

impl<'r> FnBackendCompiler<'r, '_> {
    pub fn compile(mut self) {
        self.function_declaration();

        while let Some(instruction) = self.instructions.next() {
            self.compile_instruction(instruction);
        }

        self.finalize();
    }

    fn compile_instruction(&mut self, instruction: &hir::Instruction) -> Option<()> {
        match instruction {
            hir::Instruction::Call(call) => {
                let instance = self.instance.imported_instances[call.call_id];
                let arguments = self.definition.get_arguments(call);

                let returned = self.call(instance, arguments);

                // - Store returned value
                if let Some(variable) = call.result {
                    let variables = self.variables[*variable].variables();
                    let results = self.builder.inst_results(returned);
                    assert_eq!(results.len(), variables.len());

                    for (index, var) in variables.into_iter().enumerate() {
                        let results = self.builder.inst_results(returned)[index];
                        self.builder.def_var(var, results);
                    }
                }
            }
            hir::Instruction::Assign { variable, value } => self.assign(*variable, value),
            hir::Instruction::Return(value) => self.fn_return(value),
            hir::Instruction::IfStart(condition, run_if) => self.if_start(condition, *run_if),
            hir::Instruction::IfEnd(chain_size) => self.if_end(*chain_size),
            hir::Instruction::Else => self.else_start(),
            hir::Instruction::LoopStart => self.loop_start(),
            hir::Instruction::Break => self.loop_break(),
            hir::Instruction::LoopEnd => self.loop_end(),
        }

        Some(())
    }

    fn function_declaration(&mut self) {
        self.instruction_flow = InstructionFlow::Terminated;

        let entry_block = self.builder.create_block();

        self.builder
            .append_block_params_for_function_params(entry_block);
        self.set_flow_label(entry_block);

        self.declare_variables(entry_block);
        self.declare_functions();
    }

    fn declare_variables(&mut self, entry_block: Block) {
        self.variables.clear();

        let builder = &mut self.builder;
        let mut crane_variables_count = 0;
        let mut declare_var = move |var_type: Type| -> Variable {
            let var = Variable::new(crane_variables_count);
            crane_variables_count += 1;
            builder.declare_var(var, var_type);
            var
        };

        for var in &self.instance.variables {
            let backend_var = match var {
                hir::ValueType::Unknown => {
                    unreachable!("The backend should only work with correct code")
                }
                hir::ValueType::Null => BackendVariable::Null,
                hir::ValueType::Int => BackendVariable::Int(declare_var(INT_TYPE)),
                hir::ValueType::Bool => BackendVariable::Bool(declare_var(BOOL_TYPE)),
            };

            self.variables.push(backend_var);
        }

        // Bind function parameters with variables
        let mut index = 0;
        for id in 0..self.instance.parameters().len() {
            for var in self.variables[id].variables() {
                let val = self.builder.block_params(entry_block)[index];
                index += 1;

                self.builder.def_var(var, val);
            }
        }
    }

    fn declare_functions(&mut self) {
        self.func_refs.clear();

        // TODO: See if it's better & possible to use `declare_func_in_data`
        for instance in &self.instance.imported_instances {
            let func_id = self.func_ids[**instance];
            let func_ref = self.module.declare_func_in_func(func_id, self.builder.func);
            self.func_refs.push(func_ref);
        }

        // TODO: Decide what to do with this cranelift investigation
        // let fn_ref = {
        //     let signature: ir::Signature = ();
        //     let name = ir::UserExternalName {
        //         namespace: 1,
        //         index: 0,
        //     };
        //
        //     let sig_ref = self.builder.func.import_signature(signature);
        //     let name_ref = self.builder.func.declare_imported_user_function(name);
        //     self.builder.func.import_function(ir::ExtFuncData {
        //         name: ir::ExternalName::user(name_ref),
        //         signature: sig_ref,
        //         colocated: false,
        //     })
        // };
    }

    fn finalize(mut self) {
        if self.instruction_flow != InstructionFlow::Terminated {
            assert!(
                self.instance.return_type == hir::ValueType::Null,
                "Wrong function return type"
            );
            self.fn_return(&hir::Variable::Const(hir::Value::Null));
        }

        assert!(
            self.if_scope_stack.is_empty(),
            "There are control flow instructions without end"
        );

        self.builder.finalize();
    }

    /// `instruction_flow` will be terminated
    fn jump(&mut self, block: Block) {
        self.ins().jump(block, &[]);
        self.instruction_flow = InstructionFlow::Terminated;
        self.skip_scope();
    }

    /// `instruction_flow` will be followed by the then block
    fn branch(&mut self, run_if: hir::RunIf, cond: Value, else_block: Block) {
        let then = self.builder.create_block();
        let (if_true, if_false) = match run_if {
            hir::RunIf::True => (then, else_block),
            hir::RunIf::False => (else_block, then),
        };

        self.ins().brif(cond, if_true, &[], if_false, &[]);
        self.instruction_flow = InstructionFlow::Terminated;

        self.set_flow_label(then);
    }

    /// `instruction_flow` will be followed by the then block
    fn fn_return(&mut self, value: &hir::Variable) {
        let value = self.use_var(value);
        self.ins().return_(&value.values());
        self.instruction_flow = InstructionFlow::Terminated;
    }

    /// `instruction_flow` is always terminated with lable as the successor
    fn set_flow_label(&mut self, label: Block) {
        match self.instruction_flow {
            InstructionFlow::Terminated => {}
            InstructionFlow::CleanStart(label) => {
                self.builder.seal_block(label);
                self.ins().jump(label, &[]);
            }
            InstructionFlow::Dirty => {
                self.ins().jump(label, &[]);
            }
        }

        self.builder.switch_to_block(label);
        self.instruction_flow = InstructionFlow::CleanStart(label);
    }

    /// Returns a label pointing to the current flow.
    /// This label will need to be sealead later on.
    fn get_flow_label(&mut self) -> Option<Block> {
        match self.instruction_flow {
            InstructionFlow::Terminated => None,
            InstructionFlow::CleanStart(label) => {
                // The returned label will be sealed by the caller.
                self.instruction_flow = InstructionFlow::Dirty;
                Some(label)
            }
            InstructionFlow::Dirty => {
                let label = self.builder.create_block();
                self.set_flow_label(label);
                Some(label)
            }
        }
    }

    fn if_start(&mut self, condition: &hir::Variable, run_if: hir::RunIf) {
        let cond = self.use_var(condition).expect_bool();
        let else_block = self.builder.create_block();

        self.branch(run_if, cond, else_block);
        self.if_scope_stack.push(IfScope(else_block));
    }

    fn else_start(&mut self) {
        let end_block = self.builder.create_block();
        self.jump(end_block);

        let scope = self.if_scope_stack.pop().expect("Incorrect if-else scope");
        self.set_flow_label(scope.0);

        self.if_scope_stack.push(IfScope(end_block));
    }

    fn if_end(&mut self, chain_size: usize) {
        for _ in 0..chain_size {
            let scope = self.if_scope_stack.pop().expect("Incorrect if scope");
            self.set_flow_label(scope.0);
        }
    }

    fn loop_start(&mut self) {
        let start_label = self.get_flow_label().expect("Starting an unreachable loop");
        self.loop_scope_stack.push(LoopScope {
            start_label,
            end_label: None.into(),
        });
    }

    fn loop_end(&mut self) {
        if self.instruction_flow != InstructionFlow::Terminated {
            self.loop_continue();
        }

        let scope = self.loop_scope_stack.pop().expect("Incorrect loop scope");
        self.builder.seal_block(scope.start_label);
        if let Some(end_label) = scope.end_label.expand() {
            self.set_flow_label(end_label);
        }
    }

    fn loop_break(&mut self) {
        let scope = self
            .loop_scope_stack
            .last_mut()
            .expect("Incorrect loop scope");

        let create_block = || self.builder.create_block();
        let end_block = scope.end_label.expand().unwrap_or_else(create_block);
        scope.end_label = end_block.into();

        self.jump(end_block);
    }

    fn loop_continue(&mut self) {
        let scope = self.loop_scope_stack.last().expect("Incorrect loop scope");
        self.jump(scope.start_label);
    }

    fn use_var_id(&mut self, id: hir::VariableId) -> BackendValue {
        match self.variables[*id] {
            BackendVariable::Null => BackendValue::Null,
            BackendVariable::Int(var) => BackendValue::Int(self.builder.use_var(var)),
            BackendVariable::Bool(var) => BackendValue::Bool(self.builder.use_var(var)),
        }
    }

    fn use_var(&mut self, variable: &hir::Variable) -> BackendValue {
        match variable {
            hir::Variable::Unknown => {
                unreachable!("The backend should only work with correct code")
            }
            hir::Variable::Const(var) => self.load(var),
            hir::Variable::Id(id) => self.use_var_id(*id),
        }
    }

    fn load(&mut self, value: &hir::Value) -> BackendValue {
        assert!(self.instruction_flow != InstructionFlow::Terminated);

        match value {
            hir::Value::Null => BackendValue::Null,
            hir::Value::Int(v) => BackendValue::Int(self.ins().iconst(INT_TYPE, *v)),
            hir::Value::Bool(v) => BackendValue::Bool(self.ins().iconst(BOOL_TYPE, *v as i64)),
        }
    }

    fn assign(&mut self, variable: hir::VariableId, value: &hir::Variable) {
        let value = self.use_var(value);
        match self.variables[*variable] {
            BackendVariable::Null => value.expect_null(),
            BackendVariable::Int(var) => self.builder.def_var(var, value.expect_int()),
            BackendVariable::Bool(var) => self.builder.def_var(var, value.expect_bool()),
        };
    }

    fn call<'a, A: IntoIterator<Item = &'a hir::Variable>>(
        &mut self,
        fn_id: hir::FnInstanceId,
        arguments: A,
    ) -> Inst {
        // - Prepare arguments
        // Take is necessary to release `self` lifetime
        let mut buffer = std::mem::take(self.values_buffer);
        buffer.clear();
        buffer.extend(arguments.into_iter().flat_map(|v| self.use_var(v).values()));

        // - Get FuncRef
        let fn_ref = self.func_refs[*fn_id];

        // - Call
        // TODO: Possible optimization: Use `call_return` if followed by a return
        let returned = self.ins().call(fn_ref, &buffer);

        // We give back the buffer so that the allocated memory can be reused
        *self.values_buffer = buffer;

        returned
    }

    fn ins(&mut self) -> FuncInstBuilder<'_, 'r> {
        if let InstructionFlow::CleanStart(block) = self.instruction_flow {
            self.builder.seal_block(block);
            self.instruction_flow = InstructionFlow::Dirty;
        }
        self.builder.ins()
    }

    /// It will skip all the instruction until it reaches the end of the scope.
    /// Used to skip unreachable code.
    fn skip_scope(&mut self) {
        let mut nested_scopes: usize = 0;

        while let Some(instruction) = self.instructions.next() {
            match instruction {
                hir::Instruction::Call(_)
                | hir::Instruction::Assign { .. }
                | hir::Instruction::Return(_)
                | hir::Instruction::Else
                | hir::Instruction::Break => {}

                hir::Instruction::IfStart(_, _) | hir::Instruction::LoopStart => {
                    nested_scopes += 1;
                }

                &hir::Instruction::IfEnd(size) => {
                    if nested_scopes < size {
                        panic!("Invalid scope");
                    }

                    nested_scopes -= size;

                    if nested_scopes == 0 {
                        self.compile_instruction(instruction);
                        return;
                    }
                }

                hir::Instruction::LoopEnd => {
                    if nested_scopes == 0 {
                        self.compile_instruction(instruction);
                        return;
                    } else {
                        nested_scopes -= 1;
                    }
                }
            }
        }

        assert!(nested_scopes == 0);
    }
}
