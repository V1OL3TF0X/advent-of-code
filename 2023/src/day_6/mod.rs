use std::str::{Lines, SplitWhitespace};

use regex::Regex;

pub fn task_1(file: &str) -> String {
    let (time_values, dist_values) = get_time_and_dist_values(file);
    let time_values = time_values.map(|n_str| n_str.parse::<u64>().expect(n_str));
    let dist_values = dist_values.map(|n_str| n_str.parse::<u64>().expect(n_str));
    time_values
        .zip(dist_values)
        .map(Race::from)
        .map(Race::into_run_count)
        .product::<u64>()
        .to_string()
}

pub fn task_2(file: &str) -> String {
    let (time_values, dist_values) = get_time_and_dist_values(file);
    let time = time_values
        .collect::<String>()
        .parse::<u64>()
        .expect("should be a num");
    let dist = dist_values
        .collect::<String>()
        .parse::<u64>()
        .expect("should be a num");
    std::iter::once(Race::new(time, dist))
        .map(Race::into_run_count)
        .product::<u64>()
        .to_string()
}

fn get_time_and_dist_values(file: &str) -> (SplitWhitespace<'_>, SplitWhitespace<'_>) {
    let line_reg = Regex::new(r"(?<name>\w+):\s+(?<values>\d+(?: +\d+)*)").unwrap();
    let mut lines = file.lines();
    (
        get_line(&mut lines, &line_reg),
        get_line(&mut lines, &line_reg),
    )
}

fn get_line<'a>(lines: &mut Lines<'a>, reg: &Regex) -> SplitWhitespace<'a> {
    let line = lines.next().unwrap();
    reg.captures(line)
        .expect(line)
        .name("values")
        .expect(line)
        .as_str()
        .split_whitespace()
}
#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    pub fn new(time: u64, dist: u64) -> Self {
        Self { time, dist }
    }
    pub fn delta(&self) -> f64 {
        ((self.time * self.time - 4 * self.dist) as f64).sqrt()
    }
    pub fn get_record_run_count(&self) -> u64 {
        // we have to find all x such that (t - x)*x > d
        // this means x ∈ ((t - Δ)/2, (t + Δ)/2)
        // where Δ = \sqrt{t^2 - 4d}
        let d = self.delta();
        let min_valid_x: f64 = (self.time as f64 - d) / 2.0;
        let max_valid_x: f64 = min_valid_x + d;
        // get rid of extra +-1, this is basically lt_max - gt_min + 1
        // where lt_max is first integer x < max_valid_x --> max_valid_x.ceil() - 1 (.floor() would be <=, .ceil() - 1 is <)
        //   and gt_min is first integer x > min_valid_x --> min_valid_x.floor() + 1 (.ceil() would be >=, .floor() + 1 is >)
        max_valid_x.ceil() as u64 - min_valid_x.floor() as u64 - 1
    }
    pub fn into_run_count(self) -> u64 {
        self.get_record_run_count()
    }
}

impl From<(u64, u64)> for Race {
    fn from((time, dist): (u64, u64)) -> Self {
        Race::new(time, dist)
    }
}
