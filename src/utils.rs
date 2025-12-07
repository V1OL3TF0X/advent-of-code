use std::{
    fmt::Debug,
    process::Command,
    time::{Duration, Instant},
};

pub fn get_input(year: usize, mod_name: &str) -> String {
    get_in(year, mod_name, "input")
}

fn get_in(year: usize, mod_name: &str, file_name: &str) -> String {
    let root: &'static str = env!("CARGO_MANIFEST_DIR");
    let file_path = std::path::Path::new(root)
        .join("src")
        .join(format!("y{year}"))
        .join(mod_name)
        .join(format!("{file_name}.txt"));
    std::fs::read_to_string(file_path).unwrap()
}

pub fn get_sample_input(year: usize, mod_name: &str) -> String {
    get_in(year, mod_name, "sample_input")
}

pub fn measure_elapsed<T>(f: impl FnOnce() -> T) -> (Duration, T) {
    let b = Instant::now();
    let r = f();
    (b.elapsed(), r)
}

pub fn to_nums<T: std::str::FromStr>(line: &str) -> Vec<T>
where
    T::Err: Debug,
{
    line.split(' ').map(|n| n.parse::<T>().expect(n)).collect()
}

pub fn clear_terminal_screen() {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).spawn()
    } else {
        // "clear" or "tput reset"
        Command::new("tput").arg("reset").spawn()
    };

    // Alternative solution:
    if result.is_err() {
        print!("{esc}c", esc = 27 as char);
    }
}
