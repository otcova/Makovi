use super::error::{CompilationError, LineSpan};
use super::*;
use hir::*;
use itertools::Itertools;
use parser::ParserStage;

pub struct TypeInferenceStage<'code, 'compiler> {
    pub errors: &'compiler mut CompilationErrorSet,
    pub code: CodeModule<'code>,
    pub entry_point: FnDefinitionId,
}

impl<'code, 'compiler> ParserStage<'code, 'compiler> {
    pub fn type_inference_stage(
        self,
        extern_instances: ExternalInstances,
    ) -> TypeInferenceStage<'code, 'compiler> {
        let mut stage = TypeInferenceStage {
            errors: self.errors,
            code: CodeModule {
                definitions: self.definitions,
                instances: ModuleInstances::new(extern_instances),
            },
            entry_point: self.entry_point,
        };
        stage.instantiate(self.entry_point);
        stage
    }
}

impl TypeInferenceStage<'_, '_> {
    fn instantiate(&mut self, definition_id: FnDefinitionId) -> Option<()> {
        let NewFnInstanceResult::New {
            id: instance_id,
            instance,
            definition,
        } = self.code.instances.new_instance(
            &self.code.definitions,
            definition_id.into(),
            ParamsTypes::default(),
        )
        else {
            return None;
        };

        let inference = FunctionTypeInference {
            definition,
            instance,
            instance_id,
            current_instruction: 0,

            errors: self.errors,
            definitions: &self.code.definitions,
            instances: &mut self.code.instances,
        };

        inference.analyse();

        Some(())
    }
}

struct FunctionTypeInference<'code, 'r> {
    errors: &'r mut CompilationErrorSet,
    definitions: &'r ModuleDefinitions<'code>,
    instances: &'r mut ModuleInstances,

    // Current function
    instance_id: FnInstanceId,
    instance: FnInstance,
    definition: &'r FnDefinition<'code>,
    current_instruction: usize,
}

impl FunctionTypeInference<'_, '_> {
    fn analyse(mut self) -> ValueType {
        for index in 0..self.definition.instructions.len() {
            self.current_instruction = index;
            self.infer_types();
        }
        let return_type = self.instance.return_type;
        self.instances
            .finalize_instance(self.instance_id, self.instance);
        return_type
    }

    fn infer_types(&mut self) {
        match &self.definition.instructions[self.current_instruction] {
            Instruction::Call(call) => {
                let arguments = self.definition.get_arguments(call);
                let Ok(parameters) = arguments
                    .iter()
                    .map(|arg| match self.instance.type_of(arg) {
                        ValueType::Unknown => Err(()),
                        arg_type => Ok(arg_type),
                    })
                    .try_collect()
                else {
                    // Function call can not be instanciated due to the previous
                    // type inference compilation errors
                    return;
                };

                let (instance_id, return_type) = match self.instances.new_instance(
                    self.definitions,
                    call.fn_id,
                    ParamsTypes(parameters),
                ) {
                    NewFnInstanceResult::New {
                        id: instance_id,
                        instance,
                        definition,
                    } => (
                        instance_id,
                        FunctionTypeInference {
                            definition,
                            instance,
                            instance_id,
                            current_instruction: 0,

                            errors: self.errors,
                            definitions: self.definitions,
                            instances: self.instances,
                        }
                        .analyse(),
                    ),
                    NewFnInstanceResult::Exists {
                        id,
                        stage: FnInstanceStage::Ok(instance),
                    } => (id, instance.return_type),
                    NewFnInstanceResult::Exists {
                        id,
                        stage: FnInstanceStage::WithErrors { return_type },
                    } => (id, *return_type),
                    NewFnInstanceResult::Exists {
                        id,
                        stage: FnInstanceStage::WithFatalErrors,
                    } => (id, ValueType::Unknown),
                    NewFnInstanceResult::Exists {
                        id: _,
                        stage: FnInstanceStage::BeeingCreated,
                    } => {
                        todo!("Type inference with recursive functions")
                    }
                    NewFnInstanceResult::UndefinedFunction { name } => {
                        self.errors.push(CompilationError {
                            message: format!("Undefined function {name:?}"),
                            span: LineSpan::default(),
                        });
                        return;
                    }
                    NewFnInstanceResult::UndefinedExternalInstance { fn_id } => {
                        // TODO: Make this a execution time assertion if types are not defined
                        // explicitly.
                        let parameters = arguments.iter().map(|arg| self.instance.type_of(arg));
                        self.errors.push(CompilationError {
                            message: format!(
                                "Built-in function {} does not accept the types: ({})",
                                fn_id,
                                parameters.format(", ")
                            ),
                            span: LineSpan::default(),
                        });
                        return;
                    }
                    NewFnInstanceResult::WrongArgumentCount { definition } => {
                        self.errors.push(CompilationError {
                            message: format!(
                                "Wrong number of arguments. Function {:?} is defined with {}",
                                definition.name, definition.parameters
                            ),
                            span: LineSpan::default(),
                        });
                        return;
                    }
                };

                if let Some(variable) = call.result {
                    self.assign(variable, return_type);
                }
                self.instance.imported_instances.push(instance_id);
            }
            Instruction::Assign { variable, value } => {
                self.assign(*variable, self.instance.type_of(value));
            }
            Instruction::Return(value) => {
                let value = self.instance.type_of(value);
                if value != ValueType::Unknown {
                    if self.instance.return_type == ValueType::Unknown {
                        self.instance.return_type = value;
                    } else if self.instance.return_type != value {
                        self.errors.push(CompilationError {
                            message: format!(
                                "Expected {} type, but found {}. Returning variants is not yet supported.",
                                self.instance.return_type, value
                            ),
                            span: LineSpan::default(),
                        })
                    }
                }
            }
            Instruction::IfStart(condition, _) => self.expect(condition, ValueType::Bool),
            Instruction::Else => {}
            Instruction::IfEnd(_) => {}
            Instruction::LoopStart => {}
            Instruction::Break => {}
            Instruction::LoopEnd => {}
        }
    }

    fn assign(&mut self, variable: VariableId, value_type: ValueType) {
        let var = &mut self.instance.variables[*variable];

        if value_type == ValueType::Unknown {
            *var = value_type;
            return;
        }

        match *var {
            ValueType::Unknown => *var = value_type,
            actual => {
                if actual != value_type {
                    self.errors.push(CompilationError {
                        message: format!(
                            "Expected a variable of type {value_type:?}, but found {actual:?}",
                        ),
                        span: LineSpan::default(),
                    })
                }
            }
        }
    }

    fn expect(&mut self, value: &Variable, expect: ValueType) {
        match value {
            Variable::Id(var) => self.assign(*var, expect),
            Variable::Unknown => {}
            Variable::Const(value) => {
                let actual = value.get_type();
                if actual == expect {
                    self.errors.push(CompilationError {
                        message: format!("Expected {expect:?} type, but found {actual:?}"),
                        span: LineSpan::default(),
                    })
                }
            }
        }
    }
}
