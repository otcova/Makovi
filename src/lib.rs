#![feature(never_type, map_try_insert, iterator_try_collect)]
#![cfg_attr(test, feature(test))]

mod compiler;
mod runtime;

#[cfg(test)]
mod test_utils;

pub use compiler::*;
pub use runtime::*;
