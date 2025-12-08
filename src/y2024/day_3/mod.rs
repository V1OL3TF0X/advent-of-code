use crate::task_fns::SolveMode;
use regex::Regex;

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        let regex = Regex::new(r"mul\((?<f>\d+),(?<s>\d+)\)").unwrap();
        regex
            .captures_iter(file)
            .map(|c| c["f"].parse::<u32>().unwrap() * c["s"].parse::<u32>().unwrap())
            .sum::<u32>()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        let regex =
            Regex::new(r"(mul\((?<f>\d+),(?<s>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
        let mut enabled = true;
        regex
            .captures_iter(file)
            .fold(0, |sum, c| {
                if !enabled {
                    if c.name("do").is_some() {
                        enabled = true;
                    }
                    return sum;
                }
                if c.name("dont").is_some() {
                    enabled = false;
                    return sum;
                }
                if c.name("do").is_some() {
                    return sum;
                }
                sum + c["f"].parse::<u32>().unwrap() * c["s"].parse::<u32>().unwrap()
            })
            .to_string()
    }
}
