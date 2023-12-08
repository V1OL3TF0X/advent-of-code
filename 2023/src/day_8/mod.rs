use rayon::prelude::*;
use std::{collections::HashMap, str::Lines};

use regex::{Captures, Regex};

pub fn task_1(file: &str) -> String {
    let mut lines = file.lines();
    let mut instructions = lines.next().unwrap().chars().cycle();
    lines.next().unwrap();
    let nodes = make_nodes(lines);
    let mut curr = "AAA";
    let mut count = 0;
    while curr != "ZZZ" {
        let instruction = instructions.next().unwrap();
        let opts = nodes.get(curr).unwrap();
        curr = match instruction {
            'L' => opts.0,
            'R' => opts.1,
            c => unreachable!("{c} is not a valid direction"),
        };
        count += 1;
    }
    count.to_string()
}

pub fn task_2(file: &str) -> String {
    let mut lines = file.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let instruction_num = instructions.len();
    lines.next().unwrap();
    let (curr, nodes) = make_nodes_2(lines);

    let shortest_z_cycle: Vec<_> = curr
        .par_iter()
        .map(|c| {
            let mut cycle_len = 0;
            let mut current_node: &str = c;
            while !current_node.ends_with('Z') {
                let opts = nodes.get(current_node).unwrap();
                current_node = match instructions[cycle_len % instruction_num] {
                    'L' => opts.0,
                    'R' => opts.1,
                    c => unreachable!("{c} is not a valid direction"),
                };
                cycle_len += 1;
            }
            cycle_len
        })
        .collect();
    shortest_z_cycle.into_iter().fold(1, lcm).to_string()
}

lazy_static::lazy_static! {
    static ref REG: Regex = Regex::new(r"(?<name>\w+) += +\((?<left>\w+), +(?<right>\w+)").unwrap();
}

fn get_captured<'a>(c: &Captures<'a>, name: &str) -> &'a str {
    c.name(name).unwrap().as_str()
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

fn make_nodes(lines: Lines<'_>) -> HashMap<&str, (&str, &str)> {
    lines.map(make_node).collect()
}

fn make_nodes_2(lines: Lines<'_>) -> (Vec<&str>, HashMap<&str, (&str, &str)>) {
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
