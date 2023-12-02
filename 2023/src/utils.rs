pub fn get_input(mod_name: &str) -> String {
    let file_path = format!("{}\\src\\{mod_name}\\input.txt", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(file_path).unwrap()
}
