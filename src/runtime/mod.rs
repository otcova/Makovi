mod runtime_modules;

use crate::hir::ExternalCode;
pub use runtime_modules::*;

pub trait RuntimeModule {
    fn declare(&self, definitions: &mut ExternalCode);
}
