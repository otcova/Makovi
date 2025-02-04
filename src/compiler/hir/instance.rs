use super::*;
use derive_more::derive::{Deref, DerefMut, Display, From};
use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Index;

/// A function usually only has 2-3 parameters. This means that allocating a Vec
/// only to store the types would be ridiculous overhead.
/// (ValueType size = 1 byte, Vec size = 24 bytes).
/// This struct stores the first N parameters in place, and in case of
/// some absurd function it allocates the necesary memory.
#[derive(From, Default, Deref, DerefMut, Clone, Display, Eq, PartialEq, Hash)]
#[display("({})", _0.iter().enumerate()
    .format_with(", ", |(i, ty), f| f(&format_args!("v{i}: {ty}"))))]
pub struct ParamsTypes(pub SmallVec<ValueType, 16>);

/// A collection of instanciated functions
#[derive(Default)]
pub struct ModuleInstances {
    instances: Vec<FnInstanceStage>,

    /// Value is none if the instance has been created but is beeing modifyied.
    instances_map: HashMap<FnInstanceDeclaration, FnInstanceId>,
}

#[derive(Eq, PartialEq, Hash, Display)]
#[display("fn {definition_id}{parameters}")]
pub struct FnInstanceDeclaration {
    pub definition_id: FnDefinitionId,
    pub parameters: ParamsTypes,
}

#[derive(Deref, Clone, Copy)]
pub struct FnInstanceId(usize);

pub struct FnInstance {
    parameters: usize,
    pub variables: Box<[ValueType]>,
    pub imported_instances: Vec<FnInstanceId>,
    pub return_type: ValueType,
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
}

pub enum NewFnInstanceResult<'inst, 'def, 'code> {
    // A function instance with the given declaration did not exists.
    New {
        id: FnInstanceId,
        instance: FnInstance,
        definition: &'def FnDefinition<'code>,
    },
    // The instance already exists
    Exists(&'inst FnInstanceStage),
    WrongArgumentCount {
        definition: &'def FnDefinition<'code>,
    },
    UndefinedFunction {
        name: &'code str,
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
    /// Will return None if an instance can't be created due to syntax errors
    pub fn new_instance<'inst, 'def, 'code>(
        &'inst mut self,
        definitions: &'def ModuleDefinitions<'code>,
        definition_id: FnDefinitionId,
        parameters: ParamsTypes,
    ) -> NewFnInstanceResult<'inst, 'def, 'code> {
        let definition = match &definitions[definition_id] {
            FnDefinitionStage::Defined(definition) => definition,
            FnDefinitionStage::ToDefine { name } => {
                return NewFnInstanceResult::UndefinedFunction { name };
            }
        };

        if definition.parameters != parameters.len() {
            return NewFnInstanceResult::WrongArgumentCount { definition };
        }

        let declaration = FnInstanceDeclaration {
            definition_id,
            parameters,
        };

        match self.instances_map.entry(declaration) {
            Entry::Vacant(entry) => {
                let mut variables =
                    vec![ValueType::Unknown; definition.variables()].into_boxed_slice();

                let arity = entry.key().parameters.len();
                variables[0..arity].copy_from_slice(&entry.key().parameters);

                let id = FnInstanceId(self.instances.len());
                self.instances.push(FnInstanceStage::BeeingCreated);
                entry.insert(id);

                let instance = FnInstance {
                    parameters: arity,
                    variables,
                    imported_instances: Vec::new(),
                    return_type: ValueType::Unknown,
                };
                NewFnInstanceResult::New {
                    id,
                    instance,
                    definition,
                }
            }
            Entry::Occupied(entry) => NewFnInstanceResult::Exists(&self.instances[**entry.get()]),
        }
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

    pub fn iter_declarations(
        &self,
    ) -> impl Iterator<Item = (FnInstanceId, &FnInstanceDeclaration)> {
        self.instances_map.iter().map(|(decl, id)| (*id, decl))
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

    pub fn get_entry_point_id(&self) -> Option<FnInstanceId> {
        let declaration = FnInstanceDeclaration {
            definition_id: FnDefinitionId::ENTRY_POINT,
            parameters: ParamsTypes::default(),
        };
        self.instances_map.get(&declaration).copied()
    }

    pub fn get_id(&self, name: FnDefinitionId, parameters: ParamsTypes) -> Option<FnInstanceId> {
        let declaration = FnInstanceDeclaration {
            definition_id: name,
            parameters,
        };
        self.instances_map.get(&declaration).copied()
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
        for (declaration, id) in &self.instances_map {
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
