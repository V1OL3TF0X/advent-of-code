use std::{fmt::Debug, process::Command, time::Instant};

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
