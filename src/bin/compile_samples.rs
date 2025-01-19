use makovi::*;
use std::{fs, path::Path};

fn main() {
    const CODE_DIRECTORY: &str = "code_samples";

    const SOURCE_EXTENSION: &str = ".rb";
    const AST_EXTENSION: &str = ".ast.run";
    const IR_EXTENSION: &str = ".ir.run";

    let entries = fs::read_dir(CODE_DIRECTORY).unwrap();

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();
        let path = path.to_str().unwrap();

        if !path.ends_with(SOURCE_EXTENSION) {
            continue;
        }

        let source = fs::read_to_string(path).unwrap();

        let ast_path = path.replace(SOURCE_EXTENSION, AST_EXTENSION);
        let ir_path = path.replace(SOURCE_EXTENSION, IR_EXTENSION);

        let mut jit = MakoviJit::<(), ()>::default();

        match jit.write_ast(&source) {
            Ok(ast) => update_content(ast_path, ast),
            Err(err) => {
                eprintln!("(file {path}) {err}");
                continue;
            }
        };
        match jit.write_ir(&source) {
            Ok(ir) => update_content(ir_path, ir),
            Err(err) => eprintln!("(file {path}) {err}"),
        };
    }
}

fn update_content<P: AsRef<Path>, C: AsRef<str>>(path: P, content: C) {
    let path = path.as_ref();
    let content = content.as_ref();

    let current_content = fs::read_to_string(path).unwrap_or_default();
    if current_content.trim() != content.trim() {
        println!("Updated {}", path.display());
        fs::write(path, content).unwrap();
    }
}
