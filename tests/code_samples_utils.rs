pub fn load_function_source(fn_name: &str) -> String {
    let path = &format!("code_samples/{fn_name}.run");
    std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Missing file: {}", path))
}
