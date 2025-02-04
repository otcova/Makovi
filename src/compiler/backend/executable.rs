use super::*;

pub struct Executable {
    entry_point: FuncId,
    return_type: hir::ValueType,
}

impl Executable {
    pub(super) fn new(entry_point: FuncId, return_type: hir::ValueType) -> Executable {
        Executable {
            entry_point,
            return_type,
        }
    }
}

impl BackendCompiler {
    /// TODO: Provide a way to release the lifetime of the entry point
    pub fn run(&self, executable: &Executable) -> hir::Value {
        let ptr = self.module.get_finalized_function(executable.entry_point);

        match executable.return_type {
            hir::ValueType::Unknown => hir::Value::Null,
            hir::ValueType::Null => {
                let entry = unsafe { std::mem::transmute::<*const u8, fn()>(ptr) };
                entry();
                hir::Value::Null
            }
            hir::ValueType::Int => {
                let entry = unsafe { std::mem::transmute::<*const u8, fn() -> i64>(ptr) };
                hir::Value::Int(entry())
            }
            hir::ValueType::Bool => {
                let entry = unsafe { std::mem::transmute::<*const u8, fn() -> i8>(ptr) };
                hir::Value::Bool(entry() != 0)
            }
        }
    }
}
