use std::iter;

macro_rules! gen_tests {
    ($generic_test:ident(bench, code, test_name)) => {
        gen_tests!(generic_test_wrapper(bench, code, test_name, input, output));

        fn generic_test_wrapper<In, Out>(b: &mut Bencher, code: &str, name: &str, _: In, _: Out) {
            $generic_test(b, code, name);
        }
    };
    ($generic_test:ident(bench, code, test_name, input, output)) => {
        gen_tests!{
            #$generic_test
            old_mult(2, 5) = 10;
            smallest_factor(53 * 59) = 53;
        }
    };
    (
        #$generic_test:ident
        $($test_name:ident($($params:expr),*) = $result:expr;)*
    ) => {
        $(
        #[bench]
        fn $test_name(b: &mut test::Bencher) {
            let name = stringify!($test_name);
            let code = &load_src(name, "");
            $generic_test(b, code, name, ($($params),*), $result);
        }
        )*
    };
}

pub(crate) use gen_tests;

pub fn load_src(name: &str, sufix: &str) -> String {
    let path = &format!("code_samples/{name}{sufix}.run");
    std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Missing file: {}", path))
}

pub fn assert_source_eq(expected: &str, actual: &str) {
    if expected.trim() != actual.trim() {
        println!("Expected:");
        println!("{expected}");
        println!();
        println!("But was:");
        println!("{}", diff_source(expected, actual));
        panic!();
    }
}

fn diff_source(expected: &str, parsed: &str) -> String {
    let expected = expected.trim().lines();
    let parsed = parsed.trim().lines();

    let mut result = Vec::new();
    for (l1, l2) in parsed.zip(expected.chain(iter::repeat(""))) {
        if l1 == l2 {
            result.push(l1.to_string());
        } else {
            result.push(format!("\x1b[0;31m{l1}\x1b[0m"));
        }
    }

    result.join("\n")
}
