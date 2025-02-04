use itertools::Itertools;
use similar::{Algorithm, ChangeTag, TextDiffConfig};
use std::fmt::{Display, Write};
use std::iter::repeat;
//
// macro_rules! gen_tests {
//     ($generic_test:ident(bench, code, test_name)) => {
//         gen_tests!(generic_test_wrapper(bench, code, test_name, input, output));
//
//         fn generic_test_wrapper<In, Out>(b: &mut Bencher, code: &str, name: &str, _: In, _: Out) {
//             $generic_test(b, code, name);
//         }
//     };
//     ($generic_test:ident(bench, code, test_name, input, output)) => {
//         gen_tests!{
//             #$generic_test
//             multiply(132, 431321) = 132 * 431321;
//             smallest_factor(53 * 59) = 53;
//         }
//     };
//     (
//         #$generic_test:ident
//         $($test_name:ident($($params:expr),*) = $result:expr;)*
//     ) => {
//         $(
//         #[bench]
//         fn $test_name(b: &mut test::Bencher) {
//             let name = stringify!($test_name);
//             let code = &load_src(name, ".rb");
//             $generic_test(b, code, name, ($($params),*), $result);
//         }
//         )*
//     };
// }
//
// pub(crate) use gen_tests;

fn source_example_path(name: &str) -> String {
    format!("code_samples/{name}")
}

pub fn load_source(name: &str) -> String {
    let path = &source_example_path(name);
    std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Missing file: {}", path))
}

pub fn assert_source_eq<S: Display>(stored_file: &str, generated: S) {
    let expected = &load_source(stored_file);
    let actual = &format!("{}", generated);

    if expected == actual {
        return;
    }

    let mut args = std::env::args();
    let update_flag = args.any(|arg| arg == "update");
    let update_this = args.next().is_some_and(|arg| arg == stored_file);

    if update_flag && update_this {
        std::fs::write(source_example_path(stored_file), actual).unwrap();
    } else {
        eprintln!("{}", diff_source(expected, actual));
        eprintln!("To update the expected value run: 'cargo test -- update {stored_file:?}'");
        panic!();
    }
}

pub fn assert_diff<S1: Display, S2: Display>(expected: S1, actual: S2) {
    let expected = &format!("{}", expected);
    let actual = &format!("{}", actual);

    let expected = expected.trim();
    let actual = actual.trim();

    if expected != actual {
        eprintln!("{}", diff_source(expected, actual));
        panic!();
    }
}

fn diff_source(expected: &str, actual: &str) -> String {
    let width = expected.lines().map(|s| s.len()).max().unwrap_or_default();
    let width = width.max(10);

    let mut result = String::new();
    writeln!(result, "{:<width$} ║ But was", "Expected").unwrap();
    let bar = std::iter::repeat_n('═', width).format("");
    writeln!(result, "{}═╬═{}", bar.clone(), bar).unwrap();

    const CHANGE_MARGIN: isize = 6;
    let diff: Vec<_> = TextDiffConfig::default()
        .algorithm(Algorithm::Patience)
        .diff_lines(expected, actual)
        .iter_all_changes()
        .collect();

    let mut distance = -2 * CHANGE_MARGIN;
    let mut distance_to_change = diff
        .iter()
        .map(|change| change.tag())
        .chain(repeat(ChangeTag::Equal))
        .map(move |tag| {
            if tag != ChangeTag::Equal {
                distance = CHANGE_MARGIN;
            } else {
                distance -= 1;
            }
            distance
        });

    distance_to_change.nth(CHANGE_MARGIN as usize - 1);

    let mut previous_line_displayed = false;
    let mut is_first_line = true;

    for change in &diff {
        let display_line = distance_to_change.next().unwrap() >= -CHANGE_MARGIN;

        if previous_line_displayed != display_line && !is_first_line {
            writeln!(result, "{0:<width$} ║ {0}", "...").unwrap();
        }

        if display_line {
            let line = change.as_str().unwrap_or_default().trim_end();

            // Ansi color codes
            const RED: &str = "\x1b[0;31m";
            const GREEN: &str = "\x1b[0;32m";
            const RESET: &str = "\x1b[0m";

            match change.tag() {
                ChangeTag::Delete => {
                    writeln!(result, "{RED}{:<width$}{RESET} ║", line).unwrap();
                }
                ChangeTag::Insert => {
                    writeln!(result, "{:<width$} ║ {GREEN}{}{RESET}", "", line).unwrap();
                }
                ChangeTag::Equal => {
                    writeln!(result, "{0:<width$} ║ {0}", line).unwrap();
                }
            }
        }

        previous_line_displayed = display_line;
        is_first_line = false;
    }

    result
}
