use crate::hir::{ExternalCode, ParamsTypes, ValueType};
use crate::*;
use std::ptr::NonNull;

#[derive(Default)]
pub struct BaseModule {
    //
}

impl RuntimeModule for BaseModule {
    fn declare(&self, definitions: &mut ExternalCode) {
        definitions.declare_function(
            "*",
            ParamsTypes([ValueType::Int, ValueType::Int].into()),
            ValueType::Int,
            NonNull::new(Self::mul_int as *mut u8).unwrap(),
        );
        definitions.declare_function(
            "+",
            ParamsTypes([ValueType::Int, ValueType::Int].into()),
            ValueType::Int,
            NonNull::new(Self::add_int as *mut u8).unwrap(),
        );
        definitions.declare_function(
            "/",
            ParamsTypes([ValueType::Int, ValueType::Int].into()),
            ValueType::Int,
            NonNull::new(Self::div_int as *mut u8).unwrap(),
        );
        definitions.declare_function(
            "mod",
            ParamsTypes([ValueType::Int, ValueType::Int].into()),
            ValueType::Int,
            NonNull::new(Self::mod_int as *mut u8).unwrap(),
        );
        definitions.declare_function(
            "==",
            ParamsTypes([ValueType::Int, ValueType::Int].into()),
            ValueType::Bool,
            NonNull::new(Self::eq_int as *mut u8).unwrap(),
        );
        definitions.declare_function(
            "<",
            ParamsTypes([ValueType::Int, ValueType::Int].into()),
            ValueType::Bool,
            NonNull::new(Self::lt_int as *mut u8).unwrap(),
        );
    }
}

impl BaseModule {
    extern "C" fn mul_int(a: i64, b: i64) -> i64 {
        a * b
    }
    extern "C" fn add_int(a: i64, b: i64) -> i64 {
        a + b
    }
    extern "C" fn div_int(a: i64, b: i64) -> i64 {
        a / b
    }
    extern "C" fn mod_int(a: i64, b: i64) -> i64 {
        a % b
    }
    extern "C" fn eq_int(a: i64, b: i64) -> bool {
        a == b
    }
    extern "C" fn lt_int(a: i64, b: i64) -> bool {
        a < b
    }
}
