// #![feature(concat_idents)]
//
// mod code_samples_utils;
// use code_samples_utils::*;
// use makovi::MakoviJIT;
// use std::fmt::Debug;
//
// macro_rules! gen_tests {
//     ($($name:ident($($arg:expr),*) = $result:expr;)*) => {$(
//         #[test]
//         fn $name() {
//             let source = load_function_source(stringify!($name));
//             test_function(&source, ($($arg),*), $result);
//         }
//     )*};
// }
//
// gen_tests! {
//     //multiply(2, 5) = 10;
//     old_mult(2, 5) = 10;
//     smallest_factor(53 * 59) = 53;
// }
//
// fn test_function<I, O: Eq + Debug>(fn_source: &str, input: I, expected_output: O) {
//     let mut jit = MakoviJIT::<I, O>::default();
//     jit.load_function(fn_source).unwrap();
//     assert_eq!(expected_output, jit.run_code(input));
// }
