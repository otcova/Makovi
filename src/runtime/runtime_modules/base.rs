use crate::*;

#[derive(Default)]
pub struct BaseModule {
    //
}

impl RuntimeModule for BaseModule {
    fn symbols(&self) {
        // symbols.functions.push(FunctionSymbol {
        //     name: "print",
        //     parameters: &[FunctionParameter::Explicit(Type::I64)],
        //     ptr: Self::print as *const u8,
        // });
    }
}

// impl BaseModule {
//     extern "C" fn print(value: i64) {
//         println!("{value}");
//     }
// }
