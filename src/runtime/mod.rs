mod runtime_modules;

pub use runtime_modules::*;

pub trait RuntimeModule {
    fn symbols(&self);
}
