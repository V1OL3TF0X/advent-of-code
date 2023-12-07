use std::time::Instant;

pub fn get_input(mod_name: &str) -> String {
    get_in(mod_name, "input")
}

fn get_in(mod_name: &str, file_name: &str) -> String {
    let file_path = format!(
        "{}\\src\\{mod_name}\\{file_name}.txt",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(file_path).unwrap()
}

pub fn get_sample_input(mod_name: &str) -> String {
    get_in(mod_name, "sample_input")
}

pub fn measure_elapsed<T>(f: impl FnOnce() -> T) -> T {
    let b = Instant::now();
    let r = f();
    let el = b.elapsed();
    print!("Time: {el:.2?} ");
    r
}
