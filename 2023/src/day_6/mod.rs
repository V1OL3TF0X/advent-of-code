use std::str::{Lines, SplitWhitespace};

use regex::Regex;

pub fn task_1(file: &str) -> String {
    let (time_values, dist_values) = get_time_and_dist_values(file);
    let time_values = time_values.map(|n_str| n_str.parse::<u64>().expect(n_str));
    let dist_values = dist_values.map(|n_str| n_str.parse::<u64>().expect(n_str));
    count_prod(time_values.zip(dist_values).map(Race::from)).to_string()
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
    count_prod(std::iter::once(Race::new(time, dist))).to_string()
}

fn get_time_and_dist_values(file: &str) -> (SplitWhitespace<'_>, SplitWhitespace<'_>) {
    let line_reg = Regex::new(r"(?<name>\w+):\s+(?<values>\d+(?: +\d+)*)").unwrap();
    let mut lines = file.lines();
    (
        get_line(&mut lines, &line_reg),
        get_line(&mut lines, &line_reg),
    )
}

fn count_prod(races: impl Iterator<Item = Race>) -> u64 {
    races
        .map(|race| {
            let d = race.delta();
            let min = ((race.time as f64 - d) / 2.0).floor() as u64 + 1;
            let max = ((race.time as f64 + d) / 2.0).ceil() as u64 - 1;
            max - min + 1
        })
        .product()
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
}

impl From<(u64, u64)> for Race {
    fn from((time, dist): (u64, u64)) -> Self {
        Race::new(time, dist)
    }
}
