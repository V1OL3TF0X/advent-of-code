use std::{collections::HashMap, iter, str::Lines};

use regex::{Captures, Regex};

use crate::utils::measure_elapsed;

type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

pub fn task_1(file: &str) -> String {
    measure_elapsed(|| {
        let mut lines = file.lines();
        let instructions = lines.next().unwrap().chars().cycle();
        lines.next().unwrap();
        let nodes = lines.map(make_node).collect();
        let mut curr = "AAA";
        (instructions
            .take_while(|instruction| {
                curr = make_step(&nodes, curr, *instruction);
                curr != "ZZZ"
            })
            .count()
            + 1)
        .to_string()
    })
}

pub fn task_2(file: &str) -> String {
    measure_elapsed(|| {
        let mut lines = file.lines();
        let instructions: Vec<char> = lines.next().unwrap().chars().collect();
        let instruction_num = instructions.len();
        lines.next().unwrap();
        let (curr, nodes) = make_nodes_2(lines);

        let shortest_z_cycle: Vec<_> = curr
            .into_iter()
            .map(|mut c| {
                iter::repeat(0..instruction_num)
                    .flatten()
                    .take_while(|ind| {
                        {
                            c = make_step(&nodes, c, instructions[*ind]);
                        }
                        c.as_bytes()[2] != b'Z'
                    })
                    .count()
                    + 1
            })
            .collect();
        shortest_z_cycle.into_iter().fold(1, lcm).to_string()
    })
}

lazy_static::lazy_static! {
    static ref REG: Regex = Regex::new(r"(?<name>\w+) += +\((?<left>\w+), +(?<right>\w+)").unwrap();
}

fn get_captured<'a>(c: &Captures<'a>, name: &str) -> &'a str {
    c.name(name).unwrap().as_str()
}

fn make_step<'a>(map: &NodeMap<'a>, current_node: &str, instruction: char) -> &'a str {
    let opts = map.get(current_node).unwrap();
    match instruction {
        'L' => opts.0,
        'R' => opts.1,
        c => unreachable!("{c} is not a valid direction"),
    }
}

fn make_node(line: &str) -> (&str, (&str, &str)) {
    let captured = REG.captures(line).expect(line);
    (
        get_captured(&captured, "name"),
        (
            get_captured(&captured, "left"),
            get_captured(&captured, "right"),
        ),
    )
}

fn make_nodes_2(lines: Lines<'_>) -> (Vec<&str>, NodeMap<'_>) {
    lines.fold(
        (Vec::new(), HashMap::new()),
        |(mut starting, mut map), line| {
            let n = make_node(line);
            if n.0.ends_with('A') {
                starting.push(n.0);
            }
            map.insert(n.0, n.1);
            (starting, map)
        },
    )
}

fn lcm(n1: usize, n2: usize) -> usize {
    n1 * n2 / gcd(n1, n2)
}

fn gcd(n1: usize, n2: usize) -> usize {
    if n2 == 0 {
        n1
    } else {
        gcd(n2, n1 % n2)
    }
}
