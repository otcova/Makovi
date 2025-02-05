use derive_more::derive::{Deref, Display, From};

#[derive(Display, Clone, From)]
pub enum Variable {
    /// Used when a compilation error has prevented the compiler to know the type
    #[display("unknown")]
    Unknown,

    Id(VariableId),
    Const(Value),
}

#[derive(Debug, Display, Clone, PartialEq, Eq)]
pub enum Value {
    #[display("null")]
    Null,
    Int(i64),
    Bool(bool),
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Display, PartialOrd, Ord)]
pub enum ValueType {
    /// Used when a compilation error has prevented the compiler to know the type
    // TODO: Consider if this variant can be removed (not needed) when using SSA.
    Unknown,

    Null,
    Int,
    Bool,
}

#[derive(Clone, Copy, Deref, From, Debug, Display)]
#[display("v{_0}")]
pub struct VariableId(usize);

impl From<Option<VariableId>> for Variable {
    fn from(value: Option<VariableId>) -> Self {
        match value {
            Some(var) => Variable::Id(var),
            None => Variable::Unknown,
        }
    }
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Int(_) => ValueType::Int,
            Value::Bool(_) => ValueType::Bool,
            Value::Null => ValueType::Null,
        }
    }
}
