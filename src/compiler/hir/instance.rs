use super::*;
use derive_more::derive::{Deref, DerefMut, Display, From};
use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
use std::ops::Index;
use std::ptr::NonNull;

/// A function usually only has 2-3 parameters. This means that allocating a Vec
/// only to store the types would be ridiculous overhead.
/// (ValueType size = 1 byte, Vec size = 24 bytes).
/// This struct stores the first N parameters in place, and in case of
/// some absurd function it allocates the necesary memory.
#[derive(
    Debug, Default, Clone, Display, Eq, PartialEq, Hash, PartialOrd, Ord, From, Deref, DerefMut,
)]
#[display("({})", _0.iter().enumerate()
    .format_with(", ", |(i, ty), f| f(&format_args!("v{i}: {ty}"))))]
pub struct ParamsTypes(pub SmallVec<ValueType, 16>);

/// A collection of instanciated functions
#[derive(Default)]
pub struct ModuleInstances {
    pub(super) instances: Vec<FnInstanceStage>,

    /// Value is none if the instance has been created but is beeing modifyied.
    pub(super) local_instances_map: HashMap<LocalFnInstanceDeclaration, FnInstanceId>,

    /// Value is none if the instance has been created but is beeing modifyied.
    pub(super) extern_instances_map: HashMap<ExternFnInstanceDeclaration, FnInstanceId>,
}

#[derive(Debug, Eq, PartialEq, Hash, Display, PartialOrd, Ord)]
#[display("fn {}{parameters}", FnId::Local(*fn_id))]
pub struct LocalFnInstanceDeclaration {
    pub fn_id: FnDefinitionId,
    pub parameters: ParamsTypes,
}

#[derive(Debug, Eq, PartialEq, Hash, Display, PartialOrd, Ord)]
#[display("fn {}{parameters}", FnId::Extern(*fn_id))]
pub struct ExternFnInstanceDeclaration {
    pub fn_id: ExternFnId,
    pub parameters: ParamsTypes,
}

#[derive(Deref, Clone, Copy)]
pub struct FnInstanceId(pub(super) usize);

pub struct FnInstance {
    pub id: FnInstanceId,
    pub parameters: usize,
    pub variables: Box<[ValueType]>,
    pub imported_instances: Vec<FnInstanceId>,
    pub return_type: ValueType,
    pub external_ptr: Option<NonNull<u8>>,
}

impl FnInstance {
    pub fn type_of(&self, variable: &Variable) -> ValueType {
        match variable {
            Variable::Id(index) => self.variables[**index],
            Variable::Const(value) => value.get_type(),
            Variable::Unknown => ValueType::Unknown,
        }
    }

    pub fn parameters(&self) -> &[ValueType] {
        &self.variables[0..self.parameters]
    }

    pub fn name(&self, buffer: &mut String) {
        buffer.clear();
        write!(
            buffer,
            "f{}<{}>",
            *self.id,
            self.parameters().iter().format(", ")
        )
        .expect("Formating numbers shouldn't fail");
    }
}

pub enum NewFnInstanceResult<'inst, 'def, 'code> {
    // A function instance with the given declaration did not exists.
    New {
        id: FnInstanceId,
        instance: FnInstance,
        definition: &'def FnDefinition<'code>,
    },
    // The instance already exists
    Exists {
        id: FnInstanceId,
        stage: &'inst FnInstanceStage,
    },
    WrongArgumentCount {
        definition: &'def FnDefinition<'code>,
    },
    UndefinedFunction {
        name: &'code str,
    },
    UndefinedExternalInstance {
        fn_id: ExternFnId,
    },
}

pub enum FnInstanceStage {
    /// The instance is beeing created, wait till finishes to get it
    BeeingCreated,
    /// Found errors while creating the instance, but managed to infer the return type
    WithErrors { return_type: ValueType },
    /// Found errors while creating the instance.
    WithFatalErrors,
    /// The instance has been created successfully.
    /// This means that all `ValueType`s are known.
    Ok(FnInstance),
}

impl ModuleInstances {
    pub fn new_instance<'inst, 'def, 'code>(
        &'inst mut self,
        definitions: &'def ModuleDefinitions<'code>,
        fn_id: FnId,
        parameters: ParamsTypes,
    ) -> NewFnInstanceResult<'inst, 'def, 'code> {
        match fn_id {
            FnId::Extern(fn_id) => self.get_extern_instance(fn_id, parameters),
            FnId::Local(fn_id) => self.new_local_instance(definitions, fn_id, parameters),
        }
    }

    fn get_extern_instance<'def, 'code>(
        &mut self,
        fn_id: ExternFnId,
        parameters: ParamsTypes,
    ) -> NewFnInstanceResult<'_, 'def, 'code> {
        let declaration = ExternFnInstanceDeclaration { fn_id, parameters };

        match self.extern_instances_map.entry(declaration) {
            Entry::Vacant(_) => NewFnInstanceResult::UndefinedExternalInstance { fn_id },
            Entry::Occupied(entry) => NewFnInstanceResult::Exists {
                id: *entry.get(),
                stage: &self.instances[**entry.get()],
            },
        }
    }

    fn new_local_instance<'inst, 'def, 'code>(
        &'inst mut self,
        definitions: &'def ModuleDefinitions<'code>,
        fn_id: FnDefinitionId,
        parameters: ParamsTypes,
    ) -> NewFnInstanceResult<'inst, 'def, 'code> {
        let parameters_len = parameters.len();
        let declaration = LocalFnInstanceDeclaration { fn_id, parameters };

        match self.local_instances_map.entry(declaration) {
            Entry::Vacant(entry) => {
                let definition = match &definitions[fn_id] {
                    FnDefinitionStage::Defined(definition) => definition,
                    FnDefinitionStage::ToDefine { name } => {
                        return NewFnInstanceResult::UndefinedFunction { name };
                    }
                };

                if definition.parameters != parameters_len {
                    return NewFnInstanceResult::WrongArgumentCount { definition };
                }
                let mut variables =
                    vec![ValueType::Unknown; definition.variables()].into_boxed_slice();

                let arity = entry.key().parameters.len();
                variables[0..arity].copy_from_slice(&entry.key().parameters);

                let id = FnInstanceId(self.instances.len());
                self.instances.push(FnInstanceStage::BeeingCreated);
                entry.insert(id);

                let instance = FnInstance {
                    id,
                    parameters: arity,
                    variables,
                    imported_instances: Vec::new(),
                    return_type: ValueType::Unknown,
                    external_ptr: None,
                };
                NewFnInstanceResult::New {
                    id,
                    instance,
                    definition,
                }
            }
            Entry::Occupied(entry) => NewFnInstanceResult::Exists {
                id: *entry.get(),
                stage: &self.instances[**entry.get()],
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (FnInstanceId, &FnInstanceStage)> {
        self.instances
            .iter()
            .enumerate()
            .map(|(i, f)| (FnInstanceId(i), f))
    }

    /// Iter the function declarations of the module.
    /// This are the ones that will be compiled by the backend.
    pub fn iter_local_declarations(
        &self,
    ) -> impl Iterator<Item = (FnInstanceId, &LocalFnInstanceDeclaration)> {
        self.local_instances_map
            .iter()
            .map(|(decl, id)| (*id, decl))
    }

    /// Will fail if the instance does not exist.
    /// This could happen if the provided `instance` was
    /// not obtained from `self.new_instance`
    pub fn finalize_instance(&mut self, id: FnInstanceId, instance: FnInstance) {
        let stage = &mut self.instances[*id];

        if instance.return_type == ValueType::Unknown {
            *stage = FnInstanceStage::WithFatalErrors;
            return;
        }

        for variable_type in &instance.variables {
            if *variable_type == ValueType::Unknown {
                *stage = FnInstanceStage::WithErrors {
                    return_type: instance.return_type,
                };
                return;
            }
        }

        *stage = FnInstanceStage::Ok(instance);
    }

    pub fn get_id(&self, fn_id: FnId, parameters: ParamsTypes) -> Option<FnInstanceId> {
        match fn_id {
            FnId::Extern(fn_id) => {
                let declaration = ExternFnInstanceDeclaration { fn_id, parameters };
                self.extern_instances_map.get(&declaration).copied()
            }
            FnId::Local(fn_id) => {
                let declaration = LocalFnInstanceDeclaration { fn_id, parameters };
                self.local_instances_map.get(&declaration).copied()
            }
        }
    }
}

impl Index<FnInstanceId> for ModuleInstances {
    type Output = FnInstanceStage;
    fn index(&self, index: FnInstanceId) -> &Self::Output {
        &self.instances[*index]
    }
}

impl Display for ModuleInstances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extern_instances = self
            .extern_instances_map
            .iter()
            .sorted_unstable_by_key(|(decl, _)| *decl);

        for (declaration, id) in extern_instances {
            // Name & Parameters
            write!(f, "{}", declaration)?;

            // Return
            let instance = match &self.instances[**id] {
                FnInstanceStage::BeeingCreated => return writeln!(f, " -> <in progress...>"),
                FnInstanceStage::WithFatalErrors => return writeln!(f, " -> <compilation error>"),
                FnInstanceStage::WithErrors { return_type } => {
                    writeln!(f, " -> {return_type}")?;
                    return writeln!(f, "    <compilation error>");
                }
                FnInstanceStage::Ok(instance) => instance,
            };

            writeln!(f, " -> {}", instance.return_type)?;

            // Variables
            for var in declaration.parameters.len()..instance.variables.len() {
                writeln!(f, "    v{var}: {:?}", instance.variables[var])?;
            }
        }

        let local_instances = self
            .local_instances_map
            .iter()
            .sorted_unstable_by_key(|(decl, _)| *decl);

        for (declaration, id) in local_instances {
            // Name & Parameters
            write!(f, "{}", declaration)?;

            // Return
            let instance = match &self.instances[**id] {
                FnInstanceStage::BeeingCreated => return writeln!(f, " -> <in progress...>"),
                FnInstanceStage::WithFatalErrors => return writeln!(f, " -> <compilation error>"),
                FnInstanceStage::WithErrors { return_type } => {
                    writeln!(f, " -> {return_type}")?;
                    return writeln!(f, "    <compilation error>");
                }
                FnInstanceStage::Ok(instance) => instance,
            };

            writeln!(f, " -> {}", instance.return_type)?;

            // Variables
            for var in declaration.parameters.len()..instance.variables.len() {
                writeln!(f, "    v{var}: {:?}", instance.variables[var])?;
            }
        }

        Ok(())
    }
}
