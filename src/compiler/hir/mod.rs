mod definition;
mod instance;
mod value;

pub use definition::*;
pub use instance::*;
pub use value::*;

pub struct CodeModule<'code> {
    pub definitions: ModuleDefinitions<'code>,
    pub instances: ModuleInstances,
}
