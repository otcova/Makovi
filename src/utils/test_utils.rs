use std::fmt::Write;

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
            multiply(132, 431321) = 132 * 431321;
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
            let code = &load_src(name, ".rb");
            $generic_test(b, code, name, ($($params),*), $result);
        }
        )*
    };
}

pub(crate) use gen_tests;

pub fn load_src(name: &str, sufix: &str) -> String {
    let path = &format!("code_samples/{name}{sufix}");
    std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Missing file: {}", path))
}

pub fn assert_source_eq(expected: &str, actual: &str) {
    if expected.trim() != actual.trim() {
        println!("{}", diff_source(expected, actual));
        panic!();
    }
}

fn diff_source(expected: &str, actual: &str) -> String {
    let expected = expected.trim();
    let actual = actual.trim();

    let (lines, width) = expected.lines().fold((0, 0), |(lines, width), line| {
        (lines + 1, width.max(line.len()))
    });

    let mut expected = expected.trim().lines();
    let mut actual = actual.trim().lines();

    let mut result = String::with_capacity((width * 2 + 4) * (lines + 2));
    writeln!(result, "{:<width$} ║ But was", "Expected").unwrap();
    for _ in 0..width {
        result.push('═');
    }
    result.push_str("═╬═");
    for _ in 0..width {
        result.push('═');
    }
    result.push('\n');

    loop {
        let (l1, l2) = match (expected.next(), actual.next()) {
            (Some(l1), Some(l2)) => (l1, l2),
            (None, Some(l2)) => ("", l2),
            (Some(l1), None) => (l1, ""),
            (None, None) => break,
        };

        if l1 == l2 {
            writeln!(&mut result, "{l1:<width$} ║ {l2}").unwrap();
        } else {
            writeln!(&mut result, "{l1:<width$} ║ \x1b[0;31m{l2}\x1b[0m").unwrap();
        }
    }

    result
}
