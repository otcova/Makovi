// #[derive(Default)]
// pub struct Symbols {
//     pub functions: Vec<FunctionSymbol>,
// }
//
// pub struct FunctionSymbol {
//     pub name: &'static str,
//     pub parameters: &'static [FunctionParameter],
//     pub ptr: *const u8,
// }
//
// pub enum FunctionParameter {
//     Implicit(*const u8),
//     Explicit(Type),
// }
//
// pub enum Type {
//     I64,
// }
//
// impl Symbols {
//     pub(super) fn raw_pointers(&self) -> impl Iterator<Item = (String, *const u8)> + '_ {
//         self.functions.iter().map(|f| (f.symbol_name(), f.ptr))
//     }
// }
//
// impl FunctionSymbol {
//     pub fn symbol_name(&self) -> String {
//         format!("extern_fn_{}", self.name)
//     }
// }
