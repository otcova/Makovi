use super::*;
use derive_more::{Deref, DerefMut};
use std::collections::HashMap;
use std::ptr::NonNull;

#[derive(Default, Deref, DerefMut, Debug)]
pub struct ExternalDefinitions(HashMap<&'static str, FnId>);

#[derive(Default)]
pub struct ExternalInstances {
    instances: Vec<FnInstanceStage>,
    map: HashMap<ExternFnInstanceDeclaration, FnInstanceId>,
}

#[derive(Default)]
pub struct ExternalCode {
    pub definitions: ExternalDefinitions,
    pub instances: ExternalInstances,
}

impl ExternalCode {
    pub fn declare_function(
        &mut self,
        name: &'static str,
        parameters: ParamsTypes,
        return_type: ValueType,
        fn_ptr: NonNull<u8>,
    ) {
        let new_fn_id = ExternFnId::from(self.definitions.len());
        let fn_id = match self.definitions.entry(name) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                match *occupied_entry.get() {
                    FnId::Extern(fn_id) => fn_id,
                    FnId::Local(_) => unreachable!(),
                }
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(new_fn_id.into());
                new_fn_id
            }
        };

        let declaration = ExternFnInstanceDeclaration {
            fn_id,
            parameters: parameters.clone(),
        };

        match self.instances.map.entry(declaration) {
            std::collections::hash_map::Entry::Occupied(_) => {
                panic!("Duplicated definitions for the same function instance");
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let id = FnInstanceId(self.instances.instances.len());
                let instance = FnInstance {
                    id,
                    parameters: parameters.len(),
                    external_ptr: Some(fn_ptr),
                    return_type,
                    imported_instances: Vec::default(),
                    // TODO: Prevent this allocation
                    variables: Box::from(&parameters[..]),
                };
                self.instances.instances.push(FnInstanceStage::Ok(instance));
                vacant_entry.insert(id);
            }
        }
    }
}

impl ModuleDefinitions<'_> {
    pub fn new(definitions: ExternalDefinitions) -> Self {
        Self {
            definitions: Vec::default(),
            definitions_map: definitions.0,
        }
    }
}

impl ModuleInstances {
    pub fn new(instances: ExternalInstances) -> Self {
        Self {
            instances: instances.instances,
            extern_instances_map: instances.map,
            local_instances_map: Default::default(),
        }
    }
}

impl ExternalInstances {
    pub fn iter(&self) -> impl Iterator<Item = &FnInstance> {
        self.instances.iter().filter_map(|f| match f {
            FnInstanceStage::Ok(instance) => Some(instance),
            _ => None,
        })
    }
}
