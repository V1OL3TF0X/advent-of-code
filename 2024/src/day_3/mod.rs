use regex::Regex;

pub fn task_1(file: &str) -> String {
    let regex = Regex::new(r"mul\((?<f>\d+),(?<s>\d+)\)").unwrap();
    regex
        .captures_iter(file)
        .map(|c| c["f"].parse::<u32>().unwrap() * c["s"].parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string()
}

pub fn task_2(file: &str) -> String {
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
