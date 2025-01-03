macro_rules! gen_tests {
    (fn($b:ident, $code:ident, $test_name_var:ident) $test:block) => {
        gen_tests!{
            fn($b, $code, $test_name_var, input: In, output: Out) $test
        }
    };
    (fn($b:ident, $code:ident, $test_name_var:ident, $input:ident:$In:ident, $output:ident:$Out:ident) $test:block) => {
        gen_tests!{
            old_mult(2, 5) = 10;
            smallest_factor(53 * 59) = 53;
            #fn($b, $code, $test_name_var, $input:$In, $output:$Out) $test
        }
    };
    (
        $($test_name:ident($($params:expr),*) = $result:expr;)*
        #fn($b:ident, $code:ident, $test_name_var:ident, $input:ident:$In:ident, $output:ident:$Out:ident) $test:block
    ) => {

        $(
        #[bench]
        fn $test_name(b: &mut test::Bencher) {
            let name = stringify!($test_name);
            let code = &load_src(name, "");
            generic_test(b, code, name, ($($params),*), $result);
        }
        )*

        #[allow(unused_variables)]
        fn generic_test<$In: Clone, $Out: PartialEq + std::fmt::Debug>($b: &mut test::Bencher, $code: &str, $test_name_var: &str, $input: $In, $output: $Out) {
            $test
        }
    };
}

pub fn load_src(name: &str, sufix: &str) -> String {
    let path = &format!("code_samples/{name}{sufix}.run");
    std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Missing file: {}", path))
}

pub(crate) use gen_tests;
