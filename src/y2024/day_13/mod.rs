use crate::task_fns::SolveMode;
use itertools::Itertools;
use regex::Regex;

fn solve(file: &str, modifier: f64) -> String {
    let button_regex = Regex::new(r"^Button (?:A|B): X\+(?<dx>\d+), Y\+(?<dy>\d+)$").unwrap();
    let prize_regex = Regex::new(r"^Prize: X=(?<x>\d+), Y=(?<y>\d+)$").unwrap();
    file.lines()
        .tuples()
        .map(|(a_s, b_s, p_s, _)| {
            let cap_a = button_regex.captures(a_s).unwrap();
            let cap_b = button_regex.captures(b_s).unwrap();
            let cap_p = prize_regex.captures(p_s).unwrap();
            let a_dx = cap_a["dx"].parse::<f64>().unwrap();
            let a_dy = cap_a["dy"].parse::<f64>().unwrap();
            let b_dx = cap_b["dx"].parse::<f64>().unwrap();
            let b_dy = cap_b["dy"].parse::<f64>().unwrap();
            let prize_x = cap_p["x"].parse::<f64>().unwrap() + modifier;
            let prize_y = cap_p["y"].parse::<f64>().unwrap() + modifier;
            let b_presses = (prize_y * a_dx - prize_x * a_dy) / (a_dx * b_dy - b_dx * a_dy);
            let a_presses = (prize_x - b_presses * b_dx) / a_dx;
            if a_presses.fract() >= 1e-16
                || b_presses.fract() >= 1e-16
                || a_presses < 0.0
                || b_presses < 0.0
            {
                return 0;
            }
            println!();
            (a_presses * 3.0 + b_presses) as u64
        })
        .sum::<u64>()
        .to_string()
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        solve(file, 0.0)
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        solve(file, 10000000000000.0)
    }
}
