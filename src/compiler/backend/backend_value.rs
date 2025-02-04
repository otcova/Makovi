use super::*;
use arrayvec::ArrayVec;

#[derive(Clone, Copy)]
pub enum BackendValueType {
    Null,
    Primitive(Type),
}

#[derive(Clone, Copy)]
pub enum BackendVariable {
    Null,
    Int(Variable),
    Bool(Variable),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BackendValue {
    Null,
    Int(Value),
    Bool(Value),
}

pub const INT_TYPE: Type = types::I64;
pub const BOOL_TYPE: Type = types::I8;

impl From<hir::ValueType> for BackendValueType {
    fn from(value: hir::ValueType) -> BackendValueType {
        match value {
            hir::ValueType::Unknown => {
                unreachable!("The backend should only work with correct code")
            }
            hir::ValueType::Null => BackendValueType::Null,
            hir::ValueType::Int => BackendValueType::Primitive(INT_TYPE),
            hir::ValueType::Bool => BackendValueType::Primitive(BOOL_TYPE),
        }
    }
}

impl BackendValueType {
    pub fn types(&self) -> ArrayVec<Type, 1> {
        match *self {
            BackendValueType::Null => Default::default(),
            BackendValueType::Primitive(ty) => [ty].into(),
        }
    }
}

impl BackendVariable {
    pub fn variables(&self) -> ArrayVec<Variable, 1> {
        match *self {
            BackendVariable::Null => Default::default(),
            BackendVariable::Int(var) => [var].into(),
            BackendVariable::Bool(var) => [var].into(),
        }
    }
}

impl BackendValue {
    pub fn values(&self) -> ArrayVec<Value, 1> {
        match *self {
            BackendValue::Null => Default::default(),
            BackendValue::Int(val) => [val].into(),
            BackendValue::Bool(val) => [val].into(),
        }
    }
}

impl BackendValue {
    pub fn expect_null(&self) {
        match self {
            Self::Null => {}
            _ => panic!("Expected null, found: {:?}", self),
        }
    }

    pub fn expect_int(&self) -> Value {
        match self {
            Self::Int(value) => *value,
            _ => panic!("Expected int, found: {:?}", self),
        }
    }

    pub fn expect_bool(&self) -> Value {
        match self {
            Self::Bool(value) => *value,
            _ => panic!("Expected Primitive Value, found: {:?}", self),
        }
    }
}
